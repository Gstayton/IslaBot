use std::vec;

pub struct Message {
    pub sender: Option<Sender>,
    // server: &str, // First need to have a way to handle multiple servers
    pub contents: Contents,
    pub raw: String,
}

pub struct Sender {
    pub nick: String,
    pub hostmask: String,
}

pub struct Contents {
    pub command: String, // Eventually to be made into an enum or struct
    pub parameters: Vec<String>,
}

impl Message {
    pub fn new(sender: Option<Sender>, contents: Contents, raw: &str) -> Result<Message, &'static str> {
        Ok(Message {
            sender: sender,
            contents: contents,
            raw: raw.to_owned(),
        })
    }

    pub fn FromStr(line: &str) -> Result<Message, &'static str> {
        let mut s: &str = line;
        let nick: Option<&str> = if s.starts_with(":") {
            // Consider replacing with a split()
            let prefix = s.find(' ').map(|i| &s[1..i]);
            s = s.find(' ').map_or("", |i| &s[i + 1..]);
            Some(Message::nick_from_prefix(prefix.unwrap_or("")))
        } else {
            None
        };

        let trailing: Option<&str> = if s.contains(" :") {
            // Consider replacing with a split()
            let trailing = s.find(" :").map(|i| &s[i + 2..s.len() - 2]);
            s = s.find(" :").map_or("", |i| &s[..i + 1]);
            trailing
        } else {
            None
        };

        let command = match s.find(' ').map(|i| &s[..i]) {
            Some(cmd) => {
                s = s.find(' ').map_or("", |i| &s[i + 1..]);
                cmd
            }
            _ => return Err("Cannot parse a message without a command."),
        };

        if trailing.is_none() {
            s = &s[..s.len() - 2]
        }
        let mut args: Vec<&str> = s.splitn(14, ' ').filter(|s| !s.is_empty()).collect();
        let mut params: Vec<String> = Vec::new();
        for s in args {
            params.push(s.to_owned());
        }
        match trailing {
            Some(trailing) => params.push(trailing.trim().to_owned()),
            _ => (),
        }
        let contents = Contents {
            command: command.to_owned(),
            parameters: params,
        };
        let sender = Sender {
            nick: nick.unwrap_or("NO NICK FOUND").to_string(),
            hostmask: String::new(),
        };
        Message::new(Some(sender), contents, line)
    }
    fn nick_from_prefix(prefix: &str) -> &str {
        prefix.find("!").map(|i| &prefix[..i]).unwrap_or(prefix)
    }
}
