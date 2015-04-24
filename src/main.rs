use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::from_utf8;
use std::thread;
use std::sync::mpsc::channel;

struct Client {
    stream: TcpStream
}

impl Client {
    fn new(stream: TcpStream) -> Client {
        return Client {
            stream: stream
        }
    }

    fn read(&mut self) {
        let mut buffer = [0u8; 512];
        loop {
            let usize = self.stream.read(&mut buffer).unwrap();
            if usize == 0 {
                break;
            }
            let msg = from_utf8(&buffer).unwrap();
            println!("{} {}", msg, usize);
            self.write(msg.as_bytes());
        }
    }

    fn write(&mut self, mut buffer: &[u8]) {
        self.stream.write(&mut buffer);
    }
}

#[allow(dead_code)]
struct Server {
    ip: &'static str,
    port: i32,
}

impl Server {
    fn start(&mut self) {
        let listener = TcpListener::bind("127.0.0.1:1337").unwrap();

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let client = Client { stream: stream.try_clone().unwrap() };
                    self.handle_client(client);
                }
                Err(e) => { println!("Connection failed because {}", e); }
            }
        }

        drop(listener);
    }

    fn handle_client(&mut self, mut client: Client) {
        println!("Client connected");
        thread::spawn(move || {
            client.read();
        });
    }
}

fn main() {
    let mut server = Server { ip: "127.0.0.1", port: 1337 };
    server.start();
}
