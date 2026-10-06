use candle::Device;
use prost::Message;
use wasm_bindgen::prelude::*;

/// Evaluate the vendored Candle ONNX implementation entirely on the CPU.
/// ONNX external-data files are not resolved; provide self-contained model bytes.
#[wasm_bindgen]
pub fn evaluate(model: &[u8], inputs: &[u8]) -> Result<Vec<u8>, JsError> {
    let model = candle_onnx::onnx::ModelProto::decode(model)?;
    let inputs = candle::safetensors::load_buffer(inputs, &Device::Cpu)?;
    let outputs = candle_onnx::simple_eval(&model, inputs)?;
    Ok(safetensors::tensor::serialize(outputs.iter().map(|(name,t)| (name.as_str(),t)), &None)?)
}
