use std::io::Result;

use crate::encoding::types::ContentEncoding;
use crate::encoding::types::EncodedContent;

use super::HttpResponse;
use super::HttpResponseType;

pub struct HttpResponseBuilder {
    response_type: HttpResponseType,
    content_type: Option<String>,
    body: Option<EncodedContent>,
}

impl HttpResponseBuilder {
    pub fn new(response_type: HttpResponseType) -> HttpResponseBuilder {
        HttpResponseBuilder {
            response_type,
            content_type: None,
            body: None,
        }
    }

    pub fn from(response: HttpResponse) -> HttpResponseBuilder {
        HttpResponseBuilder {
            response_type: response.response_type,
            content_type: Some(response.content_type),
            body: Some(response.body)
        }
    }

    pub fn content_type(mut self: Self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn body(mut self: Self, body: EncodedContent) -> Self {
        self.body = Some(body);
        self
    }

    pub fn encode_body(mut self: Self, encoding_type: ContentEncoding) -> Result<Self> {
        if !self.body.is_none() {
            self.body = Some(self.body.unwrap().encode(encoding_type)?);
        }
        Ok(self)
    }

    pub fn build(self: Self) -> HttpResponse {
        HttpResponse {
            response_type: self.response_type,
            content_type: self.content_type.unwrap_or(String::from("text/plain")),
            content_length: self
                .body
                .as_ref()
                .and_then(|c| Some(c.buffer.len()))
                .unwrap_or(0),
            body: self.body.unwrap_or_default(),
        }
    }
}
