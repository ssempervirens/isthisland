use anyhow::Result;
use geo::{Point, Polygon};
use shapefile::Shape;

/*
fn get_polygons_shapefile() -> Result<Vec<Polygon<f64>>, anyhow::Error> {
    // return polygons as a vector and give that to point in polygons function?
    let shapes = shapefile::read("data/ne_10m_land/ne_10m_land.shp")?;
    let polygons: Vec<Polygon<f64>> = shapes
        .into_iter()
        .filter_map(|(shape, record)| match shape {
            Shape::Polygon(poly) => Some(poly),
            _ => None,
        })
        .collect();

    Ok(polygons)
}

fn point_in_polygons(polygons: Vec<Polygon<f64>>, point: (f64, f64)) -> bool {
    // order is lon lat
    let point = Point::new(point.1, point.0);
    // iterate over each of the ten records as polygons
    for polygon in polygons {
        if polygon.contains(&point) {
            return true;
        }
    }
    false
}
 */

fn main() -> Result<(), anyhow::Error> {
    let shapes = shapefile::read("data/ne_10m_land/ne_10m_land.shp")?;
    for (shape, record) in shapes {
        let geo_shape = geo_types::Geometry::<f64>::try_from(shape)?;
        println!("")
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn point_is_land() {
        // fresno, ca should be on land
        assert_eq!(point_on_land((36.7475, -119.775)), true,)
    }

    #[test]
    fn point_in_lake() {
        // lake tahoe should not be on land
        assert_eq!(point_on_land((39.096849, -120.032351)), false,)
    }

    fn point_in_ocean() {
        // point nemo, in the middle of the ocean, should not be on land
        assert_eq!(point_on_land((-48.8766667, -123.393333)), false,)
    }
}
