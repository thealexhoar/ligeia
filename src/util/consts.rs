use std::f32::consts::PI as STD_PI;

pub static PI: f32 = STD_PI;
pub static PI_FRAC_8: f32 = STD_PI * 0.125;
pub static TWO_PI: f32 = STD_PI * 2.;

pub fn radians_to_direction(theta: f32) -> u32 {
    let theta = theta % TWO_PI;
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