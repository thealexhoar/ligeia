
pub static PPM: f32 = 32.;
pub static MPP: f32 = 1. / PPM;

pub fn pix_to_meters(pixels: f32) -> f32 {
    pixels * MPP
}

pub fn meters_to_pix(meters: f32) -> f32 {
    meters * PPM
}