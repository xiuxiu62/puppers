use async_trait::async_trait;

pub type ReqwestResult<T> = reqwest::Result<T>;

#[async_trait]
pub trait Get {
    async fn get(url: &str) -> ReqwestResult<Self>
    where
        Self: Sized;
}

#[async_trait]
pub trait Post<T, R> {
    async fn post(url: &str, data: T) -> ReqwestResult<R>;
}
