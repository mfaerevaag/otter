use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use ws::Sender;

use socket::Socket;

#[derive(Clone)]
pub struct State {
    pub sockets: Arc<RwLock<HashMap<String, Arc<Mutex<Socket>>>>>,
    token_counter: Arc<AtomicUsize>,
}

impl State {
    pub fn new() -> State {
        State {
            sockets: Arc::new(RwLock::new(HashMap::new())),
            token_counter: Arc::new(AtomicUsize::new(1)),
        }
    }

    pub fn new_socket(&mut self, out: Sender) -> Socket {
        let nick = self.generate_nick();

        let sock = Socket::new(nick.clone(), out, self.clone());

        if let Ok(ref mut sockets) = self.sockets.write() {
            println!("adding socket '{}'", nick);
            sockets.insert(String::from(nick), Arc::new(Mutex::new(sock.clone())));
        }

        sock
    }

    // pub fn get_socket(&self, nick: &str) -> Option<&Socket> {
    //     if let Ok(sockets) = self.sockets.read() {
    //         sockets.get(nick)
    //     } else {
    //         None
    //     }
    // }

    fn generate_nick(&self) -> String {
        let token = self.token_counter.fetch_add(1, Ordering::SeqCst);
        format!("anon{}", token)
    }
}
