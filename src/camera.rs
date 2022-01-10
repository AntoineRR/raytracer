use crate::ray::Ray;
use crate::utils::{cross, Base, Vec3};

pub struct Viewport {
    width: f32,
    height: f32,
    aspect_ratio: f32,
    lower_left_corner: Vec3,
}

pub struct Camera {
    position: Vec3,
    base: Base,
    focal_len: f32,
    near_clip_plane: f32,
    far_clip_plane: f32,
    viewport: Viewport,
}

impl Camera {
    pub fn new(position: Vec3, direction: Vec3, view_up: Vec3, aspect_ratio: f32) -> Self {
        let w = -direction.normalize();
        let u = cross(&view_up, &w).normalize();
        let v = cross(&w, &u);

        let width = 2.0 * aspect_ratio;
        let height = 2.0;
        let lower_left_corner = position - (u * width) / 2.0 - (v * height) / 2.0 - w;

        let viewport = Viewport {
            width,
            height,
            aspect_ratio,
            lower_left_corner,
        };

        Camera {
            position,
            base: Base::new(u, v),
            focal_len: 1.0,
            near_clip_plane: 0.1,
            far_clip_plane: 5.0,
            viewport,
        }
    }

    pub fn get_focal_len(&self) -> f32 {
        self.focal_len
    }

    pub fn set_focal_len(mut self, focal_len: f32) -> Self {
        if self.focal_len < self.near_clip_plane || self.focal_len > self.far_clip_plane {
            panic!("Tried to set a focal len that is outside of the camera frustum");
        }
        self.focal_len = focal_len;
        self
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

    pub fn set_vertical_fov(mut self, v_fov: f32) -> Self {
        let h = (v_fov.to_radians() / 2.0).tan();
        self.viewport.height = 2.0 * h;
        self.viewport.width = self.viewport.aspect_ratio * self.viewport.height;
        self.viewport.lower_left_corner = self.position
            - (self.base.u() * self.viewport.width) / 2.0
            - (self.base.v() * self.viewport.height) / 2.0
            - self.base.w();
        self
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        let direction = (self.viewport.lower_left_corner
            + self.base.u() * self.viewport.width * u
            + self.base.v() * self.viewport.height * v
            - self.position)
            .normalize();
        Ray::new(self.position, direction)
    }
}
