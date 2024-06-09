mod builder;

use crate::encoding::types::ContentEncoding;
use crate::server;

use std::fmt;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;

use builder::HttpRequestBuilder;
use builder::HttpRequestHeaderBuilder;

use server::ServerContext;

#[derive(Debug, PartialEq)]
pub enum HttpRequestType {
    Get,
    Post,
    Put,
    Delete,
    // may add more here...
}

impl fmt::Display for HttpRequestType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpRequestType::{:?}", self)
    }
}

impl HttpRequestType {
    fn from_str(type_str: &str) -> Result<HttpRequestType> {
        match type_str {
            "GET" => Ok(HttpRequestType::Get),
            "POST" => Ok(HttpRequestType::Post),
            "PUT" => Ok(HttpRequestType::Put),
            "DELETE" => Ok(HttpRequestType::Delete),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Request type is invalid: {}", type_str),
            )),
        }
    }
}

pub struct HttpRequestHeader {
    pub host: String,
    pub user_agent: String,
    pub accept: String,
    pub content_type: String,
    pub content_length: usize,
    pub accept_encoding: ContentEncoding,
}

impl fmt::Display for HttpRequestHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpRequestHeader{{")?;
        write!(f, "Host: {:?}, ", self.host)?;
        write!(f, "User-Agent: {:?}, ", self.user_agent)?;
        write!(f, "Accept: {:?}, ", self.accept)?;
        write!(f, "Content-Type: {:?}, ", self.content_type)?;
        write!(f, "Content-Length: {:?}, ", self.content_length)?;
        write!(f, "Accept-Encoding: {:?}, ", self.accept_encoding)?;
        write!(f, "}}")
    }
}

pub struct HttpRequest {
    pub request_type: HttpRequestType,
    pub path: String,
    pub header: HttpRequestHeader,
    pub body: Vec<u8>,
    pub context: Arc<ServerContext>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpRequest{{")?;
        write!(f, "Type: {}, ", self.request_type)?;
        write!(f, "Path: {}, ", self.path)?;
        write!(f, "Header: {}, ", self.header)?;
        write!(f, "Body (len): {}, ", self.body.len())?;
        write!(f, "}}")
    }
}

impl HttpRequest {
    pub fn read_from_stream(
        mut stream: &TcpStream,
        server_context: Arc<ServerContext>,
    ) -> Result<HttpRequest> {
        let mut read_buffer: Vec<u8> = vec![];
        read_buffer.reserve(128);
        read_line(&mut stream, &mut read_buffer)?;
        let mut request_builder = HttpRequestBuilder::from_request_line(
            &String::from_utf8_lossy(&read_buffer),
            server_context,
        )?;

        // Read for request header lines
        let mut request_header_builder = HttpRequestHeaderBuilder::new();
        loop {
            // Clear read buffer, Read for one line
            read_buffer.clear();
            read_line(&mut stream, &mut read_buffer)?;
            if read_buffer.is_empty() {
                // end streaming
                break;
            }
            request_header_builder =
                request_header_builder.apply_from_line(&String::from_utf8_lossy(&read_buffer));
        }
        let request_header = request_header_builder.build();
        if request_header.content_length > 0 {
            let mut body: Vec<u8> = vec![];
            body.resize(request_header.content_length, 0);
            stream.read_exact(body.as_mut_slice())?;
            request_builder = request_builder.body(body);
        }

        Ok(request_builder.header(request_header).build())
    }
}

fn read_line(mut stream: &TcpStream, read_buffer: &mut Vec<u8>) -> Result<()> {
    let delimiter: &[u8] = b"\r\n";
    // Read for first line
    loop {
        // Can probably be optimized
        let mut single_byte: [u8; 1] = [0];
        stream.read_exact(&mut single_byte)?;
        read_buffer.push(single_byte[0]);
        if single_byte[0] == delimiter[delimiter.len() - 1]
            && read_buffer.len() >= delimiter.len()
            && &read_buffer[read_buffer.len() - delimiter.len()..] == delimiter
        {
            // Remove delimiter from buffer
            read_buffer.truncate(read_buffer.len() - delimiter.len());
            break;
        }
    }
    Ok(())
}
