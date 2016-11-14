use ws::Result;

use error::{self, Error};

#[derive(Debug)]
pub enum Command {
    Message(String, String),
    Help,
    Close,
    NoCommand(String),
}

impl Command {
    pub fn parse(msg: &str) -> Result<Command> {
        let tok: Vec<&str> = msg.split(" ").collect();
        let cmd = tok[0];

        match cmd {
            "/msg" => {
                let to = tok[1].to_string();
                let msg = tok[2..].join(" ");
                if [to.clone(), msg.clone()].iter().all(|ref s| !s.is_empty()) {
                    Ok(Command::Message(to, msg))
                } else {
                    Err(error::boxed(Error::WrongNumArgs("msg".to_string(), 2)))
                }
            },
            "/close" => Ok(Command::Close),
            "/help" => Ok(Command::Help),
            "/*" => Err(error::boxed(Error::UnknownCommand("sadf".to_string()))), // TODO: fix
            _ => Ok(Command::NoCommand(tok.join(" "))),
        }
    }
}
