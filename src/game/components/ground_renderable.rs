use specs::{Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use ligeia_graphics::Renderable;
use util::Fabricator;

#[derive(Clone, Component)]
#[component(VecStorage)]
pub struct GroundRenderable {
    pub renderable: Arc<Renderable + Sync + Send + 'static>
}

impl GroundRenderable {
    pub fn new(renderable: Arc<Renderable + Sync + Send + 'static>) -> Self {
        Self {renderable}
    }

}

pub struct GroundRenderableFabricator;

impl Fabricator for GroundRenderableFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<GroundRenderable>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(ground_renderable) = data.downcast::<GroundRenderable>() {
            entity_builder.with(ground_renderable.deref().clone())
        }
            else {
                entity_builder
            }
    }
}