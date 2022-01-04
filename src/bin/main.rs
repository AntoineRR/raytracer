use raytracer::scene::Scene;
use raytracer::Config;
use raytracer::utils::Vec3;
use raytracer::camera::Camera;

fn main() {
    let config = Config {
        width: 640,
        height: 400,
        output_path: String::from("images/test.png"),
    };
    let aspect_ratio = config.width as f32 / config.height as f32;

    let camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, -1.0), 1.0, aspect_ratio);

    let scene = Scene::new(config);
    scene.render(camera);
}
