use std::collections::HashMap;

use serde::Serialize;

#[derive(Debug)]
pub struct ItemBlock {
    name: String,
    item_ids: Vec<u32>,
}

impl ItemBlock {
    #[inline]
    pub fn new(name: &str, item_ids: &[u32]) -> Self {
        Self {
            name: name.to_owned(),
            item_ids: item_ids.to_owned(),
        }
    }
}

#[derive(Debug)]
pub struct ItemSet {
    name: String,
    blocks: Vec<ItemBlock>,
    champion_id: u32,
    preferred_slots: HashMap<String, u32>,
}

impl ItemSet {
    #[inline]
    pub const fn new(
        name: String,
        blocks: Vec<ItemBlock>,
        champion_id: u32,
        preferred_slots: HashMap<String, u32>,
    ) -> Self {
        Self {
            name,
            blocks,
            champion_id,
            preferred_slots,
        }
    }
}

// Returns a serialized ItemConfig
impl Serialize for ItemSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        ItemConfig::from(self).serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct ItemConfig {
    #[serde(rename(serialize = "associatedChampions"))]
    associated_champion_ids: Vec<u32>,
    #[serde(rename(serialize = "associatedMaps"))]
    associated_map_ids: Vec<u32>,
    blocks: Vec<ParsedBlock>,
    map: String,
    mode: String,
    preferred_item_slots: Vec<ParsedPreferredItemSlots>,
    #[serde(rename(serialize = "sortrank"))]
    sort_rank: u32,
    started_from: String,
    title: String,
    #[serde(rename(serialize = "type"))]
    type_field: String,
}

impl From<&ItemSet> for ItemConfig {
    fn from(item_set: &ItemSet) -> Self {
        let parse_block = |block: &ItemBlock| -> (String, Vec<ParsedItem>) {
            (
                block.name.clone(),
                block
                    .item_ids
                    .iter()
                    .map(|item| ParsedItem::from(item.clone()))
                    .collect(),
            )
        };
        let parse_blocks = |(name, items)| -> ParsedBlock { ParsedBlock::new(name, items) };
        let parse_preferred_item_slots = |(item, slot): (&String, &u32)| {
            ParsedPreferredItemSlots::new(item.clone(), slot.clone())
        };

        let blocks: Vec<ParsedBlock> = item_set
            .blocks
            .iter()
            .map(parse_block)
            .map(parse_blocks)
            .collect();

        let preferred_item_slots: Vec<ParsedPreferredItemSlots> = item_set
            .preferred_slots
            .iter()
            .map(parse_preferred_item_slots)
            .collect();

        Self {
            associated_champion_ids: vec![item_set.champion_id],
            associated_map_ids: vec![],
            blocks,
            map: "any".to_owned(),
            mode: "any".to_owned(),
            preferred_item_slots,
            sort_rank: 0,
            started_from: "blank".to_owned(),
            title: item_set.name.to_owned(),
            type_field: "custom".to_owned(),
        }
    }
}

type ParsedItems = Vec<ParsedItem>;

#[derive(Debug, Serialize)]
struct ParsedItem {
    count: u32,
    id: String,
}

impl From<u32> for ParsedItem {
    fn from(item_id: u32) -> Self {
        Self {
            count: 1,
            id: item_id.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct ParsedBlock {
    hide_if_summoner_spell: String,
    items: Vec<ParsedItem>,
    show_if_summoner_spell: String,
    #[serde(rename(serialize = "type"))]
    type_field: String,
}

impl ParsedBlock {
    pub fn new(name: String, items: Vec<ParsedItem>) -> Self {
        Self {
            hide_if_summoner_spell: "".to_owned(),
            items,
            show_if_summoner_spell: "".to_owned(),
            type_field: name,
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct ParsedPreferredItemSlots {
    id: String,
    preferred_item_slot: u32,
}

impl ParsedPreferredItemSlots {
    pub fn new(id: String, preferred_item_slot: u32) -> Self {
        Self {
            id,
            preferred_item_slot,
        }
    }
}
