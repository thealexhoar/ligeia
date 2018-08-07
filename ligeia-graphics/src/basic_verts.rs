use {Color, Vector2f, Vertex};


/*
  0----1
  |    |
  |    |
  3----2
*/

pub static BASIC_VERTS: [Vertex; 4] = [
    Vertex { data: [
        0., 0.,
        1., 1., 1., 1.,
        0., 0.
    ]},
    Vertex { data: [
        1., 0.,
        1., 1., 1., 1.,
        1., 0.
    ]},
    Vertex { data: [
        1., 1.,
        1., 1., 1., 1.,
        1., 1.
    ]},
    Vertex { data: [
        0., 1.,
        1., 1., 1., 1.,
        0., 1.
    ]},
];


pub static SCREEN_VERTS: [Vertex; 4] = [
    Vertex { data: [
        -1., 1.,
        1., 1., 1., 1.,
        0., 0.
    ]},
    Vertex { data: [
        1., 1.,
        1., 1., 1., 1.,
        1., 0.
    ]},
    Vertex { data: [
        1., -1.,
        1., 1., 1., 1.,
        1., 1.
    ]},
    Vertex { data: [
        -1., -1.,
        1., 1., 1., 1.,
        0., 1.
    ]},
];