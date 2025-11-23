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
    image: DynamicImage,
}

impl ImageBuffer {
    fn new(image_path: String) -> Self {
        let reader = ImageReader::open(image_path).expect("Couldn't open file.");
        let image = reader.decode().expect("Couldn't decode provided file.");
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

        Self { qoi_buffer, image }
    }

    fn add_run_pixels(&mut self, run: u8) {
        assert!(run >= 1 && run <= 62);
        self.qoi_buffer.push(QOI_OP_RUN_TAG | run);
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

    fn handle(&mut self, qoi_buffer: &mut ImageBuffer, pixel: &Rgba<u8>) -> bool {
        if *pixel == self.prev_pixel && self.run_length + 1 <= 62 {
            self.run_length += 1;
            return true;
        } else if *pixel != self.prev_pixel && self.run_length != 0 {
            qoi_buffer.add_run_pixels(self.run_length);
            self.run_length = 0;
            return true;
        }
        false
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    match args.get(1).expect("Invalid no. of arguments").as_str() {
        "--encode" | "-e" => {
            let filename = *args.get(2).expect("Image file not provided.");
            let mut qoi_buffer = ImageBuffer::new(filename);

            let mut run_handler = RunHandler::new();

            let mut seen_pixels: Vec<Rgba<u8>> = Vec::with_capacity(64);
            let mut prev_pixel: Rgba<u8> = Rgba([0, 0, 0, 255]);

            // for pixel in qoi_buffer.pixels() {

            // if pixel
            // }

            qoi_buffer.end_byte_stream();
        }

        "--decode" | "-d" => {}
        "--help" | "-h" => {}
        _ => panic!("Invalid command. Expected '--encode' or '--decode' or '--help'"),
    }

    Ok(())
}
