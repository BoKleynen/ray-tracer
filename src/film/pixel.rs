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
}

impl Default for Pixel {
    fn default() -> Self {
        Pixel {
            color: RGB::black(),
            weight_sum: 0.0,
        }
    }
}
