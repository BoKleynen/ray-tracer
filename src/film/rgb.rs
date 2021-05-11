use image::Rgba;
use std::iter::Sum;
use std::ops::{Add, AddAssign, Div, Mul, Sub};

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Rgb {
    red: f64,
    green: f64,
    blue: f64,
}

impl Rgb {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        assert!(Self::is_valid_color_component(red));
        assert!(Self::is_valid_color_component(green));
        assert!(Self::is_valid_color_component(blue));

        Self { red, green, blue }
    }

    pub const fn black() -> Self {
        Rgb {
            red: 0.,
            green: 0.,
            blue: 0.,
        }
    }

    pub const fn white() -> Self {
        Rgb {
            red: 1.,
            green: 1.,
            blue: 1.,
        }
    }

    pub fn to_rgb(self) -> Rgba<u8> {
        let r = self.red.clamp(0., 255.) as u8;
        let g = self.green.clamp(0., 255.) as u8;
        let b = self.blue.clamp(0., 255.) as u8;

        Rgba([r, g, b, 255])
    }

    pub fn pow(self, exp: f64) -> Self {
        let red = self.red.powf(exp);
        let green = self.green.powf(exp);
        let blue = self.blue.powf(exp);

        Self { red, green, blue }
    }

    pub fn clamp(self, low: f64, high: f64) -> Self {
        let red = self.red.clamp(low, high);
        let green = self.green.clamp(low, high);
        let blue = self.blue.clamp(low, high);

        Self { red, green, blue }
    }

    fn is_valid_color_component(val: f64) -> bool {
        val.is_finite() && !val.is_nan()
    }
}

impl Add for Rgb {
    type Output = Rgb;

    fn add(self, rhs: Self) -> Self::Output {
        let red = self.red + rhs.red;
        let green = self.green + rhs.green;
        let blue = self.blue + rhs.blue;

        Self { red, green, blue }
    }
}

impl Sub for Rgb {
    type Output = Rgb;

    fn sub(self, rhs: Self) -> Self::Output {
        let red = self.red - rhs.red;
        let green = self.green - rhs.green;
        let blue = self.blue - rhs.blue;

        Self { red, green, blue }
    }
}

impl Mul<f64> for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: f64) -> Self::Output {
        let red = self.red * rhs;
        let green = self.green * rhs;
        let blue = self.blue * rhs;

        Self { red, green, blue }
    }
}

impl Div<f64> for Rgb {
    type Output = Rgb;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1. / rhs)
    }
}

impl Mul for Rgb {
    type Output = Rgb;

    fn mul(self, rhs: Self) -> Self::Output {
        let red = self.red * rhs.red;
        let green = self.green * rhs.green;
        let blue = self.blue * rhs.blue;

        Self { red, green, blue }
    }
}

impl AddAssign for Rgb {
    fn add_assign(&mut self, rhs: Self) {
        self.red += rhs.red;
        self.green += rhs.green;
        self.blue += rhs.blue;
    }
}

impl Sum for Rgb {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(Rgb::black(), Rgb::add)
    }
}
