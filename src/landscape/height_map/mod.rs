#[cfg(test)]
mod tests;

use bevy::prelude::*;
use georaster::geotiff::{GeoTiffReader, RasterValue};
use proj::Proj;
use std::{collections::HashMap, fs::File, io::BufReader, sync::Arc};

#[derive(Resource, Clone)]
pub struct HeightMap(Arc<HeightMapData>);

impl HeightMap {
    #[cfg(test)]
    pub fn test_dummy() -> Self {
        Self(Arc::new(HeightMapData::test_dummy()))
    }

    pub fn load_from_file(file_name: &str) -> Self {
        Self(Arc::new(HeightMapData::load_from_file(file_name)))
    }

    pub fn height_at_position(&self, x: f64, y: f64) -> f32 {
        self.0.height_at_position(x, y)
    }
}

pub struct HeightMapData {
    origin: (f64, f64),
    pixel_size: (f64, f64),
    dimensions: (u32, u32),
    converter: ProjWrapper,
    values: HashMap<(u32, u32), f32>,
}

struct ProjWrapper(Proj);

unsafe impl Sync for ProjWrapper {}
unsafe impl Send for ProjWrapper {}

impl HeightMapData {
    #[cfg(test)]
    pub fn test_dummy() -> Self {
        let converter = Proj::new_known_crs("ESRI:53004", "EPSG:32632", None).unwrap();

        Self {
            origin: (0.0, 0.0),
            pixel_size: (10.0, 10.0),
            dimensions: (10, 10),
            converter: ProjWrapper(converter),
            values: HashMap::new(),
        }
    }

    pub fn load_from_file(file_name: &str) -> Self {
        let img_file = BufReader::new(File::open(file_name).unwrap());
        let mut data = GeoTiffReader::open(img_file).unwrap();

        // Mercator to UTM32
        let converter = Proj::new_known_crs("ESRI:53004", "EPSG:32632", None).unwrap();

        let (w, h) = data.image_info().dimensions.unwrap();
        let mut values = HashMap::new();

        // warm up cache
        for (x, y, pixel) in data.pixels(0, 0, w, h) {
            if let RasterValue::F64(v) = pixel {
                values.insert((x, y), v as f32);
            }
        }

        let origin = data.origin().unwrap();
        let pixel_size = data.pixel_size().unwrap();

        Self {
            origin: (origin[0], origin[1]),
            pixel_size: (pixel_size[0], pixel_size[1]),
            dimensions: data.image_info().dimensions.unwrap(),
            values,
            converter: ProjWrapper(converter),
        }
    }

    fn get_pixel(&self, x: u32, y: u32) -> f32 {
        let x = u32::max(u32::min(x, self.dimensions.0 - 1), 0);
        let y = u32::max(u32::min(y, self.dimensions.1 - 1), 0);

        *self.values.get(&(x, y)).unwrap_or(&0.0)
    }

    pub fn height_at_position(&self, x: f64, y: f64) -> f32 {
        // merc -> utm32
        let (x, y) = self.converter.0.convert((x, y)).unwrap();
        // TOP left
        let origin = self.origin;
        let pixel_size = self.pixel_size;

        // pixels
        let (x, y) = ((x - origin.0) / pixel_size.0, (y - origin.1) / pixel_size.1);

        // remainder not modulo
        let d_x = (((x % 1.0) + 1.0) % 1.0) as f32;
        let d_y = (((y % 1.0) + 1.0) % 1.0) as f32;

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
