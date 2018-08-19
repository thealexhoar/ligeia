use specs::{Builder, Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use util::Fabricator;

#[derive(Clone, Copy, Component, Debug)]
#[component(VecStorage)]
pub struct ScreenPosition {
    pub x: f32,
    pub y: f32,
    pub theta: f32,
    pub camera_theta: f32,
    pub vertex_index: Option<usize>
}

impl ScreenPosition {
    pub fn new() -> Self {
        Self {
            x: 0.,
            y: 0.,
            theta: 0.,
            camera_theta: 0.,
            vertex_index: Option::None
        }
    }
}

pub struct ScreenPositionFabricator;

impl Fabricator for ScreenPositionFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<ScreenPosition>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(screen_pos) = data.downcast::<ScreenPosition>() {
            entity_builder.with(*screen_pos.deref())
        }
        else {
            entity_builder
        }
    }
}