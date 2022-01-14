use std::sync::Arc;

use crate::bvh::BVH;
use crate::camera::Camera;
use crate::ray::Ray;
use crate::shapes::collide::{Collide, HitRecord};
use crate::utils::Color;
use crate::Config;

type ArcCollide = Arc<dyn Collide + Send + Sync>;

/// Use this struct for building a Scene.
///
/// # Example
/// ```
/// let mut scene_builder = SceneBuilder::new(config);
///
/// scene_builder.add_shape(
///     Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0)
///         .set_material(Diffuse::new(Color::new(150, 150, 150))),
/// );
///
/// let scene = scene_builder.to_scene();
/// ```
pub struct SceneBuilder {
    config: Config,
    shapes: Vec<ArcCollide>,
}

impl SceneBuilder {
    /// Creates a new SceneBuilder
    pub fn new(config: Config) -> Self {
        SceneBuilder {
            config,
            shapes: vec![],
        }
    }

    /// Adds the shape to the SceneBuilder
    pub fn add_shape<T>(&mut self, shape: T)
    where
        T: Collide + Send + Sync + 'static,
    {
        self.shapes.push(Arc::new(shape));
    }

    /// Computes the Bounding Volume Hierarchy (BVH) for the current SceneBuilder and use it to create a Scene that can be rendered
    pub fn to_scene(mut self) -> Scene {
        let n = self.shapes.len();
        let bvh = BVH::new(&mut self.shapes, 0, n);
        Scene {
            config: self.config,
            bvh,
        }
    }
}

/// A scene that can be rendered.
/// Create it using the SceneBuilder struct.
pub struct Scene {
    config: Config,
    bvh: BVH,
}

impl Scene {
    /// Returns the config of the Scene
    pub fn get_config(&self) -> &Config {
        &self.config
    }

    fn get_ray_color(&self, camera: &Camera, ray: &Ray, depth: u32) -> Color {
        if depth == 0 {
            return Color::new(0, 0, 0);
        }

        // Get the intersection that is the closest to the camera
        let mut min_hit_record: Option<HitRecord> = None;
        let max_t = camera.get_far_clip_plane();
        let min_t = if depth == self.config.max_ray_bounce {
            camera.get_near_clip_plane()
        } else {
            0.0001
        };

        if let Some(hit_record) = self.bvh.get_intersection(&ray, min_t, max_t) {
            min_hit_record = Some(hit_record);
        }

        // If we found a shape intersecting with the ray render the shape
        if let Some(hit) = min_hit_record {
            let bouncing_ray = hit.material.scatter(ray, &hit);
            hit.material.get_attenuation() * self.get_ray_color(camera, &bouncing_ray, depth - 1)
        // Else we render the skybox
        } else {
            let t = 0.5 * (ray.direction.y + 1.0);
            Color::new(255, 255, 255) * (1.0 - t) + Color::new(128, 178, 255) * t
        }
    }

    /// Returns the computed color for the pixel at position (x,y) through the Camera.
    pub fn get_pixel_color(&self, camera: &Camera, x: u32, y: u32) -> Color {
        if self.config.anti_aliasing.is_none() {
            let u = x as f32 / (self.config.width as f32 - 1.0);
            let v = (self.config.height as f32 - y as f32) / (self.config.height as f32); // y axis goes up
            return self.get_ray_color(&camera, &camera.get_ray(u, v), self.config.max_ray_bounce);
        }
        let n_samples_root = self.config.anti_aliasing.unwrap();
        let mut color_sum = (0, 0, 0);
        for offset_u in 0..n_samples_root {
            for offset_v in 0..n_samples_root {
                let x = x as f32 + offset_u as f32 / (n_samples_root - 1) as f32;
                let y = y as f32 + offset_v as f32 / (n_samples_root - 1) as f32;
                let u = x as f32 / (self.config.width as f32 - 1.0);
                let v = (self.config.height as f32 - y) / (self.config.height as f32); // y axis goes up
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
}
