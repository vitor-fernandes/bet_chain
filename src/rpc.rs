use std::{char, sync::mpsc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

use crate::models::Transaction;

#[tokio::main]
pub async fn start(main_tx: mpsc::Sender<Option<Transaction>>) {
    let listener = TcpListener::bind("127.0.0.1:6565").await.unwrap();

    while let Ok((stream, _stream)) = listener.accept().await {
        let main_tx_cloned = main_tx.clone();

        tokio::spawn(async move {
            let res = process_incomming_request(stream).await;
            main_tx_cloned.send(res).unwrap();
        });
    }
}

async fn process_incomming_request(mut stream: TcpStream) -> Option<Transaction> {
    let mut buffer: [u8; 1024] = [0; 1024];

    stream.read(&mut buffer).await.unwrap();

    let buffer_chars = buffer.map(|x| char::from(x));

    let mut message_string = String::from("");

    for c in buffer_chars {
        if c == char::from(10) {
            break;
        }
        message_string.push(c);
    }

    let mut chars_splitted = message_string.split("|");

    if chars_splitted.clone().count() <= 1 {
        stream.try_write(b"Incorrect Number of Parameters").unwrap();
        stream.shutdown().await.unwrap();
        return None;
    }

    let method = chars_splitted.next();
    if method.is_none() {
        stream.try_write(b"Method is Required!").unwrap();
        stream.shutdown().await.unwrap();
        return None;
    }

    let data = chars_splitted.next();
    if data.is_none() {
        stream.try_write(b"Data is Required!").unwrap();
        stream.shutdown().await.unwrap();
        return None;
    }

    let result = match method.unwrap() {
        "send_tx" => send_new_tx(data.unwrap()),
        _ => {
            stream.try_write(b"Method not Found!").unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
    };

    if result.is_some() {
        stream.try_write(b"Transaction Sent successfully!").unwrap();
        stream.shutdown().await.unwrap();
        return result;
    } else {
        stream.try_write(b"Error sending the Transaction").unwrap();
        stream.shutdown().await.unwrap();

        return None;
    }
}

fn send_new_tx(data: &str) -> Option<Transaction> {
    let data_splitted = data.split(",");

    if data_splitted.clone().count() <= 2 {
        return None;
    }

    let mut from = "";
    let mut to = "";
    let mut amount: u64 = 0;

    for field in data_splitted {
        let mut tmp_field = field.split(":");
        let tmp_key = tmp_field.next().unwrap();
        let tmp_value = tmp_field.next().unwrap();

        match tmp_key {
            "from" => {
                if from.is_empty() && !tmp_key.is_empty() {
                    from = tmp_value;
                }
            }
            "to" => {
                if to.is_empty() && !tmp_key.is_empty() {
                    to = tmp_value;
                }
            }
            "amount" => {
                let zero: u64 = 0;
                if amount.eq(&zero) && !tmp_key.is_empty() {
                    amount = tmp_value.parse().unwrap();
                }
            }
            _ => (),
        }
    }

    if !from.is_empty() && !to.is_empty() {
        let tmp_tx: Transaction = Transaction::new(from.to_string(), to.to_string(), amount);
        return Some(tmp_tx);
    }

    return None;
}
