#[cfg(test)]
mod tests;

use bevy::prelude::*;
use georaster::geotiff::{GeoTiffReader, RasterValue};
use proj::Proj;
use std::{collections::HashMap, fs::File, io::BufReader};

#[derive(Resource)]
pub struct HeightMap {
    data: GeoTiffReader<BufReader<File>>,
    converter: ProjWrapper,
    lookup_cache: HashMap<(u32, u32), f32>,
}

struct ProjWrapper(Proj);

unsafe impl Sync for ProjWrapper {}
unsafe impl Send for ProjWrapper {}

impl HeightMap {
    pub fn load_from_file(file_name: &str) -> Self {
        let img_file = BufReader::new(File::open(file_name).unwrap());
        let mut data = GeoTiffReader::open(img_file).unwrap();

        // Mercator to UTM32
        let converter = Proj::new_known_crs("ESRI:53004", "EPSG:32632", None).unwrap();

        let (w, h) = data.image_info().dimensions.unwrap();
        let mut lookup_cache = HashMap::new();

        // warm up cache
        for (x, y, pixel) in data.pixels(0, 0, w, h) {
            if let RasterValue::F64(v) = pixel {
                lookup_cache.insert((x, y), v as f32);
            }
        }

        Self {
            data,
            lookup_cache,
            converter: ProjWrapper(converter),
        }
    }

    fn get_pixel(&mut self, x: u32, y: u32) -> f32 {
        let (w, h) = self.data.image_info().dimensions.unwrap();

        let x = u32::max(u32::min(x, w - 1), 0);
        let y = u32::max(u32::min(y, h - 1), 0);

        *self
            .lookup_cache
            .entry((x, y))
            .or_insert_with(|| match self.data.read_pixel(x, y) {
                RasterValue::F64(v) => v as f32,
                _ => panic!("Unexpected pixel type"),
            })
    }

    // TODO: can we avoid mutable here?!
    pub fn height_at_position(&mut self, x: f64, y: f64) -> f32 {
        // merc -> utm32
        let (x, y) = self.converter.0.convert((x, y)).unwrap();
        // TOP left
        let lower_left = self.data.origin().unwrap();
        let pixel_size = self.data.pixel_size().unwrap();

        // pixels
        let (x, y) = (
            (x - lower_left[0]) / pixel_size[0],
            (y - lower_left[1]) / pixel_size[1],
        );

        // let y = 1.0 - y;

        let mut d_x = (x % 1.0) as f32;
        let mut d_y = (y % 1.0) as f32;

        // rust modulo is weird, we need to make sure it is positive
        if d_x < 0.0 {
            d_x += 1.0
        }
        if d_y < 0.0 {
            d_y += 1.0
        }

        assert!(d_x >= 0.0);
        assert!(d_x <= 1.0);
        assert!(d_y >= 0.0);
        assert!(d_y <= 1.0);

        let d_f_f = self.get_pixel(x.floor() as u32, y.floor() as u32);
        let d_f_c = self.get_pixel(x.floor() as u32, y.ceil() as u32);
        let d_c_f = self.get_pixel(x.ceil() as u32, y.floor() as u32);
        let d_c_c = self.get_pixel(x.ceil() as u32, y.ceil() as u32);

        d_f_f * (1.0 - d_x) * (1.0 - d_y)
            + d_f_c * (1.0 - d_x) * (d_y)
            + d_c_f * d_x * (1.0 - d_y)
            + d_c_c * d_x * d_y
    }
}
