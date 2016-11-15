extern crate ws;
extern crate openssl;

use std::{io, str};

#[macro_use]
mod utils;
mod engine;
mod socket;

use utils::hexdump;
use engine::Engine;

fn main () {
    let engine = Engine::new("ws://127.0.0.1:10000");

    engine.run();
}
