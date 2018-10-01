use {Color, Vector2f};

static POS_X: usize = 0;
static POS_Y: usize = 1;
static COLOR_R: usize = 2;
static COLOR_G: usize = 3;
static COLOR_B: usize = 4;
static COLOR_A: usize = 5;
static TEX_COORD_U: usize = 6;
static TEX_COORD_V: usize = 7;

pub static VERTEX_SIZE: usize = 8;

#[derive(Clone, Copy, Debug, Default)]
pub struct Vertex {
    pub data: [f32; 8]
    // 0, 1 => position x, y
    // 2, 3, 4, 5 => color r, g, b, a
    // 6, 7 => texture u, v
}

impl Vertex {
    pub fn new(
        x: f32, y: f32,
        r: f32, g: f32, b: f32, a: f32,
        u: f32, v: f32
    ) -> Self {
        Self {
            data: [x, y, r, g, b, a, u, v]
        }
    }

    pub fn position_xy(&self) -> (f32, f32) {
        (self.data[POS_X], self.data[POS_Y])
    }

    pub fn position_x(&self) -> f32 {
        self.data[POS_X]
    }

    pub fn position_y(&self) -> f32 {
        self.data[POS_Y]
    }

    pub fn position(&self) -> Vector2f {
        Vector2f::new(self.data[POS_X], self.data[POS_Y])
    }

    pub fn color(&self) -> Color {
        Color::new(
            self.data[2],
            self.data[3],
            self.data[4],
            self.data[5]
        )
    }

    pub fn tex_coords(&self) -> Vector2f {
        Vector2f::new(self.data[6], self.data[7])
    }

    pub fn set_position_xy(&mut self, x: f32, y: f32) {
        self.data[POS_X] = x;
        self.data[POS_Y] = y;
    }

    pub fn set_position_x(&mut self, x: f32) {
        self.data[POS_X] = x;
    }

    pub fn set_position_y(&mut self, y: f32) {
        self.data[POS_Y] = y;
    }

    pub fn set_position(&mut self, pos: &Vector2f) {
        self.data[POS_X] = pos.x;
        self.data[POS_Y] = pos.y;
    }

    pub fn set_color(&mut self, color: &Color) {
        self.data[COLOR_R] = color.r;
        self.data[COLOR_G] = color.g;
        self.data[COLOR_B] = color.b;
        self.data[COLOR_A] = color.a;
    }

    pub fn set_color_rgba(&mut self, r: f32, g: f32, b: f32, a: f32) {
        self.data[COLOR_R] = r;
        self.data[COLOR_G] = g;
        self.data[COLOR_B] = b;
        self.data[COLOR_A] = a;
    }

    pub fn set_tex_coords_uv(&mut self, u: f32, v: f32) {
        self.data[TEX_COORD_U] = u;
        self.data[TEX_COORD_V] = v;
    }

    pub fn set_tex_coords(&mut self, tex_coords: &Vector2f) {
        self.data[TEX_COORD_U] = tex_coords.x;
        self.data[TEX_COORD_V] = tex_coords.y;
    }

}

