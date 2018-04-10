use specs::{Dispatcher, World};


pub type SceneID = usize;

pub struct Scene<'a> {
    _dispatcher: Box<Dispatcher<'a, 'a>>,
    _load_fn: Option<fn(&mut World)>,
    _unload_fn: Option<fn(&mut World)>
}

impl<'a> Scene<'a> {
    pub fn new(dispatcher: Box<Dispatcher<'a, 'a>>, load_fn: Option<fn(&mut World)>, unload_fn: Option<fn(&mut World)>) -> Self {
        Self {
            _dispatcher: dispatcher,
            _load_fn: load_fn,
            _unload_fn: unload_fn
        }
    }

    pub fn load(&self, world: &mut World) {
        if let Some(load_fn) = self._load_fn {
            load_fn(world)
        }
    }

    pub fn unload(&self, world: &mut World) {
        if let Some(unload_fn) = self._unload_fn {
            unload_fn(world)
        }
    }

    pub fn update(&mut self, world: &World) {
        self._dispatcher.dispatch(&world.res);
    }
}