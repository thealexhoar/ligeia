use specs::{Component, Entity, World};
use std::any::{Any, TypeId};
use std::collections::HashMap;

pub trait Deconstructor {
    fn deconstruct(&self, entity: &Entity, world: &mut World);
}

pub struct MasterDeconstructor {
    _deconstructors: HashMap<TypeId, Box<Deconstructor>>
}

impl MasterDeconstructor {
    pub fn new() -> Self {
        Self {
            _deconstructors: HashMap::new()
        }
    }

    pub fn register<T: Deconstructor + 'static>(&mut self, deconstructor: T) {
        let type_id = TypeId::of::<T>();
        if !self._deconstructors.contains_key(&type_id) {
            self._deconstructors.insert(type_id, Box::new(deconstructor));
        }
    }

    pub fn deconstruct(&self, entity: &Entity, world: &mut World) {
        for (_, deconstructor) in self._deconstructors.iter() {
            deconstructor.deconstruct(entity, world);
        }
    }
}