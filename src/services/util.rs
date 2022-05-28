use async_trait::async_trait;

pub type ReqwestResult<T> = reqwest::Result<T>;

#[async_trait]
pub trait Get<T> {
    async fn get(&self, url: &str) -> ReqwestResult<T>;
}

#[async_trait]
pub trait Post<T, R> {
    async fn post(url: &str, data: T) -> ReqwestResult<R>;
}
