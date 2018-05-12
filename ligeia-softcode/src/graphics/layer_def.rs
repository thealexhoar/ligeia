use std::collections::HashMap;

use ligeia_utils::rect::FloatRect;


pub struct LayerDef {
    pub layers: HashMap<usize, FloatRect>
}