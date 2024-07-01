use std::env;
use std::fs::read;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 128];
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                println!("Client disconnected");
            } else {
                let received_data = &buffer[..n];
                match std::str::from_utf8(received_data) {
                    Ok(text) => println!("Received: {}", text),
                    Err(_) => println!("Received non-UTF-8 data"),
                }
            }
        }
        Err(e) => println!("Failed to read from stream: {}", e),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    if mode == "server" {
        println!("Running in server mode");
        let listener = TcpListener::bind("127.0.0.1:6969")?;

        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(stream?);
        }
    } else if mode == "client" {
        let path = &args[2];
        let mut stream = TcpStream::connect("127.0.0.1:6969")?;
        let contents = read(path)?;

        stream.write(&contents)?;
    }

    Ok(())
}
