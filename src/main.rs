extern crate ws;

use std::thread;
use std::sync::{Arc, Mutex};

use ws::listen;

mod state;
mod socket;

use state::State;
use socket::Socket;

fn main() {

    // let mut state = Arc::new(Mutex::new(State::new()));
    let mut state = State::new();

    let server = thread::spawn(move || {
        let mut i = 0;

        listen("127.0.0.1:10000", |out| {

            // let mut state = state.lock().unwrap();

            state.new_socket(out)

        }).unwrap();
    });

    let _ = server.join();
}
