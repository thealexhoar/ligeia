use na::{
    geometry::Point2,
    Vector2
};
use ncollide2d::{
    bounding_volume::aabb::AABB,
    shape::{Ball, Cuboid, ShapeHandle}
};
use sdl2::{
    event::Event,
    EventPump,
    GameControllerSubsystem,
    keyboard::Scancode,
    rect::Point,
    VideoSubsystem
};
use specs::{Dispatcher, DispatcherBuilder, World};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use ligeia_graphics::{
    DirectionalSprite, LayeredSprite,
    ManagedCamera,
    Renderable,
    ShaderHandler,
    Sprite,
    Texture, TextureHandler,
    Vertex,
    Window
};
use ligeia_softcode::graphics::{LayerDef, TextureDef};
use ligeia_utils::rect::{FloatRect, UIntRect};

use game::{Scene, SceneID};
use game::components::*;
use game::resources::*;
use game::scenes::*;
use game::systems::*;
use input::{Axis, Button, KeyboardController};
use physics::{BodyDef, BodyType, ColliderDef, construct_world, METERS_PER_PIX, PhysicsWorld};
use util::{FabricationDef, MasterDeconstructor, MasterFabricator};

pub struct Core<'a> {
    _current_scene: SceneID,
    _scenes: HashMap<SceneID, Scene<'a>>,
    _shader_handler: Rc<RefCell<ShaderHandler>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _window: Rc<RefCell<Window>>,
    _world: World,
    _master_deconstructor: MasterDeconstructor,
    _master_fabricator: MasterFabricator
}

impl<'a> Core<'a> {
    pub fn new(
        sdl_video: &VideoSubsystem,
        width: u32,
        height: u32,
        internal_width: u32,
        internal_height: u32,
        use_vsync: bool,
        title: &str
    ) -> Self {
        let window = Rc::new(RefCell::new(
            Window::new(
                sdl_video,
                width,
                height,
                internal_width,
                internal_height,
                use_vsync,
                title
            )
        ));



        let shader_handler = Rc::new(RefCell::new(ShaderHandler::new()));
        let texture_handler = Rc::new(RefCell::new(TextureHandler::new()));
        let physics_world = construct_world();

        let mut core = Self {
            _current_scene: 0,
            _scenes: HashMap::new(),
            _shader_handler: Rc::clone(&shader_handler),
            _texture_handler: Rc::clone(&texture_handler),
            _window: Rc::clone(&window),
            _world: World::new(),
            _master_deconstructor: MasterDeconstructor::new(),
            _master_fabricator: MasterFabricator::new()
        };

        {
            let mut t_h = texture_handler.borrow_mut();
            let texture_defs = vec![
                TextureDef {
                    filename: String::from("assets/textures/crate.png"),
                    frames: vec![
                        UIntRect::new_square(0, 0, 11),
                        UIntRect::new_square(11, 0, 11),
                        UIntRect::new_square(22, 0, 11),
                        UIntRect::new_square(33, 0, 11),
                        UIntRect::new_square(44, 0, 11),
                    ]
                },
                TextureDef {
                    filename: String::from("assets/textures/crate_wide.png"),
                    frames: vec![
                        UIntRect::new(0, 0, 21, 11),
                        UIntRect::new(21, 0, 21, 11),
                        UIntRect::new(42, 0, 21, 11),
                        UIntRect::new(63, 0, 21, 11),
                        UIntRect::new(84, 0, 21, 11),
                    ]
                },
                TextureDef {
                    filename: String::from("assets/textures/barrel.png"),
                    frames: vec![
                        UIntRect::new_square(0, 0, 8),
                        UIntRect::new_square(8, 0, 8),
                        UIntRect::new_square(16, 0, 8),
                    ]
                },
                TextureDef {
                    filename: String::from("assets/textures/barrel_metal.png"),
                    frames: vec![
                        UIntRect::new_square(0, 0, 8),
                        UIntRect::new_square(8, 0, 8),
                        UIntRect::new_square(16, 0, 8),
                    ]
                },
                /*TextureDef {
                    filename: String::from("assets/textures/gordy.png"),
                    frames: vec![
                        UIntRect::new_square(0, 0, 16),
                        UIntRect::new_square(16, 0, 16),
                        UIntRect::new_square(32, 0, 16),
                        UIntRect::new_square(48, 0, 16),
                        UIntRect::new_square(64, 0, 16),
                        UIntRect::new_square(80, 0, 16),
                        UIntRect::new_square(96, 0, 16),
                        UIntRect::new_square(112, 0, 16),
                    ]
                },*/
                /*TextureDef {
                    filename: String::from("assets/textures/pedro.png"),
                    frames: vec![
                        UIntRect::new(0, 0, 16, 18),
                        UIntRect::new(16, 0, 16, 18),
                        UIntRect::new(32, 0, 16, 18),
                        UIntRect::new(48, 0, 16, 18),
                        UIntRect::new(64, 0, 16, 18),
                        UIntRect::new(80, 0, 16, 18),
                        UIntRect::new(96, 0, 16, 18),
                        UIntRect::new(112, 0, 16, 18),
                    ]
                },*/
                TextureDef {
                    filename: String::from("assets/textures/terrence.png"),
                    frames: vec![
                        UIntRect::new(0, 0, 9, 12),
                        UIntRect::new(9, 0, 9, 12),
                        UIntRect::new(18, 0, 9, 12),
                        UIntRect::new(27, 0, 9, 12),
                        UIntRect::new(36, 0, 9, 12),
                        UIntRect::new(45, 0, 9, 12),
                        UIntRect::new(54, 0, 9, 12),
                        UIntRect::new(63, 0, 9, 12),
                    ]
                },
            ];
            t_h.create_master_texture(texture_defs, core._shader_handler.borrow().deref());
        }

        core.init_world(internal_width, internal_height);

        let dispatcher_builder = DispatcherBuilder::new();
        let dispatcher = dispatcher_builder.build();
        let null_scene = Scene::new_single(
            Box::new(dispatcher),
            Some(|world: &mut World| {
                println!("Loaded null scene!");
                (*world.write_resource::<NextScene>().deref_mut()).id = Some(2)
            }),
            Some(|world: &mut World| { println!("Unloaded null scene!") })
        );
        core._scenes.insert(1, null_scene);

        core._scenes.insert(
            2,
            testbed(
                &shader_handler,
                &texture_handler,
                &window,
                &mut core._world
            )
        );


        core
    }

    fn try_change_scene(&mut self, next_scene_id: SceneID) -> bool {
        if !self._scenes.contains_key(&next_scene_id) {
            return false;
        }


        if let Some(scene) = self._scenes.get_mut(&self._current_scene) {
            scene.unload(&mut self._world);
        }
        self._current_scene = next_scene_id;
        (*self._world.write_resource::<NextScene>().deref_mut()).id = None;
        self._scenes[&self._current_scene].load(&mut self._world);

        true
    }

    pub fn update(
        &mut self,
        event_pump: &mut EventPump,
        controller_subsystem: &GameControllerSubsystem
    ) -> bool {

        self._world.write_resource::<ControllerBox>().deref_mut().update(
            event_pump,
            controller_subsystem
        );

        {
            let mut window = self._window.borrow_mut();
            window.begin();
            window.clear_with_rgba(1., 0.8, 0.5, 1.);
        }

        if self._current_scene > 0 {
            self._scenes.get_mut(&self._current_scene).unwrap().update(&self._world);
        }

        let mut should_close = false;
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} => {should_close = true},
                _ => {}
            }
        }


        let dt = {
            //borrow window inside a smaller scope, to drop the borrow at the end
            let mut window = self._window.borrow_mut();
            let shader_handler = self._shader_handler.borrow();
            let shader = shader_handler.get_default().unwrap();
            let texture_handler = self._texture_handler.borrow();
            let texture = texture_handler.get_master_texture();

            window.draw_framebuffer(shader);
            //window.draw_single_texture(shader, texture);
            window.display();

            //and snag delta_time while we have a borrow on window
            window.delta_time()
        };
        (*self._world.write_resource::<DeltaTime>().deref_mut()).dt = dt;


        let next_scene = self._world.read_resource::<NextScene>().deref().clone();
        if let Some(next_scene_id) = next_scene.id {
            self.try_change_scene(next_scene_id);
        }

        self.update_entities();

        //test

        should_close
    }

    fn init_world(&mut self, internal_width: u32, internal_height: u32) {
        self._world.register::<CameraFollowFlag>();
        self._world.register::<PhysicsObject>();
        self._world.register::<PlayerFlag>();
        self._world.register::<ScreenPosition>();
        self._world.register::<WorldRenderable>();
        self._world.register::<WorldPosition>();

        let mut debug_settings = DebugSettings::new();
        debug_settings.render_physics = false;


        let keyboard_controller = KeyboardController::new(
            [
                (Axis::MOVE_X, (Scancode::A, Scancode::D)),
                (Axis::MOVE_Y, (Scancode::W, Scancode::S)),
                (Axis::CAMERA_X, (Scancode::Q, Scancode::E)),
                (Axis::CAMERA_Y, (Scancode::Q, Scancode::E)), //unneeded?
            ].iter().cloned().collect(),
            [
                (Button::MENU, Scancode::Tab),
                (Button::PAUSE, Scancode::Escape)
            ].iter().cloned().collect()
        );

        self._world.add_resource(Box::new(keyboard_controller) as ControllerBox);
        self._world.add_resource(debug_settings);
        self._world.add_resource(DeltaTime::new());
        self._world.add_resource(EntitiesToAdd::new());
        self._world.add_resource(EntitiesToKill::new());
        self._world.add_resource(EntityCount::new());
        self._world.add_resource(MajorEntities::new());
        self._world.add_resource(ManagedCamera::new(50., 0., 0., internal_width as f32, internal_height as f32));
        self._world.add_resource(NextScene::with_id(1));
        self._world.add_resource(PhysicsTimeAccumulator::new());
        self._world.add_resource(construct_world());
        let h_diag = 0.5 * ((internal_width as f32).powi(2) + (internal_height as f32).powi(2)).sqrt();
        self._world.add_resource(ScreenAABB::new(
            -h_diag * METERS_PER_PIX,
            -h_diag * METERS_PER_PIX,
            h_diag * METERS_PER_PIX,
            h_diag  * METERS_PER_PIX
        ));
        self._world.add_resource(VerticesNeeded::new());

        self._master_fabricator.register(PhysicsObjectFabricator);
        self._master_fabricator.register(ScreenPositionFabricator);
        self._master_fabricator.register(WorldPositionFabricator);
        self._master_fabricator.register(WorldRenderableFabricator);

        self._master_fabricator.register(FlagFabricator::<CameraFollowFlag>::default());
        self._master_fabricator.register(FlagFabricator::<PlayerFlag>::default());

        let mut crate_layers = LayerDef { layers: HashMap::new() };
        let mut crate_wide_layers = LayerDef { layers: HashMap::new() };
        let mut barrel_layers = LayerDef { layers: HashMap::new() };

        {
            let texture_handler = self._texture_handler.borrow();



            let texture_rects = texture_handler.deref().get_subrects(String::from("assets/textures/crate.png"));
            crate_layers.layers.insert(0, texture_rects[0]);
            crate_layers.layers.insert(1, texture_rects[1]);
            crate_layers.layers.insert(2, texture_rects[2]);
            crate_layers.layers.insert(3, texture_rects[2]);
            crate_layers.layers.insert(4, texture_rects[2]);
            crate_layers.layers.insert(5, texture_rects[3]);
            crate_layers.layers.insert(6, texture_rects[4]);

            let texture_rects = texture_handler.deref().get_subrects(String::from("assets/textures/crate_wide.png"));
            crate_wide_layers.layers.insert(0, texture_rects[0]);
            crate_wide_layers.layers.insert(1, texture_rects[1]);
            crate_wide_layers.layers.insert(2, texture_rects[2]);
            crate_wide_layers.layers.insert(3, texture_rects[2]);
            crate_wide_layers.layers.insert(4, texture_rects[2]);
            crate_wide_layers.layers.insert(5, texture_rects[3]);
            crate_wide_layers.layers.insert(6, texture_rects[4]);

            /*
            let texture_rects = texture_handler.deref().get_subrects(String::from("assets/textures/barrel.png"));
            barrel_layers.layers.insert(0, texture_rects[0]);
            barrel_layers.layers.insert(1, texture_rects[2]);
            barrel_layers.layers.insert(2, texture_rects[1]);
            barrel_layers.layers.insert(3, texture_rects[2]);
            barrel_layers.layers.insert(4, texture_rects[3]);
            */

            let texture_rects = texture_handler.deref().get_subrects(String::from("assets/textures/barrel_metal.png"));
            barrel_layers.layers.insert(0, texture_rects[0]);
            barrel_layers.layers.insert(1, texture_rects[0]);
            barrel_layers.layers.insert(2, texture_rects[1]);
            barrel_layers.layers.insert(3, texture_rects[0]);
            barrel_layers.layers.insert(4, texture_rects[0]);
            barrel_layers.layers.insert(5, texture_rects[1]);
            barrel_layers.layers.insert(6, texture_rects[0]);
            barrel_layers.layers.insert(7, texture_rects[0]);
            barrel_layers.layers.insert(8, texture_rects[2]);

        }

        let crate_renderable = Arc::new(LayeredSprite::new(0., 0., 11., 11., &crate_layers)) as Arc<Renderable + Sync + Send>; //crate
        let crate_wide_renderable = Arc::new(LayeredSprite::new(0., 0., 21., 11., &crate_wide_layers)) as Arc<Renderable + Sync + Send>; //crate_wide
        let barrel_renderable = Arc::new(LayeredSprite::new(0., 0., 8., 8., &barrel_layers)) as Arc<Renderable + Sync + Send>; // barrel

        let direction_rects = {
            let texture_handler = self._texture_handler.borrow();
            (*texture_handler.deref().get_subrects(String::from("assets/textures/terrence.png"))).clone()
        };
        let player_renderable = Arc::new(DirectionalSprite::new(0., 6., 9., 12., &(direction_rects)[0..8])); //terrence

        ///////////////////////////////////////////////////////////
        // ##### ##### ##### #####       ##### ##### ####  ##### //
        //   #   #     #       #         #     #   # #   # #     //
        //   #   ##### #####   #         #     #   # #   # ##### //
        //   #   #         #   #         #     #   # #   # #     //
        //   #   ##### #####   #         ##### ##### ####  ##### //
        ///////////////////////////////////////////////////////////
        let sq = 250;
        let mut iter = 0;
        for i in (-sq / 2)..sq / 2 {
            for j in (-sq / 2)..sq / 2 {
                if !(i >= 2 || j >= 2 || i <= -2 || j <=-2) {
                    continue;
                }
                let mut test_f_def = FabricationDef::new();
                let x = 20. * i as f32 + 0.001 * j as f32;
                let y = 20. * j as f32 + 0.001 * i as f32;
                let theta = (i as f32 * 11.7) + (j as f32 * 3.9);
                //test_f_def.add_component(WorldPosition::new(x, y, theta));
                test_f_def.add_component(WorldPosition::new(0., 0., 0.));
                test_f_def.add_component(ScreenPosition::new());

                match iter % 3 {
                    0 => {
                        test_f_def.add_component(WorldRenderable::new(crate_renderable.clone()));
                        let shape_handle = ShapeHandle::new(Cuboid::new(Vector2::new(0.55, 0.55)));
                        let mut collider_def = ColliderDef::new(shape_handle);
                        let mut body_def = BodyDef::new(BodyType::Static, collider_def);
                        body_def.x = x * METERS_PER_PIX;
                        body_def.y = y * METERS_PER_PIX;
                        body_def.theta = theta;
                        test_f_def.add_component(PhysicsObject::new(&body_def));
                    },
                    1 => {
                        test_f_def.add_component(WorldRenderable::new(barrel_renderable.clone()));
                        let shape_handle = ShapeHandle::new(Ball::new(0.4));
                        let mut collider_def = ColliderDef::new(shape_handle);
                        let mut body_def = BodyDef::new(BodyType::Static, collider_def);
                        body_def.x = x * METERS_PER_PIX;
                        body_def.y = y * METERS_PER_PIX;
                        body_def.theta = theta;
                        test_f_def.add_component(PhysicsObject::new(&body_def));
                    },
                    _ => {
                        test_f_def.add_component(WorldRenderable::new(crate_wide_renderable.clone()));
                        let shape_handle = ShapeHandle::new(Cuboid::new(Vector2::new(1.05, 0.55)));
                        let mut collider_def = ColliderDef::new(shape_handle);
                        let mut body_def = BodyDef::new(BodyType::Static, collider_def);
                        body_def.x = x * METERS_PER_PIX;
                        body_def.y = y * METERS_PER_PIX;
                        body_def.theta = theta;
                        test_f_def.add_component(PhysicsObject::new(&body_def));
                    }
                };
                self._world.write_resource::<EntitiesToAdd>().entities.push(test_f_def);
                iter += 1;
            }
        }
        let mut player_def = FabricationDef::new();
        player_def.add_component(CameraFollowFlag);
        player_def.add_component(PlayerFlag);
        player_def.add_component(WorldPosition::new(0., 0., 0.));
        player_def.add_component(ScreenPosition::new());
        player_def.add_component(WorldRenderable::new(player_renderable.clone()));

        //let shape_handle = ShapeHandle::new(Cuboid::new(Vector2::new(0.5, 0.5)));
        let shape_handle = ShapeHandle::new(Ball::new(0.3));
        let mut collider_def = ColliderDef::new(shape_handle);
        collider_def.material.friction = 0.;
        let mut body_def = BodyDef::new(BodyType::Dynamic, collider_def);
        //body_def.linear_velocity = (0.001, 0.);
        body_def.fixed_rotation = true;
        //body_def.angular_velocity = 2.;

        player_def.add_component(PhysicsObject::new(&body_def));
        self._world.write_resource::<EntitiesToAdd>().entities.push(player_def);
    }

    fn update_entities(&mut self) {
        let mut entity_count = self._world.read_resource::<EntityCount>().count;
        let mut altered_count = false;

        while self._world.read_resource::<EntitiesToAdd>().entities.len() > 0 {
            let f_def = self._world.write_resource::<EntitiesToAdd>().entities.pop().unwrap();
            self._master_fabricator.build(f_def, &mut self._world);
            entity_count += 1;
            altered_count = true;
        }

        while self._world.read_resource::<EntitiesToKill>().entities.len() > 0 {
            let entity = self._world.write_resource::<EntitiesToKill>().entities.pop().unwrap();
            self._master_deconstructor.deconstruct(&entity, &mut self._world);
            self._world.delete_entity(entity);
            entity_count -= 1;
            altered_count = true;
        }

        if altered_count {
            self._world.write_resource::<EntityCount>().count = entity_count;
            println!("entity_count = {}", entity_count);
        }

        self._world.maintain();
    }
}
