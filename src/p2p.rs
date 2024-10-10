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

use crate::models::Block;

use crate::storage;

enum MessageType {
    Connect,
    ForwardBlock,
    ReceiveBlock,
}

struct Message {
    kind: MessageType,
    body: String,
}

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

    pub fn is_peer(&self, peer: String) -> bool {
        return self.get_peers().contains(&peer);
    }

    pub async fn forward_block_to_peers(&self, block: String) {
        for peer in self.get_peers().iter() {
            let mut stream = TcpStream::connect(peer).await.unwrap();
            stream
                .write_all(format!("receive_block|{:?}", block).as_bytes())
                .await
                .unwrap();
        }
    }
}

#[tokio::main]
pub async fn start(possible_peers: Vec<String>) {
    let mut p2p: P2P = P2P::new();

    for possible_peer in possible_peers.iter() {
        let res = TcpStream::connect(possible_peer).await;
        if res.is_ok() {
            p2p.insert_peer(possible_peer.to_owned());
        }
    }

    println!("All Peers: {:?}", p2p.get_peers());

    let (tx, rx) = mpsc::channel::<Option<Message>>();

    let listener = TcpListener::bind("127.0.0.1:55666").await.unwrap();

    while let Ok((stream, _)) = listener.accept().await {
        let tmp_tx = tx.clone();
        tokio::spawn(async move {
            let res = process_request(stream).await;
            tmp_tx.send(res).unwrap();
        });

        match rx.recv() {
            Ok(data) => match data {
                Some(message) => match message.kind {
                    MessageType::Connect => {
                        let peer = message.body;
                        p2p.insert_peer(peer.clone());
                        println!("Peer Added: {:?}", peer);
                    }
                    MessageType::ForwardBlock => {
                        p2p.forward_block_to_peers(message.body).await;
                    }
                    MessageType::ReceiveBlock => {
                        let block: Block = serde_json::from_str(&message.body).unwrap();
                        storage::save_blockchain_data(&block);
                    }
                },
                None => (),
            },
            Err(e) => println!("Error: {e:?}"),
        }
    }
}

async fn process_request(mut stream: TcpStream) -> Option<Message> {
    let mut buffer: [u8; 10000] = [0; 10000];
    stream.read(&mut buffer).await.unwrap();

    let buffer_chars = buffer.map(|x| char::from(x));

    let mut message_string = String::from("");
    for c in buffer_chars {
        if c == char::from(10) || c == char::from(0) {
            break;
        }
        message_string.push(c);
    }

    let mut splitted_data = message_string.split("|");

    let method = splitted_data.next().unwrap();

    match method {
        "connect" => {
            stream.write("connected".as_bytes()).await.unwrap();
            let peer = format!("{}:55666", stream.peer_addr().unwrap().ip().to_string());
            Some(Message {
                kind: MessageType::Connect,
                body: peer,
            })
        }
        "forward_block" => {
            let data = splitted_data.next().unwrap();
            let res = forward_block(data.to_string(), stream).await;
            return res;
        }
        "receive_block" => {
            let data = splitted_data.next().unwrap();
            let res = receive_block(data.to_string()).await;
            return res;
        }
        _ => {
            stream.write("Method not Found".as_bytes()).await.unwrap();
            None
        }
    }
}

async fn forward_block(data: String, stream: TcpStream) -> Option<Message> {
    // Only forward blocks received by the node itself
    if stream.peer_addr().unwrap().ip().to_string().eq("127.0.0.1") {
        let block_str: String = serde_json::from_str(&data).unwrap();

        return Some(Message {
            kind: MessageType::ForwardBlock,
            body: block_str,
        });
    }

    return None;
}

async fn receive_block(data: String) -> Option<Message> {
    let block_str: String = serde_json::from_str(&data).unwrap();

    Some(Message {
        kind: MessageType::ReceiveBlock,
        body: block_str,
    })
}
