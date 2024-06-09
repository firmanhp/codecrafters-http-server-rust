use std::io::Result;

use super::types::{ContentEncoding, EncodedContent};

pub fn encode(content: EncodedContent) -> Result<EncodedContent> {
    // TODO
    Ok(EncodedContent {
        encoding_type: ContentEncoding::Gzip,
        ..content
    })
}

pub fn decode(content: EncodedContent) -> Result<EncodedContent> {
    // TODO
    Ok(EncodedContent {
        encoding_type: ContentEncoding::NoEncoding,
        ..content
    })
}
