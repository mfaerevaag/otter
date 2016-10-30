use ws::{self, CloseCode, Sender, Handler, Message, Result, Handshake};

use engine::Engine;

#[derive(Clone)]
pub struct Socket {
    pub nick: String,
    pub out: Sender,
    engine: Engine,
}

impl Socket {
    pub fn new(nick: String, out: Sender, engine: Engine) -> Socket {
        Socket {
            nick: nick,
            out: out,
            engine: engine,
        }
    }
}

impl Handler for Socket {
    fn on_open(&mut self, _: Handshake) -> Result<()> {
        println!("{}: new conn", self.nick);

        self.out.send(Message::text(format!("welcome, {}", self.nick)))
    }

    fn on_message(&mut self, msg: Message) -> Result<()> {
        println!("{}: got '{}'. ", self.nick, msg);

        self.engine.handle_msg(&self, msg.clone());

        self.engine.send(&self.nick, &self.nick, msg.clone())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("{}: closing ({:?}) {}", self.nick, code, reason);
    }

    fn on_error(&mut self, err: ws::Error) {
        if let ws::ErrorKind::Custom(e) = err.kind {
            println!("error: {}", e);
            // if you have multiple custom errors, you can use
            // if e.is::<Error2>() {... to differentiate

            // match *e {
            //     Error::Internal(_) => {
            //         if let Err(fail) = self.out.close(CloseCode::Normal) {
            //             println!("failed to schedule close code after error: {}", fail)
            //         }
            //     },
            //     _ => {
            //         if let Err(fail) = self.out.send(Message::text(e.description())) {
            //             println!("failed to notify socket after error: {}", fail)
            //         }
            //     },
            // }

            let _ = self.out.send(Message::text(e.description()));
        }
    }
}
