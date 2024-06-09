use std::io::{Read, Result, Write};

use flate2::{read::GzDecoder, write::ZlibEncoder, Compression};

use super::types::{ContentEncoding, EncodedContent};

pub fn encode(content: EncodedContent) -> Result<EncodedContent> {
    let buffer = content.buffer;
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(buffer.as_slice())?;
    Ok(EncodedContent {
        encoding_type: ContentEncoding::Gzip,
        buffer: encoder.finish()?,
    })
}

pub fn decode(content: EncodedContent) -> Result<EncodedContent> {
    let mut decoder = GzDecoder::new(content.buffer.as_slice());
    let mut bytes_result: Vec<u8> = vec![];
    decoder.read_to_end(&mut bytes_result)?;
    Ok(EncodedContent {
        encoding_type: ContentEncoding::NoEncoding,
        buffer: bytes_result,
    })
}
