use crate::errors::QOIError;
use image::DynamicImage;
use std::fs;

struct QOIHeader {
    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,
}

impl QOIHeader {
    fn new<'a, I>(iter: &mut I) -> Result<Self, QOIError>
    where
        I: Iterator<Item = &'a u8>,
    {
        let magic_chunks = chunk(iter, 4);
        let width = u32::from_be_bytes(chunk(iter, 4).try_into().map_err(|_| {
            QOIError::ImageDecodeError(
                "Invalid QOI Header: 'width' field requires 4 bytes but stream ended".to_owned(),
            )
        })?);
        let height = u32::from_be_bytes(chunk(iter, 4).try_into().map_err(|_| {
            QOIError::ImageDecodeError(
                "Invalid QOI Header: 'height' field requires 4 bytes but stream ended".to_owned(),
            )
        })?);
        let channels = *iter.next().ok_or(QOIError::ImageDecodeError(
            "Invalid QOI Header: 'channels' field requires 1 byte but stream ended".to_owned(),
        ))?;
        let colorspace = *iter.next().ok_or(QOIError::ImageDecodeError(
            "Invalid QOI Header: 'colorspace' field requires 1 byte but stream ended".to_owned(),
        ))?;

        if magic_chunks != b"qoif"
            || (channels != 3 && channels != 4)
            || (colorspace != 0 && colorspace != 1)
        {
            return Err(QOIError::ImageDecodeError(
                "Invalid QOI Header: 'magic' field does not equal to 'qoif'".to_owned(),
            ));
        }

        Ok(Self {
            width,
            height,
            channels,
            colorspace,
        })
    }
}

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

    let header = QOIHeader::new(&mut iter)?;

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
