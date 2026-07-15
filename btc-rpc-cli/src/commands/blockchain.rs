use crate::rpc::RpcClient;
use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}

pub fn execute(rpc: &RpcClient) -> Result<()> {
    let info: BlockchainInfo = rpc.call("getblockchaininfo", &[])?;

    println!("Chain: {}", info.chain);
    println!("Blocks: {}", info.blocks);
    println!("Headers: {}", info.headers);
    println!("Difficulty: {}", info.difficulty);
    println!("Verification Progress: {}", info.verificationprogress);

    Ok(())
}
/*doesnt perform http itself
Ask rpc client

↓

Receive BlockchainInfo

↓

Print fields
*/
