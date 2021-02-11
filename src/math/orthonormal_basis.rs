use nalgebra::Vector3;

#[derive(Debug, Clone)]
pub struct OrthonormalBasis {
    pub(crate) u: Vector3<f64>,
    pub(crate) v: Vector3<f64>,
    pub(crate) w: Vector3<f64>,
}

impl OrthonormalBasis {
    pub fn from_vector(a: &Vector3<f64>) -> Option<Self> {
        let length = a.norm();
        if length == 0.0 {
            return None;
        }

        let w = (1.0 / length) * a;
        let u = if w.x.abs() > w.y.abs() {
            let inv_length = 1.0 / w.norm();
            Vector3::new(-w.z * inv_length, 0.0, w.x * inv_length)
        } else {
            let inv_length = 1.0 / w.norm();
            Vector3::new(0.0, w.z * inv_length, -w.y * inv_length)
        };
        let v = w.cross(&u);

        Some(Self { u, v, w })
    }

    pub fn from_vectors(a: &Vector3<f64>, b: &Vector3<f64>) -> Option<Self> {
        let cross = b.cross(&a);
        let length = cross.norm();

        if length == 0.0 {
            return None;
        }

        let w = a.normalize();
        let u = cross / length;
        let v = w.cross(&u);

        Some(Self { u, v, w })
    }
}
