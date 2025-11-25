use crate::{decoder::chunks::chunk, errors::QOIError};

pub struct QOIHeader {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colorspace: u8,
}

impl QOIHeader {
    pub fn new<'a, I>(iter: &mut I) -> Result<Self, QOIError>
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
