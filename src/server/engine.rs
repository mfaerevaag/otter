use std::thread;

use ws::{self, Message, Result};

use state::State;
use socket::Socket;
use command::Command;
use error::{self, Error};

#[derive(Clone)]
pub struct Engine {
    addr: String,
    state: State,
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

        // add an empty example room
        let _ = self.state.clone().new_room("example-room");

        let _ = server.join();
    }

    pub fn handle_msg(&mut self, sock: &Socket, msg: Message) -> Result<()> {
        match msg {
            Message::Text(s) => {
                match Command::parse(&s) {
                    Ok(cmd) => {
                        println!("parsed to: {:?}", cmd);

                        match cmd {
                            Command::Message(to, msg) =>
                                self.send(&sock.nick, &to, &msg),

                            Command::NoCommand(_) =>
                                sock.out.send(Message::text("TODO".to_string())),

                            Command::Help =>
                                sock.out.send(Message::text("TODO".to_string())),

                            Command::Close =>
                                sock.out.send(Message::text("TODO".to_string())),

                            Command::List => {
                                let (users, rooms) = self.state.get_names();

                                let msg = format!("users:\n\t{}\n\nrooms:\n\t{}",
                                                  users.join("\n\t"),
                                                  rooms.join("\n\t"));

                                sock.out.send(Message::text(msg))
                            },
                        }
                    },
                    Err(e) => {
                        // let msg = format!("{}", e);
                        // println!("failed to parse '{}': '{}'", s, msg);
                        Err(e)
                    }
                }
            },
            _ => Err(error::boxed(Error::UnsupportedFormat("binary".to_string())))
        }
    }

    pub fn send(&mut self, from: &str, to: &str, msg: &str) -> Result<()> {
        match self.state.get_socket(&to) {
            Some(sock) => {
                let sock = sock.lock().unwrap();
                sock.out.send(format!("/msg {} {}", from, msg))
            },
            None => Err(error::boxed(Error::UnknownNick(String::from(to)))),
        }
    }

    // pub fn notify(&self, to: &str, msg: Message) -> Result<()> {
    //     match self.state.get_socket(&to) {
    //         Some(sock) => {
    //             let sock = sock.lock().unwrap();
    //             sock.out.send(msg)
    //         },
    //         None => Err(error::boxed(Error::UnknownNick(String::from(to)))),
    //     }
    // }
}
