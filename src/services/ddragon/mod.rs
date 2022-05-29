pub mod champion;
pub mod error;
mod item;
pub mod lcu;
pub mod map;
mod patch;
pub mod rune;
pub mod summoner;

pub static BASE_URL: &str = "http://ddragon.leagueoflegends.com";

// pub use item::ItemService;
pub use map::MapService;
pub use patch::PatchService;
