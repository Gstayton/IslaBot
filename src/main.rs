use std::io::prelude::*;
use std::thread;
use std::net::TcpStream;
use std::io::{BufReader, BufWriter};
use std::string::ToString;
use std::vec::Vec;
use std::iter;
use std::str::FromStr;
use std::borrow::ToOwned;

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
        println!("### DEBUG {}", args[args.len()-1].trim() == "VERSION");

        let msg = Message::new(prefix, command, Some(args.clone())).unwrap();
        println!("{}\n{}\n{}", msg.prefix.unwrap_or(String::new()), msg.command.cmd, msg.command.args.join("/"));
        Message::new(prefix, command, Some(args))
    }
}

fn main() {
    let stream = TcpStream::connect("127.0.0.1:6667").unwrap();
    let tmpstrm = stream.try_clone().unwrap();

    send_stream(&tmpstrm, "USER IslaBot 0 * :Bot written in Rust by Kosan Nicholas").is_ok();
    send_stream(&tmpstrm, "NICK IslaBot").is_ok();
    send_stream(&tmpstrm, "JOIN #Omnius").is_ok();

    let t = thread::spawn(move || {
        let mut bufr = BufReader::new(stream.try_clone().unwrap());
        let mut bufw = BufWriter::new(stream.try_clone().unwrap());
        loop {
            let mut r = String::new();
            bufr.read_line(&mut r).is_ok();
            if r.contains("PING"){
                println!("FOUND PING");
                send_stream(&stream, "PONG");
                bufw.flush();
            }
            let msg = Message::from_str(&r).unwrap();

            if msg.command.cmd == "PRIVMSG" {
                if msg.command.args.contains("VERSION") {
                    println!("INSERT VERSION RESPONSE HERE");
                }
            }
            println!("{}", &r);
            println!("--------");
        }
    });

    t.join().is_ok();
}

fn send_stream (mut stream: &TcpStream, msg: &str) -> std::io::Result<()> {
    println!("*** {} ***", msg);
    stream.write(msg.as_bytes()).is_ok();
    stream.write(b"\r\n").is_ok();
    stream.flush()
}
