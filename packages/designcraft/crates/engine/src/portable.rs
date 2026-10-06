//! Explicit platform capability failures, before commands mutate state or touch native APIs.
//! This is a native-resource deny list, not a portable-command allowlist. New commands run
//! normally unless they require a resource unavailable on wasm32.
use serde_json::Value;

/// Explain a request's native dependency. Also usable by headless hosts for discovery.
pub fn unavailable(command: &str, params: &Value) -> Option<&'static str> {
    match command {
        "file.open" | "file.revert" => Some("native filesystem unavailable; use file.openBytes or upload document bytes"),
        "file.save" | "file.saveAs" | "file.saveACopy" => Some("native filesystem unavailable; download document bytes instead"),
        "file.print" | "file.printers" => Some("native print spooler unavailable; use file.exportPdf"),
        "file.package" | "links.copyTo" => Some("native directories unavailable; export a document with embedded assets"),
        "links.relink" | "links.relinkFolder" | "links.update" => Some("native linked files unavailable; place embedded bytes with file.place"),
        "color.loadProfile" => Some("native profile files unavailable; use color.loadProfileBytes"),
        c if c.starts_with("file.recovery.") || c.starts_with("book.") => Some("native filesystem unavailable for recovery folders and book files"),
        "file.exportPdf"
        | "file.exportIdml"
        | "file.exportEpub"
        | "file.exportFixedEpub"
        | "file.exportHtml"
        | "file.exportText"
        | "file.exportXml"
        | "file.printBooklet"
        | "snippet.export"
        | "swatch.save"
        | "library.new"
            if params.get("path").is_some_and(Value::is_string) =>
        {
            Some("native filesystem unavailable; omit path to return bytes/text (library.json for libraries)")
        }
        "file.place" | "file.openIdml" | "snippet.place" | "place.load" | "place.styles" | "swatch.load"
            if params.get("path").is_some_and(Value::is_string) && !params.get("base64").is_some_and(Value::is_string) =>
        {
            Some("native filesystem unavailable; supply base64 bytes and name")
        }
        "library.open" if params.get("path").is_some_and(Value::is_string) && !params.get("json").is_some_and(Value::is_string) => {
            Some("native filesystem unavailable; supply json")
        }
        "file.importXml" | "xml.loadDtd" if params.get("path").is_some_and(Value::is_string) && !params.get("text").is_some_and(Value::is_string) => {
            Some("native filesystem unavailable; supply text")
        }
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn resource_gates_preserve_byte_alternatives_and_unrestricted_commands() {
        for command in
            ["file.exportPdf", "file.exportIdml", "file.exportHtml", "file.exportText", "file.exportEpub", "file.exportXml", "snippet.export"]
        {
            assert!(unavailable(command, &json!({"path": "fake"})).is_some(), "{command}");
            assert!(unavailable(command, &json!({})).is_none(), "{command}");
        }
        assert!(unavailable("file.place", &json!({"path": "fake"})).is_some());
        assert!(unavailable("file.place", &json!({"path": "ignored", "base64": "abc"})).is_none());
        assert!(unavailable("file.print", &json!({"dryRun": true})).is_some());
        assert!(unavailable("future.portable.command", &json!({"count": 999999, "path": []})).is_none());
    }
}
