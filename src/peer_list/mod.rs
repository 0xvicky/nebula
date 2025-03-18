use instant::Instant;

#[derive(Debug, Clone)]
pub struct Peer {
    pub ip_addr: String,
    pub last_seen: Instant,
}

pub struct PeerList {
    pub peers: Vec<Peer>,
}

impl PeerList {
    pub fn new() -> Self {
        Self { peers: Vec::new() }
    }

    pub fn add_peer(&mut self, ip_addr: String) {
        let peer: Peer = Peer {
            ip_addr,
            last_seen: Instant::now(),
        };

        if !self.peers.iter().any(|p| p.ip_addr == peer.ip_addr) {
            println!("Added new peer: {:?}", peer);
            self.peers.push(peer);
        } else {
            println!("Peer already there !");
        }
    }

    pub fn cleanup(&mut self) {
        let now = Instant::now();
        self.peers
            .retain(|p| now.duration_since(p.last_seen).as_secs() <= 30);
    }

    pub fn list_peers(&mut self) -> &Vec<Peer> {
        for peer in &self.peers {
            println!("===========================");
            println!("Peer: {:?}", peer);
            println!("===========================");
        }
        &self.peers
    }
}
