use crate::services::{
    ddragon::BASE_URL,
    util::{Get, ReqwestResult},
};
use async_trait::async_trait;

type Cache = Vec<String>;

#[derive(Clone, Debug, Default)]
pub struct Patches {
    cache: Option<Cache>,
}

impl Patches {
    async fn get_all(&mut self) -> ReqwestResult<&Cache> {
        Ok(self.cache().await?)
    }

    pub async fn get_current(&mut self) -> ReqwestResult<&str> {
        Ok(self.cache().await?[0].as_ref())
    }

    pub async fn get_previous(&mut self) -> ReqwestResult<&str> {
        Ok(self.cache().await?[1].as_ref())
    }

    pub async fn get_current_with_underscores(&mut self) -> ReqwestResult<String> {
        Ok(self.get_current().await?.replace('.', "_"))
    }

    pub async fn get_previous_with_underscores(&mut self) -> ReqwestResult<String> {
        Ok(self.get_previous().await?.replace('.', "_"))
    }

    // UNWRAP SAFETY: if cache is none we populate it, before returning it
    async fn cache(&mut self) -> ReqwestResult<&Cache> {
        if self.cache.is_none() {
            self.populate_cache().await?;
        }

        Ok(self.cache.as_ref().unwrap())
    }
    async fn populate_cache(&mut self) -> ReqwestResult<()> {
        let url = format!("{BASE_URL}/api/versions.json");
        let response_body = self.get(&url).await?;

        self.cache = Some(response_body);

        Ok(())
    }
}

#[async_trait]
impl Get<Vec<String>> for Patches {
    async fn get(&self, url: &str) -> ReqwestResult<Vec<String>> {
        reqwest::get(url).await?.json().await
    }
}

#[cfg(test)]
mod tests {
    use super::Patches;
    use crate::services::util::ReqwestResult;

    #[tokio::test]
    async fn request_success() -> ReqwestResult<()> {
        let mut service = Patches::default();
        let response_body = service.get_all().await?;
        assert!(response_body.len() > 0);

        Ok(())
    }
}
