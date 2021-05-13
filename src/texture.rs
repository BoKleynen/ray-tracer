use image::io::Reader as ImageReader;
use image::GenericImageView;
use image::{ImageError, Pixel};

use crate::film::Rgb;
use crate::shade_rec::ShadeRec;
use crate::Point3;

pub trait Texture {
    fn get_color(&self, sr: &ShadeRec) -> Rgb;
}

pub struct ConstantColor(Rgb);

impl Texture for ConstantColor {
    fn get_color(&self, _sr: &ShadeRec) -> Rgb {
        self.0
    }
}

#[derive(Clone)]
pub struct ImageTexture {
    hres: u32,
    vres: u32,
    image: Vec<Rgb>,
    mapping: Option<Mapping>,
}

impl Texture for ImageTexture {
    fn get_color(&self, sr: &ShadeRec) -> Rgb {
        let (u, v) = match &self.mapping {
            Some(mapping) => {
                mapping.get_texel_coordinates(sr.local_hit_point, self.hres, self.vres)
            }
            None => (
                (self.hres as f64 * sr.uv.x).round() as u32 % self.hres,
                (self.vres as f64 * sr.uv.y).round() as u32 % self.vres,
            ),
        };

        let index = u + self.hres * (self.vres - v - 1);
        *self.image.get(index as usize).unwrap()
    }
}

impl ImageTexture {
    pub fn new(path: &str) -> Result<Self, ImageError> {
        const INV_MAX_VALUE: f64 = 1. / 255.;

        let image = ImageReader::open(path)?.decode()?;
        let pixels = image
            .pixels()
            .map(|(_, _, pixel)| {
                let [red, green, blue] = pixel.to_rgb().0;
                Rgb::new(
                    INV_MAX_VALUE * red as f64,
                    INV_MAX_VALUE * green as f64,
                    INV_MAX_VALUE * blue as f64,
                )
            })
            .collect();

        let texture = Self {
            hres: image.height(),
            vres: image.width(),
            image: pixels,
            mapping: None,
        };

        Ok(texture)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Mapping {
    Spherical,
}

impl Mapping {
    fn get_texel_coordinates(
        &self,
        _local_hit_point: Point3,
        _hres: u32,
        _vres: u32,
    ) -> (u32, u32) {
        match self {
            Mapping::Spherical => todo!(),
        }
    }
}
