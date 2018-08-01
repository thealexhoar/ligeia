use specs::Entity;

use game::SceneID;
use util::FabricationDef;

#[derive(Clone, Copy, Debug)]
pub struct DeltaTime {
    pub dt: f32
}

impl DeltaTime {
    pub fn new() -> Self {
        Self { dt: 0. }
    }
}

#[derive(Clone)]
pub struct EntitiesToAdd {
    pub entities: Vec<FabricationDef>
}

impl EntitiesToAdd {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }
}

#[derive(Clone, Debug)]
pub struct EntitiesToKill {
    pub entities: Vec<Entity>
}

impl EntitiesToKill {
    pub fn new() -> Self {
        Self { entities: Vec::new() }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct NextScene {
    pub id: Option<SceneID>
}

impl NextScene {
    pub fn new() -> Self {
        Self { id: None }
    }

    pub fn with_id(id: SceneID) -> Self {
        Self { id: Some(id) }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct PhysicsTimeAccumulator {
    pub time: f32
}

impl PhysicsTimeAccumulator {
    pub fn new() -> Self {
        Self { time: 0. }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct VerticesNeeded {
    pub ground: usize,
    pub physics_debug: usize,
    pub shadow: usize,
    pub world: usize,
}

impl VerticesNeeded {
    pub fn new() -> Self {
        Self {
            ground: 0,
            physics_debug: 0,
            shadow: 0,
            world: 0
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct EntityCount {
    pub count: usize
}

impl EntityCount {
    pub fn new() -> Self {
        Self {
            count: 0
        }
    }
}

