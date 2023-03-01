use std::fs;
use std::collections::HashMap;
use geojson::{FeatureCollection, Feature, GeoJson, Geometry, Value};

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

pub fn import_map() -> FeatureCollection {

    let geojson_string = fs::read_to_string("./files/counties.json")
        .expect("Should have been a file.");

    let geojson: GeoJson = geojson_string.parse::<GeoJson>().unwrap();
    let raw_collection: FeatureCollection = FeatureCollection::try_from(geojson).unwrap();

    let collection = reduce_feature_collection_by_key_string(raw_collection, "STATE", "16");
    collection
}

fn reduce_feature_collection_by_key_string(feature_collection: FeatureCollection, key_name: &str, key_value: &str) -> FeatureCollection{
    let mut reduced_features: Vec<Feature> = vec![];
    let mut full_count: u32 = 0;
    let mut reduced_count: u32 = 0;
    // a map of with a string key and vector of strings as values
    let mut reduced_map: HashMap<String, Vec<String>> = HashMap::new();

    for feature in feature_collection {
        // if map does not contain the key, add it
        let prop_val = feature.property(key_name).unwrap().as_str().unwrap();

        if !reduced_map.contains_key(prop_val) {
            reduced_map.insert(prop_val.to_string(), vec![feature.property("NAME").unwrap().to_string()]);
        } else {
            reduced_map.get_mut(prop_val).unwrap().push(feature.property("NAME").unwrap().to_string());
        }

        if prop_val == key_value {
            reduced_features.push(feature);
            reduced_count += 1;
        }
        
        full_count += 1;
    }

    for (key, value) in reduced_map.iter() {
        println!("{}: {:?}\n", key, value);
    }

    println!("Full count: {}\nReduced count: {}", full_count, reduced_count);

    FeatureCollection {
        bbox: None,
        features: reduced_features,
        foreign_members: None,
    }
}