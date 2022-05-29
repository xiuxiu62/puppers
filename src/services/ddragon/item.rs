use crate::services::{ddragon::BASE_URL, Service, ServiceResult};
use serde::{Deserialize, Deserializer};
use std::collections::HashMap;

type ItemList = HashMap<u32, String>;

#[derive(Debug, Deserialize)]
struct ItemObject {
    #[serde(deserialize_with = "deserialize_item_data")]
    data: ItemList,
}

#[derive(Debug, Deserialize)]
struct Item {
    name: String,
}

fn deserialize_item_data<'de, D>(deserializer: D) -> Result<HashMap<u32, String>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct IntermediaryMap(HashMap<u32, Item>);

    let insert_item = |mut acc: ItemList, (key, item): (u32, Item)| {
        acc.insert(key, item.name);
        acc
    };

    Ok(IntermediaryMap::deserialize(deserializer)?
        .0
        .into_iter()
        .fold(HashMap::new(), insert_item))
}

pub struct ItemService(Service<ItemList>);

impl ItemService {
    pub fn new(patch_id: &str, region: &str) -> Self {
        let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/item.json");

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
    async fn cache(&mut self) -> ServiceResult<&ItemList> {
        if self.0.cache.is_none() {
            self.populate_cache().await?;
        }

        Ok(self.0.cache.as_ref().unwrap())
    }

    async fn populate_cache(&mut self) -> ServiceResult<()> {
        let map_object: ItemObject = reqwest::get(&self.0.endpoint).await?.json().await?;
        self.0.cache = Some(map_object.data);

        Ok(())
    }
}

#[cfg(test)]
mod test {

    use super::{ItemService, ServiceResult};
    use crate::services::ddragon::PatchService;

    const TEST_DATA: [(u32, &str); 3] = [
        (1001, "Boots"),
        (1004, "Faerie Charm"),
        (1006, "Rejuvenation Bead"),
    ];

    async fn initialize() -> ServiceResult<ItemService> {
        let mut patch_service = PatchService::default();
        let current_patch = patch_service.get_current().await?;
        let region = "en_US";
        let mut service = ItemService::new(current_patch, region);
        service.cache().await?;

        Ok(service)
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn initialization_works() -> ServiceResult<()> {
        let _item_service = initialize().await?;

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_id_succeeds() -> ServiceResult<()> {
        let mut item_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = item_service.get_id_from_name(name).await?;

            assert!(result.is_some());
            assert_eq!(*result.unwrap(), id);
        }

        Ok(())
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn get_name_succeeds() -> ServiceResult<()> {
        let mut item_service = initialize().await?;
        for (id, name) in TEST_DATA {
            let result = item_service.get_name_from_id(id).await?;

            assert!(result.is_some());
            assert_eq!(result.unwrap(), name);
        }

        Ok(())
    }
}
