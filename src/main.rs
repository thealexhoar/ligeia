extern crate ligeia_graphics;
extern crate ligeia_softcode;
extern crate ligeia_utils;
extern crate nalgebra as na;
extern crate ncollide2d;
extern crate nphysics2d;
extern crate rlua;
extern crate sdl2;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod game;
mod input;
mod physics;
mod scripting;
mod util;

use std::sync::Arc;

fn main() {
    let sdl = sdl2::init().unwrap();
    let controller_subsystem = sdl.game_controller().unwrap();
    let video_subsystem = sdl.video().unwrap();
    let mut event_pump = sdl.event_pump().unwrap();

    let mut core = game::Core::new(
        &video_subsystem,
        800, 600,
        400, 300,
        false,
        "Ligeia 0.0.0"
    );

    'gameloop: loop {
        if core.update(&mut event_pump, &controller_subsystem) {
            break 'gameloop;
        }
    }
}
