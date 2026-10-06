//! Placed graphics: format sniffing, sizes and decoding for the renderer and PDF export.
//! Rasters go through `image` (PNG, JPEG, GIF, WebP, TIFF, BMP) or `psd` (Photoshop's merged
//! composite); SVG is parsed with usvg (text set in the bundled fonts) and rasterised with resvg
//! for the screen — PDF export draws the same tree as vectors.
#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

use std::sync::{Arc, OnceLock};

pub use resvg::usvg;

mod eps;
pub use eps::{bounding_box as eps_bounding_box, eps_proxy, is_eps};

/// CSS pixels (SVG user units) → points.
pub const PT_PER_PX: f64 = 0.75;

pub fn is_pdf(bytes: &[u8]) -> bool {
    bytes.starts_with(b"%PDF")
}

pub fn is_psd(bytes: &[u8]) -> bool {
    bytes.starts_with(b"8BPS")
}

/// An SVG document (optionally after an XML declaration, comments or a doctype).
pub fn is_svg(bytes: &[u8]) -> bool {
    let head = &bytes[..bytes.len().min(4096)];
    let s = String::from_utf8_lossy(head);
    let t = s.trim_start_matches('\u{feff}').trim_start();
    t.starts_with('<') && t.contains("<svg")
}

/// MIME type of a placed graphic.
pub fn mime(bytes: &[u8]) -> &'static str {
    if is_pdf(bytes) {
        return "application/pdf";
    }
    if is_psd(bytes) {
        return "image/vnd.adobe.photoshop";
    }
    if is_svg(bytes) {
        return "image/svg+xml";
    }
    match image::guess_format(bytes) {
        Ok(image::ImageFormat::Png) => "image/png",
        Ok(image::ImageFormat::Jpeg) => "image/jpeg",
        Ok(image::ImageFormat::Gif) => "image/gif",
        Ok(image::ImageFormat::WebP) => "image/webp",
        Ok(image::ImageFormat::Tiff) => "image/tiff",
        Ok(image::ImageFormat::Bmp) => "image/bmp",
        _ => "application/octet-stream",
    }
}

fn svg_options() -> &'static usvg::Options<'static> {
    static OPTS: OnceLock<usvg::Options<'static>> = OnceLock::new();
    OPTS.get_or_init(|| {
        let mut db = usvg::fontdb::Database::new();
        for f in designcraft_fonts::bundled() {
            db.load_font_data(f.to_vec());
        }
        // Japanese text in SVGs: the craft-fonts faces (none without CRAFT_FONTS_DIR).
        for f in designcraft_fonts::japanese_document_fonts() {
            db.load_font_data(f.bytes.to_vec());
        }
        db.set_serif_family("Source Serif 4");
        db.set_sans_serif_family("Source Sans 3");
        db.set_monospace_family("JetBrains Mono");
        usvg::Options { fontdb: Arc::new(db), font_family: "Source Sans 3".into(), ..Default::default() }
    })
}

/// The parsed SVG.
pub fn svg_tree(bytes: &[u8]) -> Option<usvg::Tree> {
    usvg::Tree::from_data(bytes, svg_options()).ok()
}

/// Natural size of a placed graphic in points: rasters at 72 ppi (one pixel per point), PSDs
/// likewise, SVGs from their width/height (CSS pixels), PDFs are the caller's.
pub fn natural_size(bytes: &[u8]) -> Option<(f64, f64)> {
    if is_svg(bytes) {
        let s = svg_tree(bytes)?.size();
        return Some((s.width() as f64 * PT_PER_PX, s.height() as f64 * PT_PER_PX));
    }
    let (w, h) = pixel_size(bytes)?;
    Some((w as f64, h as f64))
}

/// Pixel dimensions of a raster.
pub fn pixel_size(bytes: &[u8]) -> Option<(u32, u32)> {
    if is_psd(bytes) && bytes.len() >= 26 {
        // Header: signature, version, 6 reserved, channels, height, width (big-endian).
        let be = |i: usize| u32::from_be_bytes([bytes[i], bytes[i + 1], bytes[i + 2], bytes[i + 3]]);
        return Some((be(18), be(14)));
    }
    image::ImageReader::new(std::io::Cursor::new(bytes)).with_guessed_format().ok()?.into_dimensions().ok()
}

/// Straight (non-premultiplied) RGBA8 of a raster.
pub fn decode_rgba(bytes: &[u8]) -> Option<image::RgbaImage> {
    if is_psd(bytes) {
        let p = psd::Psd::from_bytes(bytes).ok()?;
        return image::RgbaImage::from_raw(p.width(), p.height(), p.rgba());
    }
    Some(image::load_from_memory(bytes).ok()?.to_rgba8())
}

/// An SVG rasterised so its longer side is `max_side` pixels: premultiplied RGBA8.
pub fn render_svg(bytes: &[u8], max_side: u32) -> Option<(Vec<u8>, u32, u32)> {
    let tree = svg_tree(bytes)?;
    let s = tree.size();
    let k = max_side as f32 / s.width().max(s.height()).max(1e-3);
    let (w, h) = ((s.width() * k).round().max(1.0) as u32, (s.height() * k).round().max(1.0) as u32);
    let mut pm = resvg::tiny_skia::Pixmap::new(w, h)?;
    resvg::render(&tree, resvg::tiny_skia::Transform::from_scale(k, k), &mut pm.as_mut());
    Some((pm.take(), w, h))
}

#[cfg(test)]
mod tests {
    use super::*;

    const SVG: &str = r##"<?xml version="1.0"?>
<svg xmlns="http://www.w3.org/2000/svg" width="200" height="100" viewBox="0 0 200 100">
  <rect x="0" y="0" width="100" height="100" fill="#ff0000"/>
  <text x="110" y="60" font-size="20">Hi</text>
</svg>"##;

    #[test]
    fn svg_sniff_size_and_render() {
        let b = SVG.as_bytes();
        assert!(is_svg(b));
        assert_eq!(mime(b), "image/svg+xml");
        assert_eq!(natural_size(b), Some((150.0, 75.0)));
        let (px, w, h) = render_svg(b, 400).unwrap();
        assert_eq!((w, h), (400, 200));
        // Red on the left; the text drew some ink on the right.
        assert_eq!(&px[(100 * 400 + 100) * 4..][..4], &[255, 0, 0, 255]);
        assert!(px.chunks(4).skip(220).step_by(7).any(|p| p[3] > 128 && p[0] < 100), "text pixels");
        assert!(!is_svg(b"\x89PNG"));
    }

    /// A minimal Photoshop file: 2×1 RGB, no layers, raw merged image.
    fn tiny_psd() -> Vec<u8> {
        let mut b = Vec::new();
        b.extend(b"8BPS");
        b.extend(1u16.to_be_bytes());
        b.extend([0u8; 6]);
        b.extend(3u16.to_be_bytes()); // channels
        b.extend(1u32.to_be_bytes()); // height
        b.extend(2u32.to_be_bytes()); // width
        b.extend(8u16.to_be_bytes()); // depth
        b.extend(3u16.to_be_bytes()); // RGB
        b.extend(0u32.to_be_bytes()); // colour mode data
        b.extend(0u32.to_be_bytes()); // image resources
        b.extend(0u32.to_be_bytes()); // layer and mask info
        b.extend(0u16.to_be_bytes()); // raw
        b.extend([255, 0, 0, 255, 0, 0]); // R, G, B planes: a red pixel, then a green one
        b
    }

    #[test]
    fn psd_composite_decodes() {
        let b = tiny_psd();
        assert_eq!(mime(&b), "image/vnd.adobe.photoshop");
        assert_eq!(pixel_size(&b), Some((2, 1)));
        let img = decode_rgba(&b).unwrap();
        assert_eq!(img.get_pixel(0, 0).0, [255, 0, 0, 255]);
        assert_eq!(img.get_pixel(1, 0).0, [0, 255, 0, 255]);
    }

    #[test]
    fn bmp_decodes() {
        let mut bmp = Vec::new();
        image::RgbaImage::from_pixel(3, 2, image::Rgba([0, 0, 255, 255]))
            .write_to(&mut std::io::Cursor::new(&mut bmp), image::ImageFormat::Bmp)
            .unwrap();
        assert_eq!(mime(&bmp), "image/bmp");
        assert_eq!(pixel_size(&bmp), Some((3, 2)));
        assert_eq!(decode_rgba(&bmp).unwrap().get_pixel(1, 1).0, [0, 0, 255, 255]);
    }
}
