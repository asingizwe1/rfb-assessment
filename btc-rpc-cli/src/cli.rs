//existing commands
#[derive(Parser)]
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
