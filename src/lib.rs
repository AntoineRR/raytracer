pub mod scene;
pub mod camera;
pub mod utils;
pub mod shapes;
mod ray;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
}
