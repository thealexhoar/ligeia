use specs::{Dispatcher, World};


pub type SceneID = usize;

pub struct Scene<'a> {
    _dispatchers: Vec<Box<Dispatcher<'a, 'a>>>,
    _load_fn: Option<fn(&mut World)>,
    _unload_fn: Option<fn(&mut World)>
}

impl<'a> Scene<'a> {
    pub fn new_single(dispatcher: Box<Dispatcher<'a, 'a>>, load_fn: Option<fn(&mut World)>, unload_fn: Option<fn(&mut World)>) -> Self {
        Self {
            _dispatchers: vec![dispatcher],
            _load_fn: load_fn,
            _unload_fn: unload_fn
        }
    }

    pub fn new_multi(dispatchers: Vec<Box<Dispatcher<'a, 'a>>>, load_fn: Option<fn(&mut World)>, unload_fn: Option<fn(&mut World)>) -> Self {
        Self {
            _dispatchers: dispatchers,
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
        for i in 0..self._dispatchers.len() {
            self._dispatchers[i].dispatch(&world.res);
        }
    }
}