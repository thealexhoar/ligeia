use specs::{
    Component, HashMapStorage, NullStorage
};

#[derive(Clone, Copy, Component, Default)]
#[component(NullStorage)]
pub struct BarrelFlag;

#[derive(Clone, Copy, Component, Default)]
#[component(NullStorage)]
pub struct CrateFlag;

#[derive(Clone, Copy, Component, Default)]
#[component(HashMapStorage)]
pub struct PlayerFlag;