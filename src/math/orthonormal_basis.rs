use crate::math::Vector;
use std::error::Error;

#[derive(Debug, Clone)]
pub struct OrthonormalBasis {
    u: Vector<f64, 3>,
    v: Vector<f64, 3>,
    w: Vector<f64, 3>,
}

impl OrthonormalBasis {
    pub fn from_vector(a: &Vector<f64, 3>) -> Option<Self> {
        let length = a.length();
        if length == 0.0 {
            return None;
        }

        let w = a * &(1.0 / length);
        let u = if w.x().abs() > w.y().abs() {
            let inv_length = 1.0 / w.length();
            Vector::new(-w.z() * inv_length, 0.0, w.x() * inv_length)
        } else {
            let inv_length = 1.0 / w.length();
            Vector::new(0.0, w.z() * inv_length, -w.y() * inv_length)
        };
        let v = w.cross_product(&u);

        Some(Self { u, v, w })
    }

    pub fn from_vectors(a: &Vector<f64, 3>, b: &Vector<f64, 3>) -> Option<Self> {
        let cross = b.cross_product(a);
        let length = cross.length();

        if length == 0.0 {
            return None;
        }

        let w = a.normalize();
        let u = &cross * &(1.0 / length);
        let v = w.cross_product(&u);

        Some(Self { u, v, w })
    }
}
