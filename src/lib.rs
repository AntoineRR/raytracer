use std::sync::{Arc, Mutex};

use camera::Camera;
use image::ImageBuffer;
use indicatif::{HumanDuration, ProgressBar};
use scene::Scene;

mod bvh;
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

/// Renders the Scene scene from the Camera camera.
/// Will display a progress bar to keep track of the rendering process
/// The resulting render will be saved in an image whose path is defined in the config object of the Scene.
pub fn render(scene: Scene, camera: Camera) {
    println!("Rendering scene...");
    let bar = ProgressBar::new(scene.get_config().width as u64 * scene.get_config().height as u64);
    let bar = Arc::new(bar);
    bar.set_draw_rate(10);

    let buffer: ImageBuffer<image::Rgb<u8>, _> =
        ImageBuffer::new(scene.get_config().width, scene.get_config().height);
    let buffer = Arc::new(Mutex::new(buffer));
    let thread_pool = threadpool::ThreadPool::new(100);

    let gamma_correction = scene.get_config().gamma_correction;
    let camera = Arc::new(camera);
    let scene = Arc::new(scene);

    let width = buffer.lock().unwrap().width();
    let height = buffer.lock().unwrap().height();

    for x in 0..width {
        for y in 0..height {
            let camera_clone = camera.clone();
            let buffer_clone = buffer.clone();
            let scene_clone = scene.clone();
            let bar_clone = bar.clone();

            thread_pool.execute(move || {
                let color = scene_clone
                    .get_pixel_color(&camera_clone, x as u32, y as u32)
                    .convert(gamma_correction);
                *buffer_clone.lock().unwrap().get_pixel_mut(x, y) = color;
                bar_clone.inc(1);
            });
        }
    }
    thread_pool.join();

    bar.finish();
    println!("Took: {}", HumanDuration(bar.elapsed()).to_string());

    println!("Saving image...");
    buffer
        .lock()
        .unwrap()
        .save(&scene.get_config().output_path)
        .unwrap();
    println!("Done");
}
