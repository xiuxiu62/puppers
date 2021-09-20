static LIST_ITEMS_URL: &str = "http://ddragon.leagueoflegends.com/cdn/";

#[derive(Clone, Debug)]
pub struct Versions(Vec<String>);

impl Versions {
    // Populates the version table, returning an error of request failure
    pub async fn new() -> Result<Self, reqwest::Error> {
        Ok(Self(reqwest::get(LIST_VERSIONS_URL).await?.json().await?))
    }

    // Returns an immutable referece to inner data ( an alias to self.as_ref() )
    pub fn get_all(&self) -> &Vec<String> {
        self.as_ref()
    }

    // Retruns the current patch
    pub fn get_current(&self) -> String {
        self.as_ref()[0].clone()
    }

    // Retruns the previous patch
    pub fn get_previous(&self) -> String {
        self.as_ref()[1].clone()
    }

    // Retruns the current patch with '.' replaced with '_'
    pub fn get_current_with_underscores(&self) -> String {
        self.get_current().replace('.', "_")
    }

    // Retruns the previous patch with '.' replaced with '_'
    pub fn get_previous_with_underscores(&self) -> String {
        self.get_previous().replace('.', "_")
    }
}

impl AsRef<Vec<String>> for Versions {
    // Return an immutable reference to inner data
    fn as_ref(&self) -> &Vec<String> {
        self.0.as_ref()
    }
}

#[tokio::test]
async fn request_success() -> Result<(), reqwest::Error> {
    let versions = Versions::new().await?;
    assert!(versions.as_ref().len() > 0);
    Ok(())
}
