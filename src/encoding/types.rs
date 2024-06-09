use crate::encoding::gzip;

use std::io::Result;

#[derive(Debug, PartialEq)]
pub enum ContentEncoding {
    NoEncoding,
    Gzip,
}

pub struct EncodedContent {
    pub buffer: Vec<u8>,
    pub encoding_type: ContentEncoding,
}

impl Default for EncodedContent {
    fn default() -> Self {
        Self {
            buffer: vec![],
            encoding_type: ContentEncoding::NoEncoding,
        }
    }
}

impl ContentEncoding {
    pub fn from(str: &str) -> ContentEncoding {
        match str.to_lowercase().as_str() {
            "gzip" => ContentEncoding::Gzip,
            _ => ContentEncoding::NoEncoding,
        }
    }

    pub fn to_str(self: &Self) -> &str {
        match self {
            ContentEncoding::NoEncoding => "none",
            ContentEncoding::Gzip => "gzip"
        }
    }

    pub fn encode(
        content: EncodedContent,
        encoding_type: ContentEncoding,
    ) -> Result<EncodedContent> {
        if content.encoding_type == encoding_type {
            return Ok(content);
        }

        let decoded = Self::decode(content)?;
        match encoding_type {
            ContentEncoding::NoEncoding => Ok(decoded),
            // TODO
            ContentEncoding::Gzip => gzip::encode(decoded),
        }
    }

    pub fn decode(content: EncodedContent) -> Result<EncodedContent> {
        match content.encoding_type {
            ContentEncoding::NoEncoding => Ok(content),
            // TODO
            ContentEncoding::Gzip => gzip::decode(content),
        }
    }
}

impl EncodedContent {
    pub fn from(buffer: Vec<u8>) -> EncodedContent {
        EncodedContent {
            buffer,
            encoding_type: ContentEncoding::NoEncoding,
        }
    }
}
