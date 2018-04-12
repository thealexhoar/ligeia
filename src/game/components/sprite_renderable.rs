use specs::{Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use graphics::Sprite;
use util::Fabricator;

#[derive(Clone, Copy, Component, Debug)]
#[component(VecStorage)]
pub struct SpriteRenderable {
    pub sprite: Sprite
}

impl SpriteRenderable {
    pub fn new(sprite: Sprite) -> Self {
        Self {sprite}
    }
}

pub struct SpriteRenderableFabricator;

impl Fabricator for SpriteRenderableFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<SpriteRenderable>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(sprite_renderable) = data.downcast::<SpriteRenderable>() {
            entity_builder.with(*sprite_renderable.deref())
        }
        else {
            entity_builder
        }
    }
}