use rand::{thread_rng, Rng};
use rand_distr::{num_traits::Pow, Distribution, UnitSphere};

use crate::{
    ray::Ray,
    shapes::collide::HitRecord,
    utils::{dot, Color, Vec3},
};

pub trait Material {
    /// Returns a ray that was scattered byt the material, based on the incident ray and the informations about the hit with the object.
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray>;

    /// Returns the color emitted by the Material
    fn emit(&self) -> Color {
        Color::new(0, 0, 0)
    }

    /// Returns the attenuation that the scattered ray went through. This is the albedo color of the material.
    fn get_attenuation(&self) -> Color;
}

/// A pure diffuse Material.
pub struct Diffuse {
    color: Color,
}

impl Material for Diffuse {
    fn scatter(&self, _: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let r: [f64; 3] = UnitSphere.sample(&mut rand::thread_rng());
        let target = hit_record.point + hit_record.normal + Vec3::new(r[0], r[1], r[2]);
        Some(Ray::new(hit_record.point, target - hit_record.point))
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
    fuzziness: f64,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let mut target =
            ray.direction - hit_record.normal * 2.0 * dot(&ray.direction, &hit_record.normal);
        let r = UnitSphere.sample(&mut rand::thread_rng());
        target += Vec3::new(r[0], r[1], r[2]) * self.fuzziness;
        Some(Ray::new(hit_record.point, target))
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Metal {
    /// Creates a new metal material.
    pub fn new(color: Color, fuzziness: f64) -> Self {
        Metal { color, fuzziness }
    }
}

/// A mix of the Diffuse and Metal Materials
pub struct DiffuseMetal {
    color: Color,
    fuzziness: f64,
    diffuse_part: f64,
}

impl Material for DiffuseMetal {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
        let choose_reaction: f64 = rand::random();
        if choose_reaction < self.diffuse_part {
            let r: [f64; 3] = UnitSphere.sample(&mut rand::thread_rng());
            let target = hit_record.point + hit_record.normal + Vec3::new(r[0], r[1], r[2]);
            Some(Ray::new(hit_record.point, target - hit_record.point))
        } else {
            let mut target =
                ray.direction - hit_record.normal * 2.0 * dot(&ray.direction, &hit_record.normal);
            let r = UnitSphere.sample(&mut rand::thread_rng());
            target += Vec3::new(r[0], r[1], r[2]) * self.fuzziness;
            Some(Ray::new(hit_record.point, target))
        }
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl DiffuseMetal {
    /// Creates a new diffuse and metal Material.
    pub fn new(color: Color, fuzziness: f64, diffuse_part: f64) -> Self {
        if diffuse_part < 0.0 || diffuse_part > 1.0 {
            panic!("The diffuse_part parameter should be between 0.0 and 1.0 as it represents the part of light that is diffused.")
        }
        DiffuseMetal {
            color,
            fuzziness,
            diffuse_part,
        }
    }
}

/// A pure glass Material.
pub struct Dielectric {
    color: Color,
    refraction: f64,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit_record: &HitRecord) -> Option<Ray> {
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

        Some(Ray::new(hit_record.point, target))
    }

    fn get_attenuation(&self) -> Color {
        self.color
    }
}

impl Dielectric {
    /// Creates a new dielectric material.
    pub fn new(color: Color, refraction: f64) -> Self {
        Dielectric { color, refraction }
    }

    fn reflectance(&self, cos_theta: f64, refraction_ratio: f64) -> f64 {
        let mut r0 = (1.0 - refraction_ratio) / (1.0 + refraction_ratio);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cos_theta).pow(5)
    }
}

pub struct DiffuseLight {
    color: Color,
    intensity: f64,
}

impl Material for DiffuseLight {
    fn scatter(&self, _: &Ray, _: &HitRecord) -> Option<Ray> {
        None
    }

    fn emit(&self) -> Color {
        self.color * self.intensity
    }

    fn get_attenuation(&self) -> Color {
        Color::new(255, 255, 255)
    }
}

impl DiffuseLight {
    pub fn new(color: Color, intensity: f64) -> Self {
        DiffuseLight { color, intensity }
    }
}
