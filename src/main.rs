use geometry::{import_counties, reduce_counties_feature_collection_by_state_string, make_converted_coordinate_map};

fn main() {
    let temp_geom = match import_counties() {
        Ok(geom) => geom,
        Err(e) => panic!("Error importing counties: {}", e),
    };
    
    let temp_geom_reduced = match reduce_counties_feature_collection_by_state_string(temp_geom, "Idaho") {
        Ok(geom) => geom,
        Err(e) => panic!("Error reducing counties: {}", e),
    };

    let temp_map = match make_converted_coordinate_map(temp_geom_reduced) {
        Ok(map) => map,
        Err(e) => panic!("Error making coordinate map: {}", e),
    };



    println!("{:?}", temp_map);
}
