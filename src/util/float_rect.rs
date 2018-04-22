
#[derive(Clone, Copy, Debug)]
pub struct FloatRect {
    pub left: f32,
    pub top: f32,
    pub width: f32,
    pub height: f32
}

impl FloatRect {
    pub fn new(left: f32, top: f32, width: f32, height: f32) -> Self {
        Self { left, top, width, height }
    }

    pub fn new_square(left: f32, top: f32, width: f32) -> Self {
        Self { left, top, width, height: width }
    }

    pub fn intersects(&self, other: &FloatRect) -> bool {
        self.left < other.left + other.width &&
        self.left + self.width > other.left &&
        self.top < other.top + other.height &&
        self.top + self.height > other.top
    }

    pub fn intersects_at(&self, x: f32, y: f32, other: &FloatRect) -> bool {
        self.left < other.left + other.width + x &&
        self.left + self.width > other.left + x &&
        self.top < other.top + other.height + y &&
        self.top + self.height > other.top + y
    }
}