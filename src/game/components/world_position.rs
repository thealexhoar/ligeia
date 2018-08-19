use specs::{Builder, Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use util::Fabricator;

#[derive(Clone, Copy, Component, Debug)]
#[component(VecStorage)]
pub struct WorldPosition {
    pub x: f32,
    pub y: f32,
    pub theta: f32
}

impl WorldPosition {
    pub fn new(x: f32, y: f32, theta: f32) -> Self {
        Self {x, y, theta}
    }
}

pub struct WorldPositionFabricator;

impl Fabricator for WorldPositionFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<WorldPosition>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(world_pos) = data.downcast::<WorldPosition>() {
            entity_builder.with(*world_pos.deref()) // data is copied
        }
        else {
            entity_builder
        }
    }
}


