use crate::services::{ddragon::BASE_URL, util, Service, ServiceResult};
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, PartialEq)]
struct MapObject {
    #[serde(rename(deserialize = "type"), skip)]
    type_field: (),
    #[serde(skip)]
    version: (),
    data: HashMap<String, Map>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Map {
    #[serde(rename(deserialize = "MapName"))]
    name: String,
    #[serde(
        rename(deserialize = "MapId"),
        deserialize_with = "util::deserialize_number_from_string"
    )]
    id: u32,
    #[serde(skip)]
    image: (),
}

type MapList = HashMap<u32, String>;

#[derive(Debug)]
pub struct MapService(Service<MapList>);

impl MapService {
    pub fn new(patch_id: &str, region: &str) -> Self {
        let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/map.json");

        Self(Service::new(endpoint, None))
    }

    async fn get_name_from_id(&mut self, id: u32) -> ServiceResult<Option<&str>> {
        let cache = self.cache().await?;
        let name = cache.get(&id).map(|name| name.as_ref());

        Ok(name)
    }

    async fn get_id_from_name<'a>(&'a mut self, name: &str) -> ServiceResult<Option<&u32>> {
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
        // Applies an insert operation in the Lfold
        let insert_map = |mut acc: MapList, (_, map): (String, Map)| -> MapList {
            acc.insert(map.id, map.name);

            acc
        };

        let map_object: MapObject = reqwest::get(&self.0.endpoint).await?.json().await?;
        let map_list: MapList = map_object.data.into_iter().fold(HashMap::new(), insert_map);
        self.0.cache = Some(map_list);

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
        println!("{:?}", _map_service.0.cache);

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_id_succeeds() -> ServiceResult<()> {
        let mut map_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = map_service.get_id_from_name(name).await?;

            assert!(result.is_some());
            assert_eq!(*result.unwrap(), id);
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_name_succeeds() -> ServiceResult<()> {
        let mut map_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = map_service.get_name_from_id(id).await?;

            assert!(result.is_some());
            assert_eq!(result.unwrap(), name);
        }

        Ok(())
    }
}
