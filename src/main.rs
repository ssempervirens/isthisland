use core::panic;

use geo_types::{coord, Point};
use shapefile::Reader;

fn main() -> Result<(), anyhow::Error> {
    let mut reader = Reader::from_path("data/ne_10m_land/ne_10m_land.shp").unwrap();
    for shape_record in reader.iter_shapes_and_records() {
        let (shape, record) = shape_record?;
        println!("Geometry: {}, Properties {:?}", shape, record);
    }
    Ok(())
}

fn point_on_land(point: (f64, f64)) -> bool {
    let point = Point::new(point.0, point.1);
    false
}
