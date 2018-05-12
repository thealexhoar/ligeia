use ligeia_softcode::graphics::{LayerDef, TextureDef};
use ligeia_utils::rect::{FloatRect, UIntRect};
use specs::{Dispatcher, DispatcherBuilder, World};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;
use std::sync::Arc;

use game::{Scene, SceneID};
use game::components::*;
use game::resources::*;
use game::scenes::*;
use game::systems::*;
use graphics::{DirectionalSprite, LayeredSprite, ManagedCamera, Renderable, ShaderHandler, Sprite, TextureHandler, Window};
use util::{FabricationDef, MasterFabricator};

pub struct Core<'a> {
    _current_scene: SceneID,
    _scenes: HashMap<SceneID, Scene<'a>>,
    _shader_handler: Rc<RefCell<ShaderHandler<'a>>>,
    _texture_handler: Rc<RefCell<TextureHandler>>,
    _window: Rc<RefCell<Window>>,
    _world: World,
    _master_fabricator: MasterFabricator
}

impl<'a> Core<'a> {
    pub fn new(width: u32, height: u32, internal_width: u32, internal_height: u32, pixel_factor: u32, title: &str) -> Self {
        let shader_handler = Rc::new(RefCell::new(ShaderHandler::new()));
        let texture_handler = Rc::new(RefCell::new(TextureHandler::new()));
        let window = Rc::new(RefCell::new(Window::new(width, height, internal_width, internal_height, pixel_factor, title)));

        let mut core = Self {
            _current_scene: 0,
            _scenes: HashMap::new(),
            _shader_handler: Rc::clone(&shader_handler),
            _texture_handler: Rc::clone(&texture_handler),
            _window: Rc::clone(&window),
            _world: World::new(),
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
                TextureDef {
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
                },
                TextureDef {
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
                },
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
        let null_scene = Scene::new(
            Box::new(dispatcher),
            Some(|world: &mut World| {
                println!("Loaded null scene!");
                *(world.write_resource::<NextScene>().deref_mut()) = Some(2);
            }),
            Some(|world: &mut World| { println!("Unloaded null scene!") })
        );
        core._scenes.insert(1, null_scene);

        core._scenes.insert(
            2,
            testbed(
                Rc::clone(&shader_handler),
                Rc::clone(&texture_handler),
                Rc::clone(&window)
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
        *(self._world.write_resource::<NextScene>().deref_mut()) = None;
        self._scenes[&self._current_scene].load(&mut self._world);

        true
    }

    pub fn update(&mut self) {
        {
            let mut window = self._window.borrow_mut();
            window.clear();
        }

        if self._current_scene > 0 {
            self._scenes.get_mut(&self._current_scene).unwrap().update(&self._world);
        }


        let dt = {
            //borrow window inside a smaller scope, to drop the borrow at the end
            let mut window = self._window.borrow_mut();
            window.process_events();
            window.display(&self._shader_handler.deref().borrow());

            //and snag delta_time while we have a borrow on window
            window.delta_time()
        };
        *(self._world.write_resource::<DeltaTime>().deref_mut()) = dt;


        let next_scene = self._world.read_resource::<NextScene>().deref().clone();
        if let Some(next_scene_id) = next_scene {
            self.try_change_scene(next_scene_id);
        }

        self.update_entities();

        //test
        self._world.write_resource::<ManagedCamera>().theta += 0.02;
    }

    pub fn should_close(&self) -> bool {
        self._window.borrow().deref().should_close()
    }

    fn init_world(&mut self, internal_width: u32, internal_height: u32) {
        self._world.register::<ScreenPosition>();
        self._world.register::<WorldRenderable>();
        self._world.register::<WorldPosition>();

        self._world.add_resource(0. as DeltaTime);
        self._world.add_resource(EntitiesToAdd::new());
        self._world.add_resource(EntitiesToKill::new());
        self._world.add_resource(ManagedCamera::new(0., 0., 0., internal_width as f32, internal_height as f32));
        self._world.add_resource(Some(1 as usize) as NextScene);
        self._world.add_resource(0 as VerticesNeeded);

        self._master_fabricator.register(ScreenPositionFabricator);
        self._master_fabricator.register(WorldPositionFabricator);
        self._master_fabricator.register(WorldRenderableFabricator);

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

        //test code
        let sq = 5;
        let mut iter = 0;
        for i in (-sq / 2)..sq / 2 {
            for j in (-sq / 2)..sq / 2 {
                if i != -sq / 2 && i != sq / 2 - 1 && j != -sq / 2 && j != sq / 2 - 1 {
                    continue;
                }
                let mut test_f_def = FabricationDef::new();
                test_f_def.add_component(WorldPosition::new(20. * i as f32, 20. * j as f32, (i as f32 * 11.7) + (j as f32 * 3.9)));
                test_f_def.add_component(ScreenPosition::new());

                match iter % 3 {
                    0 => test_f_def.add_component(WorldRenderable::new(crate_renderable.clone())),
                    1 => test_f_def.add_component(WorldRenderable::new(barrel_renderable.clone())),
                    _ => test_f_def.add_component(WorldRenderable::new(crate_wide_renderable.clone()))
                };
                self._world.write_resource::<EntitiesToAdd>().push(test_f_def);


                iter += 1;
            }
        }
        let mut gordy_def = FabricationDef::new();
        gordy_def.add_component(WorldPosition::new(0., 0., 0.));
        gordy_def.add_component(ScreenPosition::new());
        gordy_def.add_component(WorldRenderable::new(player_renderable.clone()));
        self._world.write_resource::<EntitiesToAdd>().push(gordy_def);
    }

    fn update_entities(&mut self) {
        while self._world.read_resource::<EntitiesToAdd>().len() > 0 {
            let f_def = self._world.write_resource::<EntitiesToAdd>().pop().unwrap();
            self._master_fabricator.build(f_def, &mut self._world);
        }

        while self._world.read_resource::<EntitiesToKill>().len() > 0 {
            let entity = self._world.write_resource::<EntitiesToKill>().pop().unwrap();
            self._world.delete_entity(entity);
        }

        self._world.maintain();
    }
}