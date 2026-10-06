//! PDF/X-4: the output intent (our Generic CMYK profile), the PDF/X version in the document
//! information and XMP, written as an incremental update of krilla's PDF 1.6 output; and a
//! check of the requirements this exporter is responsible for.

use crate::transitions::{find, int_after, object, rfind};

/// Escape a PDF literal string.
fn lit(s: &str) -> String {
    let mut o = String::from("(");
    for c in s.chars() {
        if matches!(c, '(' | ')' | '\\') {
            o.push('\\');
        }
        o.push(c);
    }
    o.push(')');
    o
}

/// Turn krilla's output into PDF/X-4. `None` when the file isn't the simple kind this
/// understands (a classic cross-reference table and plain dictionaries).
pub fn make_pdfx4(pdf: &[u8], title: &str) -> Option<Vec<u8>> {
    let t_at = rfind(pdf, b"trailer")?;
    let trailer = String::from_utf8_lossy(&pdf[t_at..]).to_string();
    let size = int_after(&trailer, "/Size")?;
    let root = int_after(&trailer, "/Root")?;
    let prev = int_after(&trailer, "startxref")?;
    let info = int_after(&trailer, "/Info");
    let id = trailer.find("/ID").and_then(|i| trailer[i..].find(']').map(|j| trailer[i..i + j + 1].to_string()));
    let cat = String::from_utf8_lossy(&pdf[object(pdf, root)?]).to_string();
    let mut out = pdf.to_vec();
    if !out.ends_with(b"\n") {
        out.push(b'\n');
    }
    let mut entries: Vec<(usize, usize)> = Vec::new();
    let mut put = |out: &mut Vec<u8>, id: usize, body: &[u8]| {
        entries.push((id, out.len()));
        out.extend_from_slice(format!("{id} 0 obj\n").as_bytes());
        out.extend_from_slice(body);
        out.extend_from_slice(b"\nendobj\n");
    };
    // The output profile and the output intent.
    let icc = designcraft_color::cms::icc_out::generic_cmyk_icc();
    let (icc_id, oi_id) = (size, size + 1);
    let mut s = format!("<</N 4/Length {}>>\nstream\n", icc.len()).into_bytes();
    s.extend_from_slice(icc);
    s.extend_from_slice(b"\nendstream");
    put(&mut out, icc_id, &s);
    let cond = designcraft_color::cms::GENERIC_CMYK;
    put(
        &mut out,
        oi_id,
        format!(
            "<</Type/OutputIntent/S/GTS_PDFX/OutputConditionIdentifier{}/OutputCondition{}/Info{}/RegistryName(http://www.color.org)/DestOutputProfile {icc_id} 0 R>>",
            lit("Custom"),
            lit(cond),
            lit(cond)
        )
        .as_bytes(),
    );
    // The catalog with the intent.
    let close = cat.rfind(">>")?;
    let cat_new = format!("{}/OutputIntents[{oi_id} 0 R]{}", cat[..close].trim(), &cat[close..]);
    put(&mut out, root, cat_new.trim().as_bytes());
    // Document information: the PDF/X version and trapping state.
    let mut next = size + 2;
    let info_id = match info {
        Some(i) => {
            let body = String::from_utf8_lossy(&pdf[object(pdf, i)?]).to_string();
            let close = body.rfind(">>")?;
            put(&mut out, i, format!("{}/GTS_PDFXVersion(PDF/X-4)/Trapped/False{}", body[..close].trim(), &body[close..]).trim().as_bytes());
            i
        }
        None => {
            let i = next;
            next += 1;
            put(&mut out, i, format!("<</Title{}/GTS_PDFXVersion(PDF/X-4)/Trapped/False>>", lit(title)).as_bytes());
            i
        }
    };
    // XMP: the PDF/X identification schema.
    if let Some(meta) = int_after(&cat, "/Metadata") {
        let r = object(pdf, meta)?;
        let body = &pdf[r];
        let s0 = find(body, b"stream", 0)? + b"stream".len();
        let s0 = s0 + body[s0..].iter().take_while(|c| **c == b'\r' || **c == b'\n').count();
        let s1 = rfind(body, b"endstream")?;
        let xml = String::from_utf8_lossy(&body[s0..s1]).trim_end().to_string();
        let dict = String::from_utf8_lossy(&body[..find(body, b"stream", 0)?]).to_string();
        if dict.contains("/Filter") {
            return None;
        }
        let add = "<rdf:Description rdf:about=\"\" xmlns:pdfxid=\"http://www.npes.org/pdfx/ns/id/\"><pdfxid:GTS_PDFXVersion>PDF/X-4</pdfxid:GTS_PDFXVersion></rdf:Description>";
        let xml = xml.replacen("</rdf:RDF>", &format!("{add}</rdf:RDF>"), 1);
        let dict = {
            let i = dict.find("/Length")?;
            let rest = &dict[i + 7..];
            let rest = rest.trim_start();
            let end = rest.find(|c: char| !c.is_ascii_digit()).unwrap_or(rest.len());
            format!("{}/Length {}{}", &dict[..i], xml.len(), &rest[end..])
        };
        let mut b = dict.trim().as_bytes().to_vec();
        b.extend_from_slice(b"\nstream\n");
        b.extend_from_slice(xml.as_bytes());
        b.extend_from_slice(b"\nendstream");
        put(&mut out, meta, &b);
    }
    // Cross-reference section and trailer.
    let xref_at = out.len();
    entries.sort();
    let mut x = String::from("xref\n");
    for (oid, off) in &entries {
        x.push_str(&format!("{oid} 1\n{off:010} 00000 n\r\n"));
    }
    let id = id.unwrap_or_default();
    x.push_str(&format!("trailer\n<</Size {next}/Root {root} 0 R/Info {info_id} 0 R{id}/Prev {prev}>>\nstartxref\n{xref_at}\n%%EOF\n"));
    out.extend_from_slice(x.as_bytes());
    Some(out)
}

/// krilla gives every transparency group it writes (placed PDF pages, opacity, isolated
/// blending) a DeviceRGB blending space. In a print document that puts DeviceRGB in a CMYK file
/// (PDF/X-4 forbids it under a CMYK output intent) and blends CMYK art through RGB, which can
/// turn a K-only black into a four-colour black. This rewrites those group dictionaries to
/// DeviceCMYK in place and at the same length (the optional `/Type/Group` gives the room), so
/// every cross-reference offset stays valid. Only object dictionaries are touched, never stream
/// data. Returns how many groups changed.
/// Whether a transparency group dictionary still names DeviceRGB as its blending space.
pub fn has_rgb_groups(pdf: &[u8]) -> bool {
    pdf.windows(b"/CS/DeviceRGB".len()).enumerate().any(|(i, w)| {
        w == b"/CS/DeviceRGB" && pdf.get(i.saturating_sub(64)..i).is_some_and(|before| before.windows(15).any(|b| b == b"/S/Transparency"))
    })
}

pub fn cmyk_group_spaces(pdf: &mut [u8]) -> usize {
    const FORMS: [(&[u8], &[u8]); 2] = [
        (b"/Group<</Type/Group/S/Transparency/I true/CS/DeviceRGB>>", b"/Group<</S/Transparency/I true/CS/DeviceCMYK>>"),
        (b"/Group<</Type/Group/S/Transparency/CS/DeviceRGB>>", b"/Group<</S/Transparency/CS/DeviceCMYK>>"),
    ];
    let Some(dicts) = dictionary_ranges(pdf) else { return 0 };
    let mut changed = 0;
    for r in dicts {
        for (from, to) in FORMS {
            let mut at = r.start;
            while let Some(i) = find(pdf.get(..r.end).unwrap_or_default(), from, at) {
                if let Some(dst) = pdf.get_mut(i..i + from.len()) {
                    dst.fill(b' ');
                    if let Some(head) = dst.get_mut(..to.len()) {
                        head.copy_from_slice(to);
                        changed += 1;
                    }
                }
                at = i + from.len();
            }
        }
    }
    changed
}

/// Byte ranges of the dictionary part of every object in a file with one classic
/// cross-reference table (krilla's output): from the object's offset to its `stream` keyword or
/// `endobj`. `None` when the table can't be read.
fn dictionary_ranges(pdf: &[u8]) -> Option<Vec<std::ops::Range<usize>>> {
    let tail = String::from_utf8_lossy(pdf.get(rfind(pdf, b"startxref")?..)?).to_string();
    let xref = int_after(&tail, "startxref")?;
    let table = pdf.get(xref..)?;
    if !table.starts_with(b"xref") {
        return None;
    }
    let end = find(table, b"trailer", 0)?;
    let text = String::from_utf8_lossy(table.get(4..end)?).to_string();
    let mut words = text.split_ascii_whitespace();
    let mut out = Vec::new();
    while let (Some(_first), Some(count)) = (words.next(), words.next()) {
        let count: usize = count.parse().ok()?;
        for _ in 0..count {
            let (off, _gen, kind) = (words.next()?, words.next()?, words.next()?);
            if kind != "n" {
                continue;
            }
            let start: usize = off.parse().ok()?;
            let endobj = find(pdf, b"endobj", start)?;
            let stop = find(pdf.get(..endobj)?, b"stream", start).unwrap_or(endobj);
            out.push(start..stop);
        }
    }
    Some(out)
}

/// PDF/X-4 checks on an exported file: what's missing (empty = passes these checks).
pub fn check_pdfx4(pdf: &[u8]) -> Vec<String> {
    let mut issues = Vec::new();
    let text = String::from_utf8_lossy(pdf);
    if !pdf.starts_with(b"%PDF-1.") || pdf.get(7).is_none_or(|v| *v > b'6') {
        issues.push("the PDF version must be 1.6 or lower".into());
    }
    if !text.contains("/S/GTS_PDFX") {
        issues.push("no PDF/X output intent".into());
    }
    if !text.contains("/DestOutputProfile") {
        issues.push("the output intent has no destination profile".into());
    }
    if !text.contains("/GTS_PDFXVersion(PDF/X-4)") {
        issues.push("the document information doesn't name PDF/X-4".into());
    }
    if !text.contains("pdfxid:GTS_PDFXVersion>PDF/X-4<") {
        issues.push("the XMP metadata doesn't name PDF/X-4".into());
    }
    if text.contains("/Encrypt") {
        issues.push("the file is encrypted".into());
    }
    let pages = text.matches("/Type/Page/").count() + text.matches("/Type/Page>").count();
    let trims = text.matches("/TrimBox").count();
    if pages > 0 && trims < pages {
        issues.push("every page needs a trim box".into());
    }
    if text.contains("/Subtype/Type3") {
        issues.push("Type 3 fonts aren't allowed".into());
    }
    issues
}
