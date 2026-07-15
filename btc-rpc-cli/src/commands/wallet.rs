use anyhow::Result;
use serde::Deserialize;

use crate::rpc::RpcClient;

#[derive(Deserialize)]
pub struct WalletInfo {
    pub walletname: String,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub txcount: u64,
}

pub fn balance(rpc: &RpcClient) -> Result<()> {
    let info: WalletInfo = rpc.call("getwalletinfo", &[])?;

    println!("Balance: {}", info.balance);

    Ok(())
}

pub fn execute(rpc: &RpcClient) -> Result<()> {
    let info: WalletInfo = rpc.call("getwalletinfo", &[])?;

    println!("Wallet: {}", info.walletname);
    println!("Balance: {}", info.balance);
    println!("Unconfirmed: {}", info.unconfirmed_balance);
    println!("Transactions: {}", info.txcount);

    Ok(())
}
/*doesnt perform http itself
Ask rpc client

↓

Receive WalletInfo

↓

Print fields
*/
//
