//everything goes through here rpc
/*
RpcClient

↓

new()

↓

call()
*/
let rpc = RpcClient::new(config);
pub struct RpcClient {
    url: String,
    user: String,
    password: String,
    http: reqwest::blocking::Client,
}

impl RpcClient {
 pub fn new(...)

    pub fn call(...)

}
