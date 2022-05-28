use crate::core::{Aabb, Bounded, Normal3, Ray, Shape, SurfaceInteraction};
use crate::util::math;
use crate::Float;
use nalgebra as na;
use nalgebra::{point, vector, Point3, Vector3};
use nalgebra_glm as glm;
use num_traits::float::FloatConst;

pub struct Sphere {
    radius: Float,
    z_min: Float,
    z_max: Float,
    theta_min: Float,
    theta_max: Float,
    phi_max: Float,
}

impl Bounded for Sphere {
    fn bbox(&self) -> Aabb {
        Aabb::new(
            &point![-self.radius, -self.radius, self.z_min],
            &point![self.radius, self.radius, self.z_max],
        )
    }
}

impl Shape for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        let a = glm::magnitude2(&ray.d);
        let b = glm::dot(&ray.o.coords, &ray.d) * 2.;
        let c = glm::magnitude2(&ray.o.coords) - self.radius * self.radius;

        let [t0, t1] = math::quadratic(a, b, c)?;
        if t0 > ray.t_max || t1 <= 0. {
            return None;
        }

        let (p_hit, phi, t_hit) = if t0 > 0. {
            self.p_hit_and_phi(ray, t0)
                .map(|(p_hit, phi)| (p_hit, phi, t0))
                .or_else(|| {
                    self.p_hit_and_phi(ray, t1)
                        .map(|(p_hit, phi)| (p_hit, phi, t1))
                })?
        } else {
            self.p_hit_and_phi(ray, t1)
                .map(|(p_hit, phi)| (p_hit, phi, t1))?
        };

        let u = phi / self.phi_max;
        let theta = (p_hit.z / self.radius).clamp(-1., 1.).acos();
        let v = (theta - self.theta_min) / (self.theta_max - self.theta_min);
        let [dpdu, dpdv] = self.dpdu_dpdv(&p_hit, theta);
        let [dndu, dndv] = self.dndu_dndv();

        let isect = SurfaceInteraction {
            t_hit,
            p: p_hit,
            wo: -ray.d,
            n: Normal3(Default::default()),
            uv: point![u, v],
            dpdu,
            dpdv,
            dndu,
            dndv,
            shape: self,
        };

        Some(isect)
    }

    fn intersects(&self, ray: &Ray) -> bool {
        let a = glm::magnitude2(&ray.d);
        let b = glm::dot(&ray.o.coords, &ray.d) * 2.;
        let c = glm::magnitude2(&ray.o.coords) - self.radius * self.radius;

        let [t0, t1] = if let Some(roots) = math::quadratic(a, b, c) {
            roots
        } else {
            return false;
        };

        if t0 > ray.t_max || t1 <= 0. {
            return false;
        }

        if t0 > 0. {
            self.p_hit_and_phi(ray, t0)
                .map(|(p_hit, phi)| (p_hit, phi, t0))
                .or_else(|| {
                    self.p_hit_and_phi(ray, t1)
                        .map(|(p_hit, phi)| (p_hit, phi, t1))
                })
                .is_some()
        } else {
            self.p_hit_and_phi(ray, t1)
                .map(|(p_hit, phi)| (p_hit, phi, t1))
                .is_some()
        }
    }

    fn area(&self) -> Float {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }
}

impl Sphere {
    /// phi_max: angel in degree
    pub fn new(radius: Float, z_min: Float, z_max: Float, phi_max: Float) -> Self {
        Self {
            radius,
            z_min: z_min.min(z_max).clamp(-radius, radius),
            z_max: z_min.max(z_max).clamp(-radius, radius),
            theta_min: (z_min / radius).clamp(-1., 1.).acos(),
            theta_max: (z_max / radius).clamp(-1., 1.).acos(),
            phi_max: phi_max.clamp(0., 360.).to_radians(),
        }
    }

    fn dndu_dndv(&self) -> [Normal3<Float>; 2] {
        // TODO
        [Normal3(Default::default()), Normal3(Default::default())]
    }

    fn dpdu_dpdv(&self, p_hit: &Point3<Float>, theta: Float) -> [Vector3<Float>; 2] {
        let z_radius = (p_hit.x * p_hit.x + p_hit.y * p_hit.y).sqrt();
        let inv_z_radius = 1. / z_radius;
        let cos_phi = p_hit.x * inv_z_radius;
        let sin_phi = p_hit.y * inv_z_radius;
        let dpdu = vector![-self.phi_max * p_hit.y, self.phi_max * p_hit.x, 0.];
        let dpdv = (self.theta_max - self.theta_min)
            * vector![
                p_hit.z * cos_phi,
                p_hit.z * sin_phi,
                -self.radius * theta.sin()
            ];

        [dpdu, dpdv]
    }

    fn p_hit_and_phi(&self, ray: &Ray, t: Float) -> Option<(Point3<Float>, Float)> {
        let mut p_hit = ray.o + ray.d * t;
        // refine sphere intersection point
        p_hit *= self.radius / na::distance(&p_hit, &Point3::origin());

        let mut phi = p_hit.y.atan2(p_hit.x);
        if phi < 0. {
            phi += 2. * Float::PI();
        }

        if (self.z_min > -self.radius && p_hit.z < self.z_min)
            || (self.z_max < self.radius && p_hit.z > self.z_max)
            || phi > self.phi_max
        {
            None
        } else {
            Some((p_hit, phi))
        }
    }
}
