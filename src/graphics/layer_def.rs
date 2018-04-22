use std::collections::HashMap;

use util::FloatRect;


pub struct LayerDef {
    pub layers: HashMap<usize, FloatRect>
}