use crate::K_EPSILON;
use nalgebra::{Unit, Vector3};

#[derive(Debug, Clone)]
pub struct OrthonormalBasis {
    pub(crate) u: Unit<Vector3<f64>>,
    pub(crate) v: Unit<Vector3<f64>>,
    pub(crate) w: Unit<Vector3<f64>>,
}

impl OrthonormalBasis {
    pub fn from_vector(a: &Vector3<f64>) -> Option<Self> {
        let w = Unit::try_new(*a, K_EPSILON)?;

        let u = if w.x.abs() > w.y.abs() {
            Unit::new_normalize(Vector3::new(-w.z, 0., w.x))
        } else {
            Unit::new_normalize(Vector3::new(0., w.z, -w.y))
        };
        let v = Unit::new_normalize(w.cross(&u));

        Some(Self { u, v, w })
    }

    pub fn from_vectors(a: &Vector3<f64>, b: &Vector3<f64>) -> Option<Self> {
        let u = Unit::try_new(b.cross(&a), K_EPSILON)?;
        let w = Unit::new_normalize(*a);
        let v = Unit::new_normalize(w.cross(&u));

        Some(Self { u, v, w })
    }
}
