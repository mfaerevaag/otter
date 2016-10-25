use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicUsize, Ordering};

use ws::Sender;

use socket::Socket;

pub struct State {
    sockets: Arc<RwLock<HashMap<String, Socket>>>,
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
        let token = self.token_counter.fetch_add(1, Ordering::SeqCst);
        let nick = generate_nick(token);

        let sock = Socket::new(&nick, out);

        if let Ok(ref mut sockets) = self.sockets.write() {
            println!("adding socket '{}'", nick);
            sockets.insert(nick.clone(), sock.clone());
        }

        // self.get_socket(&nick)
        sock
    }

    // pub fn get_socket(&self, nick: &str) -> Option<&Socket> {
    //     if let Ok(sockets) = self.sockets.read() {
    //         sockets.get(nick)
    //     } else {
    //         None
    //     }
    // }
}

fn generate_nick(tok: usize) -> String {
    format!("anon{}", tok)
}
