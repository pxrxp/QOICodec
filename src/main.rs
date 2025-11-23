use image::{DynamicImage, GenericImageView, ImageReader, Rgba};
use std::{env, error::Error};

const QOI_OP_RGB_TAG: u8 = 0b11111110;
const QOI_OP_RGBA_TAG: u8 = 0b11111111;
const QOI_OP_INDEX_TAG: u8 = 0b00 << 7;
const QOI_OP_DIFF_TAG: u8 = 0b01 << 7;
const QOI_OP_LUMA_TAG: u8 = 0b10 << 7;
const QOI_OP_RUN_TAG: u8 = 0b11 << 7;

struct ImageBuffer {
    qoi_buffer: Vec<u8>,
}

impl ImageBuffer {
    fn new(image: &DynamicImage) -> Self {
        let (w, h): (u32, u32) = image.dimensions();

        let mut qoi_buffer = Vec::with_capacity((w * h * 4) as usize);

        let magic: [u8; 4] = *b"qoif";
        let width: [u8; 4] = w.to_ne_bytes();
        let height: [u8; 4] = h.to_ne_bytes();
        let channels: u8 = if image.has_alpha() { 4 } else { 3 };
        let colorspace: u8 = match image {
            DynamicImage::ImageRgb32F(_) | DynamicImage::ImageRgba32F(_) => 1,
            _ => 0,
        };

        qoi_buffer.extend_from_slice(&magic);
        qoi_buffer.extend_from_slice(&width);
        qoi_buffer.extend_from_slice(&height);
        qoi_buffer.push(channels);
        qoi_buffer.push(colorspace);

        Self { qoi_buffer }
    }

    fn add_run_pixels(&mut self, run: u8) {
        assert!(run >= 1 && run <= 62);
        self.qoi_buffer.push(QOI_OP_RUN_TAG | run - 1);
    }

    fn add_seen_pixel(&mut self, index: u8) {
        assert!(index <= 63);
        self.qoi_buffer.push(QOI_OP_INDEX_TAG | index);
    }

    fn add_diff_pixel(&mut self, dr: u8, dg: u8, db: u8) {
        let dr = dr.wrapping_add(2);
        let dg = dg.wrapping_add(2);
        let db = db.wrapping_add(2);
        self.qoi_buffer
            .push(QOI_OP_DIFF_TAG | dr >> 4 | dg >> 2 | db);
    }

    fn add_luma_pixel(&mut self, dr: u8, dg: u8, db: u8) {
        let dg = dg.wrapping_add(32);
        let dr_dg = dr.wrapping_sub(dg).wrapping_add(8);
        let db_dg = db.wrapping_sub(dg).wrapping_add(8);

        self.qoi_buffer
            .push(QOI_OP_DIFF_TAG | dr >> 4 | dr_dg >> 2 | db_dg);
    }

    fn end_byte_stream(&mut self) {
        self.qoi_buffer.push(0x01);
        self.qoi_buffer.extend_from_slice(&[0x00; 7]);
    }
}

struct RunHandler {
    prev_pixel: Rgba<u8>,
    run_length: u8,
}

impl RunHandler {
    fn new() -> Self {
        Self {
            prev_pixel: Rgba([0, 0, 0, 255]),
            run_length: 0,
        }
    }

    fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>, handled: &mut bool) {
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

struct SeenHandler {
    seen_pixels: Vec<Rgba<u8>>,
}

impl SeenHandler {
    fn new() -> Self {
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

    fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            if self.exists(pixel) {
                qoi_buffer.add_seen_pixel(SeenHandler::hash(pixel));
                *handled = true;
            }
        }

        self.add_pixel(pixel);
    }
}

struct DiffHandler {
    prev_pixel: Rgba<u8>,
}

impl DiffHandler {
    fn new() -> Self {
        Self {
            prev_pixel: Rgba([0, 0, 0, 255]),
        }
    }

    fn diff_tag_eligible(dr: u8, dg: u8, db: u8, da: u8) -> bool {
        (da == 0)
            && (dr.wrapping_add(2) <= 3)
            && (dg.wrapping_add(2) <= 3)
            && (db.wrapping_add(2) <= 3)
    }

    fn luma_tag_eligible(dr: u8, dg: u8, db: u8, da: u8) -> bool {
        (da == 0)
            && (dr.wrapping_add(32) <= 63)
            && (dg.wrapping_add(8) <= 15)
            && (db.wrapping_add(8) <= 15)
    }

    fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            let [r, g, b, a] = pixel.0;
            let [r_prev, g_prev, b_prev, a_prev] = self.prev_pixel.0;

            let dr = r.wrapping_sub(r_prev);
            let dg = g.wrapping_sub(g_prev);
            let db = g.wrapping_sub(b_prev);
            let da = a.wrapping_sub(a_prev);

            if DiffHandler::diff_tag_eligible(dr, dg, db, da) {
                qoi_buffer.add_diff_pixel(dr, dg, db);
                *handled = true;
            }

            if DiffHandler::luma_tag_eligible(dr, dg, db, da) {
                qoi_buffer.add_luma_pixel(dr, dg, db);
                *handled = true;
            }
        }

        self.prev_pixel = *pixel;
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => {
            let image_path = args.get(2).expect("Image file not provided.");
            let reader = ImageReader::open(image_path).expect("Couldn't open file.");
            let image = reader.decode().expect("Couldn't decode provided file.");

            let mut qoi_buffer = ImageBuffer::new(&image);
            let mut run_handler = RunHandler::new();
            let mut seen_handler = SeenHandler::new();
            let mut diff_handler = DiffHandler::new();

            for (_, _, pixel) in image.pixels() {
                let mut handled = false;
                run_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
                seen_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
                diff_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
            }

            qoi_buffer.end_byte_stream();
        }

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
