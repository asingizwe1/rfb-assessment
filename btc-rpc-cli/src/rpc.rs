pub struct RpcClient {
    url: String,
    user: String,
    password: String,
    http: reqwest::blocking::Client,
}
