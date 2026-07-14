#[derive(Deserialize)]
pub struct BlockchainInfo {
    pub chain: String,
    pub blocks: u64,
    pub headers: u64,
    pub difficulty: f64,
    pub verificationprogress: f64,
}
/*doesnt perform http itself
Ask rpc client

↓

Receive BlockchainInfo

↓

Print fields
*/
