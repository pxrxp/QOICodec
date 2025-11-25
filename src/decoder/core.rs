use crate::{
    decoder::{
        chunks::{PixelChunk, decode_pixel},
        header::QOIHeader,
    },
    errors::QOIError,
    state::QOIState,
};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::fs;

pub fn decode_file(image_path: &str) -> Result<DynamicImage, QOIError> {
    let bytes: Vec<u8> = fs::read(image_path).map_err(|_| QOIError::FileReadError)?;
    decode(&bytes)
}

pub fn decode(image_bytes: &Vec<u8>) -> Result<DynamicImage, QOIError> {
    let mut iter = image_bytes.iter();

    let header = QOIHeader::new(&mut iter)?;

    let expected_buffer_size = (header.width as usize)
        .checked_mul(header.height as usize)
        .and_then(|x| x.checked_mul(4))
        .ok_or(QOIError::ImageTooLarge)?;

    let mut buffer: Vec<u8> = Vec::with_capacity(expected_buffer_size as usize);
    let mut state = QOIState::new();

    while let Some(pixel_chunk) = decode_pixel(&mut iter) {
        let pixels = match pixel_chunk {
            PixelChunk::RGB(r, g, b) => {
                let Rgba([_, _, _, a_prev]) = state.prev_pixel;
                vec![Rgba([r, g, b, a_prev])]
            }
            PixelChunk::RGBA(r, g, b, a) => vec![Rgba([r, g, b, a])],
            PixelChunk::Index(i) => vec![state.get_pixel(i as usize)],
            PixelChunk::Diff(dr, dg, db) => {
                let Rgba([r, g, b, a]) = state.prev_pixel;
                vec![Rgba([
                    r.wrapping_add(dr),
                    g.wrapping_add(dg),
                    b.wrapping_add(db),
                    a,
                ])]
            }
            PixelChunk::Luma(dg, dr_dg, db_dg) => {
                let Rgba([r, g, b, a]) = state.prev_pixel;
                vec![Rgba([
                    r.wrapping_add(dr_dg).wrapping_add(dg),
                    g.wrapping_add(dg),
                    b.wrapping_add(db_dg).wrapping_add(dg),
                    a,
                ])]
            }
            PixelChunk::Run(run) => vec![state.prev_pixel; run as usize],
        };
        for pixel in pixels {
            buffer.extend_from_slice(pixel.0.as_slice());
            state.update(&pixel);
        }
    }

    if buffer.len() != expected_buffer_size {
        buffer.resize(expected_buffer_size, 0);
    }

    let rgba_buffer =
        ImageBuffer::<Rgba<u8>, Vec<u8>>::from_raw(header.width, header.height, buffer)
            .ok_or(QOIError::ImageTooLarge)?;

    let rgba_image = DynamicImage::ImageRgba8(rgba_buffer);
    if header.channels == 4 {
        Ok(rgba_image)
    } else {
        Ok(DynamicImage::ImageRgb8(rgba_image.into_rgb8()))
    }
}
