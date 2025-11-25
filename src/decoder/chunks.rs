use crate::tags;

#[derive(Debug)]
pub enum PixelChunk {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    Index(u8),
    Diff(u8, u8, u8),
    Luma(u8, u8, u8),
    Run(u8),
}

pub fn decode_pixel<'a, I>(iter: &mut I) -> Option<PixelChunk>
where
    I: Iterator<Item = &'a u8>,
{
    let tag = iter.next()?;

    match *tag {
        tags::QOI_OP_RGB_TAG => Some(PixelChunk::RGB(*iter.next()?, *iter.next()?, *iter.next()?)),
        tags::QOI_OP_RGBA_TAG => Some(PixelChunk::RGBA(
            *iter.next()?,
            *iter.next()?,
            *iter.next()?,
            *iter.next()?,
        )),
        _ => match *tag & 0b11000000 {
            tags::QOI_OP_INDEX_TAG => Some(PixelChunk::Index(*tag & 0b00111111)),
            tags::QOI_OP_DIFF_TAG => Some(PixelChunk::Diff(
                (*tag & 0b00110000) >> 4,
                (*tag & 0b00001100) >> 2,
                *tag & 0b00000011,
            )),
            tags::QOI_OP_LUMA_TAG => {
                let next_byte = *iter.next()?;
                Some(PixelChunk::Luma(
                    *tag & 0b00111111,
                    (next_byte & 0b11110000) >> 4,
                    next_byte & 0b00001111,
                ))
            }
            tags::QOI_OP_RUN_TAG => Some(PixelChunk::Run(*tag & 0b00111111)),
            _ => None,
        },
    }
}

pub fn chunk<'a, I>(iter: &mut I, n: usize) -> Vec<u8>
where
    I: Iterator<Item = &'a u8>,
{
    iter.take(n).copied().collect()
}
