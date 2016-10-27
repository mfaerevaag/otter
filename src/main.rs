extern crate ws;

use std::thread;
use std::sync::{Arc, Mutex};

use ws::listen;

mod state;
mod socket;

use state::State;
use socket::Socket;

fn main() {

    let state = State::new();

    let server = thread::spawn(move || {
        listen("127.0.0.1:10000", |out| {

            let mut state = state.clone();

            let new_nick = state.generate_nick();

            let sock = Socket::new(new_nick.clone(), out, state.clone());

            state.add_socket(new_nick.clone(), Arc::new(Mutex::new(sock.clone())));

            sock
        }).unwrap();
    });

    let _ = server.join();
}
