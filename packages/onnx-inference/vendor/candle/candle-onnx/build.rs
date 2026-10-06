use std::io::Result;

fn main() -> Result<()> {
    if std::env::var_os("PROTOC").is_none() {
        std::env::set_var("PROTOC", protoc_bin_vendored::protoc_bin_path().map_err(std::io::Error::other)?);
    }
    prost_build::compile_protos(&["src/onnx.proto3"], &["src/"])?;
    Ok(())
}
