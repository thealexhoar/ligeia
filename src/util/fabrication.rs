use specs::{Component, Entity, EntityBuilder, World};
use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::sync::Arc;

pub trait Fabricator {
    fn get_type_id(&self) -> TypeId;
    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a>;
}

pub struct FabricationDef {
    _component_defs: HashMap<TypeId, Arc<Box<Any + Send + Sync + 'static>>>
}

impl FabricationDef {
    pub fn new() -> Self {
        Self {_component_defs: HashMap::new()}
    }

    pub fn add_component<T: Component + Send + Sync + 'static>(&mut self, component: T) {
        let type_id = TypeId::of::<T>();
        let boxed = Arc::new(Box::new(component) as Box<Any + Send + Sync + 'static>);
        self._component_defs.insert(type_id, boxed);
    }
}

pub struct MasterFabricator {
    _fabricators: HashMap<TypeId, Box<Fabricator>>
}

impl MasterFabricator {
    pub fn new() -> Self {
        Self {
            _fabricators: HashMap::new()
        }
    }

    pub fn register<T: Fabricator + 'static>(&mut self, fabricator: T) {
        let type_id = fabricator.get_type_id();
        self._fabricators.insert(type_id, Box::new(fabricator));
    }

    pub fn build(&self, mut fabrication_def: FabricationDef, world: &mut World) -> Entity {
        let mut entity_builder = world.create_entity();

        let keys = fabrication_def._component_defs.keys().map(|type_id: &TypeId| *type_id).collect::<Vec<TypeId>>();

        for type_id in keys {
            if let Some(data) = fabrication_def._component_defs.remove(&type_id) {
                if let Ok(data_box) = Arc::try_unwrap(data) {
                    let data_box = data_box as Box<Any>;
                    if let Some(fabricator) = self._fabricators.get(&type_id) {
                        entity_builder = fabricator.build_onto(data_box, entity_builder);
                    }
                    else {
                        //TODO? remove
                        panic!("Missing a fabricator!");
                    }
                }
            }
        }

        entity_builder.build()
    }
}


