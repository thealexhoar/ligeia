use ncollide2d::world::CollisionGroups;
use ncollide2d::shape::*;
use specs::{Join, ReadExpect, ReadStorage, System, WriteStorage};
use std::any::Any;
use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

use ligeia_graphics::{
    ManagedCamera, PrimitiveType, ShaderHandle, ShaderHandler,
    TextureHandle, TextureHandler, Vertex, Window
};

use game::components::ScreenPosition;
use game::resources::*;
use physics::{PhysicsWorld, meters_to_pix};
use util::{HALF_PI, PI, TWO_PI};


pub struct PhysicsRenderer {
    _shader_handler: Rc<RefCell<ShaderHandler>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _vertices: Vec<Vertex>,
    _window: Rc<RefCell<Window>>
}

impl PhysicsRenderer {
    pub fn new(
        shader_handler: Rc<RefCell<ShaderHandler>>,
        texture_handler: Rc<RefCell<TextureHandler>>,
        window: Rc<RefCell<Window>>,
    ) -> Self {
        Self {
            _shader_handler: shader_handler,
            _texture_handler: texture_handler,
            _vertices: Vec::with_capacity(1024),
            _window: window
        }
    }
}

impl<'a> System<'a> for PhysicsRenderer {
    type SystemData = (
        ReadExpect<'a, DebugSettings>,
        ReadExpect<'a, ManagedCamera>,
        ReadExpect<'a, PhysicsWorld>,
        ReadExpect<'a, VerticesNeeded>,
        ReadExpect<'a, ScreenAABB>
    );

    fn run(
        &mut self,
        (
            debug_settings,
            camera,
            world,
            vertices_needed,
            screen_aabb
        ): Self::SystemData
    ) {
        if debug_settings.render_physics {
            let mut collision_groups = CollisionGroups::new();
            collision_groups.enable_self_interaction();

            let mut vertex_count = 0;

            // implicit screenspace culling
            for collider in world
                .collision_world()
                .interferences_with_aabb(&screen_aabb.aabb, &collision_groups)
                {
                    // note that an isometry represents a rotation THEN a translation
                    let isometry = collider.position();
                    // get shape
                    // transform to fit screen
                    // draw that mofo

                    let shape = collider.shape();

                    if shape.is_shape::<Ball<f32>>() {
                        let ball: &Ball<f32> = shape.as_shape().unwrap();
                        //TODO: add ball rendering
                    }

                    if shape.is_shape::<Cuboid<f32>>() {
                        let cuboid: &Cuboid<f32> = shape.as_shape().unwrap();
                        //TODO: add cuboid rendering

                        let phys_x = isometry.translation.vector.x;
                        let phys_y = isometry.translation.vector.y;

                        let world_angle = -isometry.rotation.angle();

                        let angle_cos = world_angle.cos();
                        let angle_sin = world_angle.sin();

                        let pix_x = meters_to_pix(phys_x);
                        let pix_y = meters_to_pix(phys_y);

                        let pix_h_width = meters_to_pix(cuboid.half_extents().x);
                        let pix_h_height = meters_to_pix(cuboid.half_extents().y);

                        let mut vertices = vec![
                            (-1., -1.),
                            (1., -1.),
                            (1., 1.),
                            (-1., 1.)
                        ].iter().map(
                            |(x_factor, y_factor)| {
                                let local_x = x_factor * pix_h_width;
                                let local_y = y_factor * pix_h_height;
                                let rot_x = local_x * angle_cos - local_y * angle_sin;
                                let rot_y = local_x * angle_sin + local_y * angle_cos;
                                let (vert_x, vert_y) = camera.transform_world_point(
                                    pix_x + rot_x,
                                    pix_y + rot_y
                                );
                                Vertex::new(
                                    vert_x, vert_y,
                                    1.0, 0.05, 0.4, 1.0,
                                    0., 0.
                                )
                            }
                        ).collect::<Vec<Vertex>>();


                        if self._vertices.len() < vertex_count + 8 {
                            let new_len = (self._vertices.len() * 2) + 8;
                            self._vertices.resize(new_len, Vertex::default());
                        }

                        let len = self._vertices.len();
                        self._vertices[vertex_count] = vertices[0];
                        self._vertices[vertex_count + 1] = vertices[1];
                        self._vertices[vertex_count + 2] = vertices[1];
                        self._vertices[vertex_count + 3] = vertices[2];
                        self._vertices[vertex_count + 4] = vertices[2];
                        self._vertices[vertex_count + 5] = vertices[3];
                        self._vertices[vertex_count + 6] = vertices[3];
                        self._vertices[vertex_count + 7] = vertices[0];

                        vertex_count += 8;
                    }
                }

            if vertex_count > 0 {
                let texture_handler = self._texture_handler.borrow();
                //let texture_ref = texture_handler.get_texture(sprite.get_tex_handle()).unwrap();
                let texture_ref = texture_handler.get_blank_texture();
                //let texture_ref = texture_handler.get_simple_texture();

                let shader_handler = self._shader_handler.borrow();
                //let shader_ref = shader_handler.get_shader(sprite.get_shader_handle()).unwrap();
                let shader_ref = shader_handler.get_default().unwrap();

                let mut window = self._window.borrow_mut();

                window.draw_vertices(
                    &self._vertices[0..vertex_count],
                    texture_ref,
                    shader_ref,
                    None,
                    PrimitiveType::LINES
                );
            }
        }
    }
}