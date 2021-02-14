use crate::film::RGB;
use crate::material::Material;
use crate::math::{Ray, Transformation};
use crate::shape::{Hit, Shape};
use nalgebra::{Point3, Vector3};
use std::fs::File;
use std::io::Read;
use crate::K_EPSILON;

struct Triangle {
    v0: Point3<f64>,
    v1: Point3<f64>,
    v2: Point3<f64>,

    n0: Vector3<f64>,
    n1: Vector3<f64>,
    n2: Vector3<f64>,
}

impl Triangle {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let a = self.v0.x - self.v1.x;
        let b = self.v0.x - self.v2.x;
        let c = ray.direction().x;
        let d = self.v0.x - ray.origin().x;

        let e = self.v0.y - self.v1.y;
        let f = self.v0.y - self.v2.y;
        let g = ray.direction().y;
        let h = self.v0.y - ray.origin().y;

        let i = self.v0.z - self.v1.z;
        let j = self.v0.z - self.v2.z;
        let k = ray.direction().z;
        let l = self.v0.z - ray.origin().z;

        let m = f * k - g * j;
        let n = h * k - g * l;
        let p = f * l - h * j;
        let q = g * i - e * k;
        let s = e * j - f * i;

        let inv_denom = 1.0 / (a * m + b * q + c * s);

        let e1 = d * m - b * n - c * p;
        let beta = e1 * inv_denom;

        if beta < 0.0 {
            return None;
        }

        let r = e * l - h * i;
        let e2 = a * n + d * q + c * r;
        let gamma = e2 * inv_denom;

        if gamma < 0.0 || beta + gamma > 1.0 {
            return None;
        }

        let e3 = a * p - b * r + d * s;
        let t = e3 * inv_denom;

        if t < K_EPSILON {
            return None;
        }


        let shading_normal = beta * &self.n1 + gamma * &self.n2 + (1. - beta - gamma) * &self.n0;
        let local_hit_point = ray.origin() + t * ray.direction();

        Some(Hit {
            t,
            normal: shading_normal,
            local_hit_point,
        })
    }
}

pub struct TriangleMesh {
    triangles: Vec<Triangle>,
    transformation: Transformation,
    material: Material,
}

impl TriangleMesh {
    pub fn new(obj: Obj, material: Material, transformation: Transformation) -> Self {
        let triangles = obj
            .triangles
            .iter()
            .map(|ObjTriangle(a, b, c)| {
                let v0 = obj.vertexes[a.vertex_idx - 1].clone();
                let v1 = obj.vertexes[b.vertex_idx - 1].clone();
                let v2 = obj.vertexes[c.vertex_idx - 1].clone();

                let n0 = obj.vertex_normals[a.normal_idx - 1].clone();
                let n1 = obj.vertex_normals[b.normal_idx - 1].clone();
                let n2 = obj.vertex_normals[c.normal_idx - 1].clone();

                Triangle { v0, v1, v2, n0, n1, n2 }
            })
            .collect();

        Self {
            triangles,
            transformation,
            material,
        }
    }
}

impl Shape for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let inv_ray = self.transformation.apply_inverse(ray);

        self.triangles
            .iter()
            .filter_map(|triangle| triangle.intersect(&inv_ray))
            .min_by(|x, y| x.t.partial_cmp(&y.t).unwrap())
    }

    fn material(&self) -> Material {
        self.material.clone()
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

        let vertex_idx = parts.next()?.parse().ok()?;
        let texture_idx = parts.next()?.parse().ok()?;
        let normal_idx = parts.next()?.parse().ok()?;

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
