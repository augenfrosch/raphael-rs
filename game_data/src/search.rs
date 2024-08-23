use crate::{
    get_item_name, Consumable, Locale, CL_ICON_CHAR, HQ_ICON_CHAR, MEALS, POTIONS, RECIPES,
};

fn contains_noncontiguous(string: &str, pattern: &str) -> bool {
    let mut it = string.split_whitespace();
    for c in pattern.split_whitespace() {
        loop {
            let Some(c2) = it.next() else {
                return false;
            };
            if c2.contains(c) {
                break;
            }
        }
    }
    true
}

fn preprocess_pattern(pattern: &str) -> String {
    pattern
        .to_lowercase()
        .replace([HQ_ICON_CHAR, CL_ICON_CHAR], "")
}

pub fn find_recipes(search_string: &str, locale: Locale) -> Vec<usize> {
    let pattern = preprocess_pattern(search_string);
    RECIPES
        .iter()
        .enumerate()
        .filter_map(|(index, recipe)| {
            let item_name = get_item_name(recipe.item_id, false, locale);
            match contains_noncontiguous(&item_name.to_lowercase(), &pattern) {
                true => Some(index),
                false => None,
            }
        })
        .collect()
}

fn find_consumables(search_string: &str, locale: Locale, consumables: &[Consumable]) -> Vec<usize> {
    let pattern = preprocess_pattern(search_string);
    consumables
        .iter()
        .enumerate()
        .filter_map(|(index, consumable)| {
            let item_name = get_item_name(consumable.item_id, false, locale);
            match contains_noncontiguous(&item_name.to_lowercase(), &pattern) {
                true => Some(index),
                false => None,
            }
        })
        .collect()
}

pub fn find_meals(search_string: &str, locale: Locale) -> Vec<usize> {
    find_consumables(search_string, locale, MEALS)
}

pub fn find_potions(search_string: &str, locale: Locale) -> Vec<usize> {
    find_consumables(search_string, locale, POTIONS)
}