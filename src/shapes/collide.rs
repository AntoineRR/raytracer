use std::sync::Arc;

use crate::bvh::AABB;
use crate::material::Material;
use crate::ray::Ray;
use crate::utils::Vec3;

/// Implement this trait for all objects that can be rendered in the Scene
pub trait Collide {
    fn get_intersection(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
    fn get_bounding_box(&self) -> Option<AABB>;
}

/// Informations about the hit of an object
pub struct HitRecord {
    pub point: Vec3,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub material: Arc<dyn Material>,
}

impl HitRecord {
    pub fn new(
        point: Vec3,
        outward_normal: Vec3,
        t: f32,
        front_face: bool,
        material: Arc<dyn Material>,
    ) -> Self {
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };
        HitRecord {
            point,
            normal,
            t,
            front_face,
            material,
        }
    }
}
