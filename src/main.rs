use geometry::import_map;

fn main() {
    let temp_geom = geometry::import_map();
    println!("{:?}", temp_geom.features.len());
    println!("Hello, world!");
}
