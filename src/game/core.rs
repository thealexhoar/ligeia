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
use graphics::{LayerDef, LayeredSprite, ManagedCamera, Renderable, ShaderHandler, Sprite, TextureHandler, Window};
use util::{FabricationDef, FloatRect, MasterFabricator};

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
    pub fn new(width: u32, height: u32, internal_width: u32, internal_height: u32, title: &str) -> Self {
        let shader_handler = Rc::new(RefCell::new(ShaderHandler::new()));
        let texture_handler = Rc::new(RefCell::new(TextureHandler::new()));
        let window = Rc::new(RefCell::new(Window::new(width, height, internal_width, internal_height, title)));

        let mut core = Self {
            _current_scene: 0,
            _scenes: HashMap::new(),
            _shader_handler: Rc::clone(&shader_handler),
            _texture_handler: Rc::clone(&texture_handler),
            _window: Rc::clone(&window),
            _world: World::new(),
            _master_fabricator: MasterFabricator::new()
        };

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

        let mut t_h = texture_handler.borrow_mut();
        t_h.load_texture("assets/textures/crate_small.png");

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
            let mut window= self._window.borrow_mut();
            window.clear();
        }

        if self._current_scene > 0 {
            self._scenes.get_mut(&self._current_scene).unwrap().update(&self._world);
        }


        let dt = {
            //borrow window inside a smaller scope, to drop the borrow at the end
            let mut window = self._window.borrow_mut();
            window.process_events();
            window.display();

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
        self._world.write_resource::<ManagedCamera>().theta += 0.005;
        self._world.write_resource::<ManagedCamera>().x += 0.01;
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
        self._world.add_resource(ManagedCamera::new(10., 0., 0., internal_width as f32, internal_height as f32));
        self._world.add_resource(Some(1 as usize) as NextScene);
        self._world.add_resource(0 as VerticesNeeded);

        self._master_fabricator.register(ScreenPositionFabricator);
        self._master_fabricator.register(WorldPositionFabricator);
        self._master_fabricator.register(WorldRenderableFabricator);


        let mut layer_def_1 = LayerDef { layers: HashMap::new() };
        layer_def_1.layers.insert(0, FloatRect::new_square(0., 0., 16.));
        layer_def_1.layers.insert(1, FloatRect::new_square(16., 0., 16.));
        layer_def_1.layers.insert(2, FloatRect::new_square(32., 0., 16.));
        layer_def_1.layers.insert(3, FloatRect::new_square(32., 0., 16.));
        layer_def_1.layers.insert(4, FloatRect::new_square(32., 0., 16.));
        layer_def_1.layers.insert(5, FloatRect::new_square(16., 0., 16.));
        layer_def_1.layers.insert(6, FloatRect::new_square(48., 0., 16.));

        let mut layer_def_2 = LayerDef { layers: HashMap::new() };
        layer_def_2.layers.insert(0, FloatRect::new_square(0., 0., 8.));
        layer_def_2.layers.insert(1, FloatRect::new_square(8., 0., 8.));
        layer_def_2.layers.insert(2, FloatRect::new_square(8., 0., 8.));
        layer_def_2.layers.insert(3, FloatRect::new_square(8., 0., 8.));
        layer_def_2.layers.insert(4, FloatRect::new_square(16., 0., 8.));
        layer_def_2.layers.insert(5, FloatRect::new_square(24., 0., 8.));

        let renderable = Arc::new(LayeredSprite::new(0., 0., 8., 8., &layer_def_2)) as Arc<Renderable + Sync + Send>;


        //test code
        let sq = 122;
        for i in (-sq / 2)..sq/2 {
            for j in (-sq / 2)..sq/2 {
                let mut test_f_def = FabricationDef::new();
                test_f_def.add_component(WorldPosition::new(12. * i as f32, 12. * j as f32, (i as f32).atan2(j as f32)));
                test_f_def.add_component(ScreenPosition::new());
                test_f_def.add_component(WorldRenderable::new(renderable.clone()));
                self._world.write_resource::<EntitiesToAdd>().push(test_f_def);
            }
        }
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