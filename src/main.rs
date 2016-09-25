use std::io::prelude::*;
use std::thread;
use std::net::TcpStream;
use std::io::{BufReader, BufWriter};
use std::string::ToString;
use std::vec::Vec;
use std::iter;
use std::str::FromStr;
use std::borrow::ToOwned;
use std::fmt;

struct Command {
    cmd: String,
    args: Vec<String>,
}

struct Message {
    pub prefix: Option<String>,
    pub command: Command,
}

impl Message {
    pub fn new(prefix: Option<&str>, command: &str, params: Option<Vec<&str>>) -> Result<Message, &'static str> {
        let mut args: Vec<String> = Vec::new();
        for s in params.unwrap_or(Vec::new()){
            args.push(s.to_owned());
        };
        let cmd = Command {
            cmd: command.to_owned(),
            args: args,
        };

        Ok(Message {
            prefix: prefix.map(|s| s.to_owned()),
            command: cmd,
        })
    }
}

impl FromStr for Message {
    type Err = &'static str;
    fn from_str(line: &str) -> Result<Message, &'static str> {
        let mut s: &str = line;
        let prefix: Option<&str> = if s.starts_with(":") {
            let prefix = s.find(' ').map(|i| &s[1..i]);
            s = s.find(' ').map_or("", |i| &s[i+1..]);
            prefix
        } else {
            None
        };

        let trailing: Option<&str> = if s.contains(" :") {
            let trailing = s.find(" :").map(|i| &s[i+2..s.len()-2]);
            s = s.find(" :").map_or("", |i| &s[..i+1]);
            trailing
        } else {
            None
        };

        let command = match s.find(' ').map(|i| &s[..i]) {
            Some(cmd) => {
                s = s.find(' ').map_or("", |i| &s[i+1..]);
                cmd
            }
            _ => return Err("Cannot parse a message wihout a command.")
        };

        if trailing.is_none() { s = &s[..s.len() -2] }
        let mut args: Vec<&str> = s.splitn(14, ' ').filter(|s| !s.is_empty()).collect();
        args.push(trailing.unwrap_or("").trim());
    
        let msg = Message::new(prefix, command, Some(args.clone())).unwrap();
        println!("{}\n{}\n{}", msg.prefix.unwrap_or(String::new()), msg.command.cmd, msg.command.args.join("/"));
        Message::new(prefix, command, Some(args))
    }
}

fn main() {
    let stream = TcpStream::connect("64.86.243.181:6667").unwrap();
    let tmpstrm = stream.try_clone().unwrap();

    send_stream(&tmpstrm, "USER Isla * 0 :Isla").is_ok();
    send_stream(&tmpstrm, "NICK Isla").is_ok();

    let t = thread::spawn(move || {
        let mut bufr = BufReader::new(stream.try_clone().unwrap());
        let mut bufw = BufWriter::new(stream.try_clone().unwrap());
        loop {
            let mut r = String::new();
            bufr.read_line(&mut r).is_ok();
            if r.contains("PING"){
                let resp = r.find(":").map(|i| &r[i..]);
                send_stream(&stream, &format!("PONG {}", resp.unwrap().trim()));
            }
            let msg = Message::from_str(&r).unwrap();

            match msg.command.cmd.to_string() {
                ref s if s == "PRIVMSG" => println!("<{}>: {}", nick_from_prefix(&*msg.prefix.unwrap()), msg.command.args[msg.command.args.len()-1]),
                ref s if s == "MODE" => {
                    send_stream(&stream, "JOIN #omnius");
                    ();
                }
                _ => ()
            }

            if msg.command.cmd == "PRIVMSG" {
            }
            println!("{}", &r);
            println!("--------");
        }
    });

    send_stream(&tmpstrm, "JOIN #dev");

    t.join().is_ok();
}

fn send_stream (mut stream: &TcpStream, msg: &str) -> std::io::Result<()> {
    println!("*** {} ***", msg);
    stream.write(msg.as_bytes()).is_ok();
    stream.write(b"\r\n").is_ok();
    stream.flush()
}

fn nick_from_prefix(prefix: &str) -> &str {
    prefix.find("!").map(|i| &prefix[..i]).unwrap()
}
