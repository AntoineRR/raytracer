use rand_distr::{Distribution, UnitSphere};

use crate::{
    ray::Ray,
    shapes::shape::HitRecord,
    utils::{Color, Vec3, dot},
};

pub trait Material {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Ray;
    fn get_attenuation(&self) -> Color;
}

pub struct Diffuse {
    color: Color,
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, hit_record: HitRecord) -> Ray {
        let r: [f32; 3] = UnitSphere.sample(&mut rand::thread_rng());
        let target = hit_record.point + hit_record.normal + Vec3::new(r[0], r[1], r[2]);
        Ray::new(hit_record.point, target - hit_record.point)
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Diffuse {
    pub fn new(color: Color) -> Self {
        Diffuse { color }
    }
}

pub struct Metal {
    color: Color,
    fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Ray {
        let mut target = ray.direction - hit_record.normal * 2.0 * dot(&ray.direction, &hit_record.normal);
        let r = UnitSphere.sample(&mut rand::thread_rng());
        target += Vec3::new(r[0], r[1], r[2]) * self.fuzziness;
        Ray::new(hit_record.point, target)
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Metal {
    pub fn new(color: Color, fuzziness: f32) -> Self {
        Metal { color, fuzziness }
    }
}