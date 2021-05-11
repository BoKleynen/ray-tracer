use nalgebra::Unit;
use std::fs::File;
use std::io::Read;
use std::mem;
use std::sync::Arc;

use crate::math::Ray;
use crate::shape::aabb::Aabb;
use crate::shape::compound::Compound;
use crate::shape::{Bounded, Hit, Intersect};
use crate::{Point, Vector, K_EPSILON};

#[derive(Default)]
pub struct Mesh {
    vertexes: Vec<Point>,
    normals: Vec<Unit<Vector>>,
}

#[repr(transparent)]
pub struct SmoothTriangle {
    inner: Triangle,
}

impl Bounded for SmoothTriangle {
    fn bbox(&self) -> Aabb {
        self.inner.bounding_box()
    }
}

impl Intersect for SmoothTriangle {
    type Intersection = ();

    fn intersect(&self, ray: &Ray) -> Option<Hit<()>> {
        self.inner.intersect(ray).map(|hit| {
            let beta = hit.beta;
            let gamma = hit.gamma;
            let normal = beta * *self.inner.n1()
                + gamma * *self.inner.n2()
                + (1. - beta - gamma) * *self.inner.n0();

            Hit {
                t: hit.t,
                normal,
                local_hit_point: hit.local_hit_point,
                shape: (),
            }
        })
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }
}

#[repr(transparent)]
pub struct FlatTriangle {
    inner: Triangle,
}

impl Bounded for FlatTriangle {
    fn bbox(&self) -> Aabb {
        self.inner.bounding_box()
    }
}

impl Intersect for FlatTriangle {
    type Intersection = ();

    fn intersect(&self, ray: &Ray) -> Option<Hit<()>> {
        self.inner.intersect(ray).map(|hit| Hit {
            t: hit.t,
            normal: *self.inner.normal,
            local_hit_point: hit.local_hit_point,
            shape: (),
        })
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }
}

struct Triangle {
    mesh: Arc<Mesh>,
    idx0: usize,
    idx1: usize,
    idx2: usize,
    normal: Unit<Vector>,
}

impl Triangle {
    fn bounding_box(&self) -> Aabb {
        let v0 = self.v0();
        let v1 = self.v1();
        let v2 = self.v2();

        let min_x = v0.x.min(v1.x).min(v2.x);
        let max_x = v0.x.max(v1.x).max(v2.x);
        let min_y = v0.y.min(v1.y).min(v2.y);
        let max_y = v0.y.max(v1.y).max(v2.y);
        let min_z = v0.z.min(v1.z).min(v2.z);
        let max_z = v0.z.max(v1.z).max(v2.z);

        Aabb::new(
            Point::new(min_x, min_y, min_z),
            Point::new(max_x, max_y, max_z),
        )
    }

    fn intersect(&self, ray: &Ray) -> Option<TriangleHit> {
        let v0 = self.mesh.vertexes[self.idx0];
        let v1 = self.mesh.vertexes[self.idx1];
        let v2 = self.mesh.vertexes[self.idx2];

        let a = v0.x - v1.x;
        let b = v0.x - v2.x;
        let c = ray.direction().x;
        let d = v0.x - ray.origin().x;

        let e = v0.y - v1.y;
        let f = v0.y - v2.y;
        let g = ray.direction().y;
        let h = v0.y - ray.origin().y;

        let i = v0.z - v1.z;
        let j = v0.z - v2.z;
        let k = ray.direction().z;
        let l = v0.z - ray.origin().z;

        let m = f * k - g * j;
        let n = h * k - g * l;
        let p = f * l - h * j;
        let q = g * i - e * k;
        let s = e * j - f * i;

        let inv_denom = 1. / (a * m + b * q + c * s);

        let e1 = d * m - b * n - c * p;
        let beta = e1 * inv_denom;

        if beta < 0. {
            return None;
        }

        let r = e * l - h * i;
        let e2 = a * n + d * q + c * r;
        let gamma = e2 * inv_denom;

        if gamma < 0. || beta + gamma > 1. {
            return None;
        }

        let e3 = a * p - b * r + d * s;
        let t = e3 * inv_denom;

        if t < K_EPSILON {
            return None;
        }

        let local_hit_point = ray.origin() + t * ray.direction();
        Some(TriangleHit {
            t,
            local_hit_point,
            beta,
            gamma,
        })
    }

    fn n0(&self) -> Unit<Vector> {
        self.mesh.normals[self.idx0]
    }

    fn n1(&self) -> Unit<Vector> {
        self.mesh.normals[self.idx1]
    }

    fn n2(&self) -> Unit<Vector> {
        self.mesh.normals[self.idx2]
    }

    fn v0(&self) -> Point {
        self.mesh.vertexes[self.idx0]
    }

    fn v1(&self) -> Point {
        self.mesh.vertexes[self.idx1]
    }

    fn v2(&self) -> Point {
        self.mesh.vertexes[self.idx2]
    }
}

struct TriangleHit {
    t: f64,
    local_hit_point: Point,
    beta: f64,
    gamma: f64,
}

pub struct Obj {
    vertexes: Vec<Point>,
    texture_coordinates: Vec<(f64, f64)>,
    vertex_normals: Vec<Vector>,
    triangles: Vec<ObjTriangle>,
}

impl Obj {
    pub fn load(path: &str) -> Option<Obj> {
        let input = read_file(path)?;

        let mut obj = Self {
            vertexes: Vec::new(),
            texture_coordinates: Vec::new(),
            vertex_normals: Vec::new(),
            triangles: Vec::new(),
        };

        for line in input.lines() {
            let mut parts = line.split_ascii_whitespace();
            match parts.next()? {
                "v" => {
                    let x = parts.next()?.parse().ok()?;
                    let y = parts.next()?.parse().ok()?;
                    let z = parts.next()?.parse().ok()?;

                    obj.vertexes.push(Point::new(x, y, z));
                }
                "vt" => {
                    let u = parts.next()?.parse().ok()?;
                    let v = parts.next()?.parse().ok()?;

                    obj.texture_coordinates.push((u, v));
                }
                "vn" => {
                    let x = parts.next()?.parse().ok()?;
                    let y = parts.next()?.parse().ok()?;
                    let z = parts.next()?.parse().ok()?;

                    obj.vertex_normals.push(Vector::new(x, y, z));
                }
                "f" => {
                    let a = ObjTriangleCorner::parse(parts.next()?)?;
                    let b = ObjTriangleCorner::parse(parts.next()?)?;
                    let c = ObjTriangleCorner::parse(parts.next()?)?;

                    obj.triangles.push(ObjTriangle(a, b, c))
                }
                _ => return None,
            }
        }

        Some(obj)
    }

    pub fn smooth(self) -> Compound<SmoothTriangle> {
        Compound::new(self.smooth_triangles())
    }

    pub fn flat(self) -> Compound<FlatTriangle> {
        Compound::new(self.flat_triangles())
    }

    fn smooth_triangles(self) -> Vec<SmoothTriangle> {
        unsafe {
            let mut triangles = mem::ManuallyDrop::new(self.triangles());
            Vec::from_raw_parts(
                triangles.as_mut_ptr() as *mut SmoothTriangle,
                triangles.len(),
                triangles.capacity(),
            )
        }
    }

    fn flat_triangles(self) -> Vec<FlatTriangle> {
        unsafe {
            let mut triangles = mem::ManuallyDrop::new(self.triangles());
            Vec::from_raw_parts(
                triangles.as_mut_ptr() as *mut FlatTriangle,
                triangles.len(),
                triangles.capacity(),
            )
        }
    }

    fn triangles(self) -> Vec<Triangle> {
        let mut normals = vec![Vec::new(); self.vertexes.len()];
        self.triangles.iter().for_each(|ObjTriangle(a, b, c)| {
            if let Some(ns) = normals.get_mut(a.vertex_idx) {
                ns.push(self.vertex_normals[a.normal_idx])
            }
            if let Some(ns) = normals.get_mut(b.vertex_idx) {
                ns.push(self.vertex_normals[b.normal_idx])
            }
            if let Some(ns) = normals.get_mut(c.vertex_idx) {
                ns.push(self.vertex_normals[c.normal_idx])
            }
        });
        let normals = normals
            .iter()
            .map(|ns| Unit::new_normalize(ns.iter().sum::<Vector>() / ns.len() as f64))
            .collect();

        let mesh = Arc::new(Mesh {
            vertexes: self.vertexes,
            normals,
        });

        self.triangles
            .iter()
            .map(|ObjTriangle(a, b, c)| {
                let n0 = mesh.normals[a.vertex_idx];
                let n1 = mesh.normals[b.vertex_idx];
                let n2 = mesh.normals[c.vertex_idx];

                let normal = Unit::new_normalize((*n0 + *n1 + *n2) / 3.);

                Triangle {
                    mesh: mesh.clone(),
                    idx0: a.vertex_idx,
                    idx1: b.vertex_idx,
                    idx2: c.vertex_idx,
                    normal,
                }
            })
            .collect()
    }
}

struct ObjTriangle(ObjTriangleCorner, ObjTriangleCorner, ObjTriangleCorner);

struct ObjTriangleCorner {
    vertex_idx: usize,
    texture_idx: usize,
    normal_idx: usize,
}

impl ObjTriangleCorner {
    fn parse(s: &str) -> Option<Self> {
        let mut parts = s.split_terminator('/');

        let vertex_idx = parts.next()?.parse::<usize>().ok()? - 1;
        let texture_idx = parts.next()?.parse::<usize>().ok()? - 1;
        let normal_idx = parts.next()?.parse::<usize>().ok()? - 1;

        Some(Self {
            vertex_idx,
            texture_idx,
            normal_idx,
        })
    }
}

fn read_file(path: &str) -> Option<String> {
    let mut input = String::new();
    let mut file = File::open(path).ok()?;
    let _ = file.read_to_string(&mut input).ok()?;
    Some(input)
}
