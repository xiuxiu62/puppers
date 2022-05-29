pub mod ddragon;
pub mod lcu;
pub mod ugg;

mod auth;
mod error;
mod util;

pub use error::{ServiceError, ServiceResult};

use async_trait::async_trait;
use reqwest::Response;
use serde::de::DeserializeOwned;

#[derive(Debug)]
pub(super) struct Service<T>
where
    T: DeserializeOwned + Send + Sync,
{
    endpoint: String,
    cache: Option<T>,
}

impl<T> Service<T>
where
    T: DeserializeOwned + Send + Sync,
{
    pub fn new(endpoint: String, cache: Option<T>) -> Self {
        Self { endpoint, cache }
    }
}

/// Default implementation of caching logic
///
/// can be implemented in specific services for fine control over deserializers
impl<T> Service<T>
where
    T: DeserializeOwned + Send + Sync,
{
    // UNWRAP SAFETY: in the case that cache is None, we populate it before returning
    async fn cache(&mut self) -> ServiceResult<&T> {
        if self.cache.is_none() {
            self.populate_cache().await?;
        }

        Ok(self.cache.as_ref().unwrap())
    }

    async fn populate_cache(&mut self) -> ServiceResult<()> {
        let response_body = self.get(&self.endpoint).await?;
        self.cache = Some(response_body.json().await?);

        Ok(())
    }
}

impl<T> Get for Service<T> where T: DeserializeOwned + Send + Sync {}

#[async_trait]
pub trait Get {
    async fn get(&self, url: &str) -> ServiceResult<Response> {
        Ok(reqwest::get(url).await?)
    }
}

#[async_trait]
pub trait Post<T, R> {
    async fn post(url: &str, data: T) -> ServiceResult<R>;
}
