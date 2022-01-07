use raytracer::camera::Camera;
use raytracer::scene::Scene;
use raytracer::shapes::shape::*;
use raytracer::shapes::sphere::Sphere;
use raytracer::utils::Vec3;
use raytracer::Config;

fn main() {
    // Config for the output image
    let config = Config {
        width: 640,
        height: 400,
        output_path: String::from("images/test.png"),
        anti_aliasing: Some(5),
        max_ray_bounce: 20,
        gamma_correction: 1.0,
    };
    let aspect_ratio = config.width as f32 / config.height as f32;

    // Camera that will render the scene
    let camera = Camera::new(
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -1.0),
        1.0,
        aspect_ratio,
    );

    // Configuration of the scene to render
    let mut scene = Scene::new(config);

    scene.add_shape(
        ShapeBuilder::new(Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5))).to_shape(),
    );

    scene.add_shape(
        ShapeBuilder::new(Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0))).to_shape(),
    );

    // Render the scene
    scene.render(camera);
}
