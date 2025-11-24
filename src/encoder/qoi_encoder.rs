use crate::errors::QOIError;

use super::{DiffHandler, ImageBuffer, NormalHandler, RunHandler, SeenHandler};

use image::{DynamicImage, GenericImageView, ImageReader};

pub fn encode_file(image_path: &str) -> Result<ImageBuffer, QOIError> {
    let reader = ImageReader::open(image_path).map_err(|_| QOIError::FileReadError)?;
    let image = reader.decode().map_err(|_| QOIError::ImageDecodeError)?;
    encode(&image)
}

pub fn encode(image: &DynamicImage) -> Result<ImageBuffer, QOIError> {
    let mut qoi_buffer = ImageBuffer::new(&image);
    let mut run_handler = RunHandler::new();
    let mut seen_handler = SeenHandler::new();
    let mut diff_handler = DiffHandler::new();
    let mut normal_handler = NormalHandler::new(image.has_alpha());

    for (_, _, pixel) in image.pixels() {
        let mut handled = false;
        run_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        seen_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        diff_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
        normal_handler.handle(&mut qoi_buffer, &pixel, &mut handled);
    }

    run_handler.cleanup(&mut qoi_buffer);
    qoi_buffer.end_byte_stream();

    Ok(qoi_buffer)
}
