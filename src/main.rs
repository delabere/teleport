use std::env;
use std::fs::read;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::str::Utf8Error;

fn handle_client(mut stream: TcpStream) -> Result<(), Utf8Error> {
    let mut buffer = [0; 128];
    match stream.read(&mut buffer) {
        Ok(n) => {
            if n == 0 {
                println!("Client disconnected");
            } else {
                let received_data = &buffer[..n];
                let text = std::str::from_utf8(received_data)?;
                println!("Received: {}", text);
            }
        }
        Err(e) => println!("Failed to read from stream: {}", e),
    }
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let mode = &args[1];

    if mode == "server" {
        println!("Running in server mode");
        let listener = TcpListener::bind("127.0.0.1:6969").expect("could not bind to address");

        // accept connections and process them serially
        for stream in listener.incoming() {
            handle_client(stream.expect("could not get stream")).expect("something went wrong");
        }
    } else if mode == "client" {
        let path = &args[2];
        let mut stream = TcpStream::connect("127.0.0.1:6969").expect("could not connect to stream");
        let contents = read(path).expect("could not read file");

        stream.write(&contents).expect("could not write to stream");
    }
}
