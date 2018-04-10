use specs::{Dispatcher, DispatcherBuilder, World};
use std::collections::HashMap;

use game::{Scene, SceneID};
use game::components::*;
use game::resources::*;
use game::systems::*;
use graphics::{ManagedCamera, Window};
use util::{FabricationDef, MasterFabricator};

pub struct Core<'a> {
    _current_scene: SceneID,
    _next_scene: Option<SceneID>,
    _scenes: Vec<Scene<'a>>,
    _window: Window,
    _world: World,
    _master_fabricator: MasterFabricator
}

impl<'a> Core<'a> {
    pub fn new(width: u32, height: u32, internal_width: u32, internal_height: u32, title: &str) -> Self {
        let mut core = Self {
            _current_scene: 0,
            _next_scene: None,
            _scenes: Vec::new(),
            _window: Window::new(width, height, internal_width, internal_height, title),
            _world: World::new(),
            _master_fabricator: MasterFabricator::new()
        };

        core.init_world(internal_width, internal_height);

        let dispatcher_builder = DispatcherBuilder::new();
        let dispatcher = dispatcher_builder.build();
        let null_scene = Scene::new(
            Box::new(dispatcher),
            Some(|world: &mut World| { println!("Loaded null scene!") }),
            Some(|world: &mut World| { println!("Unloaded null scene!") })
        );
        core._scenes.push(null_scene);

        core
    }

    pub fn add_scene(&mut self, scene: Scene<'a>) -> SceneID {
        self._scenes.push(scene);

        self._scenes.len() - 1
    }

    pub fn set_scene(&mut self, scene_id: SceneID) -> bool {
        if self._scenes.len() <= scene_id {
            return false;
        }
        self._next_scene = Some(scene_id);

        true
    }

    pub fn update(&mut self) {

        self._scenes[self._current_scene].update(&self._world);

        self._window.process_events();
        self._window.clear();
        self._window.display();

        if let Some(next_scene) = self._next_scene {
            self._scenes[self._current_scene].unload(&mut self._world);
            self._current_scene = next_scene;
            self._next_scene = None;
            self._scenes[self._current_scene].load(&mut self._world);
        }

        self.update_entities();
    }

    pub fn should_close(&self) -> bool {
        self._window.should_close()
    }

    fn init_world(&mut self, internal_width: u32, internal_height: u32) {
        self._world.register::<ScreenPosition>();
        self._world.register::<WorldPosition>();

        self._world.add_resource(0. as DeltaTime);
        self._world.add_resource(EntitiesToAdd::new());
        self._world.add_resource(EntitiesToKill::new());
        self._world.add_resource(ManagedCamera::new(0., 0., 0., internal_width as f32, internal_height as f32));


        self._master_fabricator.register(WorldPositionFabricator{});


        //test code
        let mut test_f_def = FabricationDef::new();
        test_f_def.add_component(WorldPosition{});
        self._world.write_resource::<EntitiesToAdd>().push(test_f_def);
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