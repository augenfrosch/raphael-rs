use std::collections::HashSet;
use std::io::Write;
use std::{fs::File, io::BufWriter};

use raphael_data_updater::*;

const NCI_ARRAY_DARA_GENERATOR_RUST_CODEGEN_DEFAULT_CONFIG:
    non_contiguously_indexed_array::BuildConfiguration =
    non_contiguously_indexed_array::BuildConfiguration {
        output_format: non_contiguously_indexed_array::OutputFormat::RustCodegen,
        value_formatting: non_contiguously_indexed_array::ValueFormatting::Display,
    };

async fn fetch_and_parse<T: SheetData>(lang: &str) -> Vec<T> {
    const XIV_API: &str = "https://v2.xivapi.com/api";
    const BOILMASTER_KO: &str = "https://boilmaster_ko.augenfrosch.dev/api";
    let domain = match lang {
        "ko" => BOILMASTER_KO,
        _ => XIV_API,
    };
    let mut rows = Vec::new();
    loop {
        let last_row_id = rows.last().map_or(0, |row: &T| row.row_id());
        let query = format!(
            "{domain}/sheet/{}?limit=1000&fields={}&after={}&language={}",
            T::SHEET,
            T::REQUIRED_FIELDS.join(","),
            last_row_id,
            lang,
        );
        let response = reqwest::get(query).await.unwrap();
        let json = json::parse(&response.text().await.unwrap()).unwrap();

        let size = rows.len();
        rows.extend(json["rows"].members().filter_map(T::from_json));
        if size == rows.len() {
            return rows;
        }
        log::debug!("\"{}\": total fetched: {}", T::SHEET, rows.len());
    }
}

fn export_rlvls(rlvls: &[RecipeLevel]) {
    let path = std::path::absolute("./raphael-data/data/rlvls.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(&mut writer, "&[").unwrap();
    writeln!(&mut writer, "{},", RecipeLevel::default()).unwrap(); // index 0
    for rlvl in rlvls.iter() {
        writeln!(&mut writer, "{rlvl},").unwrap();
    }
    writeln!(&mut writer, "]").unwrap();
    log::info!("rlvls exported to \"{}\"", path.display());
}

fn export_level_adjust_table(level_adjust_table_entries: &[LevelAdjustTableEntry]) {
    let path = std::path::absolute("./raphael-data/data/level_adjust_table.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(&mut writer, "&[").unwrap();
    writeln!(&mut writer, "{},", u16::default()).unwrap(); // index 0
    for entry in level_adjust_table_entries.iter() {
        writeln!(&mut writer, "{entry},").unwrap();
    }
    writeln!(&mut writer, "]").unwrap();
    log::info!("Level adjust table exported to \"{}\"", path.display());
}

fn export_recipes(recipes: &[Recipe]) {
    let mut nci_array_data_generator = non_contiguously_indexed_array::NciArrayDataGenerator::new();
    for recipe in recipes {
        nci_array_data_generator.entry(recipe.id as usize, recipe);
    }
    let path = std::path::absolute("./raphael-data/src/data/recipes.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(writer, "use non_contiguously_indexed_array::NciArrayData;").unwrap();
    writeln!(writer, "use crate::{{Recipe, Ingredient}};").unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "type RecipeData = {};",
        nci_array_data_generator.build_type("Recipe")
    )
    .unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "pub const RECIPE_DATA: RecipeData = NciArrayData {};",
        nci_array_data_generator.build(NCI_ARRAY_DARA_GENERATOR_RUST_CODEGEN_DEFAULT_CONFIG)
    )
    .unwrap();
    log::info!("recipes exported to \"{}\"", path.display());
}

fn export_items(items: &[Item]) {
    let mut nci_array_data_generator = non_contiguously_indexed_array::NciArrayDataGenerator::new();
    for item in items {
        nci_array_data_generator.entry(item.id as usize, item);
    }
    let path = std::path::absolute("./raphael-data/src/data/items.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(writer, "use non_contiguously_indexed_array::NciArrayData;").unwrap();
    writeln!(writer, "use crate::Item;").unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "type ItemData = {};",
        nci_array_data_generator.build_type("Item")
    )
    .unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "pub const ITEM_DATA: ItemData = NciArrayData {};",
        nci_array_data_generator.build(NCI_ARRAY_DARA_GENERATOR_RUST_CODEGEN_DEFAULT_CONFIG)
    )
    .unwrap();
    log::info!("items exported to \"{}\"", path.display());
}

fn export_meals(consumables: &[Consumable]) {
    let path = std::path::absolute("./raphael-data/data/meals.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(&mut writer, "&[").unwrap();
    for consumable in consumables.iter() {
        writeln!(&mut writer, "{consumable},").unwrap();
    }
    writeln!(&mut writer, "]").unwrap();
    log::info!("meals exported to \"{}\"", path.display());
}

fn export_potions(consumables: &[Consumable]) {
    let path = std::path::absolute("./raphael-data/data/potions.rs").unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(&mut writer, "&[").unwrap();
    for consumable in consumables.iter() {
        writeln!(&mut writer, "{consumable},").unwrap();
    }
    writeln!(&mut writer, "]").unwrap();
    log::info!("potions exported to \"{}\"", path.display());
}

fn export_item_names(item_names: &[ItemName], lang: &str) {
    let mut nci_array_data_generator = non_contiguously_indexed_array::NciArrayDataGenerator::new();
    for item_name in item_names {
        nci_array_data_generator.entry(item_name.id as usize, item_name.name.clone());
    }
    let path =
        std::path::absolute(format!("./raphael-data/src/data/item_names_{lang}.rs")).unwrap();
    let mut writer = BufWriter::new(File::create(&path).unwrap());
    writeln!(writer, "use non_contiguously_indexed_array::NciArrayData;").unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "type ItemNameData = {};",
        nci_array_data_generator.build_type("&str")
    )
    .unwrap();
    writeln!(writer, "").unwrap();
    writeln!(
        writer,
        "pub const ITEM_NAME_DATA: ItemNameData = NciArrayData {};",
        nci_array_data_generator.build(non_contiguously_indexed_array::BuildConfiguration {
            output_format: non_contiguously_indexed_array::OutputFormat::RustCodegen,
            value_formatting: non_contiguously_indexed_array::ValueFormatting::Debug
        })
    )
    .unwrap();
    log::info!("item names exported to \"{}\"", path.display());
}

#[tokio::main]
async fn main() {
    env_logger::builder().format_timestamp(None).init();

    let rlvls = tokio::spawn(async { fetch_and_parse::<RecipeLevel>("en").await });
    let level_adjust_table_entries =
        tokio::spawn(async { fetch_and_parse::<LevelAdjustTableEntry>("en").await });
    let recipes = tokio::spawn(async { fetch_and_parse::<Recipe>("en").await });
    let items = tokio::spawn(async { fetch_and_parse::<Item>("en").await });
    let item_actions = tokio::spawn(async { fetch_and_parse::<ItemAction>("en").await });
    let item_foods = tokio::spawn(async { fetch_and_parse::<ItemFood>("en").await });

    let item_names_en = tokio::spawn(async { fetch_and_parse::<ItemName>("en").await });
    let item_names_de = tokio::spawn(async { fetch_and_parse::<ItemName>("de").await });
    let item_names_fr = tokio::spawn(async { fetch_and_parse::<ItemName>("fr").await });
    let item_names_jp = tokio::spawn(async { fetch_and_parse::<ItemName>("ja").await });
    let item_names_kr = tokio::spawn(async { fetch_and_parse::<ItemName>("ko").await });

    let rlvls = rlvls.await.unwrap();
    let level_adjust_table_entries = level_adjust_table_entries.await.unwrap();
    let mut recipes = recipes.await.unwrap();
    let mut items = items.await.unwrap();

    let item_actions = item_actions.await.unwrap();
    let item_foods = item_foods.await.unwrap();
    let (meals, potions) = instantiate_consumables(&items, item_actions, item_foods);

    let mut item_names_en = item_names_en.await.unwrap();
    let mut item_names_de = item_names_de.await.unwrap();
    let mut item_names_fr = item_names_fr.await.unwrap();
    let mut item_names_jp = item_names_jp.await.unwrap();
    let mut item_names_kr = item_names_kr.await.unwrap();

    // For some reason some recipes have items with ID 0 as their result
    recipes.retain(|recipe| recipe.item_id != 0);

    // Remove recipe ingredients that don't have a HQ variant
    // as those are not used when calculating initial Quality.
    let hq_items: HashSet<_> = items
        .iter()
        .filter_map(|item| if item.can_be_hq { Some(item.id) } else { None })
        .collect();
    for recipe in recipes.iter_mut() {
        recipe
            .ingredients
            .retain(|ingredient| hq_items.contains(&ingredient.item_id));
    }

    // Only retain necessary items to reduce binary size
    let mut necessary_items: HashSet<u32> = HashSet::new();
    for recipe in recipes.iter() {
        necessary_items.insert(recipe.item_id);
        necessary_items.extend(
            recipe
                .ingredients
                .iter()
                .map(|ingredient| ingredient.item_id),
        );
    }
    necessary_items.extend(
        meals
            .iter()
            .chain(potions.iter())
            .map(|consumable| consumable.item_id),
    );
    items.retain(|item| necessary_items.contains(&item.id));
    item_names_en.retain(|item_name| necessary_items.contains(&item_name.id));
    item_names_de.retain(|item_name| necessary_items.contains(&item_name.id));
    item_names_fr.retain(|item_name| necessary_items.contains(&item_name.id));
    item_names_jp.retain(|item_name| necessary_items.contains(&item_name.id));
    item_names_kr.retain(|item_name| necessary_items.contains(&item_name.id));

    export_rlvls(&rlvls);
    export_level_adjust_table(&level_adjust_table_entries);
    export_recipes(&recipes);
    export_meals(&meals);
    export_potions(&potions);
    export_items(&items);

    export_item_names(&item_names_en, "en");
    export_item_names(&item_names_de, "de");
    export_item_names(&item_names_fr, "fr");
    export_item_names(&item_names_jp, "jp");
    export_item_names(&item_names_kr, "kr");
}
