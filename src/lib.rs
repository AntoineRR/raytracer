pub mod camera;
pub mod material;
mod ray;
pub mod scene;
pub mod shapes;
pub mod utils;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
    pub anti_aliasing: Option<u32>,
    pub max_ray_bounce: u32,
    pub gamma_correction: f32,
}
