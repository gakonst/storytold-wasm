// Transport-independent point arithmetic from src/point.rs and src/main.rs.
#[no_mangle]
pub extern "C" fn coordinate(random: u32, min: f32, max: f32) -> f32 {
    min + (max - min) * ((random as f64 / u32::MAX as f64) as f32)
}
#[no_mangle]
pub extern "C" fn random_next(mut state: u32) -> u32 {
    state ^= state << 13; state ^= state >> 17; state ^= state << 5; state
}
#[no_mangle]
pub extern "C" fn color(seconds: u32) -> u32 {
    match seconds % 10 { 0 | 1 => 0x000000, 2 | 3 => 0x0000ff, 4 | 5 => 0xff0000, 6 | 7 => 0x00ff00, _ => 0xffffff }
}
