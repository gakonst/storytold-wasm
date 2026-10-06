//! Live preflight (the Preflight panel and the status-bar indicator): overset text, missing
//! fonts, missing or low-resolution graphics, RGB content in print documents, empty text frames.

use designcraft_doc::{Content, Document, Intent, Item, SpreadRef};
use serde::Serialize;
use serde_json::{Value, json};

use super::{CommandSpec, cmd, has_doc};
use crate::{Result, Session};

#[derive(Clone, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Issue {
    /// `error` or `warning`.
    pub severity: &'static str,
    pub kind: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<usize>,
}

pub fn specs() -> Vec<CommandSpec> {
    vec![cmd!(query "preflight.run", "Preflight Document", ["Window", "Output", "Preflight"], None,
        "{minPpi?: 150} → issues [{severity, kind, message, item?, page?}]", has_doc, run)]
}

fn page_of(d: &Document, sr: SpreadRef, it: &Item) -> Option<usize> {
    let SpreadRef::Doc(si) = sr else { return None };
    let sp = d.spreads.get(si)?;
    Some(d.first_page_of_spread(si) + sp.page_at_x(it.bounds().center().x).unwrap_or(0))
}

pub fn check(s: &Session, min_ppi: f64) -> Vec<Issue> {
    let Some(st) = s.active() else { return vec![] };
    let d = &st.doc;
    let mut out = Vec::new();
    // Stories: overset and fonts.
    let db = designcraft_fonts::FontDb::global();
    let mut missing_fonts: Vec<String> = Vec::new();
    for story in d.stories.values() {
        let cs = s.cache.get(d, story.id, None);
        if cs.is_overset() {
            let last = story.frames.last().copied();
            let page = last.and_then(|f| d.find(f).zip(d.item(f))).and_then(|(loc, it)| page_of(d, loc.spread, it));
            let n = story.text[cs.overset_at.unwrap_or(0).min(story.len())..].chars().count();
            out.push(Issue { severity: "error", kind: "overset", message: format!("Overset text: {n} characters"), item: last.map(|i| i.0), page });
        }
        for (pi, pf) in story.paras.iter().enumerate() {
            let (_, base) = d.styles.resolve_para(pf);
            let _ = pi;
            let mut fams = vec![base.font_family.clone()];
            for (_, f) in story.runs() {
                fams.push(d.styles.resolve_char(&base, f).font_family);
            }
            for fam in fams {
                if !db.has_family(&fam) && !missing_fonts.contains(&fam) {
                    missing_fonts.push(fam);
                }
            }
        }
    }
    for f in missing_fonts {
        out.push(Issue { severity: "error", kind: "missingFont", message: format!("Missing font: {f}"), item: None, page: None });
    }
    // Items.
    for (si, sp) in d.spreads.iter().enumerate() {
        for top in &sp.items {
            top.walk(&mut |it| {
                let page = page_of(d, SpreadRef::Doc(si), it);
                match &it.content {
                    Content::Graphic(g) => match d.assets.get(&g.asset) {
                        None => {
                            out.push(Issue { severity: "error", kind: "missingLink", message: "Missing graphic".into(), item: Some(it.id.0), page })
                        }
                        Some(a) if a.data.is_empty() => out.push(Issue {
                            severity: "error",
                            kind: "missingLink",
                            message: format!("Missing link: {}", a.link.as_deref().unwrap_or(&a.name)),
                            item: Some(it.id.0),
                            page,
                        }),
                        Some(a) => {
                            // Effective ppi: pixels per inch at the placed size. Placed PDF and
                            // SVG pages export as vectors; their `pixels` is only the preview's.
                            let vector = designcraft_images::is_pdf(&a.data) || designcraft_images::is_svg(&a.data);
                            if let Some((pw, _)) = a.pixels.filter(|_| !vector) {
                                let placed_w = (g.xf * it.xf).as_coeffs()[0].hypot((g.xf * it.xf).as_coeffs()[1]) * g.size.0;
                                let ppi = pw as f64 / (placed_w / 72.0).max(1e-6);
                                if d.settings.intent == Intent::Print && ppi < min_ppi {
                                    out.push(Issue {
                                        severity: "warning",
                                        kind: "lowResolution",
                                        message: format!("{}: effective {ppi:.0} ppi (< {min_ppi:.0})", a.name),
                                        item: Some(it.id.0),
                                        page,
                                    });
                                }
                            }
                        }
                    },
                    Content::Text(tf) if d.story(tf.story).is_some_and(|s| s.is_empty()) => {
                        out.push(Issue { severity: "warning", kind: "emptyText", message: "Empty text frame".into(), item: Some(it.id.0), page });
                    }
                    _ => {}
                }
            });
        }
    }
    // RGB swatches in use in a print document.
    if d.settings.intent == Intent::Print {
        for sw in &d.swatches {
            if let designcraft_color::SwatchValue::Color { color: designcraft_color::Color::Rgb { .. }, .. } = sw.value {
                out.push(Issue {
                    severity: "warning",
                    kind: "rgbColor",
                    message: format!("RGB swatch in a print document: {}", sw.name),
                    item: None,
                    page: None,
                });
            }
        }
    }
    out
}

fn run(s: &mut Session, p: &Value) -> Result<Value> {
    let min = p.get("minPpi").and_then(Value::as_f64).unwrap_or(150.0);
    let issues = check(s, min);
    let errors = issues.iter().filter(|i| i.severity == "error").count();
    Ok(json!({"errors": errors, "warnings": issues.len() - errors, "issues": issues}))
}

#[cfg(test)]
mod placed_vector_tests {
    use serde_json::json;

    use crate::Session;
    use crate::cmd::base64_encode;

    /// A placed PDF logo was reported as
    /// "logo.pdf: effective 72 ppi (< 150)", though it exports as vectors.
    #[test]
    fn vector_graphics_have_no_resolution() {
        let mut logo = Session::new();
        logo.execute("file.new", &json!({"width": 100, "height": 50})).unwrap();
        let pdf = logo.execute("file.exportPdf", &json!({})).unwrap();
        let mut s = Session::new();
        s.execute("file.new", &json!({})).unwrap();
        s.execute("file.place", &json!({"base64": pdf["base64"], "name": "logo.pdf", "x": 72, "y": 72, "width": 300})).unwrap();
        // Deselect the logo, or the photo would replace it in its frame.
        s.execute("selection.set", &json!({"ids": []})).unwrap();
        let png = designcraft_render::Rendered { width: 40, height: 20, pixels: vec![200; 40 * 20 * 4] }.to_png();
        s.execute("file.place", &json!({"base64": base64_encode(&png), "name": "photo.png", "x": 72, "y": 300, "width": 300})).unwrap();
        let r = s.execute("preflight.run", &json!({})).unwrap();
        let low: Vec<&str> =
            r["issues"].as_array().unwrap().iter().filter(|i| i["kind"] == "lowResolution").map(|i| i["message"].as_str().unwrap()).collect();
        assert_eq!(low, ["photo.png: effective 10 ppi (< 150)"]);
    }
}
