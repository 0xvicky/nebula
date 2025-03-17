use std::io::{Read, Write};
use std::net::{self, TcpStream};
fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();
    println!(
        "Received:{:?}",
        String::from_utf8_lossy(&buffer[..bytes_read])
    );
}

fn main() {
    println!("NEBULA : P2P FILE SHARING SYSTEM");
    let tcp_server = net::TcpListener::bind("192.168.56.1:6969").unwrap();
    println!("Server listening at 6969");
    for stream in tcp_server.incoming() {
        if let Ok(stream) = stream {
            println!("Connection established! : {:?}", stream);
            handle_client(stream);
        }
    }
}
