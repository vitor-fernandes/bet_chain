/*
    ! P2P Protocol Implementation !
        -> Messages
        -> P2P Server
*/

use std::sync::mpsc;

use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

struct P2P {
    peers: Vec<String>,
}

impl P2P {
    pub fn new() -> P2P {
        P2P {
            peers: Vec::<String>::new(),
        }
    }

    pub fn insert_peer(&mut self, peer: String) {
        if !self.get_peers().contains(&peer) && peer.ne("") {
            self.peers.push(peer);
        }
    }

    pub fn get_peers(&self) -> Vec<String> {
        return self.peers.clone();
    }
}

#[tokio::main]
pub async fn start() {
    let mut p2p: P2P = P2P::new();

    let (tx, rx) = mpsc::channel::<Option<String>>();

    let listener = TcpListener::bind("127.0.0.1:55666").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let tmp_tx = tx.clone();

        tokio::spawn(async move {
            let res = process_request(stream).await;
            tmp_tx.send(res).unwrap();
        });

        match rx.recv() {
            Ok(data) => match data {
                Some(peer) => {
                    p2p.insert_peer(peer.clone());
                    println!("Peer Added: {:?}", peer);
                }
                None => (),
            },
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

async fn process_request(mut stream: TcpStream) -> Option<String> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).await.unwrap();

    let buffer_chars = buffer.map(|x| char::from(x));

    let mut message_string = String::from("");

    for c in buffer_chars {
        if c == char::from(10) || c == char::from(0) {
            break;
        }
        message_string.push(c);
    }

    match message_string.as_str() {
        "connect" => {
            stream.write("connected".as_bytes()).await.unwrap();
            Some(format!(
                "{}:55666",
                stream.peer_addr().unwrap().ip().to_string()
            ))
        }
        _ => {
            stream.write("Method not Found".as_bytes()).await.unwrap();
            None
        }
    }
}
