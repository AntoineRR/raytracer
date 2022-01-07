use crate::camera::Camera;
use crate::ray::Ray;
use crate::shapes::shape::HitRecord;
use crate::shapes::shape::Shape;
use crate::utils::Color;
use crate::utils::Vec3;
use crate::Config;
use image::ImageBuffer;

pub struct Scene {
    config: Config,
    shapes: Vec<Shape>,
}

impl Scene {
    pub fn new(config: Config) -> Self {
        Scene {
            config,
            shapes: vec![],
        }
    }

    pub fn add_shape(&mut self, shape: Shape) {
        self.shapes.push(shape);
    }

    fn get_ray_color(&self, camera: &Camera, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0, 0, 0);
        }

        // Get the intersection that is the closest to the camera
        let mut min_hit_record: Option<HitRecord> = None;
        let mut hit_shape: Option<&Shape> = None;
        let mut max_t = camera.get_far_clip_plane();
        let min_t = if depth == self.config.max_ray_bounce {
            camera.get_near_clip_plane()
        } else {
            0.0001
        };
        for shape in &self.shapes {
            if let Some(hit_record) = shape.object.get_intersection(&ray, min_t, max_t) {
                max_t = hit_record.t;
                min_hit_record = Some(hit_record);
                hit_shape = Some(shape);
            }
        }
        // If we found a shape intersecting with the ray render the shape
        if let (Some(hit), Some(s)) = (min_hit_record, hit_shape) {
            let bouncing_ray = s.material.scatter(hit);
            self.get_ray_color(camera, &bouncing_ray, depth - 1) * 0.5
        // Else we render the skybox
        } else {
            let t = 0.5 * (ray.direction.y + 1.0);
            Color::new(255, 255, 255) * (1.0 - t) + Color::new(128, 178, 255) * t
        }
    }

    fn get_pixel_color(&self, camera: &Camera, x: u32, y: u32) -> Color {
        if self.config.anti_aliasing.is_none() {
            let u = x as f32 / (self.config.width as f32 - 1.0);
            let v = (self.config.height as f32 - y as f32) / (self.config.height as f32 - 1.0); // y axis goes up
            return self.get_ray_color(&camera, &camera.get_ray(u, v), self.config.max_ray_bounce);
        }
        let n_samples_root = self.config.anti_aliasing.unwrap();
        let mut color_sum = (0, 0, 0);
        for offset_u in 0..n_samples_root {
            for offset_v in 0..n_samples_root {
                let x = x as f32 + offset_u as f32 / (n_samples_root - 1) as f32;
                let y = y as f32 + offset_v as f32 / (n_samples_root - 1) as f32;
                let u = x as f32 / (self.config.width as f32 - 1.0);
                let v = (self.config.height as f32 - y) / (self.config.height as f32 - 1.0); // y axis goes up
                let ray = camera.get_ray(u, v);
                let color = self.get_ray_color(&camera, &ray, self.config.max_ray_bounce);
                color_sum.0 += color.r as u32;
                color_sum.1 += color.g as u32;
                color_sum.2 += color.b as u32;
            }
        }
        Color::new(
            (color_sum.0 / (n_samples_root * n_samples_root)) as u8,
            (color_sum.1 / (n_samples_root * n_samples_root)) as u8,
            (color_sum.2 / (n_samples_root * n_samples_root)) as u8,
        )
    }

    pub fn render(&self, camera: Camera) {
        // temp
        let mut buffer = ImageBuffer::new(self.config.width, self.config.height);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            *pixel = self.get_pixel_color(&camera, x, y).convert(self.config.gamma_correction);
        }
        buffer.save(&self.config.output_path).unwrap();
    }
}
