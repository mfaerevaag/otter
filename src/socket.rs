use ws::{CloseCode, Sender, Handler, Message, Result, Handshake};

use state::State;

#[derive(Clone)]
pub struct Socket {
    pub nick: String,
    pub out: Sender,
    state: State,
}

impl Socket {
    pub fn new(nick: String, out: Sender, state: State) -> Socket {
        Socket {
            nick: nick,
            out: out,
            state: state,
        }
    }
}

impl Handler for Socket {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("{}: new conn", self.nick);
        Ok(())
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}: got '{}'. ", self.nick, msg);

        if let Ok(sockets) = self.state.sockets.read() {
            // for (nick, sock) in &sockets {
            //     let sock = sock.lock().unwrap();
            //     sock.out.send(msg.clone());
            // }
            match sockets.get("anon1") {
                Some(sock) => {
                    let sock = sock.lock().unwrap();
                    let _ = sock.out.send(msg.clone());
                },
                None => ()
            };
            match sockets.get("anon2") {
                Some(sock) => {
                    let sock = sock.lock().unwrap();
                    let _ = sock.out.send(msg.clone());
                },
                None => ()
            };
        };

        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("{}: closing for ({:?}) {}", self.nick, code, reason);
    }
}
