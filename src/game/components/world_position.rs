use specs::{Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use util::Fabricator;

#[derive(Clone, Copy, Component, Debug)]
#[component(VecStorage)]
pub struct WorldPosition {}

pub struct WorldPositionFabricator;

impl Fabricator for WorldPositionFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<WorldPosition>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(world_pos) = data.downcast::<WorldPosition>() {
            println!("built world position!");
            entity_builder.with(*world_pos.deref())
        }
        else {
            entity_builder
        }
    }
}


