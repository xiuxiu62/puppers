use std::collections::HashMap;

pub struct ItemBlock<'a> {
    name: &'a str,
    item_ids: &'a [u32],
}

impl<'a> ItemBlock<'a> {
    pub fn new(name: &'a str, item_ids: &'a [u32]) -> Self {
        Self { name, item_ids }
    }

    pub fn name(&self) -> &str {
        self.name
    }

    pub fn item_ids(&self) -> &[u32] {
        self.item_ids
    }
}

pub struct ItemSet<'a> {
    name: &'a str,
    blocks: Vec<ItemBlock<'a>>,
    champion_id: u32,
    preferred_slots: HashMap<&'a str, u32>,
}

impl<'a> ItemSet<'a> {
    pub fn new(
        name: &'a str,
        blocks: Vec<ItemBlock<'a>>,
        champion_id: u32,
        preferred_slots: HashMap<&'a str, u32>,
    ) -> Self {
        Self {
            name,
            blocks,
            champion_id,
            preferred_slots,
        }
    }
}
