use sfml::graphics::FloatRect;
use specs::{Dispatcher, DispatcherBuilder, World};
use std::cell::RefCell;
use std::collections::HashMap;
use std::ops::{Deref, DerefMut};
use std::rc::Rc;

use game::{Scene, SceneID};
use game::components::*;
use game::resources::*;
use game::scenes::*;
use game::systems::*;
use graphics::{ManagedCamera, ShaderHandler, Sprite, TextureHandler, Window};
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
        t_h.load_texture("assets/textures/dungeon_sheet.png");

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

        {
            let mut window= self._window.borrow_mut();
            window.process_events();
            window.display();
        }

        let next_scene = self._world.read_resource::<NextScene>().deref().clone();

        if let Some(next_scene_id) = next_scene {
            self.try_change_scene(next_scene_id);
        }

        self.update_entities();

        self._world.write_resource::<ManagedCamera>().theta += 0.001;
    }

    pub fn should_close(&self) -> bool {
        self._window.borrow().deref().should_close()
    }

    fn init_world(&mut self, internal_width: u32, internal_height: u32) {
        self._world.register::<ScreenPosition>();
        self._world.register::<SpriteRenderable>();
        self._world.register::<WorldPosition>();

        self._world.add_resource(0. as DeltaTime);
        self._world.add_resource(EntitiesToAdd::new());
        self._world.add_resource(EntitiesToKill::new());
        self._world.add_resource(ManagedCamera::new(0., 0., 0., internal_width as f32, internal_height as f32));
        self._world.add_resource(Some(1 as usize) as NextScene);

        self._master_fabricator.register(WorldPositionFabricator);
        self._master_fabricator.register(ScreenPositionFabricator);
        self._master_fabricator.register(SpriteRenderableFabricator);


        //test code
        let mut test_f_def = FabricationDef::new();
        test_f_def.add_component(WorldPosition{x: 0., y: 0., theta: 0.});
        test_f_def.add_component(ScreenPosition{x: 0., y: 0., theta: 0.});
        test_f_def.add_component(SpriteRenderable::new(
            Sprite::new_centered(328., 160., 0)
        ));
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