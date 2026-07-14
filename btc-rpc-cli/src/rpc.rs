pub struct RpcClient {
    url: String,
    user: String,
    password: String,
    http: reqwest::blocking::Client,
}

//generic method to call
pub fn call<T: serde::de::DeserializeOwned>(
    &self,
    method: &str,
    params: &[serde_json::Value],
) -> anyhow::Result<T>
/*Http request body for Bitcoin core
{
    "jsonrpc": "1.0",
    "id": "btc-cli",
    "method": "getblockchaininfo",
    "params": []
}
*/


