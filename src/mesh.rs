use std::{io::BufReader, sync::Arc};

use nom_stl::parse_stl;

use crate::{shapes::collide::{Collide, HitRecord}, triangle::Triangle, utils::Vec3, bvh::{AABB, BVH}, material::Material};

pub struct STLMesh {
    bvh: BVH,
}

impl Collide for STLMesh {
    fn get_intersection(&self, ray: &crate::ray::Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        self.bvh.get_intersection(ray, t_min, t_max)
    }

    fn get_bounding_box(&self) -> Option<AABB> {
        self.bvh.get_bounding_box()
    }
}

impl STLMesh {
    pub fn new<T>(path: &str, material: T) -> Self
    where
        T: Material + Send  + Sync + 'static,
    {
        let arc_material = Arc::new(material);
        let file = std::fs::File::open(path).expect("Error opening file");
        let mut buffer = BufReader::new(&file);
        let mesh = parse_stl(&mut buffer).expect("Error parsing file as STL");
        let mut triangles: Vec<Arc<dyn Collide + Send + Sync>> = vec![];
        for triangle in mesh.triangles() {
            let to_convert = triangle.vertices();
            let vertices = [
                Vec3::new(to_convert[0][0] as f64, to_convert[0][1] as f64, to_convert[0][2] as f64),
                Vec3::new(to_convert[1][0] as f64, to_convert[1][1] as f64, to_convert[1][2] as f64),
                Vec3::new(to_convert[2][0] as f64, to_convert[2][1] as f64, to_convert[2][2] as f64)
            ];
            let normal = Vec3::new(triangle.normal()[0] as f64, triangle.normal()[1] as f64, triangle.normal()[2] as f64);
            let mut t = Triangle::new(vertices, normal);
            t.set_material(arc_material.clone());
            triangles.push(Arc::new(t));
        }
        let n = triangles.len();
        let bvh = BVH::new(&mut triangles, 0, n);

        STLMesh {
            bvh,
        }
    }
}