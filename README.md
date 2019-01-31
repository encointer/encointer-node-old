# Encointer Node
A Encointer blockchain node based on Parity Substrate, written in Rust.

Please find the encointer whitepepaer at https://github.com/encointer/whitepaper

## Getting Started With a Local Testnet

Build wasm runtime, node, purge (if necessary) and start node
```
./build.sh
cargo build --release
substrate purge-chain --dev
./target/release/encointer-node --dev
```

run node
```
  ./target/release/encointer-node --dev --log DEBUG
```

optionally, run web UI: https://github.com/encointer/encointer-ui

purging chain (for ubuntu)
```
  rm -rf ~/.local/share/Substrate/chains/dev
```
TODO...docker-compose

## Encointer Ceremony Bots For Testnet
The testnet will provide bots for taking out key virtual signing events

## Private Substrate Runtime
Further down the roadmap...

The goal is to execute the wasmi in an Intel SGX enclave using https://github.com/baidu/rust-sgx-sdk
