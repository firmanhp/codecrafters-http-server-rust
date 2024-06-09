use crate::encoding::gzip;

use std::io::Result;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
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
    pub fn from(str: &str) -> Option<ContentEncoding> {
        match str.to_lowercase().as_str() {
            "gzip" => Some(ContentEncoding::Gzip),
            _ => None,
        }
    }

    pub fn to_str(self: &Self) -> &str {
        match self {
            ContentEncoding::NoEncoding => "none",
            ContentEncoding::Gzip => "gzip"
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

    pub fn encode(
        self: Self,
        encoding_type: ContentEncoding,
    ) -> Result<EncodedContent> {
        if self.encoding_type == encoding_type {
            return Ok(self);
        }

        let decoded = Self::decode(self)?;
        match encoding_type {
            ContentEncoding::NoEncoding => Ok(decoded),
            // TODO
            ContentEncoding::Gzip => gzip::encode(decoded),
        }
    }

    pub fn decode(self: Self) -> Result<EncodedContent> {
        match self.encoding_type {
            ContentEncoding::NoEncoding => Ok(self),
            // TODO
            ContentEncoding::Gzip => gzip::decode(self),
        }
    }
}
