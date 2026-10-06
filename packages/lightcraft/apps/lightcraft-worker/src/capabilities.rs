//! Capability denials name concrete native dependencies. All other registry commands dispatch.
use serde_json::Value;

pub fn unavailable(id: &str) -> Option<&'static str> {
    match id {
        "library.devices" => Some("camera/device discovery requires native hardware"),
        "library.browse" | "folder.rename" | "folder.move" | "photo.rename" | "photo.renamePreview" |
        "photo.relink" | "library.findMissing" | "library.autoImportScan" | "library.autoImport" |
        "photo.duplicate" | "photo.convertToDng" | "photo.editExternal" | "export.checkTarget" =>
            Some("native filesystem operation unavailable; use uploaded originals and byte exports"),
        "photo.saveMetadataToFile" | "photo.readMetadataFromFile" | "library.toggleAutoWriteXmp" =>
            Some("native sidecar files unavailable; original exports include an XMP sidecar"),
        "library.smartPreviews" | "library.smartPreviewsLocation" | "library.buildPreviews" =>
            Some("disk preview cache/background threads unavailable; use byte exports for previews"),
        "preset.import" | "preset.export" | "profile.import" | "curve.import" | "curve.export" => Some("native preset/profile paths unavailable"),
        "merge.hdr" | "merge.panorama" | "merge.hdrPanorama" => Some("use the merge byte API to download the engine's DNG output"),
        _ => None,
    }
}

pub fn check(id: &str, p: &Value) -> Result<(), String> {
    if let Some(reason) = unavailable(id) { return Err(format!("unavailable: {id}: {reason}")); }
    if id == "library.import" && (p.get("mode").and_then(Value::as_str).is_some_and(|s| s != "add") || p.get("dng").and_then(Value::as_bool) == Some(true)) {
        return Err("unavailable: native copy/move import; upload bytes and use mode add, then export DNG if needed".into());
    }
    if id == "library.xmpPreferences" && p.get("autoWrite").and_then(Value::as_bool) == Some(true) {
        return Err("unavailable: automatic filesystem sidecars; download original export's XMP sidecar".into());
    }
    if id == "photo.autoTagTracklog" && p.get("gpx").and_then(Value::as_str).is_none() {
        return Err("unavailable: filesystem GPX; supply gpx text".into());
    }
    Ok(())
}
