
#[derive(Clone, Copy, Debug, Default)]
pub struct Vector2<T: Clone + Default> {
    pub x: T,
    pub y: T
}

pub type Vector2f = Vector2<f32>;
pub type Vector2i = Vector2<i32>;
pub type Vector2u = Vector2<u32>;

impl<T: Clone + Default>  Vector2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self {x, y}
    }
}