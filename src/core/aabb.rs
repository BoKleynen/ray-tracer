use crate::core::{Axis, Ray};
use crate::{Float, Point3, Vector3, K_EPSILON};

#[derive(Debug, Copy, Clone)]
pub struct Aabb {
    pub p0: Point3,
    pub p1: Point3,
}

impl Default for Aabb {
    fn default() -> Self {
        let p0 = Point3::new(Float::MAX, Float::MAX, Float::MAX);
        let p1 = Point3::new(Float::MIN, Float::MIN, Float::MIN);

        Self { p0, p1 }
    }
}

impl Aabb {
    pub fn new(p0: Point3, p1: Point3) -> Self {
        debug_assert!(p0.x <= p1.x);
        debug_assert!(p0.y <= p1.y);
        debug_assert!(p0.z <= p1.z);

        Self { p0, p1 }
    }

    pub fn intersects(&self, ray: &Ray) -> bool {
        self.intersect(ray).is_some()
    }

    pub fn intersect(&self, ray: &Ray) -> Option<Float> {
        let ox = ray.origin.x;
        let oy = ray.origin.y;
        let oz = ray.origin.z;

        let dx = ray.direction.x;
        let dy = ray.direction.y;
        let dz = ray.direction.z;

        let a = 1. / dx;
        let (tx_min, tx_max) = if a >= 0. {
            ((self.p0.x - ox) * a, (self.p1.x - ox) * a)
        } else {
            ((self.p1.x - ox) * a, (self.p0.x - ox) * a)
        };

        let b = 1. / dy;
        let (ty_min, ty_max) = if b >= 0. {
            ((self.p0.y - oy) * b, (self.p1.y - oy) * b)
        } else {
            ((self.p1.y - oy) * b, (self.p0.y - oy) * b)
        };

        let c = 1. / dz;
        let (tz_min, tz_max) = if c >= 0. {
            ((self.p0.z - oz) * c, (self.p1.z - oz) * c)
        } else {
            ((self.p1.z - oz) * c, (self.p0.z - oz) * c)
        };

        // find largest entering t value
        let t0 = tx_min.max(ty_min).max(tz_min);

        // find smallest exiting t value
        let t1 = tx_max.min(ty_max).min(tz_max);

        if t0 <= t1 && t1 > K_EPSILON {
            if t0 > K_EPSILON {
                Some(t0)
            } else {
                Some(t1)
            }
        } else {
            None
        }
    }

    pub fn vertices(&self) -> [Point3; 8] {
        let x0 = self.p0.x;
        let y0 = self.p0.y;
        let z0 = self.p0.z;
        let x1 = self.p1.x;
        let y1 = self.p1.y;
        let z1 = self.p1.z;

        [
            Point3::new(x0, y0, z0),
            Point3::new(x0, y0, z1),
            Point3::new(x0, y1, z0),
            Point3::new(x0, y1, z1),
            Point3::new(x1, y0, z0),
            Point3::new(x1, y0, z1),
            Point3::new(x1, y1, z0),
            Point3::new(x1, y1, z1),
        ]
    }

    pub fn centroid(&self) -> Point3 {
        self.p0 + 0.5 * (self.p1 - self.p0)
    }

    pub fn diagonal(&self) -> Vector3 {
        self.p1 - self.p0
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
    pub fn offset(&self, p: Point3) -> Vector3 {
        let mut res = p - self.p0;
        res.x /= self.p1.x - self.p0.x;
        res.y /= self.p1.y - self.p0.y;
        res.z /= self.p1.z - self.p0.z;
        res
    }

    pub fn longest_axis(&self) -> (Axis, Float) {
        self.longest_axis_of(&Axis::all())
    }

    pub fn longest_axis_of(&self, directions: &[Axis]) -> (Axis, Float) {
        directions
            .iter()
            .map(|&axis| (axis, self.p1[axis as usize] - self.p0[axis as usize]))
            .max_by(|(_, l1), (_, l2)| l1.partial_cmp(l2).unwrap())
            .unwrap()
    }

    pub fn overlaps(&self, other: &Aabb) -> bool {
        let x = self.p1.x >= other.p0.x && self.p0.x <= other.p1.x;
        let y = self.p1.y >= other.p0.y && self.p0.y <= other.p1.y;
        let z = self.p1.z >= other.p0.z && self.p0.z <= other.p1.z;
        x && y && z
    }

    pub fn is_inside(&self, p: Point3) -> bool {
        p.x >= self.p0.x
            && p.x <= self.p1.x
            && p.y >= self.p0.y
            && p.y <= self.p1.y
            && p.z >= self.p0.z
            && p.z <= self.p1.z
    }
}

pub trait Union<T> {
    fn union(&self, other: T) -> Aabb;
}

impl Union<Point3> for Aabb {
    fn union(&self, p: Point3) -> Self {
        let p0 = Point3::new(self.p0.x.min(p.x), self.p0.y.min(p.y), self.p0.z.min(p.z));
        let p1 = Point3::new(self.p1.x.max(p.x), self.p1.y.max(p.y), self.p1.z.max(p.z));

        Self { p0, p1 }
    }
}

impl Union<Aabb> for Aabb {
    fn union(&self, bbox: Aabb) -> Self {
        let p0 = Point3::new(
            self.p0.x.min(bbox.p0.x),
            self.p0.y.min(bbox.p0.y),
            self.p0.z.min(bbox.p0.z),
        );
        let p1 = Point3::new(
            self.p1.x.max(bbox.p1.x),
            self.p1.y.max(bbox.p1.y),
            self.p1.z.max(bbox.p1.z),
        );

        Self { p0, p1 }
    }
}
