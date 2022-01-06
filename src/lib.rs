pub mod camera;
mod ray;
pub mod scene;
pub mod shapes;
pub mod utils;

pub struct Config {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
    pub anti_aliasing: AntiAliasing,
}

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum AntiAliasing {
    Rays4 = 4,
    Rays9 = 9,
    Rays16 = 16,
    Rays25 = 25,
}
