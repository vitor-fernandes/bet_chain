use crate::{models::Transaction, storage};
use std::{char, sync::mpsc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
pub async fn start(main_tx: mpsc::Sender<Option<Transaction>>) {
    let listener = TcpListener::bind("0.0.0.0:6565").await.unwrap();

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

    match method.unwrap() {
        "send_tx" => {
            let res = send_new_tx(data.unwrap());

            if res.is_some() {
                stream
                    .try_write(res.clone().unwrap().hash.to_string().as_bytes())
                    .unwrap();
                stream.shutdown().await.unwrap();
                return res;
            } else {
                stream.try_write(b"Error sending the Transaction").unwrap();
                stream.shutdown().await.unwrap();

                return None;
            }
        }
        "get_block_by_number" => {
            let response = query_block_data(data.unwrap());
            stream.try_write(response.as_bytes()).unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
        "get_balance_of" => {
            let response = storage::get_balance_of(data.unwrap().to_string());
            stream.try_write(response.to_string().as_bytes()).unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
        "get_transaction" => {
            let response = query_transaction_data(data.unwrap().to_string());
            stream.try_write(response.as_bytes()).unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
        "get_user_transactions" => {
            let response = get_transactions_of_user(data.unwrap().to_string());
            stream.try_write(response.as_bytes()).unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
        _ => {
            stream.try_write(b"Method not Found!").unwrap();
            stream.shutdown().await.unwrap();
            return None;
        }
    };
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
        let mut user_nonce: u64 = storage::get_user_nonce(from.to_string());
        let previous_from_balance: u64 = storage::get_balance_of(from.to_string());
        let previous_to_balance: u64 = storage::get_balance_of(to.to_string());

        let after_from_balance = previous_from_balance.checked_sub(amount);

        if after_from_balance.is_none() {
            return None;
        }

        let after_to_balance = previous_to_balance.checked_add(amount);

        if after_to_balance.is_none() {
            return None;
        }

        let tmp_tx: Transaction =
            Transaction::new(from.to_string(), to.to_string(), amount, user_nonce);

        user_nonce += 1;

        storage::set_user_nonce(from.to_string(), user_nonce);

        storage::save_balance_of(from.to_string(), after_from_balance.unwrap());
        storage::save_balance_of(to.to_string(), after_to_balance.unwrap());

        storage::save_transaction(tmp_tx.clone());

        let mut user_from_txs: Vec<String> =
            storage::get_transactions_of_user(from.to_string()).unwrap();

        let mut user_to_txs: Vec<String> =
            storage::get_transactions_of_user(to.to_string()).unwrap();

        user_from_txs.push(tmp_tx.clone().hash);
        user_to_txs.push(tmp_tx.clone().hash);

        storage::save_transaction_of_user(from.to_string(), user_from_txs);
        storage::save_transaction_of_user(to.to_string(), user_to_txs);

        return Some(tmp_tx);
    }

    return None;
}

fn query_block_data(data: &str) -> String {
    let block = storage::get_block_by_number(data);
    match block {
        Some(data) => {
            return serde_json::to_string(&data).unwrap();
        }
        None => return String::from("Block Not Found"),
    }
}

fn query_transaction_data(data: String) -> String {
    let tx = storage::get_transaction(data);
    match tx {
        Some(tx_data) => {
            return serde_json::to_string(&tx_data).unwrap();
        }
        None => return String::from("Transaction Not Found"),
    }
}

fn get_transactions_of_user(data: String) -> String {
    let mut all_txs: Vec<String> = Vec::new();

    match storage::get_transactions_of_user(data) {
        Some(txs) => {
            all_txs = txs;
        }
        None => (),
    }

    let tmp_str_txs: String = all_txs.join(",");

    return tmp_str_txs;
}
