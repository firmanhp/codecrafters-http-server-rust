use crate::server;

use std::fmt;
use std::io::Error;
use std::io::ErrorKind;
use std::io::Result;
use std::sync::Arc;

use itertools::Itertools;

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
                format!("Request line is invalid: {}", type_str),
            )),
        }
    }
}

#[derive(Default)]
pub struct HttpRequestHeader {
    pub host: Option<String>,
    pub user_agent: Option<String>,
    pub accept: Option<String>,
    pub content_type: Option<String>,
    pub content_length: Option<usize>,
}

impl fmt::Display for HttpRequestHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpRequestHeader{{")?;
        write!(f, "Host: {:?}, ", self.host)?;
        write!(f, "User-Agent: {:?}, ", self.user_agent)?;
        write!(f, "Accept: {:?}, ", self.accept)?;
        write!(f, "Content-Type: {:?}, ", self.content_type)?;
        write!(f, "Content-Length: {:?}, ", self.content_length)?;
        write!(f, "}}")
    }
}

impl HttpRequestHeader {
    fn from_buffer_lines(lines: &[&str]) -> HttpRequestHeader {
        let mut header: HttpRequestHeader = Default::default();
        for line in lines {
            // We reached the end here i.e. \r\n\r\n
            if *line == "" {
                break;
            }
            let key_value = (*line).splitn(2, ": ").collect_vec();
            if key_value.len() != 2 {
                println!("WARNING: ignoring header {}", *line);
                continue;
            }
            match key_value[0].to_lowercase().as_str() {
                "host" => {
                    header.host = Some(String::from(key_value[1]));
                }
                "user-agent" => header.user_agent = Some(String::from(key_value[1])),
                "accept" => header.accept = Some(String::from(key_value[1])),
                "content-type" => header.content_type = Some(String::from(key_value[1])),
                "content-length" => {
                    header.content_length = Some(key_value[1].parse::<usize>().unwrap_or(0))
                }
                _ => {
                    println!("WARNING: unknown header key: {}", key_value[0]);
                }
            }
        }

        header
    }
}

pub struct HttpRequest {
    pub req_type: HttpRequestType,
    pub path: String,
    pub header: HttpRequestHeader,
    pub body: Vec<u8>,
    pub context: Arc<ServerContext>,
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HttpRequest{{")?;
        write!(f, "Type: {}, ", self.req_type)?;
        write!(f, "Path: {}, ", self.path)?;
        write!(f, "Header: {}, ", self.header)?;
        write!(f, "Body (len): {}, ", self.body.len())?;
        write!(f, "}}")
    }
}

impl HttpRequest {
    pub fn from_bytes(buf: &[u8], server_context: Arc<ServerContext>) -> Result<HttpRequest> {
        let buf_str = String::from_utf8_lossy(buf);
        let mut request_type: HttpRequestType = HttpRequestType::Get;
        let mut path: String = String::from("");

        let delimiter: &str = "\r\n";
        let req_lines = buf_str.split(delimiter).collect_vec();

        if req_lines.len() == 0 {
            return Err(Error::new(ErrorKind::InvalidData, "Unspecified request"));
        }

        // First request line
        if req_lines.len() >= 1 {
            let req_line = req_lines[0].split(" ").collect_vec();
            if req_line.len() != 3 {
                return Err(Error::new(
                    ErrorKind::InvalidData,
                    "Request line is invalid",
                ));
            }
            request_type = HttpRequestType::from_str(req_line[0])?;
            path = String::from(req_line[1]);
            // ignore HTTP version from now
        }

        Ok(HttpRequest {
            req_type: request_type,
            path: path,
            header: HttpRequestHeader::from_buffer_lines(&req_lines[2..]),
            body: vec![],
            context: server_context,
        })
    }
}
