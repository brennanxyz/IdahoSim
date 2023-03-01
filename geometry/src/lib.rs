use std::fs;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use geojson::{FeatureCollection, Feature, GeoJson};
use serde_json::{Value};

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

pub fn import_counties() -> Result<FeatureCollection, Error> {

    // read file to string
    let geojson_string = match fs::read_to_string("./files/counties.json") {
        Ok(string) => string,
        _ => return Err(Error::new(ErrorKind::NotFound, "Counties file not found")),
    };

    // parse string to GeoJson
    let geojson: GeoJson = match geojson_string.parse::<GeoJson>() {
        Ok(geojson) => geojson,
        _ => return Err(Error::new(ErrorKind::InvalidInput, "Cannot parse counties string to GeoJson")),
    };

    // convert GeoJson to FeatureCollection
    match FeatureCollection::try_from(geojson) {
        Ok(collection) => Ok(collection),
        _ => Err(Error::new(ErrorKind::InvalidInput, "Cannot convert counties GeoJson to FeatureCollection")),
    }
}

pub fn reduce_counties_feature_collection_by_state_string(feature_collection: FeatureCollection, state_name: &str) -> Result<FeatureCollection, Error>{
    let mut reduced_features: Vec<Feature> = vec![];
    let mut full_count: u32 = 0;
    let mut reduced_count: u32 = 0;
    
    // read file to string
    let state_codes_string = match fs::read_to_string("./files/state_codes.json") {
        Ok(string) => string,
        _ => return Err(Error::new(ErrorKind::NotFound, "State codes file not found")),
    };

    // read string to HashMap
    let state_codes: HashMap<String, String> = match serde_json::from_str(&state_codes_string) {
        Ok(map) => map,
        _ => return Err(Error::new(ErrorKind::InvalidInput, "Cannot parse state codes string to HashMap")),
    };

    // get state code from state name
    let state_code = match state_codes.get(state_name) {
        Some(code) => code,
        None => return Err(Error::new(ErrorKind::NotFound, "State name not found in list")),
    };

    // iterate through features
    for feature in feature_collection {
        let state_val = match feature.property("STATE") {
            Some(Value::String(state)) => state,
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Cannot get state value from feature")),
        };

        if state_val == state_code {
            reduced_features.push(feature);
            reduced_count += 1;
        }
        
        full_count += 1;
    }

    println!("Total features: {}\nReduced features: {}", full_count, reduced_count);

    Ok(FeatureCollection {
        bbox: None,
        features: reduced_features,
        foreign_members: None,
    })

}