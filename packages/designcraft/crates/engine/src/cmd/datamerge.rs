//! Data Merge (Window → Utilities → Data Merge): a template spread with `<<Field>>` placeholders
//! in its stories is duplicated once per CSV record and the placeholders are replaced. One undo step.

use designcraft_doc::{Content, ItemId, SpreadRef};
use designcraft_geom::Vec2;
use serde_json::{Value, json};

use super::{CommandSpec, bad, cmd, has_doc, str_param};
use crate::{Result, Session};

pub fn specs() -> Vec<CommandSpec> {
    vec![
        cmd!(
            "data.merge",
            "Create Merged Document…",
            ["Window", "Utilities", "Data Merge"],
            None,
            "{csv (header row first) | rows: [{field: value}], spread?: template spread index (default 0)} → {records, pages}",
            has_doc,
            merge
        ),
        cmd!(query "data.fields", "Data Merge Fields", [], None, "{csv} → field names", has_doc, |_, p| {
            let rows = parse_csv(str_param(p, "csv").unwrap_or(""));
            Ok(json!(rows.first().cloned().unwrap_or_default()))
        }),
    ]
}

/// RFC 4180-ish CSV: quoted fields, doubled quotes, CRLF/LF.
pub fn parse_csv(s: &str) -> Vec<Vec<String>> {
    let mut rows = Vec::new();
    let mut row = Vec::new();
    let mut field = String::new();
    let mut quoted = false;
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        if quoted {
            match c {
                '"' if chars.peek() == Some(&'"') => {
                    field.push('"');
                    chars.next();
                }
                '"' => quoted = false,
                c => field.push(c),
            }
            continue;
        }
        match c {
            '"' if field.is_empty() => quoted = true,
            ',' => row.push(std::mem::take(&mut field)),
            '\r' => {}
            '\n' => {
                row.push(std::mem::take(&mut field));
                if !(row.len() == 1 && row[0].is_empty()) {
                    rows.push(std::mem::take(&mut row));
                } else {
                    row.clear();
                }
            }
            c => field.push(c),
        }
    }
    if !field.is_empty() || !row.is_empty() {
        row.push(field);
        rows.push(row);
    }
    rows
}

fn merge(s: &mut Session, p: &Value) -> Result<Value> {
    let records: Vec<Vec<(String, String)>> = if let Some(rows) = p.get("rows").and_then(Value::as_array) {
        rows.iter()
            .map(|r| {
                r.as_object()
                    .map(|o| o.iter().map(|(k, v)| (k.clone(), v.as_str().map(str::to_string).unwrap_or_else(|| v.to_string()))).collect())
                    .unwrap_or_default()
            })
            .collect()
    } else {
        let rows = parse_csv(str_param(p, "csv").ok_or_else(|| bad("data.merge", "give `csv` or `rows`"))?);
        let Some((head, body)) = rows.split_first() else { return Err(bad("data.merge", "empty CSV")) };
        body.iter().map(|r| head.iter().cloned().zip(r.iter().cloned().chain(std::iter::repeat(String::new()))).collect()).collect()
    };
    if records.is_empty() {
        return Err(bad("data.merge", "no records"));
    }
    let si = p.get("spread").and_then(Value::as_u64).unwrap_or(0) as usize;
    s.edit(|d, sel| {
        let template = d.spreads.get(si).cloned().ok_or_else(|| bad("data.merge", "no such spread"))?;
        let tpl_ids: Vec<ItemId> = template.items.iter().map(|i| i.id).collect();
        let parent = template.pages.first().and_then(|p| p.parent);
        let src = d.clone();
        // Spreads to fill: the template itself for record 0, then one copy per further record.
        let mut targets: Vec<Vec<ItemId>> = vec![tpl_ids.clone()];
        for _ in 1..records.len() {
            let after = d.page_count() - 1;
            d.insert_pages(Some(after), template.pages.len(), parent)?;
            let (nsi, _) = d.page_loc(after + 1).ok_or_else(|| bad("data.merge", "insert failed"))?;
            let ids = super::object::duplicate_from(d, &src, &tpl_ids, SpreadRef::Doc(nsi), Vec2::ZERO)?;
            targets.push(ids);
        }
        for (rec, ids) in records.iter().zip(&targets) {
            let mut stories = Vec::new();
            for id in ids {
                if let Some(it) = d.item(*id) {
                    it.walk(&mut |i| {
                        if let Content::Text(tf) = &i.content
                            && !stories.contains(&tf.story)
                        {
                            stories.push(tf.story);
                        }
                    });
                }
            }
            for sid in stories {
                let Some(st) = d.story_mut(sid) else { continue };
                for (k, v) in rec {
                    let pat = format!("<<{k}>>");
                    while let Some(pos) = st.text.find(&pat) {
                        st.replace(pos..pos + pat.len(), v);
                    }
                }
            }
        }
        *sel = Default::default();
        Ok(json!({"records": records.len(), "pages": d.page_count()}))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn csv_parsing() {
        let r = parse_csv("Name,Title\n\"Ada, Countess\",\"Analyst \"\"No. 1\"\"\"\r\nGrace,Admiral\n");
        assert_eq!(r, vec![vec!["Name", "Title"], vec!["Ada, Countess", "Analyst \"No. 1\""], vec!["Grace", "Admiral"]]);
    }

    #[test]
    fn merge_duplicates_pages_and_fills_fields() {
        let mut s = Session::new();
        s.execute("file.new", &json!({"facingPages": false})).unwrap();
        s.execute("frame.create", &json!({"rect": [36, 36, 400, 100], "content": "text", "text": "Dear <<Name>>, welcome aboard as <<Title>>."}))
            .unwrap();
        let r = s.execute("data.merge", &json!({"csv": "Name,Title\nAda,Analyst\nGrace,Admiral\nKatherine,Mathematician\n"})).unwrap();
        assert_eq!(r["records"], 3);
        assert_eq!(r["pages"], 3);
        let d = &s.doc().unwrap().doc;
        let mut texts: Vec<String> = d.stories.values().map(|st| st.text.clone()).collect();
        texts.sort();
        assert_eq!(
            texts,
            vec![
                "Dear Ada, welcome aboard as Analyst.".to_string(),
                "Dear Grace, welcome aboard as Admiral.".to_string(),
                "Dear Katherine, welcome aboard as Mathematician.".to_string(),
            ]
        );
        assert_eq!(s.doc().unwrap().history.undo.len(), 2); // create frame + merge
    }
}
