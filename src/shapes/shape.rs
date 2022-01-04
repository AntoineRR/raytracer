use crate::ray::Ray;
use crate::utils::Vec3;

pub trait Shape {
    fn get_intersection(&self, ray: &Ray) -> Option<f32>;
    fn get_normal_at(&self, p: &Vec3) -> Vec3;
}