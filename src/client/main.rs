extern crate ws;

use std::{str, thread};
use std::io::{stdin, self, Write};

use ws::{connect, Handler, Message, Sender, Handshake, Result};

#[macro_export]
macro_rules! log_msg {
    ($fmt:expr) => {{
        print!(concat!("\r", $fmt, "\n> "));
        io::stdout().flush().unwrap();
    }};
    ($fmt:expr, $($arg:tt)*) => {{
        print!(concat!("\r", $fmt, "\n> "), $($arg)*);
        io::stdout().flush().unwrap();
    }};
}

fn main () {

    struct Client {
        out: Sender,
    }

    impl Handler for Client {
        fn on_open(&mut self, _: Handshake) -> Result<()> {
            let out = self.out.clone();

            let _ = thread::spawn(move || {

                loop {
                    print!("\r> ");
                    io::stdout().flush().unwrap();

                    let mut input = String::new();

                    stdin().read_line(&mut input).unwrap();

                    let trimmed = input.trim();

                    let message = match trimmed {
                        // "/close" => {
                        //     println!("\\close TODO")
                        // }
                        _ => Message::text(trimmed.to_string()),
                    };

                    if let Err(e) = out.send(message) {
                        log_msg!("repl: {:?}", e);
                        break;
                    }
                }
            });

            Ok(())
        }

        fn on_message(&mut self, msg: Message) -> Result<()> {
            log_msg!("received '{}'. ", msg);

            Ok(())
        }
    }


    let client = thread::Builder::new().name("client".to_owned()).spawn(move || {
        connect("ws://127.0.0.1:10000", |out| {

            Client {
                out: out
            }

        }).unwrap()
    }).unwrap();


    let _ = client.join();

}
