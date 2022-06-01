use super::BASE_URL;
use crate::service::{Service, ServiceResult};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

type MapList = HashMap<u32, String>;

#[derive(Debug, Deserialize, PartialEq)]
struct MapObject {
    #[serde(deserialize_with = "deserialize_map_data")]
    data: MapList,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Map {
    #[serde(rename(deserialize = "MapName"))]
    name: String,
}

fn deserialize_map_data<'de, D>(deserializer: D) -> Result<MapList, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IntermediaryMap(HashMap<u32, Map>);

    let insert_map = |mut acc: MapList, (key, item): (u32, Map)| {
        acc.insert(key, item.name);
        acc
    };

    Ok(IntermediaryMap::deserialize(deserializer)?
        .0
        .into_iter()
        .fold(HashMap::new(), insert_map))
}

#[derive(Debug)]
pub struct MapService(Service<MapList>);

impl MapService {
    pub fn new(patch_id: &str, region: &str) -> Self {
        let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/map.json");

        Self(Service::new(endpoint, None))
    }

    async fn get_name(&mut self, id: u32) -> ServiceResult<Option<&str>> {
        let cache = self.cache().await?;
        let name = cache.get(&id).map(|name| name.as_ref());

        Ok(name)
    }

    async fn get_id<'a>(&'a mut self, name: &str) -> ServiceResult<Option<&u32>> {
        let cache = self.cache().await?;
        let id = cache.iter().find_map(|(key, value): (&'a u32, &String)| {
            if value == name {
                return Some(key);
            }

            None
        });

        Ok(id)
    }

    // UNWRAP SAFETY: in the case that cache is None, we populate it before returning
    async fn cache(&mut self) -> ServiceResult<&MapList> {
        if self.0.cache.is_none() {
            self.populate_cache().await?;
        }

        Ok(self.0.cache.as_ref().unwrap())
    }

    async fn populate_cache(&mut self) -> ServiceResult<()> {
        let map_object: MapObject = reqwest::get(&self.0.endpoint).await?.json().await?;
        self.0.cache = Some(map_object.data);

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::{MapService, ServiceResult};
    use crate::services::ddragon::PatchService;

    const TEST_DATA: [(u32, &str); 3] = [
        (11, "Summoner's Rift"),
        (12, "Howling Abyss"),
        (21, "Nexus Blitz"),
    ];

    async fn initialize() -> ServiceResult<MapService> {
        let mut patch_service = PatchService::default();
        let current_patch = patch_service.get_current().await?;
        let region = "en_US";
        let mut service = MapService::new(current_patch, region);
        service.cache().await?;

        Ok(service)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn initialization_works() -> ServiceResult<()> {
        let _map_service = initialize().await?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_id_succeeds() -> ServiceResult<()> {
        let mut map_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = map_service.get_id(name).await?;

            assert!(result.is_some());
            assert_eq!(*result.unwrap(), id);
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_name_succeeds() -> ServiceResult<()> {
        let mut map_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = map_service.get_name(id).await?;

            assert!(result.is_some());
            assert_eq!(result.unwrap(), name);
        }

        Ok(())
    }
}
