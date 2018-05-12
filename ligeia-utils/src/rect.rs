use num_traits::Num;

#[derive(Clone, Copy, Debug)]
pub struct Rect<T: Copy + Num + PartialOrd> {
    pub left: T,
    pub top: T,
    pub width: T,
    pub height: T
}

pub type FloatRect = Rect<f32>;
pub type IntRect = Rect<i32>;
pub type UIntRect = Rect<u32>;

impl<T: Copy + Num + PartialOrd> Rect<T> {
    pub fn new(left: T, top: T, width: T, height: T) -> Self {
        Self { left, top, width, height }
    }

    pub fn new_square(left: T, top: T, width: T) -> Self {
        Self { left, top, width, height: width }
    }

    pub fn intersects(&self, other: &Rect<T>) -> bool {
        self.left < other.left + other.width &&
        self.left + self.width > other.left &&
        self.top < other.top + other.height &&
        self.top + self.height > other.top
    }

    pub fn intersects_at(&self, x: T, y: T, other: &Rect<T>) -> bool {
        self.left < other.left + other.width + x &&
        self.left + self.width > other.left + x &&
        self.top < other.top + other.height + y &&
        self.top + self.height > other.top + y
    }
}