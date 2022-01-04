use crate::utils::Vec3;
use crate::ray::Ray;

pub struct Viewport {
    width: f32,
    height: f32,
}

pub struct Camera {
    origin: Vec3,
    direction: Vec3,
    focal_len: f32,
    viewport: Viewport,
}

impl Camera {
    pub fn new(origin: Vec3, direction: Vec3, focal_len: f32, aspect_ratio: f32) -> Self {
        let viewport = Viewport {
            width: 2.0 * aspect_ratio,
            height: 2.0,
        };
        Camera {
            origin,
            direction,
            focal_len,
            viewport,
        }
    }

    fn lower_left_corner(&self) -> Vec3 {
        Vec3 {
            x: self.origin.x - self.viewport.width/2.0,
            y: self.origin.y - self.viewport.height/2.0,
            z: self.origin.z - self.focal_len,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // for now, consider that the direction is (0,0,-1)
        let corner = self.lower_left_corner();
        let direction = Vec3::new(
            corner.x + u*self.viewport.width - self.origin.x,
            corner.y + v*self.viewport.height - self.origin.y,
            corner.z - self.origin.z,
        ).normalize(); 
        Ray::new(self.origin, direction)
    }
}