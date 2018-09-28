use physics::{PIX_PER_METER, METERS_PER_PIX};


pub fn pix_to_meters(pixels: f32) -> f32 {
    pixels * METERS_PER_PIX
}

pub fn meters_to_pix(meters: f32) -> f32 {
    meters * PIX_PER_METER
}