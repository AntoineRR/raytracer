pub mod camera;
pub mod material;
mod ray;
pub mod scene;
pub mod shapes;
pub mod utils;

/// Configuration of the output image
///
/// output_path must be a valid path with the name of the file to create. The extension of the file should be either png or jpeg.
/// anti_aliasing holds the value of the square root of the number of rays thrown per pixel.
/// For example, if anti_aliasing = Some(5), 25 rays per pixel will be thrown. Increasing this value will result in less performance.
/// max_ray_bounce set the maximum time a ray should bounce on objects before considering it was fully absorbed.
/// gamma_correction is the gamma correction that should be applied to the image.
///
/// # Example
/// ```
/// let config = Config {
///     width: 640,
///     height: 400,
///     output_path: String::from("images/test.png"),
///     anti_aliasing: Some(10),
///     max_ray_bounce: 20,
///     gamma_correction: 1.0,
/// };
/// ```
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub output_path: String,
    pub anti_aliasing: Option<u32>,
    pub max_ray_bounce: u32,
    pub gamma_correction: f32,
}
