use crate::services::util::{Get, ReqwestResult};
use async_trait::async_trait;

static LIST_VERSIONS_URL: &str = "http://ddragon.leagueoflegends.com/api/versions.json";

#[derive(Clone, Debug)]
pub struct Versions(Vec<String>);

impl Versions {
    // Populates the version table on success, otherwise an empty list
    pub async fn new() -> Self {
        match Self::get(LIST_VERSIONS_URL).await {
            Ok(v) => v,
            _ => Self::default(),
        }
    }

    // Returns an immutable referece to inner data ( an alias to self.as_ref() )
    pub fn get_all(&self) -> &Vec<String> {
        self.inner()
    }

    // Retruns the current patch
    pub fn get_current(&self) -> &str {
        self.inner()[0].as_ref()
    }

    // Retruns the previous patch
    pub fn get_previous(&self) -> &str {
        self.inner()[1].as_ref()
    }

    // Retruns the current patch with '.' replaced with '_'
    pub fn get_current_with_underscores(&self) -> String {
        self.get_current().replace('.', "_")
    }

    // Retruns the previous patch with '.' replaced with '_'
    pub fn get_previous_with_underscores(&self) -> String {
        self.get_previous().replace('.', "_")
    }

    // Return an immutable reference to inner data
    fn inner(&self) -> &Vec<String> {
        self.0.as_ref()
    }
}

#[async_trait]
impl Get for Versions {
    // Fetch versions from ddragon api
    async fn get(url: &str) -> ReqwestResult<Self> {
        Ok(Self(reqwest::get(url).await?.json().await?))
    }
}

impl Default for Versions {
    fn default() -> Self {
        Self(vec![])
    }
}

#[cfg(test)]
mod tests {
    use super::Versions;

    #[tokio::test]
    async fn request_success() {
        let versions = Versions::new().await;
        assert!(versions.inner().len() > 0);
    }
}
