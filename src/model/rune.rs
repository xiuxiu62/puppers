use serde::Serialize;

pub struct RunePage {
    name: String,
    rune_ids: Vec<u32>,
    primary_style: u32,
    secondary_style: u32,
    active: bool,
}

impl RunePage {
    pub fn new(
        name: String,
        rune_ids: Vec<u32>,
        primary_style: u32,
        secondary_style: u32,
        active: bool,
    ) -> Self {
        Self {
            name,
            rune_ids,
            primary_style,
            secondary_style,
            active,
        }
    }
}

// Returns a serialized RunePageConfig
impl Serialize for RunePage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        RunePageConfig::from(self).serialize(serializer)
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all(serialize = "camelCase"))]
struct RunePageConfig {
    auto_modified_selections: Vec<()>,
    id: u32,
    is_deletable: bool,
    is_editable: bool,
    is_valid: bool,
    last_modified: u32,
    order: u32,
    name: String,
    current: bool,
    is_active: bool,
    primary_style_id: u32,
    sub_style_id: u32,
    selected_perk_ids: Vec<u32>,
}

impl From<&RunePage> for RunePageConfig {
    fn from(rune_page: &RunePage) -> Self {
        Self {
            auto_modified_selections: vec![],
            id: 0,
            is_deletable: true,
            is_editable: true,
            is_valid: true,
            last_modified: 0,
            order: 0,
            name: rune_page.name.to_owned(),
            current: rune_page.active,
            is_active: rune_page.active,
            primary_style_id: rune_page.primary_style,
            sub_style_id: rune_page.secondary_style,
            selected_perk_ids: rune_page.rune_ids.to_owned(),
        }
    }
}
