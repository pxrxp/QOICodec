use super::qoi_encoder::QoiEncoder;
use image::Rgba;

pub struct DiffHandler {
    prev_pixel: Rgba<u8>,
}

impl DiffHandler {
    pub fn new() -> Self {
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

    pub fn handle(&mut self, qoi_buffer: &mut QoiEncoder, pixel: &Rgba<u8>, handled: &mut bool) {
        if !*handled {
            let [r, g, b, a] = pixel.0;
            let [r_prev, g_prev, b_prev, a_prev] = self.prev_pixel.0;

            let dr = r.wrapping_sub(r_prev);
            let dg = g.wrapping_sub(g_prev);
            let db = b.wrapping_sub(b_prev);
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
