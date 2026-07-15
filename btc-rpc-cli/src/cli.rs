use clap::{Parser, Subcommand};
/*
-cli description and doesnt connect to Btc and only parses CLI- args
-existing commands
btc-rpc-cli
── blockchain-info
── wallet-info
── balance
── new-address
── rpc */
#[derive(Parser)]
#[command(name = "btc-rpc-cli")]
#[command(version)]
#[command(about = "Bitcoin Core RPC CLI")]

pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    BlockchainInfo,
    WalletInfo,
    Balance,
    NewAddress,
    Rpc { method: String, params: Vec<String> },
}
