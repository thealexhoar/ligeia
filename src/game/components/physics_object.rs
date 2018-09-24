use nphysics2d::object::BodyHandle;
use specs::{Builder, Component, EntityBuilder, FlaggedStorage, VecStorage};
use std::any::{Any, TypeId};
use std::ops::Deref;

use physics::{BodyDef, BodyType};
use util::Fabricator;

pub enum PhysicsHandle {
    Uninitialized,
    Initialized (BodyHandle)
}

pub struct PhysicsObject {
    pub body_def: BodyDef,
    pub body_handle: PhysicsHandle
}

impl Component for PhysicsObject {
    type Storage = FlaggedStorage<Self, VecStorage<Self>>;
}

pub struct PhysicsObjectFabricator;

impl Fabricator for PhysicsObjectFabricator {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<PhysicsObject>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(phys_object) = data.downcast::<PhysicsObject>() {
            entity_builder.with(
                PhysicsObject {
                    body_def: phys_object.body_def.clone(),
                    body_handle: PhysicsHandle::Uninitialized
                }
            )
        }
        else {
            entity_builder
        }
    }
}