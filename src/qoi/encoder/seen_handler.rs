use super::qoi_encoder::QoiEncoder;
use image::Rgba;

pub struct SeenHandler {
    seen_pixels: Vec<Rgba<u8>>,
}

impl SeenHandler {
    pub fn new() -> Self {
        Self {
            seen_pixels: vec![Rgba([0, 0, 0, 0]); 64],
        }
    }

    fn hash(pixel: &Rgba<u8>) -> u8 {
        let [r, g, b, a] = pixel.0;
        (r * 3 + g * 5 + b * 7 + a * 11) % 64
    }

    fn add_pixel(&mut self, pixel: &Rgba<u8>) {
        self.seen_pixels[SeenHandler::hash(pixel) as usize] = *pixel;
    }

    fn exists(&self, pixel: &Rgba<u8>) -> bool {
        self.seen_pixels[SeenHandler::hash(pixel) as usize] == *pixel
    }

    pub fn handle(&mut self, qoi_buffer: &mut QoiEncoder, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            if self.exists(pixel) {
                qoi_buffer.add_seen_pixel(SeenHandler::hash(pixel));
                *handled = true;
            }
        }

        self.add_pixel(pixel);
    }
}
