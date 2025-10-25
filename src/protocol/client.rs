use serde::{Deserialize, Serialize};
use std::error::Error as StdError;

pub struct Client {
    client: reqwest::Client,
}

impl Client {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub async fn post_json<T: Serialize, R: for<'de> Deserialize<'de>>(
        &self,
        url: &str,
        body: &T,
    ) -> Result<R, Box<dyn StdError>> {
        let response = self.client.post(url).json(body).send().await?;

        if response.status().is_success() {
            Ok(response.json::<R>().await?)
        } else {
            Err(format!("HTTP Error: {}", response.status()).into())
        }
    }
}
