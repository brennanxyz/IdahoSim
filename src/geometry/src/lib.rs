use std::fs;
use std::io::{Error, ErrorKind};
use std::collections::HashMap;
use geojson::{FeatureCollection, Feature, GeoJson, Value as GeoJsonValue};
use serde_json::{Value};
use proj::{Proj};

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

/// Imports counties from a GeoJSON file.
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

/// Reduces a FeatureCollection to only features that match a given state name.
pub fn reduce_counties_feature_collection_by_state_string(feature_collection: FeatureCollection, state_name: &str) -> Result<FeatureCollection, Error>{
    let mut reduced_features: Vec<Feature> = vec![];
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
    }

    println!("Number of features: {}", reduced_count);

    Ok(FeatureCollection {
        bbox: None,
        features: reduced_features,
        foreign_members: None,
    })

}

/// Converts a FeatureCollection to a HashMap of county names and vectors of 2D projected (x, y) tuples.
pub fn make_converted_coordinate_map(feature_collection: FeatureCollection) -> Result<HashMap<String, Vec<(f64, f64)>>, Error>{
    let mut coordinates: HashMap<String, Vec<(f64, f64)>> = HashMap::new();
    let mut count = 0;

    for feature in feature_collection {

        let county_name = match feature.property("NAME") {
            Some(Value::String(name)) => name.to_string(),
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Cannot get county name from feature")),
        };

        println!("{:-<20}{:->10}", county_name, count + 1);

        let county_geometry = match feature.geometry {
            Some(geom) => match geom.value {
                GeoJsonValue::Polygon(polygon) => {
                    vec![polygon]
                },
                // TODO: Handle MultiPolygon
                GeoJsonValue::MultiPolygon(multi_polygon) => {
                    multi_polygon
                },
                _ => return Err(Error::new(ErrorKind::InvalidInput, "Other than polygon geometry not supported")),
            },
            _ => return Err(Error::new(ErrorKind::InvalidInput, "Cannot get polygon from feature")),
        };

        // TODO: Handle MultiPolygon
        let county_coordinates_list = &county_geometry;
        println!("Number of polygons: {}", county_coordinates_list.len());

        // let mut county_tuples = make_tuples_from_coordinates(county_coordinates.to_vec());

        // let converted_tuples = match convert_long_lat_array_to_xy(&mut county_tuples) {
        //     Ok(tuples) => tuples,
        //     Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Cannot convert coordinates: {}", e))),
        // };

        // coordinates.insert(county_name, converted_tuples);
        count += 1;
    }

    Ok(coordinates)
}

/// Converts a vector of (longitude, latitude) vectors to (x, y) tuples.
fn make_tuples_from_coordinates(coordinates: Vec<Vec<f64>>) -> Vec<(f64, f64)> {
    let mut tuples: Vec<(f64, f64)> = coordinates.into_iter().map(|coord| (coord[0], coord[1])).collect();
    tuples
}

/// Converts latitude and longitude to UTM coordinates.
fn convert_long_lat_to_xy(long: f64, lat: f64) -> Result<Vec<f64>, Error> {

    let from = "EPSG:4326";
    let to = "EPSG:3857"; // Web Mercator https://proj.org/operations/projections/webmerc.html; alternative: ESRI:54052 https://epsg.io/54052

    let proj = match Proj::new_known_crs(&from, &to, None) {
        Ok(proj) => proj,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Cannot create projection: {}", e))),
    };

    let coord = match proj.convert((long, lat)) {
        Ok(coord) => coord,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Cannot convert coordinates: {}", e))),
    };

    Ok(vec![coord.0, coord.1, 0.0])
}

/// Converts a vector of (longitude, latitude) tuples to (x, y) tuples.
fn convert_long_lat_array_to_xy(array_in: &mut Vec<(f64, f64)>) -> Result<Vec<(f64, f64)>, Error> {

    let from = "EPSG:4326";
    let to = "EPSG:3857"; // Web Mercator https://proj.org/operations/projections/webmerc.html; alternative: ESRI:54052 https://epsg.io/54052

    let proj = match Proj::new_known_crs(&from, &to, None) {
        Ok(proj) => proj,
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Cannot create projection: {}", e))),
    };

    let coord = match proj.convert_array(array_in) {
        Ok(coord) => coord.to_vec(),
        Err(e) => return Err(Error::new(ErrorKind::InvalidInput, format!("Cannot convert coordinates: {}", e))),
    };

    Ok(coord)
}
