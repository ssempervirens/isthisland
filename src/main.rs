use core::panic;
use geo::{Point, Polygon};
use shapefile::{Reader, Shape};

fn main(point: (f64, f64)) -> bool {
    let mut reader = Reader::from_path("data/ne_10m_land/ne_10m_land.shp").unwrap();
    // order is lon lat
    let point = Point::new(point.1, point.0);
    // iterate over each of the ten records as polygons
    for shape_record in reader.iter_shapes_and_records() {
        let shape = shape_record.unwrap().0;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_on_land() {
        // fresno, ca should be on land
        assert_eq!(main((-119.775, 36.747)), true,)
    }

    #[test]
    fn point_in_lake() {
        // lake tahoe should not be on land
        assert_eq!(main((-120.032351, 39.096849)), false,)
    }

    fn point_in_ocean() {
        // point nemo, in the middle of the ocean, should not be on land
        assert_eq!(main((-123.393333, -48.8766667)), false,)
    }
}
