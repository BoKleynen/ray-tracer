use crate::camera::Camera;
use crate::math::homogeneous::{Point, Ray};
use crate::math::{OrthonormalBasis, Vector};
use std::f64;
use std::num::{NonZeroU64, NonZeroUsize};

pub struct PerspectiveCamera {
    origin: Point,
    basis: OrthonormalBasis,
    width: f64,
    height: f64,
    inv_x_res: f64,
    inv_y_res: f64,
}

impl PerspectiveCamera {
    pub fn new_with_lookat(
        x_res: NonZeroUsize,
        y_res: NonZeroUsize,
        origin: Point,
        lookat: Vector<f64, 3>,
        up: Vector<f64, 3>,
        fov: f64,
    ) -> Self {
        if x_res < NonZeroUsize::new(1).unwrap() {
            panic!()
        }

        if y_res < NonZeroUsize::new(1).unwrap() {
            panic!()
        }

        if fov < 0.0 {
            panic!()
        }

        if fov >= 180.0 {
            panic!()
        }

        let basis = OrthonormalBasis::from_vectors(&(-(&lookat)), &up).unwrap();

        let inv_x_res = 1.0 / x_res.get() as f64;
        let inv_y_res = 1.0 / y_res.get() as f64;
        let width = 2.0 * (0.5 * fov.to_radians()).tan();
        let height = (y_res.get() as f64 * width) * inv_x_res;

        Self {
            origin,
            basis,
            inv_x_res,
            inv_y_res,
            width,
            height,
        }
    }

    fn new_with_destination() -> Self {
        unimplemented!()
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: (f64, f64)) -> Ray {
        let u = self.width * (sample.0 * self.inv_x_res - 0.5);
        let v = self.height * (sample.1 * self.inv_y_res - 0.5);

        let direction = &(&(&self.basis.u * &u) + &(&self.basis.v * &v)) - &self.basis.w;

        Ray::new(self.origin.clone(), direction.into())
    }
}
