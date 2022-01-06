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

    fn get_ray_color(&self, camera: &Camera, ray: &Ray) -> Color {
        // Get the intersection that is the closest to the camera
        let mut min_hit_record: Option<HitRecord> = None;
        let mut max_t = camera.get_far_clip_plane();
        for shape in &self.shapes {
            if let Some(hit_record) =
                shape
                    .object
                    .get_intersection(&ray, camera.get_near_clip_plane(), max_t)
            {
                max_t = hit_record.t;
                min_hit_record = Some(hit_record);
            }
        }
        // If we found a shape intersecting with the ray render the shape
        if let Some(hit) = min_hit_record {
            let normal = (hit.normal + Vec3::new(1.0, 1.0, 1.0)) * 0.5;
            Color::new(
                (normal.x * 255.0) as u8,
                (normal.y * 255.0) as u8,
                (normal.z * 255.0) as u8,
            )
        // Else we render the skybox
        } else {
            let t = 0.5 * (ray.direction.y + 1.0);
            Color::new(255, 255, 255) * (1.0 - t) + Color::new(128, 178, 255) * t
        }
    }

    pub fn render(&self, camera: Camera) {
        // temp
        let mut buffer = ImageBuffer::new(self.config.width, self.config.height);
        let aliasing = ((self.config.anti_aliasing as u8) as f32).sqrt() as u32;
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let mut color_sum = (0, 0, 0);
            for offset_u in 0..aliasing {
                for offset_v in 0..aliasing {
                    let x = x as f32 + offset_u as f32 / (aliasing - 1) as f32;
                    let y = y as f32 + offset_v as f32 / (aliasing - 1) as f32;
                    let u = x as f32 / (self.config.width as f32 - 1.0);
                    let v =
                        (self.config.height as f32 - y) as f32 / (self.config.height as f32 - 1.0); // y axis goes up
                    let ray = camera.get_ray(u, v);
                    let color = self.get_ray_color(&camera, &ray);
                    color_sum.0 += color.r as u32;
                    color_sum.1 += color.g as u32;
                    color_sum.2 += color.b as u32;
                }
            }
            let color = Color::new(
                (color_sum.0 / (aliasing * aliasing)) as u8,
                (color_sum.1 / (aliasing * aliasing)) as u8,
                (color_sum.2 / (aliasing * aliasing)) as u8,
            );
            *pixel = color.convert();
        }
        buffer.save(&self.config.output_path).unwrap();
    }
}
