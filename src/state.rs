use std::collections::HashMap;
use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};

use ws::{Message, Sender, Result};

use socket::Socket;
use error::{self, Error};

#[derive(Clone)]
pub struct State {
    sockets: Arc<RwLock<HashMap<String, Arc<Mutex<Socket>>>>>,
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

        // add new socket
        let sock = Socket::new(nick.clone(), out, self.clone());
        if let Ok(ref mut sockets) = self.sockets.write() {
            println!("adding socket '{}'", nick);
            sockets.insert(String::from(nick.clone()), Arc::new(Mutex::new(sock.clone())));
        }

        // greet the new guy
        self.notify(&nick, Message::text(format!("welcome, {}", nick))).unwrap();

        sock
    }

    pub fn send(&mut self, from: &str, to: &str, msg: Message) -> Result<()> {
        if let Ok(sockets) = self.sockets.read() {
            match sockets.get(to) {
                Some(sock) => {
                    let sock = sock.lock().unwrap();
                    sock.out.send(msg)
                },
                None => {
                    self.notify(from, Message::text(format!("no suck client '{}'", to)))
                }
            }

        } else {
            Err(error::new_boxed(Error::Internal("could not read sockets".to_string())))
        }
    }

    pub fn notify(&self, to: &str, msg: Message) -> Result<()> {
        if let Ok(sockets) = self.sockets.read() {
            match sockets.get(to) {
                Some(sock) => {
                    let sock = sock.lock().unwrap();
                    sock.out.send(msg)
                },
                None => Err(error::new_boxed(Error::UnknownNick(to.to_string())))
            }
        } else {
            Err(error::new_boxed(Error::Internal("could not read sockets".to_string())))
        }
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
