mod peer_list;
use std::io::{Read, Write};
use std::net::{self, TcpStream};

use peer_list::{Peer, PeerList};

fn handle_client_tcp(mut stream: TcpStream, peers: &mut PeerList) {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).unwrap();

    let _message = String::from_utf8_lossy(&buffer[..bytes_read]);
    let msg = _message.trim();

    if msg == "HELLO" {
        stream
            .write_all(b"Hello From Nebula !")
            .expect("Failed to write to stream");
    } else if msg == "ADD_ME" {
        let info = stream.peer_addr().unwrap();
        println!("Adding peer: {:?}", info);
        // let peer = Peer{
        //     ip:stream.peer_addr(),
        // }
        peers.add_peer(info.to_string());
        let list = peers.list_peers();
        for peer in list {
            stream
                .write_all(format!("{:?}", peer).as_bytes())
                .expect("Failed to write to stream");
        }
    } else if msg == "WHO_IS_ONLINE" {
    }
}

fn main() {
    println!("NEBULA : P2P FILE SHARING SYSTEM");
    let mut peers: PeerList = peer_list::PeerList::new();
    match net::TcpListener::bind("0.0.0.0:4444") {
        Ok(listener) => {
            for stream in listener.incoming() {
                if let Ok(stream) = stream {
                    handle_client_tcp(stream, &mut peers);
                }
            }
        }
        Err(e) => {
            println!("Failed to bind to port 4444: {}", e);
            return;
        }
    }
}
