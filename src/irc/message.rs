use std::vec;

struct Message {
    sender: Option<Sender>,
    // server: &str, // First need to have a way to handle multiple servers
    contents: Command,
    raw: String
}

struct Sender {
    nick: String,
    hostmask: String,
}

struct Command {
    command: String, // Eventually to be made into an enum or struct
    parameters: Vec<String>
}

impl Message {
    fn new(sender: Option<Sender>, contents: Command, raw: &str) -> Result<Message, &'static str> {
        Ok(Message{
            sender: sender,
            contents: contents,
            raw: raw.to_owned(),
        });
    }

    fn FromStr(line: &str) -> Result<Message, &'static str> {
        let mut s: &str = line;
        let prefix: Option<&str> = if s.starts_with(":") {
            // Consider replacing with a split()
            let prefix = s.find(' ').map(|i| &s[1..i]);
            s = s.find(' ').map_or("", |i| &s[i+1..]);
            prefix
        };

        let trailing: Option<&str> = if s.contains(" :") {
            // Consider replacing with a split()
            let trailing = s.find(" :").map(|i| &s[i+2..s.len()-2]);
            s = s.find(" :").map_or("", |i| &s[..i+1]);
            trailing
        };

        let command = match s.find(' ').map(|i| &s[..i]) {
            Some(cmd) => {
                s = s.find(' ').map_or("", |i| &s[i+1..]);
                cmd
            }
            _ => return Err("Cannot parse a message without a command.")
        };

        if trailing.is_none() { s = &s[..s.len() -2] }
        let mut args: Vec<&str> = s.splitn(14, ' ').filter(|s| !s.is_empty()).collect();
        match trailing {
            Some(trailing) => args.push(trailing.trim()),
            _ => None
        }
        Message::new(prefix, command, Some(args))
    }
}
