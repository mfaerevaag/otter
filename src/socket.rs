use ws::{CloseCode, Sender, Handler, Message, Result, Handshake};

#[derive(Debug, Clone)]
pub struct Socket {
    nick: String,
    out: Sender,
}

impl Socket {
    pub fn new(nick: &str, out: Sender) -> Socket {
        Socket {
            nick: String::from(nick),
            out: out,
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

        self.out.send(msg)
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("{}: closing for ({:?}) {}", self.nick, code, reason);
    }
}
