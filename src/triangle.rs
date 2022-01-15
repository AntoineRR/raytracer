use std::sync::Arc;

use crate::{utils::{Vec3, Color, dot, cross}, material::{Material, Diffuse}, shapes::collide::{Collide, HitRecord}, bvh::AABB, ray::Ray};

type ArcMaterial = Arc<dyn Material + Send + Sync>;

pub struct Triangle {
    vertices: [Vec3; 3],
    normal: Vec3,
    material: ArcMaterial,
}

impl Collide for Triangle {
    fn get_intersection(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        // Möller–Trumbore algorithm
        let v0v1 = self.vertices[1] - self.vertices[0];
        let v0v2 = self.vertices[2] - self.vertices[0];
        let p_vec = cross(&ray.direction, &v0v2);
        let det = dot(&v0v1, &p_vec);

        if det < 0.0000001 {
            return None;
        }

        let inv_det = 1.0 / det;
        let t_vec = ray.origin - self.vertices[0];
        let u = dot(&t_vec, &p_vec) * inv_det;
        if u < 0.0 || u > 1.0 { return None; }

        let q_vec = cross(&t_vec, &v0v1);
        let v = dot(&ray.direction, &q_vec) * inv_det;
        if v < 0.0 || u + v > 1.0 { return None; }

        let t = dot(&v0v2, &q_vec) * inv_det;
        if t < t_min || t > t_max { return None; }
        let p = ray.origin + ray.direction * t;

        Some(HitRecord::new(p, self.normal, t, true, self.material.clone()))
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        let min = Vec3::new(
            self.vertices[0].x.min(self.vertices[1].x.min(self.vertices[2].x)),
            self.vertices[0].y.min(self.vertices[1].y.min(self.vertices[2].y)),
            self.vertices[0].z.min(self.vertices[1].z.min(self.vertices[2].z)),
        );
        let max = Vec3::new(
            self.vertices[0].x.max(self.vertices[1].x.max(self.vertices[2].x)),
            self.vertices[0].y.max(self.vertices[1].y.max(self.vertices[2].y)),
            self.vertices[0].z.max(self.vertices[1].z.max(self.vertices[2].z)),
        );
        Some(AABB::new(min, max))
    }
}

impl Triangle {
    pub fn new(vertices: [Vec3; 3], normal: Vec3) -> Self {
        Triangle {
            vertices,
            normal,
            material: Arc::new(Diffuse::new(Color::random())),
        }
    }

    pub fn set_material(&mut self, material: Arc<dyn Material + Send + Sync + 'static>) {
        self.material = material;
    }
}