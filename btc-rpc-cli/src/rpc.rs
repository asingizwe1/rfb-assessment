//everything goes through here rpc
/*
RpcClient

↓

new()

↓

call()
*/
use anyhow::{anyhow, Result};
use reqwest::blocking::Client;
use serde::de::DeserializeOwned;
use serde_json::{json, Value};

use crate::config::Config;

pub struct RpcClient {
    url: String,
    user: String,
    password: String,
    http: reqwest::blocking::Client,
}

impl RpcClient {
    pub fn new(config: Config) -> Self {
        Self {
            url: config.rpc_url,
            user: config.rpc_user,
            password: config.rpc_password,
            http: Client::new(),
        }
    }

    pub fn call<T: DeserializeOwned>(&self, method: &str, params: &[Value]) -> Result<T> {
        let body = json!({
            "jsonrpc":"1.0",
            "id":"btc-cli",
            "method":method,
            "params":params
        });

        let response: Value = self
            .http
            .post(&self.url)
            .basic_auth(&self.user, Some(&self.password))
            .json(&body)
            .send()?
            .json()?;

        if !response["error"].is_null() {
            return Err(anyhow!("{}", response["error"]));
        }

        let result = serde_json::from_value(response["result"].clone())?;

        Ok(result)
    }
}
