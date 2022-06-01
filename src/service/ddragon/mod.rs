pub mod error;

mod champion;
mod item;
mod map;
mod patch;
mod rune;
mod summoner;

// pub use item::ItemService;
pub use champion::ChampionService;
pub use item::ItemService;
pub use map::MapService;
pub use patch::PatchService;
pub use summoner::SummonerService;

pub static BASE_URL: &str = "http://ddragon.leagueoflegends.com";
