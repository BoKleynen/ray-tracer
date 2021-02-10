use super::{Pixel, RGB};
use image::{ImageBuffer, RgbaImage};
use std::num::NonZeroUsize;

pub struct FrameBuffer {
    buffer: Vec<Pixel>,
    x_res: usize,
    y_res: usize,
}

impl FrameBuffer {
    pub fn new(x_res: NonZeroUsize, y_res: NonZeroUsize) -> Self {
        let x_res = x_res.get();
        let y_res = y_res.get();
        let buffer = vec![Pixel::default(); x_res * y_res];

        Self {
            buffer,
            x_res,
            y_res,
        }
    }

    pub fn buffer(&mut self) -> &mut [Pixel] {
        &mut self.buffer
    }

    fn get_pixel(&self, x: usize, y: usize) -> Pixel {
        self.buffer[self.x_res * y + x]
    }

    pub fn to_buffered_image(&self, sensitivity: f64, gamma: f64) -> RgbaImage {
        let inv_sensitivity = 1.0 / sensitivity;
        let inv_gamma = 1.0 / gamma;

        ImageBuffer::from_fn(self.x_res as u32, self.y_res as u32, |x, y| {
            let pixel = self.get_pixel(x as usize, y as usize);
            RGB::to_rgb(
                (pixel.spectrum().clamp(0.0, inv_sensitivity) * sensitivity).pow(inv_gamma) * 255.0,
            )
        })
    }
}
