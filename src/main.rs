use std::net::TcpListener;
use std::net::TcpStream;
use std::io::Read;
use std::env;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    println!("connection listenin");
    for stream in listener.incoming() {
	match stream {
	    Ok(stream) => handle_connection(stream),
	    Err(e) => println!("{}",e),
	};
    }
}

pub fn handle_connection(mut stream: TcpStream) {
    let mut line = String::with_capacity(512);
    match stream.read_to_string(&mut line){
        Ok(_) => {
	    println!("{}",line);
	},
	_ => {
	    println!("no data");
	},
    };
}

