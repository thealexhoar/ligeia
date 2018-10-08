use specs::{Entities, Entity, Join, ParJoin, ReadExpect, ReadStorage, System, WriteExpect, WriteStorage};
use specs::prelude::ParallelIterator;
use std::ops::{Deref, DerefMut};
use std::sync::mpsc::channel;

use game::components::{ScreenPosition, WorldRenderable};
use game::resources::VerticesNeeded;
use ligeia_graphics::{ManagedCamera, Renderable};
use game::resources::EntityCount;

pub struct ScreenSort;

impl<'a> System<'a> for ScreenSort {
    type SystemData = (
        Entities<'a>,
        ReadExpect<'a, EntityCount>,
        ReadExpect<'a, ManagedCamera>,
        WriteExpect<'a, VerticesNeeded>,
        ReadStorage<'a, WorldRenderable>,
        WriteStorage<'a, ScreenPosition>
    );

    fn run(&mut self, (entities, entity_count, camera, mut vertices_needed, world_renderable, mut screen_pos): Self::SystemData) {
        /*let mut entity_vec = Vec::with_capacity(entity_count.count);
        
        //screenspace culling
        for (entity, world_renderable, mut screen_pos) in (entities.deref(), &world_renderable, &mut screen_pos).join() {
            screen_pos.vertex_index = None;
            if camera.overlaps_with(
                screen_pos.x,
                screen_pos.y,
                world_renderable.renderable.radius_2(),
                world_renderable.renderable.rect()
            ) {
                entity_vec.push(entity);
            }
        }*/

        let (sender, reciever) = channel();
        (entities.deref(), &world_renderable, &mut screen_pos)
            .par_join()
            .for_each_with(sender,|sender, (entity, world_renderable, mut screen_pos)| {
                screen_pos.vertex_index = None;
                if camera.overlaps_with(
                    screen_pos.x,
                    screen_pos.y,
                    world_renderable.renderable.radius_2(),
                    world_renderable.renderable.rect()
                ) {
                    sender.send(entity).unwrap();
                }
            });

        let mut entity_vec: Vec<Entity> = reciever.iter().collect();


        // TODO: optimize sort?
        //entity_vec.sort_by(|e1, e2| (screen_pos.get(*e1).unwrap().y.partial_cmp(&screen_pos.get(*e2).unwrap().y).unwrap()));
        //unstable is more performant, and we don't need stability anyways
        entity_vec.sort_unstable_by(
            |e1, e2| {
                let y1 = screen_pos.get(*e1).unwrap().y;
                let y2 = screen_pos.get(*e2).unwrap().y;
                y1.partial_cmp(&y2).unwrap()
            }
        );

        let mut vert_sum = 0;
        for e in &entity_vec {
            let mut sp = screen_pos.get_mut(*e).unwrap();
            let verts = world_renderable.get(*e).unwrap().renderable.vertices_needed();
            sp.vertex_index = Some(vert_sum);
            vert_sum += verts;
        }
        (*vertices_needed.deref_mut()).world = vert_sum;
    }
}

