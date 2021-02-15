use super::{Pixel, RGB};
use image::{ImageBuffer, RgbaImage};

pub struct FrameBuffer {
    buffer: Vec<Pixel>,
    x_res: usize,
    y_res: usize,
}

impl FrameBuffer {
    pub fn new(x_res: usize, y_res: usize) -> Self {
        let x_res = x_res;
        let y_res = y_res;
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
        self.buffer[self.x_res * (self.y_res - y - 1) + x]
    }

    pub fn to_rgba_image(&self, sensitivity: f64, gamma: f64) -> RgbaImage {
        let inv_sensitivity = 1. / sensitivity;
        let inv_gamma = 1. / gamma;

        ImageBuffer::from_fn(self.x_res as u32, self.y_res as u32, |x, y| {
            let pixel = self.get_pixel(x as usize, y as usize);
            RGB::to_rgb(
                (pixel.spectrum().clamp(0., inv_sensitivity) * sensitivity).pow(inv_gamma) * 255.,
            )
        })
    }
}
