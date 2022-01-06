use crate::ray::Ray;
use crate::shapes::shape::Collide;
use crate::shapes::shape::HitRecord;
use crate::utils::{dot, Vec3};

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Collide for Sphere {
    fn get_intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let offset_center = ray.origin - self.center;
        let a = ray.direction.len_squared();
        let half_b = dot(&ray.direction, &offset_center);
        let c = offset_center.len_squared() - self.radius * self.radius;
        let delta = half_b * half_b - a * c;

        // The ray does not intersect our sphere
        if delta < 0.0 {
            return None;
        }

        // Find the smallest root that lies in the [t_min, t_max] range
        let sqrt_delta = delta.sqrt();
        let mut t = (-half_b - sqrt_delta) / a;
        if t < t_min || t > t_max {
            t = (-half_b + sqrt_delta) / a;
            if t < t_min || t > t_max {
                return None;
            }
        }

        // Return a HitRecord if a valid t was found
        let point = ray.at(t);
        let normal = (point - self.center).normalize();
        Some(HitRecord { point, normal, t })
    }
}
