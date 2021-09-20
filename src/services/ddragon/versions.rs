static LIST_VERSIONS_URL: &str = "http://ddragon.leagueoflegends.com/api/versions.json";

#[derive(Clone, Debug)]
pub struct Versions(Vec<String>);

impl Versions {
    pub async fn new() -> Result<Self, reqwest::Error> {
        Ok(Self(reqwest::get(LIST_VERSIONS_URL).await?.json().await?))
    }
}

impl AsRef<Vec<String>> for Versions {
    fn as_ref(&self) -> &Vec<String> {
        self.0.as_ref()
    }
}

#[tokio::test]
async fn it_works() -> Result<(), reqwest::Error> {
    let versions = Versions::new().await?;
    assert!(versions.as_ref().len() > 0);
    println!("{:?}", versions.as_ref());
    Ok(())
}
