use crate::Config;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::utils::Color;
use image::ImageBuffer;

pub struct Scene {
    config: Config,
}

impl Scene {
    pub fn new(config: Config) -> Self {
        Scene { config }
    }

    fn get_ray_color(&self, ray: Ray) -> Color {
        let t = 0.5 * (ray.direction.y + 1.0);
        Color::new(128, 178, 255) * (1.0-t) + Color::new(255, 255, 255) * t
    }

    pub fn render(&self, camera: Camera) {
        // temp
        let mut buffer = ImageBuffer::new(self.config.width, self.config.height);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let u = x as f32 / (self.config.width as f32 - 1.0);
            let v = y as f32 / (self.config.height as f32 - 1.0);
            let ray = camera.get_ray(u, v);
            *pixel = self.get_ray_color(ray).convert();
        }
        buffer.save(&self.config.output_path).unwrap();
    }
}
