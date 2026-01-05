use std::{collections::HashMap, result};

type Collection = HashMap<String, Vec<String>>;

pub fn add_animal_to_section(animal: &str, section: &str, registry: &mut Collection) {
    let animals = registry.entry(section.to_string()).or_insert_with(Vec::new);

    let animal_string = animal.to_string();
    if !animals.contains(&animal_string) {
        animals.push(animal_string);
    }
}

pub fn get_animals_in_section(section: &str, registry: &Collection) -> Vec<String> {
    let mut result: Vec<String> = match registry.get(section) {
        Some(hayawenet) => hayawenet.clone(),
        None => vec![],
    };
    // TODO: implement this function
    result.sort();
    result
}

pub fn get_all_animals_sorted(registry: &Collection) -> Vec<String> {
    let mut result: Vec<String> = vec![];

    for animals in registry.values() {
        result.extend(animals.iter().cloned());
    }

    result.sort();
    result
}
