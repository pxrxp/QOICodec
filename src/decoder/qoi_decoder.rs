use std::fs;

use image::DynamicImage;

use crate::errors::QOIError;

pub struct DecodedImage {
    image: Option<DynamicImage>,
    errors: Vec<String>,
}

pub fn decode_file(image_path: &str) -> Result<DecodedImage, QOIError> {
    let bytes: Vec<u8> = fs::read(image_path).map_err(|_| QOIError::FileReadError)?;
    decode(&bytes)
}

pub fn decode(image_bytes: &Vec<u8>) -> Result<DecodedImage, QOIError> {
    let mut iter = image_bytes.iter();

    let magic_chunks = chunk(&mut iter, 4);
    let width = u32::from_be_bytes(
        chunk(&mut iter, 4)
            .try_into()
            .map_err(|_| QOIError::ImageDecodeError)?,
    );
    let height = u32::from_be_bytes(
        chunk(&mut iter, 4)
            .try_into()
            .map_err(|_| QOIError::ImageDecodeError)?,
    );
    let channels = iter.next().unwrap();
    let colorspace = iter.next().unwrap();

    assert_eq!(magic_chunks, b"qoif");
    println!("Width: {width}");
    println!("Height: {height}");
    println!("Channels: {channels}");
    println!("Colorspace: {colorspace}");

    Ok(DecodedImage {
        image: None,
        errors: vec![],
    })
}

fn chunk<'a, I>(iter: &mut I, n: usize) -> Vec<u8>
where
    I: Iterator<Item = &'a u8>,
{
    iter.take(n).copied().collect()
}
