use crate::{js_error, json};
use base64::Engine;
use jwt_light::{common_claims::CommonClaims, error::JwtError, parse_jwt_claims_trait::ParseJwtClaims};
use serde::Deserialize;
use serde_json::{json, Value};
use wasm_bindgen::JsValue;

pub fn inspect_bytes(bytes: &[u8]) -> Result<String, JsValue> {
    let mime = mimetypes::mimetype_for_bytes::get_mimetype_for_bytes_or_default(bytes);
    json(json!({"bytes": bytes.len(), "blake3": file_hashing::hash_file::hash_bytes_blake3(bytes),
        "mime": mime, "extension": mimetypes::mimetype_to_extension::mimetype_to_extension(mime)}))
}

#[derive(Deserialize)]
#[serde(tag = "op", rename_all = "snake_case", deny_unknown_fields)]
enum Request {
    JwtClaims { token: String },
    UrlExtension { url: String, #[serde(default)] accept: Option<Value> },
    MimeExtension { mime: String },
    DataUrl { base64: String, #[serde(default)] mime: Option<String> },
    Trim { text: String },
    Truncate { text: String, characters: usize },
    UppercaseFirst { text: String },
    ParseBool { text: String },
    Lines { text: String },
    Remap { value: f64, old_min: f64, old_max: f64, new_min: f64, new_max: f64 },
}
struct Claims(Value);
impl ParseJwtClaims for Claims {
    fn extract_claims(common: CommonClaims, extra: serde_json::Map<String, Value>) -> Result<Self, JwtError> {
        Ok(Self(json!({"claims": extra, "created": common.created.to_rfc3339(),
            "expiration": common.expiration.to_rfc3339(), "signature_verified": false})))
    }
}

pub fn utility(request: &str) -> Result<String, JsValue> {
    use primitives::*;
    use url_utils::extension::{extension::Extension, extract_extension_from_url::{ExtractExtensions, extract_extension_from_url_str}};
    let request: Request = serde_json::from_str(request).map_err(js_error)?;
    let result = match request {
        Request::JwtClaims { token } => Claims::parse_claims(&token).map_err(js_error)?.0,
        Request::UrlExtension { url, accept } => {
            let filter = match accept.as_ref() {
                None | Some(Value::Null) => ExtractExtensions::All,
                Some(Value::String(s)) => match s.as_str() {
                    "all" => ExtractExtensions::All, "image" => ExtractExtensions::KnownImage,
                    "audio" => ExtractExtensions::KnownAudio, "video" => ExtractExtensions::KnownVideo,
                    "media" => ExtractExtensions::KnownMedia, _ => return Err(js_error("unknown extension filter")),
                },
                Some(Value::Array(a)) => ExtractExtensions::from_vec(a.iter().map(|v| v.as_str().map(Extension::new)
                    .ok_or_else(|| js_error("extension filter must contain strings"))).collect::<Result<_,_>>()?),
                _ => return Err(js_error("accept must be a filter name or extension array")),
            };
            json!(extract_extension_from_url_str(&url, &filter).map(|e| e.without_period().to_owned()))
        },
        Request::MimeExtension { mime } => json!(mimetypes::mimetype_to_extension::mimetype_to_extension(&mime)),
        Request::DataUrl { base64, mime } => {
            let bytes = web_base64::web_base64_decode::web_base64_decode(&base64).map_err(js_error)?;
            let mime = mime.unwrap_or_else(|| mimetypes::mimetype_for_bytes::get_mimetype_for_bytes_or_default(&bytes).to_owned());
            json!(format!("data:{mime};base64,{}", base64::prelude::BASE64_STANDARD.encode(bytes)))
        },
        Request::Trim { text } => json!(trim_or_empty::trim_or_empty(&text)),
        Request::Truncate { text, characters } => json!(truncate_str::truncate_str(&text, characters)),
        Request::UppercaseFirst { text } => json!(str::first_letter_uppercase::first_letter_uppercase(&text)),
        Request::ParseBool { text } => json!(str_to_bool::str_to_bool(&text)),
        Request::Lines { text } => json!(iterators::iterate_trimmed_lines_without_comments::iterate_trimmed_lines_without_comments(text.lines()).collect::<Vec<_>>()),
        Request::Remap { value, old_min, old_max, new_min, new_max } => {
            if old_min == old_max { return Err(js_error("source interval must have nonzero length")); }
            json!(numerics::remap_range_f64::remap_range_f64(numerics::remap_range_f64::Args { value, old_min, old_max, new_min, new_max }))
        },
    };
    json(result)
}
