pub mod builder;

use std::io::{Result, Write};
use std::net::TcpStream;

use crate::encoding::types::{ContentEncoding, EncodedContent};

pub enum HttpResponseType {
    Ok,
    NotFound,
    InternalServerError,
    Created,
    ServiceUnavailable,
    Conflict,
}

pub struct HttpResponse {
    pub response_type: HttpResponseType,
    pub content_type: String,
    pub content_length: usize,
    pub body: EncodedContent,
}

impl HttpResponseType {
    fn to_code(&self) -> u16 {
        match self {
            HttpResponseType::Ok => 200,
            HttpResponseType::NotFound => 404,
            HttpResponseType::InternalServerError => 500,
            HttpResponseType::Created => 201,
            HttpResponseType::ServiceUnavailable => 503,
            HttpResponseType::Conflict => 409,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            HttpResponseType::Ok => "OK",
            HttpResponseType::NotFound => "Not Found",
            HttpResponseType::InternalServerError => "Internal Server Error",
            HttpResponseType::Created => "Created",
            HttpResponseType::ServiceUnavailable => "Service Unavailable",
            HttpResponseType::Conflict => "Conflict",
        }
    }

    fn to_raw_line(&self) -> String {
        String::from(format!("HTTP/1.1 {} {}", self.to_code(), self.to_str()))
    }
}

impl HttpResponse {
    fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.reserve(self.content_length);

        
        result.extend_from_slice(self.response_type.to_raw_line().as_bytes());
        result.extend_from_slice(b"\r\n");
        
        if self.has_body() {
            result.extend_from_slice(b"Content-Type: ");
            result.extend_from_slice(self.content_type.as_bytes());
            result.extend_from_slice(b"\r\n");
    
            result.extend_from_slice(b"Content-Length: ");
            result.extend_from_slice(self.content_length.to_string().as_bytes());
            result.extend_from_slice(b"\r\n");

            if self.body.encoding_type != ContentEncoding::NoEncoding {
                result.extend_from_slice(b"Content-Encoding: ");
                result.extend_from_slice(self.body.encoding_type.to_str().as_bytes());
                result.extend_from_slice(b"\r\n");
            }
        }

        result.extend_from_slice(b"\r\n");
        // body...
        if self.has_body() {
            result.extend_from_slice(&self.body.buffer);
        }
        result
    }

    pub fn respond(mut stream: TcpStream, response: &HttpResponse) -> Result<()> {
        stream.write_all(&response.as_bytes())
    }

    pub fn has_body(self: &Self) -> bool {
        return self.body.buffer.len() > 0;
    }
}
