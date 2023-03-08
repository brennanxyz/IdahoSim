use geometry::{import_counties, reduce_counties_feature_collection_by_state_string};

fn main() {
    let temp_geom = match import_counties() {
        Ok(geom) => geom,
        Err(e) => panic!("Error importing counties: {}", e),
    };
    
    let temp_geom_reduced = match reduce_counties_feature_collection_by_state_string(temp_geom, "Idaho") {
        Ok(geom) => geom,
        Err(e) => panic!("Error reducing counties: {}", e),
    };

    // println!("{:?}", temp_geom_reduced);
}
