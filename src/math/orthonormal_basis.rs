use nalgebra::Unit;

use crate::{Vector, K_EPSILON};

#[derive(Debug, Clone)]
pub struct OrthonormalBasis {
    pub(crate) u: Unit<Vector>,
    pub(crate) v: Unit<Vector>,
    pub(crate) w: Unit<Vector>,
}

impl OrthonormalBasis {
    pub fn from_vector(a: &Vector) -> Option<Self> {
        let w = Unit::try_new(*a, K_EPSILON)?;

        let u = if w.x.abs() > w.y.abs() {
            Unit::new_normalize(Vector::new(-w.z, 0., w.x))
        } else {
            Unit::new_normalize(Vector::new(0., w.z, -w.y))
        };
        let v = Unit::new_normalize(w.cross(&u));

        Some(Self { u, v, w })
    }

    pub fn from_vectors(a: &Vector, b: &Vector) -> Option<Self> {
        let u = Unit::try_new(b.cross(&a), K_EPSILON)?;
        let w = Unit::new_normalize(*a);
        let v = Unit::new_normalize(w.cross(&u));

        Some(Self { u, v, w })
    }
}
