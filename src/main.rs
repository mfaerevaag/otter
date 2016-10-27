extern crate ws;

use std::thread;

use ws::listen;

mod state;
mod socket;

use state::State;

fn main() {
    let state = State::new();

    let server = thread::spawn(move || {
        println!("listening to 127.0.0.1:10000...");

        listen("127.0.0.1:10000", |out| {

            let mut state = state.clone();
            state.new_socket(out)

        }).unwrap();
    });

    let _ = server.join();
}
