use raytracer::camera::Camera;
use raytracer::material::{Dielectric, Diffuse, DiffuseLight, DiffuseMetal, Metal};
use raytracer::scene::{Scene, SceneBuilder};
use raytracer::shapes::mesh::STLMesh;
use raytracer::shapes::sphere::Sphere;
use raytracer::utils::{Color, Vec3};
use raytracer::{render, Config};

#[allow(dead_code)]
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
                x as f64 + 0.9 * rand::random::<f64>(),
                0.2,
                z as f64 + 0.9 * rand::random::<f64>(),
            );

            if (center - Vec3::new(4.0, 0.2, 0.0)).len() <= 0.9 {
                continue;
            }

            let choose_material = rand::random::<f64>();

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

#[allow(dead_code)]
fn get_light_scene(config: Config) -> Scene {
    let mut scene = SceneBuilder::new(config).set_skybox_color(Color::new(20, 20, 20));

    // Ground
    scene.add_shape(
        Sphere::new(Vec3::new(0.0, -1000.0, -1.0), 1000.0)
            .set_material(Diffuse::new(Color::new(150, 150, 150))),
    );

    // A metallic sphere on the ground
    scene.add_shape(
        Sphere::new(Vec3::new(0.0, 1.2, 0.0), 1.2)
            .set_material(Metal::new(Color::new(255, 220, 130), 0.0)),
    );

    // A diffuse sphere on the ground
    scene.add_shape(
        Sphere::new(Vec3::new(2.0, 0.6, -1.5), 0.6)
            .set_material(Diffuse::new(Color::new(150, 230, 130))),
    );

    // A white light
    scene.add_shape(
        Sphere::new(Vec3::new(1.0, 0.5, 2.0), 0.5)
            .set_material(DiffuseLight::new(Color::new(255, 255, 255), 10.0)),
    );

    // An intense red light floating
    scene.add_shape(
        Sphere::new(Vec3::new(0.0, 2.0, -2.0), 0.3)
            .set_material(DiffuseLight::new(Color::new(255, 100, 100), 100.0)),
    );

    scene.to_scene()
}

#[allow(dead_code)]
fn get_teapot_scene(config: Config) -> Scene {
    let mut scene = SceneBuilder::new(config).set_skybox_color(Color::new(180, 180, 180));

    // Ground
    scene.add_shape(
        Sphere::new(Vec3::new(0.0, 0.0, -1000.0), 1000.0)
            .set_material(Metal::new(Color::new(230, 230, 230), 0.0)),
    );

    // Add the teapot
    scene.add_shape(STLMesh::new(
        "models/utah_teapot.stl",
        DiffuseMetal::new(Color::new(180, 180, 180), 0.0, 0.9),
    ));

    // Add light
    scene.add_shape(
        Sphere::new(Vec3::new(15.0, -15.0, 2.0), 2.0)
            .set_material(DiffuseLight::new(Color::new(200, 200, 255), 5.0)),
    );

    scene.add_shape(
        Sphere::new(Vec3::new(15.0, 15.0, 20.0), 4.0)
            .set_material(DiffuseLight::new(Color::new(150, 255, 200), 5.0)),
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
    let aspect_ratio = config.width as f64 / config.height as f64;

    // Camera that will render the scene
    let camera = Camera::new(
        Vec3::new(15.0, 10.0, 15.0),
        Vec3::new(-15.0, -10.0, -15.0),
        Vec3::new(-1.0, -1.0, 0.0),
        aspect_ratio,
    )
    .set_vertical_fov(50.0);

    // Configuration of the scene to render
    let scene = get_teapot_scene(config);

    // Render the scene
    render(scene, camera);
}
