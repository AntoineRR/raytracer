use raytracer::scene::Scene;
use raytracer::Config;

fn main() {
    let config = Config {
        width: 256,
        height: 256,
        output_path: String::from("test.png"),
    };

    let scene = Scene::new(config);
    scene.render();
}
