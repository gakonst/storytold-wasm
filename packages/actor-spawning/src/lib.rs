#[no_mangle]
pub extern "C" fn location(position: f64, offset: f64, forward: f64, distance: f64) -> f64 { position + offset + forward * distance }
#[no_mangle]
pub extern "C" fn between(min: f64, max: f64, unit: f64) -> f64 { min + (max - min) * unit }
