# BetChain

A simple app developed to fix some concepts about Rust and Blockchains

## TODO

1. Initial Version ✅
2. Block Mining ✅
3. Persistence (leveldb) ✅
3. Transaction Implementation ✅
4. TXPool ✅
5. RPC ✅
6. State
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
1. send_tx -> Which will send a simple tx (really?!) from one account to another one
```
Example: send_tx|from:user1,to:user2,amount:100

echo 'send_tx|from:user1,to:user2,amount:100' | ncat localhost 6565
```
