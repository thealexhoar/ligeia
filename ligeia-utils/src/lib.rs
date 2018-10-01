extern crate image;
extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_json;

pub mod rect;
mod file_error;
mod file_loader;
mod image_loader;
mod input;
mod stopwatch;

pub use file_error::FileError;
pub use file_loader::FileLoader;
pub use image_loader::ImageLoader;
pub use stopwatch::Stopwatch;


use std::f32::consts::PI as STD_PI;

pub static PI: f32 = STD_PI;
pub static PI_FRAC_8: f32 = STD_PI * 0.125;
pub static PI_FRAC_4: f32 = STD_PI * 0.25;
pub static TWO_PI: f32 = STD_PI * 2.;

pub fn radians_to_direction8(theta: f32) -> u32 {
    let mut theta = theta % TWO_PI;
    if theta < 0. {
        theta += TWO_PI;
    }
    if theta >= PI_FRAC_8 && theta < 3. * PI_FRAC_8 {
        1
    }
    else if theta >= 3. * PI_FRAC_8 && theta < 5. * PI_FRAC_8 {
        2
    }
    else if theta >= 5. * PI_FRAC_8 && theta < 7. * PI_FRAC_8 {
        3
    }
    else if theta >= 7. * PI_FRAC_8 && theta < 9. * PI_FRAC_8 {
        4
    }
    else if theta >= 9. * PI_FRAC_8 && theta < 11. * PI_FRAC_8 {
        5
    }
    else if theta >= 11. * PI_FRAC_8 && theta < 13. * PI_FRAC_8 {
        6
    }
    else if theta >= 13. * PI_FRAC_8 && theta < 15. * PI_FRAC_8 {
        7
    }
    else {
        0
    }
}

pub fn radians_to_direction4(theta: f32) -> u32 {
    let theta = theta % TWO_PI;
    if theta >= PI_FRAC_4 && theta < 3. * PI_FRAC_4 {
        2
    }
    else if theta >= 3. * PI_FRAC_4 && theta < 5. * PI_FRAC_4 {
        4
    }
    else if theta >= 5. * PI_FRAC_4 && theta < 7. * PI_FRAC_4 {
        6
    }
    else {
        0
    }
}
