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

### Executing

Just run
```
cargo run
```

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
