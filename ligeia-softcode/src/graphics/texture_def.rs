use std::collections::HashMap;

use ligeia_utils::rect::UIntRect;

pub struct TextureDef {
    pub filename: String,
    pub frames: Vec<UIntRect>
}

