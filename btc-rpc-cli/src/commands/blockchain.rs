use crate::rpc::RpcClient;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

pub fn execute(rpc: &RpcClient) -> Result<()> {
    let info: BlockchainInfo = rpc.call("getblockchaininfo", &[])?;

    println!("{}", serde_json::to_string_pretty(&info)?);

    Ok(())
}
/*doesnt perform http itself
Ask rpc client

↓

Receive BlockchainInfo

↓

Print fields
*/
