extern crate ws;

mod state;
mod socket;
mod error;
mod command;
mod engine;

use engine::Engine;

fn main() {
    let engine = Engine::new("127.0.0.1:10000");

    engine.run();
}
