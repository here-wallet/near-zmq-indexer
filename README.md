NEAR Indexer ZMQ
==================================

Implementation of a NEAR Blockchein RPC node to publish all blocks by ZMQ. 

Technical requirements for running

- 8+ Gb RAM
- 4+ CPUS
- 500+ Gb SSD


## Build indexer

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt-get update
sudo apt-get upgrade

sudo apt-get install cargo
source $HOME/.cargo/env

sudo apt-get install clang libzmq3-dev libssl-dev pkg-config libpq-dev build-essential awscli git -y

# Init genesis
https://github.com/here-wallet/near-indexer

```


## Download data for RPC node

```bash
mkdir ~/.near/mainnet/data
cd ~/.near/mainnet/data

aws s3 --no-sign-request cp s3://near-protocol-public/backups/mainnet/rpc/latest .
LATEST=$(cat latest)
aws s3 --no-sign-request cp --no-sign-request --recursive s3://near-protocol-public/backups/mainnet/rpc/$LATEST . 

```


## Run

- `-z` - zmq port
- `--home` - path to folder with config and near blockchein data
- `--block-height` - "0" if u want send to zmq all transactions from 0 block
```bash

cargo run --release -- --home /near/mainnet init --chain-id mainnet --download-config	--download-genesis

cargo run --release -- --home ~/.near/mainnet/ run sync-from-latest

```


## Run with docker

Init genesis

```
docker build -t near-zmq-indexer .
NEAR_HOME=~/.near/mainnet


docker run --rm  --name near-init -v $NEAR_HOME:/near/mainnet/   near-zmq-indexer cargo run --release -- --home /near/mainnet init --chain-id mainnet --download-config	--download-genesis	
```

Run indexer

```

docker run  --name near-mainnet -d --restart unless-stopped -p 3030:3030 -p 9555:9555 -p 24567:24567 -v $NEAR_HOME:/near/mainnet/  near-zmq-indexer cargo run --release -- --home /near/mainnet run  sync-from-latest

```

Commands to run NEAR Indexer

| Command 	| Key/Subcommand               	| Required/Default                                                 	| Responsible for                                                                                                                                                                                                                                                                                                                                                         	|
|---------	|--------------------------	|------------------------------------------------------------------	|-------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------	|
|         	| `--home`                 	| Default <br>`~/.near`                                            	| Tells the node where too look for necessary files: <br>`config.json`<br>, <br>`genesis.json`<br>, <br>`node_key.json`<br>, and <br>`data`<br> folder                                                                                                                                                                                                                    	|
| `init`  	|                              	|                                                                  	| Tells the node to generate config files in `--home-dir`                                                                                                                                                                                                                                                                                                                 	|
|         	| `--chain-id`                 	| Required<br><br>  * `localnet`<br>  * `testnet`<br>  * `mainnet` 	| Defines the chain to generate config files for                                                                                                                                                                                                                                                                                                                          	|
|         	| `--download-config`          	| Optional                                                         	| If provided tells the node to download `config.json` from the public URL. You can download them manually<br><br> - [testnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/config.json)<br> - [mainnet config.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/config.json)      	|
|         	| `--download-genesis`         	| Optional                                                         	| If provided tells the node to download `genesis.json` from the public URL. You can download them manually<br><br> - [testnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/testnet/genesis.json)<br> - [mainnet genesis.json](https://s3-us-west-1.amazonaws.com/build.nearprotocol.com/nearcore-deploy/mainnet/genesis.json) 	|
|         	| TODO:<br>Other `neard` keys  	|   



## Proces events on pyhon

`pip install pyzmq==22.3.0`

```python

import zmq


ctx = zmq.Context()
socket = ctx.socket(zmq.SUB)
socket.connect("tcp://0.0.0.0:9555")
socket.subscribe("")  # noqa

while True:
    messages = socket.recv_multipart()  # noqa
    for message in messages:
        dat = json.loads(message.decode("utf8"))
        print(dat)
```