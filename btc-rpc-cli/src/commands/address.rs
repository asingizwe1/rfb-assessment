/*doesnt perform http itself
Ask rpc client

↓

Receive String

↓

Print address
*/
use anyhow::Result;

use crate::rpc::RpcClient;

pub fn execute(rpc: &RpcClient) -> Result<()> {
    let address: String = rpc.call("getnewaddress", &[])?;

    println!("New Address: {}", address);

    Ok(())
}
