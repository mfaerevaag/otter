use std::{str, thread};
use std::io::{self, Write};

use ws;

pub struct Socket {
    out: ws::Sender,
}

impl Socket {
    pub fn new(out: ws:: Sender) -> Socket {
        Socket {
            out: out
        }
    }
}

impl ws::Handler for Socket {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let out = self.out.clone();

        let _ = thread::spawn(move || {

            loop {
                print!("\r> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();

                let message = match trimmed {
                    // "/close" => {
                    //     println!("\\close TODO")
                    // }
                    _ => ws::Message::text(trimmed.to_string()),
                };

                if let Err(e) = out.send(message) {
                    log_msg!("repl: {:?}", e);
                    break;
                }
            }
        });

        Ok(())
    }

    fn on_close(&mut self, code: ws::CloseCode, reason: &str) {
        println!("closing ({:?}) {}", code, reason);
    }

    fn on_message(&mut self, msg: ws::Message) -> ws::Result<()> {
        log_msg!("{}", msg);

        Ok(())
    }
}
