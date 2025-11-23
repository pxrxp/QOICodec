use image::{DynamicImage, GenericImageView};

const QOI_OP_RGB_TAG: u8 = 0b11111110;
const QOI_OP_RGBA_TAG: u8 = 0b11111111;
const QOI_OP_INDEX_TAG: u8 = 0b00 << 7;
const QOI_OP_DIFF_TAG: u8 = 0b01 << 7;
const QOI_OP_LUMA_TAG: u8 = 0b10 << 7;
const QOI_OP_RUN_TAG: u8 = 0b11 << 7;

pub struct QoiEncoder {
    qoi_buffer: Vec<u8>,
}

impl QoiEncoder {
    pub fn new(image: &DynamicImage) -> Self {
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

    pub fn add_run_pixels(&mut self, run: u8) {
        assert!(run >= 1 && run <= 62);
        self.qoi_buffer.push(QOI_OP_RUN_TAG | run - 1);
    }

    pub fn add_seen_pixel(&mut self, index: u8) {
        assert!(index <= 63);
        self.qoi_buffer.push(QOI_OP_INDEX_TAG | index);
    }

    pub fn add_diff_pixel(&mut self, dr: u8, dg: u8, db: u8) {
        let dr = dr.wrapping_add(2);
        let dg = dg.wrapping_add(2);
        let db = db.wrapping_add(2);
        self.qoi_buffer
            .push(QOI_OP_DIFF_TAG | dr >> 4 | dg >> 2 | db);
    }

    pub fn add_luma_pixel(&mut self, dr: u8, dg: u8, db: u8) {
        let dg = dg.wrapping_add(32);
        let dr_dg = dr.wrapping_sub(dg).wrapping_add(8);
        let db_dg = db.wrapping_sub(dg).wrapping_add(8);

        self.qoi_buffer
            .push(QOI_OP_LUMA_TAG | dr >> 4 | dr_dg >> 2 | db_dg);
    }

    pub fn add_rgb_pixel(&mut self, r: u8, g: u8, b: u8) {
        self.qoi_buffer.push(QOI_OP_RGB_TAG);
        self.qoi_buffer.push(r);
        self.qoi_buffer.push(g);
        self.qoi_buffer.push(b);
    }

    pub fn add_rgba_pixel(&mut self, r: u8, g: u8, b: u8, a: u8) {
        self.qoi_buffer.push(QOI_OP_RGBA_TAG);
        self.qoi_buffer.push(r);
        self.qoi_buffer.push(g);
        self.qoi_buffer.push(b);
        self.qoi_buffer.push(a);
    }

    pub fn end_byte_stream(&mut self) {
        self.qoi_buffer.push(0x01);
        self.qoi_buffer.extend_from_slice(&[0x00; 7]);
    }
}
