use sfml::graphics::FloatRect;
use std::collections::HashMap;



pub struct LayerDef {
    pub layers: HashMap<usize, FloatRect>
}