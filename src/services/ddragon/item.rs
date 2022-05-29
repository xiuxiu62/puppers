use crate::services::{ddragon::BASE_URL, Get, Service, ServiceResult};
use async_trait::async_trait;
use serde::Deserialize;
use std::collections::HashMap;

// #[derive(Debug, Deserialize, PartialEq)]
// struct MapImage {
//     full: String,
//     sprite: String,
//     group: String,
//     x: u32,
//     y: u32,
//     w: u32,
//     h: u32,
// }

// #[derive(Debug, Deserialize, PartialEq)]
// struct Map {
//     map_name: String,
//     map_id: String,
//     image: MapImage,
// }

// type MapList = HashMap<u32, Map>;

// struct Item {
//     name: String,
//     description: String,
//     colloq: String,
//     plaintext: String,
// }

// ItemList<

// #[derive(Debug)]
// pub struct ItemService(Service<ItemList>);

// impl ItemService {
//     pub fn new(patch_id: &str, region: &str) -> Self {
//         let endpoint = format!("{BASE_URL}/cdn/{patch_id}/data/{region}/item.json");

//         Self(Service::new(endpoint, None))
//     }

//     async fn get_item_from_id(&mut self, id: u32) -> ServiceResult<Option<&str>> {
//         let cache = self.cache().await?;
//         let name = cache.get(&id).map(|item| item.map_name.as_str());

//         Ok(name)
//     }

//     async fn get_id_from_item<'a>(&'a mut self, name: &str) -> ServiceResult<Option<&u32>> {
//         let cache = self.cache().await?;
//         let id = cache.iter().find_map(|(key, value): (&'a u32, &Item)| {
//             if value.map_name == name {
//                 return Some(key);
//             }

//             None
//         });

//         Ok(id)
//     }

//     async fn cache(&mut self) -> ServiceResult<&ItemList> {
//         self.0.cache().await
//     }

//     // // UNWRAP SAFETY: if cache is none we populate it, before returning it
//     // async fn cache(&mut self) -> ServiceResult<&ItemList> {
//     //     if self.0.cache.is_none() {
//     //         self.populate_cache().await?;
//     //     }

//     //     Ok(self.0.cache.as_ref().unwrap())
//     // }

//     // async fn populate_cache(&mut self) -> ServiceResult<()> {
//     //     let response_body = self.get(&self.0.endpoint).await?;
//     //     self.0.cache = Some(response_body);

//     //     Ok(())
//     // }
// }

// #[async_trait]
// impl Get<ItemList> for ItemService {
//     async fn get(&self, url: &str) -> ServiceResult<ItemList> {
//         Ok(reqwest::get(url).await?.json().await?)
//     }
// }

// #[cfg(test)]
// mod test {
//     use super::{ItemService, ServiceResult};
//     use crate::services::ddragon::PatchService;

//     const TEST_DATA: [(u32, &str); 3] = [
//         (11, "Summoner's Rift"),
//         (11, "Summoner's Rift"),
//         (11, "Summoner's Rift"),
//     ];

//     async fn initialize() -> ServiceResult<ItemService> {
//         let mut patch_service = PatchService::default();
//         let current_patch = patch_service.get_current().await?;
//         let region = "en_US";

//         Ok(ItemService::new(current_patch, region))
//     }

//     #[tokio::test]
//     async fn initialization_works() -> ServiceResult<()> {
//         let _item_service = initialize().await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_id_succeeds() -> ServiceResult<()> {
//         let item_service = initialize().await?;

//         Ok(())
//     }

//     #[tokio::test]
//     async fn get_name_succeeds() -> ServiceResult<()> {
//         let item_service = initialize().await?;

//         Ok(())
//     }
// }
