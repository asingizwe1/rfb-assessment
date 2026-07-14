#[derive(Deserialize)]
pub struct WalletInfo {
    pub walletname: String,
    pub balance: f64,
    pub unconfirmed_balance: f64,
    pub txcount: u64,
}
/*doesnt perform http itself
Ask rpc client

↓

Receive WalletInfo

↓

Print fields
*/
