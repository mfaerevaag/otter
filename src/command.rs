#[derive(Debug)]
pub enum Command {
    Message(String, String),
    Help,
    Close,
    None(String),
}

impl Command {
    pub fn parse(msg: &str) -> Option<Command> {
        let tok: Vec<&str> = msg.split(" ").collect();
        let cmd = tok[0];

        let res = match cmd {
            "/msg" => Command::Message(tok[1].to_string(), tok[2..].join(" ")),
            "/close" => Command::Close,
            "/help" => Command::Help,
            _ => Command::None(tok.join(" ")),
        };

        Some(res) // TODO: validate
    }
}
