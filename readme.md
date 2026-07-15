## Bitcoin RPC CLI

A command-line application written in Rust that communicates with a local Bitcoin Core node through the JSON-RPC interface.

## Features
Connects to Bitcoin Core using JSON-RPC
Retrieves blockchain information
Retrieves wallet information
Displays wallet balance
Generates new wallet addresses
Supports arbitrary RPC commands
Uses Clap for CLI parsing
Uses Reqwest for HTTP communication
Uses Serde for JSON serialization/deserialization


## Architecture
CLI

↓

Configuration

↓

RPC Client

↓

Bitcoin Core

↓

JSON Response

↓

Display Result

## Dependencies
clap
reqwest
serde
serde_json
anyhow

## Setup
1) Clone
2) git clone https://github.com/asingizwe1/3 rfb-assessment.git 
4) cd btc-rpc-cli
5) Install dependencies
6) cargo build
7) Start Bitcoin Core

Start a Bitcoin Core node using Polar.


### Blockchain information

cargo run -- blockchain-info

### Wallet information

cargo run -- wallet-info

### Balance

cargo run -- balance

### Generate address

cargo run -- new-address

### Generic RPC

cargo run -- rpc getblockcount

## OUTPUTS
## -cargo run -- blockchain-info
{
  "chain": "regtest",
  "blocks": 1,
  "headers": 1,
  "difficulty": 4.6565423739069247e-10,
  "verificationprogress": 1.0
}

## -cargo run -- new-address
New Address: bcrt1qfa6rjmhgdnfxukv255v0lk6ql5c0uz0gfqf5dc

## -cargo run -- rpc getblockcount
     Running `target\debug\btc-rpc-cli.exe rpc getblockcount`
1

## -cargo run -- balance
Balance: 0 BTC

## -cargo run -- wallet-info
Wallet: 
Transactions: 2
PS F:\rust2\rfb-assessm

## Windows PowerShell
$env:BITCOIN_RPC_URL="http://127.0.0.1:18443"
$env:BITCOIN_RPC_USER="polaruser"
$env:BITCOIN_RPC_PASSWORD="polarpass"

