use crate::ray::Ray;
use crate::utils::Vec3;

pub struct Viewport {
    width: f32,
    height: f32,
}

pub struct Camera {
    position: Vec3,
    direction: Vec3,
    focal_len: f32,
    near_clip_plane: f32,
    far_clip_plane: f32,
    viewport: Viewport,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, focal_len: f32, aspect_ratio: f32) -> Self {
        let viewport = Viewport {
            width: 2.0 * aspect_ratio,
            height: 2.0,
        };
        Camera {
            position,
            direction,
            focal_len,
            near_clip_plane: 0.1 * focal_len,
            far_clip_plane: 5.0 * focal_len,
            viewport,
        }
    }

    pub fn get_near_clip_plane(&self) -> f32 {
        self.near_clip_plane
    }

    pub fn set_near_clip_plane(mut self, near_clip_plane: f32) -> Self {
        if near_clip_plane > self.focal_len {
            panic!("Tried to set the near clip plane to a value greater than the focal length of the Camera");
        }
        self.near_clip_plane = near_clip_plane;
        self
    }

    pub fn get_far_clip_plane(&self) -> f32 {
        self.far_clip_plane
    }

    pub fn set_far_clip_plane(mut self, far_clip_plane: f32) -> Self {
        if far_clip_plane < self.focal_len {
            panic!("Tried to set the far clip plane to a value smaller than the focal length of the Camera");
        }
        self.far_clip_plane = far_clip_plane;
        self
    }

    fn lower_left_corner(&self) -> Vec3 {
        Vec3 {
            x: self.position.x - self.viewport.width / 2.0,
            y: self.position.y - self.viewport.height / 2.0,
            z: self.position.z - self.focal_len,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        // for now, consider that the direction is (0,0,-1)
        let corner = self.lower_left_corner();
        let direction = Vec3::new(
            corner.x + u * self.viewport.width - self.position.x,
            corner.y + v * self.viewport.height - self.position.y,
            corner.z - self.position.z,
        )
        .normalize();
        Ray::new(self.position, direction)
    }
}
