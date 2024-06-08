use std::io::{Result, Write};
use std::net::TcpStream;

pub enum HttpResponseType {
    Ok,
    NotFound,
}

impl HttpResponseType {
    fn to_code(&self) -> u16 {
        match self {
            HttpResponseType::Ok => 200,
            HttpResponseType::NotFound => 404,
        }
    }

    fn to_str(&self) -> &str {
        match self {
            HttpResponseType::Ok => "OK",
            HttpResponseType::NotFound => "Not Found",
        }
    }

    fn to_raw_line(&self) -> String {
        String::from(format!("HTTP/1.1 {} {}", self.to_code(), self.to_str()))
    }
}

pub struct HttpResponse {
    response_type: HttpResponseType,
    content_type: String,
    content_length: usize,
    body: Vec<u8>,
}

impl HttpResponse {
    pub fn from_str(response_type: HttpResponseType, body: &str) -> HttpResponse {
        HttpResponse {
            response_type: response_type,
            content_type: String::from("text/plain"),
            content_length: body.len(),
            body: body.to_owned().into_bytes(),
        }
    }

    pub fn of(response_type: HttpResponseType) -> HttpResponse {
        HttpResponse {
            response_type: response_type,
            content_type: String::from("text/plain"),
            content_length: 0,
            body: vec![]
        }
    }

    fn as_bytes(&self) -> Vec<u8> {
        let mut result: Vec<u8> = vec![];
        result.reserve(self.content_length);

        result.extend_from_slice(self.response_type.to_raw_line().as_bytes());
        result.extend_from_slice(b"\r\n");

        result.extend_from_slice(b"Content-Type: ");
        result.extend_from_slice(self.content_type.as_bytes());
        result.extend_from_slice(b"\r\n");

        result.extend_from_slice(b"Content-Length: ");
        result.extend_from_slice(self.content_length.to_string().as_bytes());
        result.extend_from_slice(b"\r\n");

        result.extend_from_slice(b"\r\n");
        result.extend_from_slice(&self.body);
        result
    }

    pub fn respond(mut stream: TcpStream, response: &HttpResponse) -> Result<()> {
        stream.write_all(&response.as_bytes())
    }
}
