# Run Bitswap 💱

Welcome to the Documentation around Bitswap ⚡

⚠️**DEX running on Testnet**

⚠️**Use small amount**

### Pre Requisite

- RGB CLI
- RGB Core
- RGB STD
- RGB Schemata
- Strict Type
- AluVM
- Rust
- Rust Lightning
- Rust DLC
- Bitcoin Core Node
- Electrum Node
- Lightning Node (testnet)
- Sats for fund wallet allow send, receive and swap RGB Assets

### Clone the Git repository

```git
git clone https://github.com/Bitswap-BiFi/Bitswap-core.git
```

### Navigate to the project directory
```cd
cd Bitswapcore
```

### Build project to generate/update Cargo.lock
```cargo
cargo build
cargo build --release
```

### Run

```cargo
cargo run
```
## Rustup update

```rustup
rustup update
```
## Use CLI

Use ``app`` for see options and run DEX
```cli
$ bit--help
A CLI for manage channels, swaps and liquidity around RGB and Lightning Network

[arguments]

$app - for bitswap management

-contract
-wallet
-swap
-opeen_channel
-close_channel

```
## Run via Docker

TBD
