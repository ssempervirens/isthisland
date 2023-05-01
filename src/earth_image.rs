use anyhow::Result;
use geo::Point;
use image::{ImageBuffer, Pixel, Rgb, RgbImage};

use crate::{get_polygons_shapefile, point_on_land};

fn coords_for_pixel((x, y): (u32, u32), (max_x, max_y): (u32, u32)) -> Point {
    // maybe flipped if y=0 is top row?
    // let lat = (f64::from(y) / f64::from(max_y)) * 180.0 - 90.;
    let lat = (f64::from(y) / f64::from(max_y)) * (-180.0) + 90.;
    let lon = (f64::from(x) / f64::from(max_x)) * 360.0 - 180.;
    Point::new(lon, lat)
}

fn construct_image(
) -> Result<ImageBuffer<Rgb<u8>, Vec<<Rgb<u8> as Pixel>::Subpixel>>, anyhow::Error> {
    let polygons = get_polygons_shapefile().unwrap();

    let bounds = (2000, 1000);

    let image = ImageBuffer::from_fn(bounds.0, bounds.1, |x, y| {
        // turn the pixel into a point
        let coords = coords_for_pixel((x, y), bounds);
        // check if that pixel is on land
        let result = point_on_land(&polygons, coords);
        if result {
            // generate a white pixel
            Rgb([0, 0, 0])
        } else {
            // generate a black pixel
            Rgb([255, 255, 255])
        }
    });

    Ok(image)
}

pub fn save_image() {
    // save image so we can look at it
    let image = construct_image().unwrap();
    image.save("test_image.png").unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dump_image() {
        save_image()
    }
}
