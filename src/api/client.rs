#![allow(dead_code)]

use reqwest::Client;
use serde::de::DeserializeOwned;

use crate::api::error::{ApiError, ApiErrorResponse};

pub struct ApiClient {
    pub(crate) client: Client,
    pub(crate) base_url: String,
}

impl ApiClient {
    pub fn new(base_url: impl Into<String>) -> Result<Self, ApiError> {
        let client = Client::builder()
            .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) \
                        AppleWebKit/537.36 (KHTML, like Gecko) \
                        Chrome/118.0.0.0 Safari/537.36")
            .build()?;

        Ok(Self {
            client,
            base_url: base_url.into(),
        })
    }

    pub(crate) async fn get_json<T>(&self, path: &str) -> Result<T, ApiError> 
    where 
        T: DeserializeOwned
    {
        let url = format!("{}/{}", self.base_url, path);

        let response = self.client
            .get(url)
            .send()
            .await?;

        let status = response.status();

        let text = response.text().await?;
        if !status.is_success() {
            let error_response = serde_json::from_str::<ApiErrorResponse>(&text)?;
            return Err(ApiError::Api(error_response))
        }
            
        let parsed = serde_json::from_str(&text)?;

        Ok(parsed)
    }

    pub(crate) async fn get_json_debug<T>(&self, path: &str) -> Result<T, ApiError> 
    where 
        T: DeserializeOwned
    {
        let url = format!("{}/{}", self.base_url, path);

        let response = self.client
            .get(url)
            .send()
            .await?;

        let status = response.status();

        let text = response.text().await?;
        if !status.is_success() {
            let error_response = serde_json::from_str::<ApiErrorResponse>(&text)?;
            return Err(ApiError::Api(error_response))
        }
        
        println!("{text}");
        let parsed = serde_json::from_str(&text)?;

        Ok(parsed)
    }

    pub(crate) async fn get_text(&self, path: &str) -> Result<String, ApiError> {
        let url = format!("{}/{}", self.base_url, path);

        let response = self.client
            .get(url)
            .send()
            .await?;

        let text = response.text().await?;

        Ok(text)
    }
}