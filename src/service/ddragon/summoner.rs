use super::BASE_URL;
use crate::service::{Service, ServiceResult};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

type SummonerList = HashMap<String, String>;

#[derive(Debug, Deserialize)]
struct SummonerObject {
    #[serde(deserialize_with = "deserialize_item_data")]
    data: SummonerList,
}

#[derive(Debug, Deserialize)]
struct Summoner {
    name: String,
}

fn deserialize_item_data<'de, D>(deserializer: D) -> Result<SummonerList, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IntermediaryMap(HashMap<String, Summoner>);

    let insert_summoner = |mut acc: SummonerList, (key, summoner): (String, Summoner)| {
        acc.insert(key, summoner.name);
        acc
    };

    Ok(IntermediaryMap::deserialize(deserializer)?
        .0
        .into_iter()
        .fold(HashMap::new(), insert_summoner))
}

pub struct SummonerService(Service<SummonerList>);

impl SummonerService {
    pub fn new(patch_id: &str, region: &str) -> Self {
        let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/summoner.json");

        Self(Service::new(endpoint, None))
    }

    async fn get_name(&mut self, id: &str) -> ServiceResult<Option<&str>> {
        let cache = self.cache().await?;
        let name = cache.get(id).map(|name| name.as_ref());

        Ok(name)
    }

    async fn get_id<'a>(&'a mut self, name: &str) -> ServiceResult<Option<&String>> {
        let cache = self.cache().await?;
        let id = cache.iter().find_map(|(key, value): (&String, &String)| {
            if value == name {
                return Some(key);
            }

            None
        });

        Ok(id)
    }

    // UNWRAP SAFETY: in the case that cache is None, we populate it before returning
    async fn cache(&mut self) -> ServiceResult<&SummonerList> {
        if self.0.cache.is_none() {
            self.populate_cache().await?;
        }

        Ok(self.0.cache.as_ref().unwrap())
    }

    async fn populate_cache(&mut self) -> ServiceResult<()> {
        let map_object: SummonerObject = reqwest::get(&self.0.endpoint).await?.json().await?;
        self.0.cache = Some(map_object.data);

        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::{ServiceResult, SummonerService};
    use crate::services::ddragon::PatchService;

    const TEST_DATA: [(&str, &str); 3] = [
        ("SummonerBarrier", "Barrier"),
        ("SummonerBoost", "Cleanse"),
        ("SummonerDot", "Ignite"),
    ];

    async fn initialize() -> ServiceResult<SummonerService> {
        let mut patch_service = PatchService::default();
        let current_patch = patch_service.get_current().await?;
        let region = "en_US";
        let mut service = SummonerService::new(current_patch, region);
        service.cache().await?;

        Ok(service)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn initialization_works() -> ServiceResult<()> {
        let _summoner_service = initialize().await?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_id_succeeds() -> ServiceResult<()> {
        let mut summoner_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = summoner_service.get_id(name).await?;

            assert!(result.is_some());
            assert_eq!(*result.unwrap(), id);
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_name_succeeds() -> ServiceResult<()> {
        let mut summoner_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = summoner_service.get_name(id).await?;

            assert!(result.is_some());
            assert_eq!(result.unwrap(), name);
        }

        Ok(())
    }
}
