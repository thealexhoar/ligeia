use specs::{Builder, Component, EntityBuilder, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;
use std::sync::Arc;

use ligeia_graphics::Renderable;
use util::Fabricator;

#[derive(Clone, Component)]
#[component(VecStorage)]
pub struct WorldRenderable {
    pub renderable: Arc<Renderable + Sync + Send + 'static>
}

impl WorldRenderable {
    pub fn new(renderable: Arc<Renderable + Sync + Send + 'static>) -> Self {
        Self {renderable}
    }

}

pub struct WorldRenderableFabricator;

impl Fabricator for WorldRenderableFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<WorldRenderable>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(world_renderable) = data.downcast::<WorldRenderable>() {
            entity_builder.with(world_renderable.deref().clone())
        }
        else {
            entity_builder
        }
    }
}