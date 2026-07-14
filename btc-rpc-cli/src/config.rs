Config::load() //reading env vars

pub struct Config {
    pub rpc_url: String,
    pub rpc_user: String,
    pub rpc_password: String,
    pub wallet: Option<String>,
}
//setting defaults to be set back to if not found results

impl Config{
//setting up behavior om Config because config is basically data
pub fn load() -> Self {
      Self{
   rpc_url:env::var("BITCOIN_RPC_URL").unwrap_or("http://127.0.0.1:18443".into()),
   rpc_user:env::var("BITCOIN_RPC_USER").unwrap_or_default(),
   rpc_password:env::var("BITCOIN_RPC_PASSWORD").unwrap_or_default(),
   wallet:env::var("BITCOIN_WALLET").ok(),
  }
    }
}