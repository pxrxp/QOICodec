pub struct QOIState {
    pub prev_pixel: image::Rgba<u8>,
    seen_pixels: Vec<image::Rgba<u8>>,
    pub run_length: u8,
}

impl QOIState {
    pub fn new() -> Self {
        Self {
            seen_pixels: vec![image::Rgba([0, 0, 0, 0]); 64],
            prev_pixel: image::Rgba([0, 0, 0, 255]),
            run_length: 0,
        }
    }

    pub fn hash(pixel: &image::Rgba<u8>) -> u8 {
        let [r, g, b, a] = pixel.0;
        (((r as usize * 3) + (g as usize * 5) + (b as usize * 7) + (a as usize * 11)) % 64) as u8
    }

    pub fn add_pixel(&mut self, pixel: &image::Rgba<u8>) {
        self.seen_pixels[Self::hash(pixel) as usize] = *pixel;
    }

    pub fn exists(&self, pixel: &image::Rgba<u8>) -> bool {
        self.seen_pixels[Self::hash(pixel) as usize] == *pixel
    }

    pub fn get_pixel(&self, index: usize) -> image::Rgba<u8> {
        self.seen_pixels
            .get(index)
            .copied()
            .unwrap_or(image::Rgba([0, 0, 0, 0]))
    }

    pub fn update(&mut self, pixel: &image::Rgba<u8>) {
        self.prev_pixel = *pixel;
        self.add_pixel(pixel);
    }
}
