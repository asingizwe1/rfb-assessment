use anyhow::Result;

use crate::rpc::RpcClient;

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
