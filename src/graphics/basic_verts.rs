use sfml::graphics::{Color, Vertex};
use sfml::system::Vector2f;

pub static BASIC_VERTS: [Vertex; 4] = [
    Vertex {
        position: Vector2f {x: 0., y: 0.},
        color: Color::WHITE,
        tex_coords: Vector2f {x: 0., y: 0.}
    },
    Vertex {
        position: Vector2f {x: 1., y: 0.},
        color: Color::WHITE,
        tex_coords: Vector2f {x: 1., y: 0.}
    },
    Vertex {
        position: Vector2f {x: 1., y: 1.},
        color: Color::WHITE,
        tex_coords: Vector2f {x: 1., y: 1.}
    },
    Vertex {
        position: Vector2f {x: 0., y: 1.},
        color: Color::WHITE,
        tex_coords: Vector2f {x: 0., y: 1.}
    },
];