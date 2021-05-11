use super::Rgb;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Pixel {
    color: Rgb,
    weight_sum: f64,
}

impl Pixel {
    pub fn spectrum(self) -> Rgb {
        self.color
    }

    pub fn add(&mut self, spectrum: Rgb, weight: f64) {
        self.color += spectrum * weight
    }

    pub fn set(&mut self, spectrum: Rgb) {
        self.color = spectrum
    }
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: Rgb::black(),
            weight_sum: 0.,
        }
    }
}
