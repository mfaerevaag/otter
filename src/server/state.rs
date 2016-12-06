use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use ws::Sender;

use socket::Socket;
use engine::Engine;

#[derive(Clone)]
pub struct State {
    sockets: Arc<RwLock<HashMap<String, Arc<Mutex<Socket>>>>>,
    rooms: Arc<RwLock<HashMap<String, Arc<Mutex<Vec<Socket>>>>>>,
    token_counter: Arc<AtomicUsize>,
}

impl State {
    pub fn new() -> State {
        State {
            sockets: Arc::new(RwLock::new(HashMap::new())),
            rooms: Arc::new(RwLock::new(HashMap::new())),
            token_counter: Arc::new(AtomicUsize::new(1)),
        }
    }

    pub fn new_socket(&mut self, engine: Engine, out: Sender) -> Socket {
        let nick = self.generate_nick();

        // add new socket
        let sock = Socket::new(nick.clone(), out, engine.clone());
        if let Ok(ref mut sockets) = self.sockets.write() {
            println!("adding socket '{}'", nick);

            sockets.insert(String::from(nick.clone()), Arc::new(Mutex::new(sock.clone())));
        }

        sock
    }

    pub fn new_room(&mut self, name: &str) -> Result<(), &str> {
        if let Ok(ref mut rooms) = self.rooms.write() {
            println!("adding room '{}'", name);

            rooms.insert(String::from(name), Arc::new(Mutex::new(Vec::new())));

            Ok(())
        } else {
            Err("fail")
        }
    }

    pub fn get_socket(&self, nick: &str) -> Option<Arc<Mutex<Socket>>> {
        let map = self.sockets.read().unwrap();
        map.get(nick).cloned()
    }

    pub fn get_names(&self) -> (Vec<String>, Vec<String>) {
        let mut users: Vec<String> = Vec::new();
        let mut rooms: Vec<String> = Vec::new();

        let map = self.sockets.read().unwrap();
        for (name, _) in map.iter() {
            users.push(name.to_string());
        }

        let map = self.rooms.read().unwrap();
        for (name, _) in map.iter() {
            rooms.push(name.to_string());
        }

        (users, rooms)
    }

    fn generate_nick(&self) -> String {
        let token = self.token_counter.fetch_add(1, Ordering::SeqCst);
        format!("anon{}", token)
    }
}
