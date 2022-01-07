use rand_distr::{Distribution, UnitSphere};

use crate::{
    ray::Ray,
    shapes::shape::HitRecord,
    utils::{Color, Vec3},
};

pub trait Material {
    fn scatter(&self, hit_record: HitRecord) -> Ray;
}

pub struct Diffuse {
    color: Color,
}

impl Material for Diffuse {
    fn scatter(&self, hit_record: HitRecord) -> Ray {
        let r: [f32; 3] = UnitSphere.sample(&mut rand::thread_rng());
        let target = hit_record.point + hit_record.normal + Vec3::new(r[0], r[1], r[2]);
        Ray::new(hit_record.point, target - hit_record.point)
    }
}

impl Diffuse {
    pub fn new(color: Color) -> Self {
        Diffuse { color }
    }
}
