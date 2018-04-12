extern crate liquidfun;
extern crate serde;
#[macro_use]
extern crate serde_json;
extern crate sfml;
extern crate specs;
#[macro_use]
extern crate specs_derive;

mod game;
mod graphics;
mod physics;
mod util;

fn main() {
    let mut core = game::Core::new(800, 600, 200, 150, "Ligeia 0.0.0");
    loop {
        core.update();
        if core.should_close() {
            break;
        }
    }
}
