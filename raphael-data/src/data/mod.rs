

use non_contiguously_indexed_array::NciArray;
use crate::{Item, Recipe};

mod recipes;
pub use recipes::RecipeData;
// pub const RECIPES: NciArray<Recipe> = NciArray::new(&recipes::RECIPE_DATA.index_range_starting_indices, &recipes::RECIPE_DATA.index_range_skip_amounts, &recipes::RECIPE_DATA.values);

mod items;
pub use items::ItemData;
// pub const ITEMS: NciArray<Item> = NciArray::new(&items::ITEM_DATA.index_range_starting_indices, &items::ITEM_DATA.index_range_skip_amounts, &items::ITEM_DATA.values);


mod item_names_en;
pub use item_names_en::ItemNameData as ItemNameDataGlobal;
// pub const ITEM_NAMES_EN: NciArray<&str> = NciArray::new(&item_names_en::ITEM_NAME_DATA.index_range_starting_indices, &item_names_en::ITEM_NAME_DATA.index_range_skip_amounts, &item_names_en::ITEM_NAME_DATA.values);

mod item_names_de;
// pub const ITEM_NAMES_DE: NciArray<&str> = NciArray::new(&item_names_de::ITEM_NAME_DATA.index_range_starting_indices, &item_names_de::ITEM_NAME_DATA.index_range_skip_amounts, &item_names_de::ITEM_NAME_DATA.values);

mod item_names_fr;
// pub const ITEM_NAMES_FR: NciArray<&str> = NciArray::new(&item_names_fr::ITEM_NAME_DATA.index_range_starting_indices, &item_names_fr::ITEM_NAME_DATA.index_range_skip_amounts, &item_names_fr::ITEM_NAME_DATA.values);

mod item_names_jp;
// pub const ITEM_NAMES_JP: NciArray<&str> = NciArray::new(&item_names_jp::ITEM_NAME_DATA.index_range_starting_indices, &item_names_jp::ITEM_NAME_DATA.index_range_skip_amounts, &item_names_jp::ITEM_NAME_DATA.values);

mod item_names_kr;
pub use item_names_kr::ItemNameData as ItemNameDataKR;
// pub const ITEM_NAMES_KR: NciArray<&str> = NciArray::new(&item_names_kr::ITEM_NAME_DATA.index_range_starting_indices, &item_names_kr::ITEM_NAME_DATA.index_range_skip_amounts, &item_names_kr::ITEM_NAME_DATA.values);
