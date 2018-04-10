use specs::{Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use util::Fabricator;

#[derive(Clone, Copy, Component, Debug)]
#[component(VecStorage)]
pub struct ScreenPosition {}

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