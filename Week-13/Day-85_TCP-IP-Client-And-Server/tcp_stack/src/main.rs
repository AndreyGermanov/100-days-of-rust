use std::io::{stdin,stdout};
use std::env;
use std::thread;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};

fn main() {
    if env::args().collect::<Vec<_>>().contains(&"client".to_string()) {
        start_client();
    } else {
        start_server();
    };
}

fn start_client() {
    match TcpStream::connect("localhost:3333") {
        Ok(mut stream) => {
            println!("Successfully connected to server in port 3333");

            let mut msg = String::new();
            loop {
                let mut msg = String::new();
                let _ = stdout().flush();
                stdin().read_line(&mut msg).expect("Could not read from STDIN");
                if msg == "exit" {
                    break;
                }
                stream.write(&msg.into_bytes()).unwrap();
                let mut data = [0u8;200];
                match stream.read(&mut data) {
                    Ok(size) => {
                        let text = data.to_vec().iter()
                            .take(size)
                            .map(|item|char::from(*item).to_string())
                            .collect::<Vec<String>>().join("");
                        print!("{}",text);
                    },
                    Err(e) => {
                        println!("Failed to receive data: {}", e);
                    }
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
}

fn start_server() {
    let listener = TcpListener::bind("0.0.0.0:3333").unwrap();
    // accept connections and process them, spawning a new thread for each one
    println!("Server listening on port 3333");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    handle_connection(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}

fn handle_connection(mut stream: TcpStream) {
    let mut data = [0u8; 200];
    while match stream.read(&mut data) {
        Ok(size) => {
            stream.write(&data[0..size]).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}