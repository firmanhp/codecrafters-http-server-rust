use std::io::{Result, Write};
use std::net::TcpStream;

pub enum ResponseType {
    Ok,
    NotFound,
}

impl ResponseType {
    pub fn to_code(&self) -> u16 {
        match self {
            ResponseType::Ok => 200,
            ResponseType::NotFound => 404,
        }
    }

    pub fn to_str(&self) -> &str {
        match self {
            ResponseType::Ok => "OK",
            ResponseType::NotFound => "Not Found",
        }
    }

    pub fn to_raw_line(&self) -> String {
        String::from(format!("HTTP/1.1 {} {}", self.to_code(), self.to_str()))
    }
}

pub struct Response {
    response_type: ResponseType,
    content_type: String,
    content_length: usize,
    body: Vec<u8>,
}

impl Response {
    fn from_str(response_type: ResponseType, body: &str) -> Response {
        Response {
            response_type: response_type,
            content_type: String::from("text/plain"),
            content_length: body.len(),
            body: body.to_owned().into_bytes(),
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

    fn respond(mut stream: TcpStream, response: &Response) -> Result<()> {
        stream.write_all(&response.as_bytes())
    }

    pub fn respond_200(stream: TcpStream, body: &str) -> Result<()> {
        Self::respond(stream, &Response::from_str(ResponseType::Ok, body))
    }

    pub fn respond_404(stream: TcpStream) -> Result<()> {
        Self::respond(stream, &Response::from_str(ResponseType::NotFound, ""))
    }
}
