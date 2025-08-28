mod recipe;
pub use recipe::Recipe;

mod rlvl;
pub use rlvl::RecipeLevel;

mod level_adjust_table;
pub use level_adjust_table::LevelAdjustTableEntry;

mod item;
pub use item::{Item, ItemName};

mod consumable;
pub use consumable::{Consumable, ItemAction, ItemFood, instantiate_consumables};

mod non_contiguously_indexed_array_generator;
pub use non_contiguously_indexed_array_generator::{
    BuildConfiguration, NciArrayGenerator, OutputFormat, ValueFormatting,
};

pub trait SheetData: Sized {
    const SHEET: &'static str;
    const REQUIRED_FIELDS: &[&str];
    fn row_id(&self) -> u32;
    fn from_json(value: &json::JsonValue) -> Option<Self>;
}
