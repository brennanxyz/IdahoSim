use geometry::{import_counties, reduce_counties_feature_collection_by_state_string, make_converted_coordinate_map};
use std::time::Instant;

fn main() {
    let states = [
        "Alabama",
        "Alaska",
        "Arizona",
        "Arkansas",
        "California",
        "Colorado",
        "Connecticut",
        "Delaware",
        "District of Columbia",
        "Florida",
        "Georgia",
        "Hawaii",
        "Idaho",
        "Illinois",
        "Indiana",
        "Iowa",
        "Kansas",
        "Kentucky",
        "Louisiana",
        "Maine",
        "Maryland",
        "Massachusetts",
        "Michigan",
        "Minnesota",
        "Mississippi",
        "Missouri",
        "Montana",
        "Nebraska",
        "Nevada",
        "New Hampshire",
        "New Jersey",
        "New Mexico",
        "New York",
        "North Carolina",
        "North Dakota",
        "Ohio",
        "Oklahoma",
        "Oregon",
        "Pennsylvania",
        "Puerto Rico",
        "Rhode Island",
        "South Carolina",
        "South Dakota",
        "Tennessee",
        "Texas",
        "Utah",
        "Vermont",
        "Virginia",
        "Washington",
        "West Virginia",
        "Wisconsin",
        "Wyoming"
    ];

    let whole_start = Instant::now();

    let temp_geom = match import_counties() {
        Ok(geom) => geom,
        Err(e) => panic!("Error importing counties: {}", e),
    };

    for state in states.iter() {
        let start = Instant::now();
        
        let temp_geom_reduced = match reduce_counties_feature_collection_by_state_string(temp_geom.clone(), state) {
            Ok(geom) => geom,
            Err(e) => panic!("Error reducing counties: {}", e),
        };

        let _temp_map = match make_converted_coordinate_map(temp_geom_reduced) {
            Ok(map) => map,
            Err(e) => panic!("Error making coordinate map: {}", e),
        };

        let elapsed = start.elapsed();

        println!("{} took: {} ms", state, elapsed.as_millis());
    }

    let whole_elapsed = whole_start.elapsed();
    println!("Whole program took: {} ms", whole_elapsed.as_millis());
}
