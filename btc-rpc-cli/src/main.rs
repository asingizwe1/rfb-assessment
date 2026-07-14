/*flow of main.rs
Parse CLI

↓

Load Config

↓

Create RpcClient

↓

Match command

↓

Call command module
*/
mod cli;
mod commands;
mod config;
mod error;
mod rpc;

use anyhow::Result;
use clap::Parser;

use cli::{Cli, Command};
use config::Config;
use rpc::RpcClient;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let config = Config::load();

    let rpc = RpcClient::new(config);

    match cli.command {
        Command::BlockchainInfo => {
            commands::blockchain::execute(&rpc)?;
        }

        Command::WalletInfo => {
            commands::wallet::execute(&rpc)?;
        }

        Command::Balance => {
            commands::wallet::balance(&rpc)?;
        }

        Command::NewAddress => {
            commands::address::execute(&rpc)?;
        }

        Command::Rpc { method, params } => {
            use serde_json::Value;

            let params: Vec<Value> = params
                .iter()
                .map(|p| {
                    p.parse::<i64>()
                        .map(Value::from)
                        .unwrap_or(Value::from(p.clone()))
                })
                .collect();

            let result: Value = rpc.call(&method, &params)?;

            println!("{}", serde_json::to_string_pretty(&result)?);
        }
    }

    Ok(())
}
