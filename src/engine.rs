use std::thread;

use ws::{self, Message, Result};

use state::State;
use socket::Socket;
use command::Command;
use error::{self, Error};

#[derive(Clone)]
pub struct Engine {
    pub addr: String,
    pub state: State,
}

impl Engine {
    pub fn new(addr: &str) -> Engine {
        Engine {
            addr: String::from(addr),
            state: State::new(),
        }
    }

    pub fn run(&self) {
        let self_clone = self.clone();

        let server = thread::spawn(move || {
            println!("listening to {}...", self_clone.addr.as_str());

            ws::listen(self_clone.addr.as_str(), |out| {

                let mut engine = self_clone.clone();
                engine.state.new_socket(self_clone.clone(), out)

            }).unwrap();
        });

        let _ = server.join();
    }

    pub fn handle_msg(&self, sock: &Socket, msg: Message) {
        match msg {
            Message::Text(s) => {
                match Command::parse(&s) {
                    Some(cmd) => {
                        println!("parsed to: {:?}", cmd);

                        // TODO:
                        // match cmd {
                        //     Close =>
                        // }
                    },
                    None => {
                        let reply = "could not parse message";
                        sock.out.send(Message::text(reply)).unwrap();
                        println!("{}: {}", reply, s);
                    }
                }
            },
            _ => {
                println!("got binary message: {:?}", msg);
            }
        }
    }

    pub fn send(&mut self, from: &str, to: &str, msg: Message) -> Result<()> {
        if let Ok(sockets) = self.state.sockets.read() {
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
        if let Ok(sockets) = self.state.sockets.read() {
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
}