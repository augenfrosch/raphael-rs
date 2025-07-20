
mod recipes;
pub use recipes::RecipeData;

mod items;
pub use items::ItemData;

mod item_names_en;
#[cfg(not(feature = "dynamically-load-game-data"))]
pub use item_names_en::ItemNameData as ItemNameDataGlobal;
#[cfg(feature = "dynamically-load-game-data")]
pub use item_names_en::ItemNameDataGlobal;

mod item_names_de;

mod item_names_fr;

mod item_names_jp;

mod item_names_kr;
#[cfg(not(feature = "dynamically-load-game-data"))]
pub use item_names_kr::ItemNameData as ItemNameDataKR;
#[cfg(feature = "dynamically-load-game-data")]
pub use item_names_kr::ItemNameDataKR;
