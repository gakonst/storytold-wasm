//! Excel workbooks (`.xlsx`) → a story holding one table: the first worksheet's used range, cell
//! values as shown text (shared and inline strings, numbers, booleans).

use designcraft_doc::{CharFormat, ParaFormat, Story, StoryId, Table};

use crate::docx::{El, parse, part};
use crate::{ImportError, Imported};

/// "B12" → (row 11, column 1).
fn cell_ref(r: &str) -> Option<(usize, usize)> {
    let letters: String = r.chars().take_while(|c| c.is_ascii_alphabetic()).collect();
    let row: usize = r[letters.len()..].parse().ok()?;
    let col = letters.chars().try_fold(0usize, |acc, c| {
        let digit = (c.to_ascii_uppercase() as u8).checked_sub(b'A')? as usize + 1;
        acc.checked_mul(26)?.checked_add(digit)
    })?;
    Some((row.checked_sub(1)?, col.checked_sub(1)?))
}

/// The worksheet part of the first sheet (workbook order), via the workbook relationships.
fn first_sheet(zip: &mut zip::ZipArchive<std::io::Cursor<&[u8]>>) -> Option<String> {
    let wb = parse(&part(zip, "xl/workbook.xml")?).ok()?;
    let rid = wb.child("sheets")?.els().find(|e| e.name == "sheet")?.attr("id")?.to_string();
    let rels = parse(&part(zip, "xl/_rels/workbook.xml.rels")?).ok()?;
    let target = rels.els().find(|r| r.attr("Id") == Some(rid.as_str()))?.attr("Target")?.trim_start_matches('/').to_string();
    Some(if target.starts_with("xl/") { target } else { format!("xl/{target}") })
}

fn value(c: &El, shared: &[String]) -> String {
    let v = || c.child("v").map(El::text).unwrap_or_default();
    match c.attr("t") {
        Some("s") => v().trim().parse::<usize>().ok().and_then(|i| shared.get(i).cloned()).unwrap_or_default(),
        Some("inlineStr") => c.child("is").map(El::text).unwrap_or_default(),
        Some("b") => {
            if v().trim() == "1" {
                "TRUE".into()
            } else {
                "FALSE".into()
            }
        }
        _ => {
            let raw = v();
            // Whole numbers without the trailing ".0" Excel never shows.
            match raw.trim().parse::<f64>() {
                Ok(f) if f.fract() == 0.0 && f.abs() < 1e15 => format!("{}", f as i64),
                Ok(f) => format!("{}", (f * 1e10).round() / 1e10),
                Err(_) => raw,
            }
        }
    }
}

pub fn import(bytes: &[u8]) -> Result<Imported, ImportError> {
    let mut zip = zip::ZipArchive::new(std::io::Cursor::new(bytes)).map_err(|e| ImportError::Corrupt(e.to_string()))?;
    let sheet_part = first_sheet(&mut zip).unwrap_or_else(|| "xl/worksheets/sheet1.xml".into());
    let sheet = parse(&part(&mut zip, &sheet_part).ok_or_else(|| ImportError::Corrupt(format!("no {sheet_part} (not an Excel workbook)")))?)?;
    let shared: Vec<String> = part(&mut zip, "xl/sharedStrings.xml")
        .map(|x| parse(&x))
        .transpose()?
        .map(|sst| sst.els().filter(|e| e.name == "si").map(El::text).collect())
        .unwrap_or_default();
    let mut cells: Vec<((usize, usize), String)> = Vec::new();
    if let Some(data) = sheet.child("sheetData") {
        for (ri, row) in data.els().filter(|e| e.name == "row").enumerate() {
            for (ci, c) in row.els().filter(|e| e.name == "c").enumerate() {
                let at = c.attr("r").and_then(cell_ref).unwrap_or((ri, ci));
                let v = value(c, &shared);
                if !v.is_empty() {
                    cells.push((at, v));
                }
            }
        }
    }
    let mut warnings = Vec::new();
    if cells.is_empty() {
        warnings.push("the worksheet is empty".into());
    }
    // The used range, from the first used row/column. Cell references come from the file: the
    // table is capped before anything is allocated for it.
    const MAX_CELLS: usize = 100_000;
    const MAX_COLS: usize = 1_000;
    let (r0, c0) = cells.iter().fold((usize::MAX, usize::MAX), |(r, c), ((a, b), _)| (r.min(*a), c.min(*b)));
    let (r1, c1) = cells.iter().fold((0, 0), |(r, c), ((a, b), _)| (r.max(*a), c.max(*b)));
    let (nrows, ncols) = if cells.is_empty() { (1, 1) } else { ((r1 - r0).saturating_add(1), (c1 - c0).saturating_add(1)) };
    let ncols_kept = ncols.min(MAX_COLS);
    if ncols_kept < ncols {
        warnings.push(format!("only the first {ncols_kept} columns were placed"));
    }
    let nrows_kept = nrows.min((MAX_CELLS / ncols_kept).max(1));
    if nrows_kept < nrows {
        warnings.push(format!("only the first {nrows_kept} rows were placed"));
    }
    let mut rows: Vec<Vec<String>> = vec![vec![String::new(); ncols_kept]; nrows_kept];
    for ((r, c), v) in cells {
        if let Some(cell) = rows.get_mut(r - r0).and_then(|row| row.get_mut(c - c0)) {
            *cell = v;
        }
    }
    let mut story = Story::new(StoryId(0));
    let width = 72.0 * ncols_kept as f64;
    let t = Table::from_strings(1, &rows, width, &ParaFormat::default(), &CharFormat::default());
    story.insert_table(0, t);
    Ok(Imported { story, para_styles: vec![], char_styles: vec![], warnings })
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use super::*;

    fn workbook() -> Vec<u8> {
        workbook_with(
            r#"<worksheet><sheetData><row r="2"><c r="B2" t="s"><v>0</v></c><c r="C2" t="s"><v>1</v></c></row><row r="3"><c r="B3" t="s"><v>2</v></c><c r="C3"><v>3.5</v></c></row><row r="4"><c r="B4" t="inlineStr"><is><t>Cake</t></is></c><c r="C4"><v>12.0</v></c></row></sheetData></worksheet>"#,
        )
    }

    fn workbook_with(sheet: &str) -> Vec<u8> {
        let mut buf = std::io::Cursor::new(Vec::new());
        {
            let mut z = zip::ZipWriter::new(&mut buf);
            let o = zip::write::SimpleFileOptions::default();
            for (name, body) in [
                (
                    "xl/workbook.xml",
                    r#"<workbook xmlns="http://schemas.openxmlformats.org/spreadsheetml/2006/main" xmlns:r="http://schemas.openxmlformats.org/officeDocument/2006/relationships"><sheets><sheet name="Prices" sheetId="1" r:id="rId1"/></sheets></workbook>"#,
                ),
                (
                    "xl/_rels/workbook.xml.rels",
                    r#"<Relationships xmlns="http://schemas.openxmlformats.org/package/2006/relationships"><Relationship Id="rId1" Type="x" Target="worksheets/sheet1.xml"/></Relationships>"#,
                ),
                ("xl/sharedStrings.xml", r#"<sst><si><t>Item</t></si><si><t>Price</t></si><si><r><t>Te</t></r><r><t>a</t></r></si></sst>"#),
                ("xl/worksheets/sheet1.xml", sheet),
            ] {
                z.start_file(name, o).unwrap();
                z.write_all(body.as_bytes()).unwrap();
            }
            z.finish().unwrap();
        }
        buf.into_inner()
    }

    #[test]
    fn first_sheet_used_range_becomes_a_table() {
        let imp = import(&workbook()).unwrap();
        let t = imp.story.tables.values().next().expect("a table");
        assert_eq!((t.nrows(), t.ncols()), (3, 2), "the used range B2:C4");
        let cell = |r, c| t.cell(r, c).unwrap().text.text.clone();
        assert_eq!([cell(0, 0), cell(0, 1), cell(1, 0), cell(1, 1), cell(2, 0), cell(2, 1)], ["Item", "Price", "Tea", "3.5", "Cake", "12"]);
        assert_eq!(cell_ref("AA10"), Some((9, 26)));
    }

    /// A cell far away from the others used to allocate the whole used range (billions of cells)
    /// before the cell cap applied; a long column name overflowed the column number.
    #[test]
    fn far_away_cell_references_are_capped() {
        let wb = workbook_with(
            r#"<worksheet><sheetData><row><c r="A1" t="inlineStr"><is><t>a</t></is></c><c r="A999999999" t="inlineStr"><is><t>z</t></is></c><c r="XFDXFDXFDXFDXFDXFD1" t="inlineStr"><is><t>far</t></is></c></row></sheetData></worksheet>"#,
        );
        let imp = import(&wb).unwrap();
        let t = imp.story.tables.values().next().expect("a table");
        assert!(t.nrows() * t.ncols() <= 100_000);
        assert!(!imp.warnings.is_empty());
        assert_eq!(cell_ref("XFDXFDXFDXFDXFDXFD1"), None);
    }
}
