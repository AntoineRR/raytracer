use raytracer::camera::Camera;
use raytracer::material::{Dielectric, Diffuse, Metal};
use raytracer::scene::{Scene, SceneBuilder};
use raytracer::shapes::sphere::Sphere;
use raytracer::utils::{Color, Vec3};
use raytracer::{render, Config};

fn get_random_scene(config: Config) -> Scene {
    let mut scene = SceneBuilder::new(config);

    // Ground
    scene.add_shape(
        Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0)
            .set_material(Diffuse::new(Color::new(150, 150, 150))),
    );

    // Random spheres
    for x in -11..11 {
        for z in -11..11 {
            let center = Vec3::new(
                x as f32 + 0.9 * rand::random::<f32>(),
                0.2,
                z as f32 + 0.9 * rand::random::<f32>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() <= 0.9 {
                continue;
            }

            let choose_material = rand::random::<f32>();

            match choose_material {
                c if c < 0.8 => {
                    scene.add_shape(
                        Sphere::new(center, 0.2).set_material(Diffuse::new(Color::random())),
                    );
                }
                c if c < 0.95 => {
                    scene.add_shape(
                        Sphere::new(center, 0.2)
                            .set_material(Metal::new(Color::random(), rand::random())),
                    );
                }
                _ => {
                    scene.add_shape(
                        Sphere::new(center, 0.2)
                            .set_material(Dielectric::new(Color::new(255, 255, 255), 1.5)),
                    );
                }
            };
        }
    }

    scene.add_shape(
        Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0)
            .set_material(Dielectric::new(Color::new(255, 255, 255), 1.5)),
    );

    scene.add_shape(
        Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0)
            .set_material(Metal::new(Color::new(200, 250, 255), 0.0)),
    );

    scene.add_shape(
        Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0)
            .set_material(Metal::new(Color::new(200, 250, 150), 1.0)),
    );

    scene.to_scene()
}

fn main() {
    // Config for the output image
    let config = Config {
        width: 640,
        height: 400,
        output_path: String::from("images/test.png"),
        anti_aliasing: Some(10),
        max_ray_bounce: 20,
        gamma_correction: 1.0,
    };
    let aspect_ratio = config.width as f32 / config.height as f32;

    // Camera that will render the scene
    let camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(-13.0, -2.0, -3.0),
        Vec3::new(0.0, 1.0, 0.0),
        aspect_ratio,
    )
    .set_vertical_fov(20.0);

    // Configuration of the scene to render
    let scene = get_random_scene(config);

    // Render the scene
    render(scene, camera);
}
