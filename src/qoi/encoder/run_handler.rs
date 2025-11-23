use super::qoi_encoder::QoiEncoder;
use image::Rgba;

pub struct RunHandler {
    prev_pixel: Rgba<u8>,
    run_length: u8,
}

impl RunHandler {
    pub fn new() -> Self {
        Self {
            prev_pixel: Rgba([0, 0, 0, 255]),
            run_length: 0,
        }
    }

    pub fn handle(&mut self, qoi_buffer: &mut QoiEncoder, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            if *pixel == self.prev_pixel && self.run_length + 1 <= 62 {
                self.run_length += 1;
                *handled = true;
            } else if *pixel != self.prev_pixel && self.run_length != 0 {
                qoi_buffer.add_run_pixels(self.run_length);
                self.run_length = 0;
            }
        }

        self.prev_pixel = *pixel;
    }
}
