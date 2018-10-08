
pub fn true_atan(x: f32, y: f32) -> f32 {
    y.atan2(x)
}

pub fn normalize(x: f32, y: f32) -> (f32, f32) {
    let a = (x * x + y * y).sqrt();
    (x / a, y / a)
}