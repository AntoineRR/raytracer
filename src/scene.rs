use crate::Config;
use image::ImageBuffer;

pub struct Scene {
    config: Config,
}

impl Scene {
    pub fn new(config: Config) -> Self {
        Scene { config }
    }

    pub fn render(&self) {
        // temp
        let mut buffer = ImageBuffer::new(self.config.width, self.config.height);
        for (x, y, pixel) in buffer.enumerate_pixels_mut() {
            let r = x as u8;
            let g = y as u8;
            let b = (0.25 * 255.0) as u8;
            *pixel = image::Rgb([r, g, b]);
        }
        buffer.save(&self.config.output_path).unwrap();
    }
}
