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

        self.state.send(&self.nick, &self.nick, msg.clone())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("{}: closing ({:?}) {}", self.nick, code, reason);
    }
}
