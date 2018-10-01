use specs::{
    world::EntitiesRes,
    Entities, Join, System, ReadExpect,
    ReadStorage,  WriteExpect, WriteStorage,
};
use std::ops::{Deref, DerefMut};

use game::components::{
    PhysicsObject, PlayerFlag
};
use game::resources::{
    MajorEntities
};


pub struct PlayerControl {

}

impl<'a> System<'a> for PlayerControl {
    type SystemData = (
        ReadExpect<'a, EntitiesRes>,
        WriteStorage<'a, PhysicsObject>,
        ReadStorage<'a, PlayerFlag>,
        WriteExpect<'a, MajorEntities>
    );

    fn run(
        &mut self,
        (
            entities,
            physics_objects,
            player_flags,
            mut major_entities
        ): Self::SystemData
    ) {
        //TODO: implement
        let player = match major_entities.player {
            Some(player) => {
                Some(player)
            },
            None => {
                let mut player_entity = None;
                for (e, _) in (&entities, &player_flags).join() {
                    player_entity = Some(e);
                }
                player_entity
            }
        };

    }
}