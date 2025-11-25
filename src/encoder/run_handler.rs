use crate::{QOIState, encoder::ImageBuffer};
use image::Rgba;

pub fn handle(qoi_buffer: &mut ImageBuffer, state: &mut QOIState, pixel: &Rgba<u8>) -> bool {
    if *pixel == state.prev_pixel {
        state.run_length += 1;

        if state.run_length == 62 {
            qoi_buffer.add_run_pixels(state.run_length);
            state.run_length = 0;
        }
        return true;
    }

    if state.run_length > 0 {
        qoi_buffer.add_run_pixels(state.run_length);
        state.run_length = 0;
    }

    false
}

pub fn cleanup(qoi_buffer: &mut ImageBuffer, state: &mut QOIState) {
    if state.run_length != 0 {
        qoi_buffer.add_run_pixels(state.run_length);
        state.run_length = 0;
    }
}
