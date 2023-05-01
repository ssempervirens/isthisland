mod earth_image;

use anyhow::Result;
use earth_image::save_image;
use geo::{Contains, MultiPolygon, Point};
use shapefile::Shape;

fn get_polygons_shapefile() -> Result<Vec<MultiPolygon<f64>>, anyhow::Error> {
    // return polygons as a vector and give that to point in polygons function?
    let shapes = shapefile::read("data/ne_10m_land/ne_10m_land.shp")?;
    let polygons: Vec<MultiPolygon<f64>> = shapes
        .into_iter()
        .filter_map(|(shape, _record)| match shape {
            Shape::Polygon(polygon) => Some(polygon.into()),
            _ => None,
        })
        .collect();

    Ok(polygons)
}

fn point_on_land(polygons: &[MultiPolygon<f64>], point: Point<f64>) -> bool {
    // order is lon lat
    //let point = Point::new(point.1, point.0);
    // iterate over each of the ten records as polygons
    for polygon in polygons {
        if polygon.contains(&point) {
            return true;
        }
    }
    false
}

fn main() -> Result<(), anyhow::Error> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_land() {
        // fresno, ca should be on land
        let polygons = get_polygons_shapefile().unwrap();
        assert_eq!(
            point_on_land(&polygons, Point::new(-119.775, 36.7475)),
            true,
        )
    }

    #[test]
    fn point_in_lake() {
        // lake tahoe should not be on land
        let polygons = get_polygons_shapefile().unwrap();
        //assert_eq!(point_on_land(polygons, (39.096849, -120.032351)), false,)
    }

    #[test]
    fn point_in_ocean() {
        // point nemo, in the middle of the ocean, should not be on land
        let polygons = get_polygons_shapefile().unwrap();
        //assert_eq!(point_on_land(polygons, (-48.8766667, -123.393333)), false,)
    }
}
