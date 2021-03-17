use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::aabb::AABB;
use crate::shape::{Hit, Shape};
use crate::K_EPSILON;
use nalgebra::{Point3, Unit, Vector3};
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Read;
use std::sync::Arc;

#[derive(Default)]
pub struct Mesh {
    vertexes: Vec<Point3<f64>>,
    normals: Vec<Unit<Vector3<f64>>>,
}

pub struct SmoothTriangle {
    inner: Triangle,
}

impl Shape for SmoothTriangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
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
            }
        })
    }

    fn count_intersection_tests(&self, _ray: &Ray) -> usize {
        1
    }
}

pub struct FlatTriangle {
    inner: Triangle,
}

impl Shape for FlatTriangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        self.inner.intersect(ray).map(|hit| Hit {
            t: hit.t,
            normal: *self.inner.normal,
            local_hit_point: hit.local_hit_point,
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
    normal: Unit<Vector3<f64>>,
}

impl Triangle {
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
        return Some(TriangleHit {
            t,
            local_hit_point,
            beta,
            gamma,
        });
    }

    fn n0(&self) -> Unit<Vector3<f64>> {
        self.mesh.normals[self.idx0]
    }

    fn n1(&self) -> Unit<Vector3<f64>> {
        self.mesh.normals[self.idx1]
    }

    fn n2(&self) -> Unit<Vector3<f64>> {
        self.mesh.normals[self.idx2]
    }
}

struct TriangleHit {
    t: f64,
    local_hit_point: Point3<f64>,
    beta: f64,
    gamma: f64,
}

pub struct TriangleMesh {
    triangles: Vec<SmoothTriangle>,
    aabb: AABB,
}

impl TriangleMesh {
    pub fn new(obj: Obj) -> Self {
        let min_x = obj
            .vertexes
            .iter()
            .min_by(|v1, v2| v1.x.partial_cmp(&v2.x).unwrap())
            .unwrap()
            .x;
        let min_y = obj
            .vertexes
            .iter()
            .min_by(|v1, v2| v1.y.partial_cmp(&v2.y).unwrap())
            .unwrap()
            .y;
        let min_z = obj
            .vertexes
            .iter()
            .min_by(|v1, v2| v1.z.partial_cmp(&v2.z).unwrap())
            .unwrap()
            .z;

        let max_x = obj
            .vertexes
            .iter()
            .max_by(|v1, v2| v1.x.partial_cmp(&v2.x).unwrap())
            .unwrap()
            .x;
        let max_y = obj
            .vertexes
            .iter()
            .max_by(|v1, v2| v1.y.partial_cmp(&v2.y).unwrap())
            .unwrap()
            .y;
        let max_z = obj
            .vertexes
            .iter()
            .max_by(|v1, v2| v1.z.partial_cmp(&v2.z).unwrap())
            .unwrap()
            .z;

        let aabb = AABB::new(
            Point3::new(min_x, min_y, min_z),
            Point3::new(max_x, max_y, max_z),
        );

        let mut normals = vec![Vec::new(); obj.vertexes.len()];
        obj.triangles.iter().for_each(|ObjTriangle(a, b, c)| {
            normals
                .get_mut(a.vertex_idx)
                .map(|ns| ns.push(obj.vertex_normals[a.normal_idx]));
            normals
                .get_mut(b.vertex_idx)
                .map(|ns| ns.push(obj.vertex_normals[b.normal_idx]));
            normals
                .get_mut(c.vertex_idx)
                .map(|ns| ns.push(obj.vertex_normals[c.normal_idx]));
        });
        let normals = normals
            .iter()
            .map(|ns| Unit::new_normalize(ns.iter().sum::<Vector3<f64>>() / ns.len() as f64))
            .collect();

        let mesh = Arc::new(Mesh {
            vertexes: obj.vertexes,
            normals,
        });

        let triangles = obj
            .triangles
            .iter()
            .map(|ObjTriangle(a, b, c)| {
                let n0 = mesh.normals[a.vertex_idx];
                let n1 = mesh.normals[b.vertex_idx];
                let n2 = mesh.normals[c.vertex_idx];

                let normal = Unit::new_normalize((*n0 + *n1 + *n2) / 3.);

                SmoothTriangle {
                    inner: Triangle {
                        mesh: mesh.clone(),
                        idx0: a.vertex_idx,
                        idx1: b.vertex_idx,
                        idx2: c.vertex_idx,
                        normal,
                    },
                }
            })
            .collect();

        Self { triangles, aabb }
    }
}

impl Shape for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        if !self.aabb.hit(&ray) {
            return None;
        }

        self.triangles
            .iter()
            .filter_map(|triangle| triangle.intersect(&ray))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }

    fn count_intersection_tests(&self, ray: &Ray) -> usize {
        if self.aabb.hit(&ray) {
            1 + self.triangles.len()
        } else {
            1
        }
    }

    fn hit(&self, ray: &Ray) -> bool {
        self.aabb.hit(&ray)
            && self
                .triangles
                .iter()
                .any(|triangle| triangle.intersect(&ray).is_some())
    }
}

pub struct Obj {
    vertexes: Vec<Point3<f64>>,
    texture_coordinates: Vec<(f64, f64)>,
    vertex_normals: Vec<Vector3<f64>>,
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

                    obj.vertexes.push(Point3::new(x, y, z));
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

                    obj.vertex_normals.push(Vector3::new(x, y, z));
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
