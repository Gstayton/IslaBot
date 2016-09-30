use std::io::prelude::*;
use std::thread;
use std::net::{TcpStream, ToSocketAddrs};
use std::net::SocketAddr;
use std::io::{BufReader, BufWriter};
use std::str::FromStr;

mod irc;

fn main() {
    let config = irc::Config::read_config();
    // TODO: Move server connection info into a config file
    let mut server_details: String = String::new();
    server_details.push_str(&config.server.host);
    server_details.push_str(":");
    server_details.push_str(&config.server.port);
    let socket: SocketAddr = server_details.parse().unwrap(); 
    let stream = TcpStream::connect(socket).unwrap();
    let tmpstrm = stream.try_clone().unwrap();

    send_stream(&tmpstrm, &config.user.user).is_ok();
    send_stream(&tmpstrm, &config.user.nick).is_ok();

    let t = thread::spawn(move || {
        let mut bufr = BufReader::new(stream.try_clone().unwrap());
        let mut bufw = BufWriter::new(stream.try_clone().unwrap());
        loop {
            let mut r = String::new();
            bufr.read_line(&mut r).is_ok();
            if r.contains("PING") {
                let resp = r.find(":").map(|i| &r[i..]);
                send_stream(&stream, &format!("PONG {}", resp.unwrap().trim()));
            }
            let msg = irc::Message::FromStr(&r).unwrap();

            match msg.contents.command.to_string() {
                ref s if s == "PRIVMSG" => {
                    println!("<{}>: {}",
                             msg.sender.unwrap().nick,
                             msg.contents.parameters[msg.contents.parameters.len() - 1])
                }
                ref s if s == "MODE" => {
                    send_stream(&stream, "JOIN #omnius");
                    ();
                }
                _ => (),
            }

            if msg.contents.command == "PRIVMSG" {
            }
            println!("{}", &r);
            println!("--------");
        }
    });

    t.join().is_ok();
}

fn send_stream(mut stream: &TcpStream, msg: &str) -> std::io::Result<()> {
    println!("*** {} ***", msg);
    stream.write(msg.as_bytes()).is_ok();
    stream.write(b"\r\n").is_ok();
    stream.flush()
}

