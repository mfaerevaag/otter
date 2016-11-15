use std::thread;

use ws;

use socket::Socket;

#[derive(Clone)]
pub struct Engine {
    addr: String
}

impl Engine {
    pub fn new(addr: &str) -> Engine {
        Engine {
            addr: String::from(addr)
        }
    }

    pub fn run(&self) {
        let self_clone = self.clone();

        let client = thread::spawn(move || {
            println!("connecting to {}...", self_clone.addr.as_str());

            ws::connect(self_clone.addr.as_str(), |out| {
                Socket::new(out.clone())
            }).unwrap()
        });

        let _ = client.join();
    }
}
