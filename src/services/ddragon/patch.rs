use crate::services::{ddragon::BASE_URL, Service, ServiceResult};

type PatchList = Vec<String>;

#[derive(Debug)]
pub struct PatchService(Service<PatchList>);

impl Default for PatchService {
    fn default() -> Self {
        let endpoint = format!("{BASE_URL}/api/versions.json");

        Self(Service::new(endpoint, None))
    }
}

impl PatchService {
    async fn get_all(&mut self) -> ServiceResult<&PatchList> {
        Ok(self.cache().await?)
    }

    pub async fn get_current(&mut self) -> ServiceResult<&str> {
        Ok(self.cache().await?[0].as_ref())
    }

    pub async fn get_previous(&mut self) -> ServiceResult<&str> {
        Ok(self.cache().await?[1].as_ref())
    }

    pub async fn get_current_with_underscores(&mut self) -> ServiceResult<String> {
        Ok(self.get_current().await?.replace('.', "_"))
    }

    pub async fn get_previous_with_underscores(&mut self) -> ServiceResult<String> {
        Ok(self.get_previous().await?.replace('.', "_"))
    }

    async fn cache(&mut self) -> ServiceResult<&PatchList> {
        self.0.cache().await
    }
}

#[cfg(test)]
mod tests {
    use super::{PatchService, ServiceResult};

    #[tokio::test(flavor = "multi_thread")]
    async fn request_succeeds() -> ServiceResult<()> {
        let mut service = PatchService::default();
        let response_body = service.get_all().await?;
        assert!(response_body.len() > 0);

        Ok(())
    }
}
