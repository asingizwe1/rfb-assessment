use crate::rpc::RpcClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct WalletInfo {
    pub walletname: String,
    pub balance: Option<f64>,
    pub unconfirmed_balance: Option<f64>,
    pub txcount: u64,
}

pub fn balance(rpc: &RpcClient) -> Result<()> {
    // let info: WalletInfo = rpc.call("getwalletinfo", &[])?;

    // println!("Balance: {}", info.balance.unwrap_or(0.0));
    //future proofing code since Bitcoin Core doesn't have balance in getwalletinfo
    let balances: serde_json::Value = rpc.call("getbalances", &[])?;

    let confirmed = balances["mine"]["trusted"].as_f64().unwrap_or(0.0);
    println!("Balance: {} BTC", confirmed);

    Ok(())
}

pub fn execute(rpc: &RpcClient) -> Result<()> {
    let info: WalletInfo = rpc.call("getwalletinfo", &[])?;

    println!("Wallet: {}", info.walletname);
    println!("Transactions: {}", info.txcount);
    if let Some(bal) = info.balance {
        println!("Balance: {} BTC", bal);
    }
    if let Some(unconf) = info.unconfirmed_balance {
        println!("Unconfirmed: {} BTC", unconf);
    }

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
