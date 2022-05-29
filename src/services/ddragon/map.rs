use crate::services::{ddragon::BASE_URL, Service, ServiceResult};
use serde::Deserialize;
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Deserialize, PartialEq)]
struct MapImage {
    full: String,
    sprite: String,
    group: String,
    x: u32,
    y: u32,
    w: u32,
    h: u32,
}

#[derive(Debug, Deserialize, PartialEq)]
struct Map {
    map_name: String,
    map_id: u32,
    image: MapImage,
}

type MapList = HashMap<u32, Map>;

#[derive(Debug)]
pub struct MapService(Service<MapList>);

impl MapService {
    pub fn new(patch_id: &str, region: &str) -> Self {
        let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/item.json");

        Self(Service::new(endpoint, None))
    }

    async fn get_name_from_id(&mut self, id: u32) -> ServiceResult<Option<&str>> {
        let cache = self.cache().await?;
        let name = cache.get(&id).map(|item| item.map_name.as_str());

        Ok(name)
    }

    async fn get_id_from_name<'a>(&'a mut self, name: &str) -> ServiceResult<Option<&u32>> {
        let cache = self.cache().await?;
        let id = cache.iter().find_map(|(key, value): (&'a u32, &Map)| {
            if value.map_name == name {
                return Some(key);
            }

            None
        });

        Ok(id)
    }

    async fn cache(&mut self) -> ServiceResult<&MapList> {
        self.0.cache().await
    }
}

impl Display for MapService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "map service")
    }
}

#[macro_export]
macro_rules! arc_mutex {
    ($value:ident) => {
        Arc::new(Mutex::new($value))
    };
}

#[cfg(test)]
mod test {
    use std::sync::{Arc, Mutex};

    use super::{MapService, ServiceResult};
    use crate::services::{ddragon::PatchService, ServiceError};
    use lazy_static::lazy_static;

    lazy_static! {
        static ref PATCH_SERVICE: Arc<Mutex<Option<MapService>>> = arc_mutex!(None);
    }

    const TEST_DATA: [(u32, &str); 3] = [
        (11, "Summoner's Rift"),
        (12, "Howling Abyss"),
        (13, "Nexus Blitz"),
    ];

    async fn initialize() -> ServiceResult<MapService> {
        let mut patch_service = PatchService::default();
        let current_patch = patch_service.get_current().await?;
        let region = "en_US";
        let service = MapService::new(current_patch, region);

        match Arc::clone(&PATCH_SERVICE).lock() {
            Ok(mut value) => *value = Some(service),
            Err(_) => return Err(ServiceError::LockPoisoned("map service".to_owned())),
        }

        Ok(MapService::new(current_patch, region))
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
