use image::Rgba;
use std::cmp::{max, min};
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct RGB {
    red: f64,
    green: f64,
    blue: f64,
}

impl RGB {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        assert!(Self::is_valid_color_component(red));
        assert!(Self::is_valid_color_component(green));
        assert!(Self::is_valid_color_component(blue));

        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        RGB {
            red: 0.0,
            green: 0.0,
            blue: 0.0,
        }
    }

    pub fn to_rgb(self) -> Rgba<u8> {
        let r = min(255, max(0, self.red.round() as i32)) as u8;
        let g = min(255, max(0, self.green.round() as i32)) as u8;
        let b = min(255, max(0, self.blue.round() as i32)) as u8;

        Rgba([r, g, b, 255])
    }

    pub fn pow(self, exp: f64) -> Self {
        let red = self.red.powf(exp);
        let green = self.green.powf(exp);
        let blue = self.blue.powf(exp);

        Self { red, green, blue }
    }

    pub fn clamp(self, low: f64, high: f64) -> Self {
        let red = high.min(low.max(self.red));
        let green = high.min(low.max(self.green));
        let blue = high.min(low.max(self.blue));

        Self { red, green, blue }
    }

    fn is_valid_color_component(val: f64) -> bool {
        val.is_finite() && !val.is_nan()
    }
}

impl Add for RGB {
    type Output = RGB;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;

        Self { red, green, blue }
    }
}

impl Sub for RGB {
    type Output = RGB;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red - rhs.red;
        let green = self.green - rhs.green;
        let blue = self.blue - rhs.blue;

        Self { red, green, blue }
    }
}

impl Mul<f64> for RGB {
    type Output = RGB;

    fn mul(self, rhs: f64) -> Self::Output {
        let red = self.red * rhs;
        let green = self.green * rhs;
        let blue = self.blue * rhs;

        Self { red, green, blue }
    }
}

impl Div<f64> for RGB {
    type Output = RGB;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

impl Mul for RGB {
    type Output = RGB;

    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;

        Self { red, green, blue }
    }
}

impl AddAssign for RGB {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}
