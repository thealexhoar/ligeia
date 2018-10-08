use specs::{
    Builder, Component, EntityBuilder, HashMapStorage, NullStorage
};
use std::any::{Any, TypeId};
use std::marker::PhantomData;
use std::ops::Deref;
use std::sync::Arc;

use util::Fabricator;

#[derive(Default)]
pub struct FlagFabricator<T> where T: Clone + Component + Send + Sync {
    phantom_flag: PhantomData<T>
}

impl<T: 'static> Fabricator for FlagFabricator<T> where T: Clone + Component + Send + Sync {
    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }

    fn build_onto<'a>(&self, data: Box<Any>, entity_builder: EntityBuilder<'a>) -> EntityBuilder<'a> {
        if let Ok(flag) = data.downcast::<T>() {
            entity_builder.with(flag.deref().clone())
        }
        else {
            entity_builder
        }
    }
}

#[derive(Clone, Copy, Component, Default)]
#[component(NullStorage)]
pub struct BarrelFlag;

#[derive(Clone, Copy, Component, Default)]
#[component(NullStorage)]
pub struct CameraFollowFlag;

#[derive(Clone, Copy, Component, Default)]
#[component(NullStorage)]
pub struct CrateFlag;

#[derive(Clone, Copy, Component, Default)]
#[component(HashMapStorage)]
pub struct PlayerFlag;
