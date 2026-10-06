use designcraft_color::swatch::{ColorType, SwatchValue};
use designcraft_color::{Color, Swatch};
use designcraft_compose::Cache;
use designcraft_doc::build::NewDocument;
use designcraft_doc::{Document, Fill, Item, ItemId, ParaFormat, SpreadRef};
use designcraft_geom::Rect;

use crate::*;

fn doc_with_text(text: &str) -> Document {
    let mut d = Document::new(&NewDocument { pages: 2, ..NewDocument::default() });
    let lid = d.default_layer();
    d.add_text_frame(SpreadRef::Doc(0), Rect::new(36.0, 36.0, 576.0, 300.0), lid, text, ParaFormat::default()).unwrap();
    d
}

fn add_box(d: &mut Document, r: Rect, swatch: &str) {
    let lid = d.default_layer();
    let id = ItemId(d.alloc());
    let mut b = Item::new(id, lid, designcraft_doc::Shape::Rectangle, designcraft_geom::shapes::rectangle(r));
    b.fill = Fill::swatch(swatch);
    d.insert_item(SpreadRef::Doc(0), b, None).unwrap();
}

fn uncompressed(d: &Document, opts: PdfOptions) -> String {
    let bytes = export_pdf(d, &Cache::new(), &PdfOptions { compress: false, ..opts }).expect("export");
    String::from_utf8_lossy(&bytes).into_owned()
}

/// Text of every page through hayro's interpreter (glyph → Unicode via ToUnicode / ActualText).
pub(crate) fn extract_text(bytes: &[u8]) -> Vec<String> {
    use hayro_interpret::font::Glyph;
    use hayro_interpret::{
        BlendMode, ClipPath, Context, Device, GlyphDrawMode, Image, InterpreterCache, InterpreterSettings, Paint, PathDrawMode, SoftMask,
        interpret_page,
    };
    use hayro_syntax::Pdf;
    struct Ex(String);
    impl Device<'_> for Ex {
        fn set_soft_mask(&mut self, _: Option<SoftMask<'_>>) {}
        fn set_blend_mode(&mut self, _: BlendMode) {}
        fn draw_path(&mut self, _: &kurbo::BezPath, _: kurbo::Affine, _: &Paint<'_>, _: &PathDrawMode) {}
        fn push_clip_path(&mut self, _: &ClipPath) {}
        fn push_transparency_group(&mut self, _: f32, _: Option<SoftMask<'_>>, _: BlendMode) {}
        fn draw_glyph(&mut self, g: &Glyph<'_>, _: kurbo::Affine, _: kurbo::Affine, _: &Paint<'_>, _: &GlyphDrawMode) {
            match g.as_unicode() {
                Some(hayro_cmap::BfString::Char(c)) => self.0.push(c),
                Some(hayro_cmap::BfString::String(s)) => self.0.push_str(&s),
                None => self.0.push('\u{FFFD}'),
            }
        }
        fn draw_image(&mut self, _: Image<'_, '_>, _: kurbo::Affine) {}
        fn pop_clip_path(&mut self) {}
        fn pop_transparency_group(&mut self) {}
    }
    let pdf = Pdf::new(bytes.to_vec()).expect("parse");
    let cache = InterpreterCache::new();
    pdf.pages()
        .iter()
        .map(|page| {
            let mut ctx =
                Context::new(kurbo::Affine::IDENTITY, kurbo::Rect::new(0.0, 0.0, 1.0, 1.0), &cache, pdf.xref(), InterpreterSettings::default());
            let mut ex = Ex(String::new());
            interpret_page(page, &mut ctx, &mut ex);
            ex.0
        })
        .collect()
}

#[test]
fn exports_valid_pdf_with_one_page_per_page() {
    let d = doc_with_text("Hello DesignCraft");
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions::default()).unwrap();
    assert!(bytes.starts_with(b"%PDF-1.7"));
    let pdf = hayro_syntax::Pdf::new(bytes.clone()).expect("parse");
    assert_eq!(pdf.pages().len(), 2);
}

#[test]
fn text_is_real_text() {
    let d = doc_with_text("Hello DesignCraft — efficient affine");
    let s = uncompressed(&d, PdfOptions::default());
    assert!(s.contains("/FontFile2") || s.contains("/FontFile3"), "font embedded");
    assert!(s.contains("/ToUnicode"));
    assert!(s.contains(")Tj") || s.contains("]TJ"), "text operators");
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions::default()).unwrap();
    let text = extract_text(&bytes);
    let flat: String = text[0].split_whitespace().collect::<Vec<_>>().join(" ");
    assert!(flat.contains("Hello"), "{flat}");
    assert!(text[0].contains("DesignCraft"), "{text:?}");
    assert!(text[0].contains("efficient") && text[0].contains("affine"), "ligatures map back: {text:?}");
}

#[test]
fn colours_keep_their_space() {
    let mut d = doc_with_text("x");
    add_box(&mut d, Rect::new(36.0, 400.0, 136.0, 500.0), "C=100 M=0 Y=0 K=0");
    d.swatches.push(Swatch::color("Brand Red", Color::rgb(0.9, 0.1, 0.1)));
    add_box(&mut d, Rect::new(200.0, 400.0, 300.0, 500.0), "Brand Red");
    d.swatches.push(Swatch {
        name: "PANTONE Test".into(),
        value: SwatchValue::Color { color: Color::cmyk(0.0, 0.5, 1.0, 0.0), color_type: ColorType::Spot },
        locked: false,
        named: true,
        hidden: false,
    });
    add_box(&mut d, Rect::new(320.0, 400.0, 420.0, 500.0), "PANTONE Test");
    let s = uncompressed(&d, PdfOptions::default());
    assert!(s.contains("1 0 0 0 k"), "DeviceCMYK fill");
    assert!(s.contains(" rg"), "DeviceRGB fill");
    assert!(s.contains("/Separation") && s.contains("/PANTONE#20Test"), "spot as Separation");
}

/// [Paper] went into print PDFs as `1 1 1 rg`, DeviceRGB white.
#[test]
fn paper_prints_as_no_ink() {
    let mut d = doc_with_text("x");
    add_box(&mut d, Rect::new(36.0, 400.0, 136.0, 500.0), "[Paper]");
    let s = uncompressed(&d, PdfOptions::default());
    assert!(s.contains("0 0 0 0 k"), "CMYK, no ink");
    assert!(!s.contains("1 1 1 rg"), "no RGB white");
    // A web document keeps the paper colour.
    d.settings.intent = designcraft_doc::Intent::Web;
    assert!(uncompressed(&d, PdfOptions::default()).contains("1 1 1 rg"));
    // PDF/X has a CMYK output intent whatever the document's intent.
    assert!(!uncompressed(&d, PdfOptions { standard: Standard::PdfX4, ..Default::default() }).contains("1 1 1 rg"));
}

#[test]
fn boxes_bleed_and_marks() {
    let mut d = doc_with_text("x");
    d.settings.bleed = [9.0; 4];
    let s = uncompressed(&d, PdfOptions { bleed: true, ..Default::default() });
    assert!(s.contains("/TrimBox[9 9 621 801]"), "{}", &s[..s.len().min(4000)]);
    assert!(s.contains("/MediaBox[0 0 630 810]"));
    assert!(s.contains("/BleedBox[0 0 630 810]"));
    let s = uncompressed(&d, PdfOptions { bleed: true, marks: Marks::ALL, pages: Some(vec![1]), ..Default::default() });
    // margin = max(offset 6, bleed 9) + 18 + 6 = 33
    assert!(s.contains("/MediaBox[0 0 678 858]"), "marks area");
    assert!(s.contains("/All"), "registration colour");
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions { marks: Marks::ALL, pages: Some(vec![1]), ..Default::default() }).unwrap();
    let t = extract_text(&bytes);
    assert_eq!(t.len(), 1);
    assert!(t[0].contains("Page 2"), "{t:?}");
}

#[test]
fn spreads_and_ranges() {
    let mut d = Document::new(&NewDocument { pages: 4, facing_pages: true, ..NewDocument::default() });
    let _ = &mut d;
    let n = d.page_count();
    assert_eq!(n, 4);
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions { spreads: true, ..Default::default() }).unwrap();
    let pages = hayro_syntax::Pdf::new(bytes).unwrap().pages().len();
    assert_eq!(pages, d.spreads.len());
    assert_eq!(parse_page_range("1-2, 4", 4).unwrap(), vec![0, 1, 3]);
    assert_eq!(parse_page_range("3-", 4).unwrap(), vec![2, 3]);
    assert_eq!(parse_page_range("", 2).unwrap(), vec![0, 1]);
    assert!(parse_page_range("5", 4).is_err());
    assert!(parse_page_range("x", 4).is_err());
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions { pages: Some(vec![1, 2]), ..Default::default() }).unwrap();
    assert_eq!(hayro_syntax::Pdf::new(bytes).unwrap().pages().len(), 2);
    assert_eq!(export_pdf(&d, &Cache::new(), &PdfOptions { pages: Some(vec![9]), ..Default::default() }), Err(PdfError::BadPage(10)));
}

#[test]
fn standards() {
    let d = doc_with_text("Archive me");
    let r = export_pdf_with_report(&d, &Cache::new(), &PdfOptions { standard: Standard::PdfA2b, created: Some(1_700_000_000), ..Default::default() });
    let r = r.expect("PDF/A-2b export");
    assert!(String::from_utf8_lossy(&r.bytes).contains("pdfaid"));
    let r = export_pdf_with_report(&d, &Cache::new(), &PdfOptions { standard: Standard::PdfX4, ..Default::default() }).unwrap();
    assert!(r.bytes.starts_with(b"%PDF-1.6"));
    // Output intent, identification and checks all in place; the update still reads.
    assert!(r.warnings.is_empty(), "{:?}", r.warnings);
    assert!(crate::check_pdfx4(&r.bytes).is_empty());
    let text = String::from_utf8_lossy(&r.bytes).to_string();
    assert!(text.contains("/OutputIntents[") && text.contains("DesignCraft Generic CMYK"));
    // A plain export fails the checks; both read with the same pages.
    let plain = export_pdf(&d, &Cache::new(), &PdfOptions::default()).unwrap();
    assert_eq!(hayro_syntax::Pdf::new(r.bytes.clone()).unwrap().pages().len(), hayro_syntax::Pdf::new(plain.clone()).unwrap().pages().len());
    assert!(crate::check_pdfx4(&plain).len() >= 3);
    assert_eq!(Standard::parse("PDF/X-4"), Some(Standard::PdfX4));
    assert_eq!(Standard::parse("pdfa-2b"), Some(Standard::PdfA2b));
}

#[test]
fn tables_export_cell_text() {
    let mut d = Document::new(&NewDocument::default());
    let lid = d.default_layer();
    let (_, sid) = d.add_text_frame(SpreadRef::Doc(0), Rect::new(36.0, 36.0, 336.0, 400.0), lid, "Before", ParaFormat::default()).unwrap();
    let mut t = designcraft_doc::Table::new(9, 2, 2, 1, 0, 300.0);
    t.cell_mut(0, 0).unwrap().text.insert(0, "Ink");
    t.cell_mut(0, 0).unwrap().fill = "C=100 M=0 Y=0 K=0".into();
    t.cell_mut(1, 1).unwrap().text.insert(0, "Plum");
    t.cell_mut(2, 0).unwrap().text.insert(0, "Sunset");
    d.story_mut(sid).unwrap().insert_table(6, t);
    let bytes = export_pdf(&d, &Cache::new(), &PdfOptions::default()).unwrap();
    let text = extract_text(&bytes).concat();
    for w in ["Before", "Ink", "Plum", "Sunset"] {
        assert!(text.contains(w), "{w} missing from {text:?}");
    }
}
