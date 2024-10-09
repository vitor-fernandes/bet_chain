# BetChain

A simple app developed to fix some concepts about Rust and Blockchains

## TODO

1. Initial Version ✅
2. Block Mining ✅
3. Persistence (leveldb) ✅
3. Transaction Implementation ✅
4. TXPool ✅
5. RPC ✅
6. State ✅
7. Signatures
8. EVM
9. P2P

### Executing

Just run
```
cargo run
```

### P2P Protocol

The implementation of the Peer-To-Peer Protocol, allowing the nodes to communicate between after estabilishing the connection.

A new TCP port will be opened to deal with the protocol, in this case **55666**

This implementation has three kind messages:
  1. Connect -> Initial message, meaning a new node wants to connect with the current node.
  ```
  Message structure: [connection] -> A simple string
  What current node must do:
    - Stores the new peer address (peer_ip:55666) into the known peers array.
  Returns: [connected] -> A simple string
  ```
  2. ForwardBlock -> Sent by the node's miner when a new block is created (only the node itself can execute this message)
  ```
  Message structure: [forward_block|BLOCK] -> A simple string containing the block to be forwarded to it's peers
  What current node must do:
    - The current node will forward the block to all it's connected peers trough the ReceiveBlock message
  ```
  3. ReceiveBlock -> Received when other peer finished the block creation firstly.
  ```
  Message structure: [receive_block|BLOCK] -> Same as ForwardBlock
  What current node must do:
    - The current node needs to commit it to it's ledger
  ```

Only connected nodes can share information (i.e, forwarding blocks)

When a node receive a new TX or produce a new block, it will save locally and forward this information to it's peers. The peers must validate the information and save it in their ledger.


### Structure of Requests

Each request (or method) is structured like this:
```
method|args
```

So, you need to send in this format to execute the RPC methods.

### RPC

The RPC listen on the 6565 port and have only one method so far:
1. send_tx -> Send a simple tx from one account to another one
```
Example: send_tx|from:user1,to:user2,amount:100

echo 'send_tx|from:user1,to:user2,amount:100' | ncat localhost 6565
```

2. get_block_by_number -> Returns all information about a block by it's number
```
Example: get_block_by_number|23

echo 'get_block_by_number|23' | ncat localhost 6565
```

3. get_balance_of -> Returns the current balance of an User
```
Example: get_balance_of|betty

echo 'get_balance_of|betty' | ncat localhost 6565
```

4. get_transaction -> Returns the information of an executed transaction
```
Example: get_transaction|tx_hash

echo 'get_transaction|463d49fe26b4b71937901b21918c39399e6b9acbe3dae4a7bb73a833880fcb39' | ncat localhost 6565
```

5. get_user_transactions -> Returns all txs which the user was involved (sent and received)
```
Example: get_user_transactions|user

echo 'get_user_transactions|betty' | ncat localhost 6565
```
