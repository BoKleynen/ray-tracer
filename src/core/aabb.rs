use crate::core::{Axis, Ray};
use crate::Float;
use nalgebra::{Point3, Vector3};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct Aabb {
    pub p_min: Point3<Float>,
    pub p_max: Point3<Float>,
}

impl Default for Aabb {
    /// default returns an empty box.
    fn default() -> Self {
        let p_min = Point3::new(Float::MAX, Float::MAX, Float::MAX);
        let p_max = Point3::new(Float::MIN, Float::MIN, Float::MIN);

        Self { p_min, p_max }
    }
}

impl From<Point3<Float>> for Aabb {
    fn from(p: Point3<Float>) -> Self {
        let p_min = p;
        let p_max = p;

        Self { p_min, p_max }
    }
}

impl Aabb {
    pub fn new(p1: &Point3<Float>, p2: &Point3<Float>) -> Self {
        let p_min = Point3::new(p1.x.min(p2.x), p1.y.min(p2.y), p1.z.min(p2.z));
        let p_max = Point3::new(p1.x.max(p2.x), p1.y.max(p2.y), p1.z.max(p2.z));

        Self { p_min, p_max }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<(Float, Float)> {
        let mut t0 = 0.;
        let mut t1 = Float::MAX;

        for i in 0..3 {
            let inv_ray_dir = 1. / ray.o[i];
            let mut t_near = (self.p_min[i] - ray.o[i]) * inv_ray_dir;
            let mut t_far = (self.p_max[i] - ray.o[i]) * inv_ray_dir;

            if t_near > t_far {
                std::mem::swap(&mut t_near, &mut t_far)
            }

            t0 = if t_near > t0 { t_near } else { t0 };
            t1 = if t_far < t1 { t_far } else { t1 };
            if t0 > t1 {
                return None;
            }
        }

        Some((t0, t1))
    }

    pub fn centroid(&self) -> Point3<Float> {
        self.p_min + 0.5 * (self.p_max - self.p_min)
    }

    pub fn diagonal(&self) -> Vector3<Float> {
        self.p_min - self.p_max
    }

    pub fn surface_area(&self) -> Float {
        let diag = self.diagonal();
        2. * (diag.x * diag.y + diag.x * diag.z + diag.y * diag.z)
    }

    pub fn volume(&self) -> Float {
        let diag = self.diagonal();
        diag.x * diag.y * diag.z
    }

    /// returns the continuous position of a point relative to the corners
    /// of the box, where a point at the minimum corner has offset (0, 0, 0),
    /// a point at the maximum corner has offset (1, 1, 1), and so forth.
    pub fn offset(&self, p: Point3<Float>) -> Vector3<Float> {
        let mut res = p - self.p_min;
        res.x /= self.p_max.x - self.p_min.x;
        res.y /= self.p_max.y - self.p_min.y;
        res.z /= self.p_max.z - self.p_min.z;
        res
    }

    pub fn longest_axis(&self) -> (Axis, Float) {
        self.longest_axis_of(&Axis::all())
    }

    pub fn longest_axis_of(&self, directions: &[Axis]) -> (Axis, Float) {
        directions
            .iter()
            .map(|&axis| (axis, self.p_max[axis as usize] - self.p_min[axis as usize]))
            .max_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap())
            .unwrap()
    }

    pub fn overlaps(&self, other: &Aabb) -> bool {
        let x = self.p_max.x >= other.p_min.x && self.p_min.x <= other.p_max.x;
        let y = self.p_max.y >= other.p_min.y && self.p_min.y <= other.p_max.y;
        let z = self.p_max.z >= other.p_min.z && self.p_min.z <= other.p_max.z;
        x && y && z
    }

    pub fn is_inside(&self, p: Point3<Float>) -> bool {
        p.x >= self.p_min.x
            && p.x <= self.p_max.x
            && p.y >= self.p_min.y
            && p.y <= self.p_max.y
            && p.z >= self.p_min.z
            && p.z <= self.p_max.z
    }

    pub fn intersection(&self, other: &Aabb) -> Aabb {
        let p_min = Point3::new(
            self.p_min.x.max(other.p_min.x),
            self.p_min.y.max(other.p_min.y),
            self.p_min.z.max(other.p_min.z),
        );
        let p_max = Point3::new(
            self.p_max.x.min(other.p_max.x),
            self.p_max.y.min(other.p_max.y),
            self.p_max.z.min(other.p_max.z),
        );

        Aabb { p_min, p_max }
    }

    pub fn union(&self, other: impl Union) -> Self {
        other.union(self)
    }
}

pub trait Union {
    fn union(&self, aabb: &Aabb) -> Aabb;
}

impl Union for Point3<Float> {
    fn union(&self, bbox: &Aabb) -> Aabb {
        let p_min = Point3::new(
            self.x.min(bbox.p_min.x),
            self.y.min(bbox.p_min.y),
            self.z.min(bbox.p_min.z),
        );
        let p_max = Point3::new(
            self.x.max(bbox.p_min.x),
            self.y.max(bbox.p_min.y),
            self.z.max(bbox.p_min.z),
        );

        Aabb { p_min, p_max }
    }
}

impl Union for Aabb {
    fn union(&self, bbox: &Aabb) -> Aabb {
        let p_min = Point3::new(
            self.p_min.x.min(bbox.p_min.x),
            self.p_min.y.min(bbox.p_min.y),
            self.p_min.z.min(bbox.p_min.z),
        );
        let p_max = Point3::new(
            self.p_max.x.max(bbox.p_max.x),
            self.p_max.y.max(bbox.p_max.y),
            self.p_max.z.max(bbox.p_max.z),
        );

        Aabb { p_min, p_max }
    }
}
