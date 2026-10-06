#[no_mangle]
pub extern "C" fn meters(mm: i32, invert: i32) -> f32 { mm as f32 * if invert != 0 { -0.001 } else { 0.001 } }
