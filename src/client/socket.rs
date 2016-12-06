use std::{self, str, thread};
use std::io::{self, Write, Read};
use std::fs::File;

use openssl;
use openssl::pkey::PKey;
use openssl::rsa::PKCS1_PADDING;

use rustc_serialize::base64::{self, ToBase64, FromBase64};

use ws;

type Buffer = Vec<u8>;

pub struct Socket {
    out: ws::Sender,
}

impl Socket {
    pub fn new(out: ws:: Sender) -> Socket {
        Socket {
            out: out,
        }
    }
}

impl ws::Handler for Socket {
    fn on_open(&mut self, _: ws::Handshake) -> ws::Result<()> {
        let out = self.out.clone();

        let _ = thread::spawn(move || {
            let key = load_public_key().unwrap();
            let rsa = key.rsa().unwrap();

            loop {
                print!("\r> ");
                io::stdout().flush().unwrap();

                let mut input = String::new();
                io::stdin().read_line(&mut input).unwrap();
                let trimmed = input.trim();
                let tok: Vec<&str> = trimmed.split(" ").collect();
                let cmd = tok[0];

                let message = match cmd {
                    "/msg" => {
                        let to = tok[1].to_string();
                        let msg = tok[2..].join(" ");

                        let mut cipher: [u8; 256] = [0; 256];

                        let _ = rsa.public_encrypt(&msg.as_bytes(), &mut cipher, PKCS1_PADDING).unwrap();

                        let base = cipher.to_base64(base64::STANDARD);

                        ws::Message::text(format!("{} {} {}", cmd, to, base))
                    }
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
        let msg = msg.into_text().unwrap();

        let tok: Vec<&str> = msg.split(" ").collect();
        let cmd = tok[0].to_string();

        match cmd.as_ref() {
            "/msg" => {
                let key = load_private_key().unwrap();
                let rsa = key.rsa().unwrap();

                let from = tok[1].to_string();
                let cipher = tok[2..].join(" ").from_base64().unwrap();

                let mut plain: [u8; 256] = [0; 256];

                let _ = rsa.private_decrypt(&cipher, &mut plain, PKCS1_PADDING).unwrap();

                log_msg!("<{}> [RSA2048] {}", from, str::from_utf8(&plain).unwrap());
            },
            _ => log_msg!("{}", msg)
        }

        Ok(())
    }
}

fn load_buffer(path : &str) -> Result<Buffer, std::io::Error> {
    let file = File::open(path);

    let mut source_file = match file {
        Ok(x) => x,
        Err(_) => {
            println!("file '{}' not found!", path);
            panic!()
        },
    };

    let mut file_buffer : Buffer = Vec::new();

    source_file.read_to_end(&mut file_buffer).unwrap();

    Ok(file_buffer)
}

fn load_private_key() -> Result<PKey, openssl::error::ErrorStack> {
    let pem_buff = load_buffer("keys/private.pem").unwrap();

    PKey::private_key_from_pem(&pem_buff)
}

fn load_public_key() -> Result<PKey, openssl::error::ErrorStack> {
    let pem_buff = load_buffer("keys/public.pem").unwrap();

    PKey::public_key_from_pem(&pem_buff)
}
