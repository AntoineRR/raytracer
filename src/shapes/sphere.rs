use crate::utils::{ Vec3, dot };
use crate::shapes::shape::Shape;
use crate::ray::Ray;

pub struct Sphere {
    center: Vec3,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Sphere { center, radius }
    }
}

impl Shape for Sphere {
    fn get_intersection(&self, ray: &Ray) -> Option<f32> {
        let offset_center = ray.origin - self.center;
        let a = dot(&ray.direction, &ray.direction);
        let b = 2.0 * dot(&ray.direction, &offset_center);
        let c = dot(&offset_center, &offset_center) - self.radius*self.radius;
        let delta = b*b - 4.0*a*c;
        
        if delta < 0.0 {
            None
        }
        else {
            let smallest_root = (-b - delta.sqrt())/(2.0*a);
            Some(smallest_root)
        }
    }

    fn get_normal_at(&self, p: &Vec3) -> Vec3 {
        (*p - self.center).normalize()
    }
}