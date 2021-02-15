use super::RGB;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    color: RGB,
    weight_sum: f64,
}

impl Pixel {
    pub fn spectrum(self) -> RGB {
        self.color
    }

    pub fn add(&mut self, spectrum: RGB, weight: f64) {
        self.color += spectrum * weight
    }

    pub fn set(&mut self, spectrum: RGB) {
        self.color = spectrum
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: RGB::black(),
            weight_sum: 0.,
        }
    }
}
