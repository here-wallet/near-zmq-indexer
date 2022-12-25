NEAR Indexer ZMQ
==================================



### Build indexer

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

sudo apt-get update
sudo apt-get upgrade

sudo apt-get install cargo
source $HOME/.cargo/env

sudo apt-get install clang libzmq3-dev libssl-dev pkg-config libpq-dev build-essential awscli git -y

# Init genesis
git clone 
cd indexer-zmq
cargo run --release -- --home-dir ~/.near/mainnet init --chain-id mainnet --download-config  --download-genesis

cargo run --release -- --home-dir ~/.near/testnet init --chain-id testnet --download-config  --download-genesis


```


### Download data for RPC node

```bash
mkdir ~/.near/mainnet/data
cd ~/.near/mainnet/data

aws s3 --no-sign-request cp s3://near-protocol-public/backups/mainnet/rpc/latest .

aws s3 --no-sign-request cp s3://near-protocol-public/backups/testnet/rpc/latest .


LATEST=$(cat latest)

aws s3 --no-sign-request cp --no-sign-request --recursive s3://near-protocol-public/backups/testnet/rpc/$LATEST . 


aws s3 --no-sign-request cp --no-sign-request --recursive s3://near-protocol-public/backups/mainnet/rpc/$LATEST . 

aws s3 ls s3://near-protocol-public/backups/mainnet/rpc --recursive --human-readable --summarize

aws s3 ls s3://near-protocol-public/backups/mainnet/archive/2022-08-05T23:00:26Z --recursive --human-readable --summarize


aws s3 --no-sign-request cp s3://near-protocol-public/backups/mainnet/archive/2022-08-05T23:00:26Z


2022-06-02 04:06:04   64.3 MiB backups/mainnet/rpc/2022-06-02T11:00:44Z/5454687.sst
```


### 3.1 Run

- `-z` - zmq port
- `--home-dir` - path to folder with config and near blockchein data
- `--block-height` - "0" if u want send to zmq all transactions from 0 block
- to edit RPC port change it in ` ~/.near/testnet/config.json`
```bash

cargo run --release -- --home-dir ~/.near/mainnet/  --block-height 0 -z 9555  run
cargo run --release -- --home-dir ~/.near/mainnet/ -z 9555  run

```


### 3.2 Run cross docker

Init genesis

```
docker build -t near-zmq-indexer .


docker run  --name near-init -v /root/.near/mainnet/:/near/mainnet/  near-zmq-indexer cargo run --release -- --home-dir /near/mainnet init
```

After init genesis and download data u can run indexer with docker (indexer folder)

```

docker run  --name near-mainnet -d --restart unless-stopped -p 3030:3030 -p 9555:9555 -p 24567:24567 -v $HOME/.near/mainnet/:/near/mainnet/  near-zmq-indexer cargo run --release -- -z 9555 --home-dir /near/mainnet run

docker run  --name near-mainnet -d --restart unless-stopped -p 3030:3030 -p 9555:9555 -p 24567:24567 -v /root/.near/mainnet/:/near/mainnet/  near-zmq-indexer cargo run --release -- -z 9555 --home-dir /near/mainnet --block-height 0 run


# for testnet

docker run  --name near-testnet -d --restart unless-stopped -p 3031:3030 -p 9556:9555 -p 24568:24568 -v /root/.near/testnet/:/near/mainnet/  near-zmq-indexer cargo run --release -- -z 9555 --home-dir /near/mainnet run

```