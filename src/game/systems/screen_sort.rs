use specs::{Entities, Fetch, FetchMut, Join, ReadStorage, System, WriteStorage};
use std::collections::LinkedList;
use std::ops::{Deref, DerefMut};

use game::components::{ScreenPosition, WorldRenderable};
use game::resources::VerticesNeeded;
use graphics::{ManagedCamera, Renderable};

pub struct ScreenSort;

impl<'a> System<'a> for ScreenSort {
    type SystemData = (
        Entities<'a>,
        Fetch<'a, ManagedCamera>,
        FetchMut<'a, VerticesNeeded>,
        ReadStorage<'a, WorldRenderable>,
        WriteStorage<'a, ScreenPosition>
    );

    fn run(&mut self, (entities, camera, mut vertices_needed, world_renderable, mut screen_pos): Self::SystemData) {
        let mut entity_vec = Vec::new();
        for (entity, world_renderable, mut screen_pos) in (entities.deref(), &world_renderable, &mut screen_pos).join() {
            screen_pos.vertex_index = None;
            if camera.overlaps_with(screen_pos.x, screen_pos.y, world_renderable.renderable.rect()) {
                entity_vec.push(entity);
            }
        }

        entity_vec.sort_by(|e1, e2| (screen_pos.get(*e1).unwrap().y.partial_cmp(&screen_pos.get(*e2).unwrap().y).unwrap()));

        let mut vert_sum = 0;
        for e in &entity_vec {
            let mut sp = screen_pos.get_mut(*e).unwrap();
            let verts = world_renderable.get(*e).unwrap().renderable.vertices_needed();
            sp.vertex_index = Some(vert_sum);
            vert_sum += verts;
        }
        *vertices_needed.deref_mut() = vert_sum;
    }
}

