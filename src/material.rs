use rand::{thread_rng, Rng};
use rand_distr::{num_traits::Pow, Distribution, UnitSphere};

use crate::{
    ray::Ray,
    shapes::shape::HitRecord,
    utils::{dot, Color, Vec3},
};

pub trait Material {
    /// Returns a ray that was scattered byt the material, based on the incident ray and the informations about the hit with the object.
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Ray;

    /// Returns the attenuation that the scattered ray went through. This is the albedo color of the material.
    fn get_attenuation(&self) -> Color;
}

/// A pure diffuse Material.
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
    /// Creates a new diffuse material.
    pub fn new(color: Color) -> Self {
        Diffuse { color }
    }
}

/// A pure reflective Material.
pub struct Metal {
    color: Color,
    fuzziness: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Ray {
        let mut target =
            ray.direction - hit_record.normal * 2.0 * dot(&ray.direction, &hit_record.normal);
        let r = UnitSphere.sample(&mut rand::thread_rng());
        target += Vec3::new(r[0], r[1], r[2]) * self.fuzziness;
        Ray::new(hit_record.point, target)
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Metal {
    /// Creates a new metal material.
    pub fn new(color: Color, fuzziness: f32) -> Self {
        Metal { color, fuzziness }
    }
}

/// A pure glass Material.
pub struct Dielectric {
    color: Color,
    refraction: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: HitRecord) -> Ray {
        let refraction_ratio = if hit_record.front_face {
            1.0 / self.refraction
        } else {
            self.refraction
        };
        let normalized_dir = ray.direction.normalize();
        let cos_theta = dot(&-normalized_dir, &hit_record.normal);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let target = if refraction_ratio * sin_theta > 1.0
            || self.reflectance(cos_theta, refraction_ratio) > thread_rng().gen()
        {
            ray.direction - hit_record.normal * 2.0 * dot(&ray.direction, &hit_record.normal)
        } else {
            let r_perp = (normalized_dir + hit_record.normal * cos_theta) * refraction_ratio;
            let r_par = -hit_record.normal * (1.0 - r_perp.len_squared()).abs().sqrt();
            r_par + r_perp
        };

        Ray::new(hit_record.point, target)
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Dielectric {
    /// Creates a new dielectric material.
    pub fn new(color: Color, refraction: f32) -> Self {
        Dielectric { color, refraction }
    }

    fn reflectance(&self, cos_theta: f32, refraction_ratio: f32) -> f32 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).pow(5)
    }
}
