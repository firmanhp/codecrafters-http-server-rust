use std::{
    collections::HashSet,
    io::{Error, ErrorKind, Result},
    sync::Arc,
};

use itertools::Itertools;

use crate::{encoding::types::ContentEncoding, server::ServerContext};

use super::{HttpRequest, HttpRequestHeader, HttpRequestType};

#[derive(Default)]
pub struct HttpRequestHeaderBuilder {
    host: Option<String>,
    user_agent: Option<String>,
    accept: Option<String>,
    content_type: Option<String>,
    content_length: Option<usize>,
    accept_encoding: HashSet<ContentEncoding>,
}

impl HttpRequestHeaderBuilder {
    pub fn new() -> HttpRequestHeaderBuilder {
        HttpRequestHeaderBuilder::default()
    }

    pub fn apply_from_line(self: Self, line: &str) -> Self {
        let key_value = line.splitn(2, ": ").collect_vec();
        if key_value.len() < 2 {
            println!("WARNING: ignoring header {}", line);
            return self;
        }
        match key_value[0].to_lowercase().as_str() {
            "host" => self.host(String::from(key_value[1])),
            "user-agent" => self.user_agent(String::from(key_value[1])),
            "accept" => self.accept(String::from(key_value[1])),
            "content-type" => self.content_type(String::from(key_value[1])),
            "content-length" => self.content_length(key_value[1].parse::<usize>().unwrap_or(0)),
            // encoding can be multiple schemas separated by a comma
            "accept-encoding" => self.accept_encodings_from_line(key_value[1]),
            _ => {
                println!("WARNING: unknown header key: {}", key_value[0]);
                self
            }
        }
    }

    pub fn host(mut self: Self, host: String) -> Self {
        self.host = Some(host);
        self
    }

    pub fn user_agent(mut self: Self, user_agent: String) -> Self {
        self.user_agent = Some(user_agent);
        self
    }

    pub fn accept(mut self: Self, accept: String) -> Self {
        self.accept = Some(accept);
        self
    }

    pub fn content_type(mut self: Self, content_type: String) -> Self {
        self.content_type = Some(content_type);
        self
    }

    pub fn content_length(mut self: Self, content_length: usize) -> Self {
        self.content_length = Some(content_length);
        self
    }

    pub fn accept_encoding(mut self: Self, accept_encoding: ContentEncoding) -> Self {
        self.accept_encoding.insert(accept_encoding);
        self
    }

    fn accept_encodings_from_line(self: Self, line: &str) -> Self {
        let delimiter = ", ";
        let encodings = line.split(delimiter).collect_vec();
        let mut builder = self;
        for encoding in encodings {
            match ContentEncoding::from(encoding) {
                Some(encoding) => {
                    builder = builder.accept_encoding(encoding);
                }
                _ => {}
            }
        }
        builder
    }

    pub fn build(self: Self) -> HttpRequestHeader {
        let mut header = HttpRequestHeader {
            host: self.host.unwrap_or(String::from("unknown")),
            user_agent: self.user_agent.unwrap_or(String::from("unknown")),
            accept: self.accept.unwrap_or(String::from("*/*")),
            content_type: self.content_type.unwrap_or(String::from("")),
            content_length: self.content_length.unwrap_or(0),
            accept_encoding: self.accept_encoding,
        };

        // accept-encoding must contain at least 1 encoding
        if header.accept_encoding.is_empty() {
            header.accept_encoding.insert(ContentEncoding::NoEncoding);
        }
        header
    }
}

pub struct HttpRequestBuilder {
    request_type: HttpRequestType,
    context: Arc<ServerContext>,
    path: String,
    header: Option<HttpRequestHeader>,
    body: Option<Vec<u8>>,
}

impl HttpRequestBuilder {
    pub fn from_request_line(
        line: &str,
        context: Arc<ServerContext>,
    ) -> Result<HttpRequestBuilder> {
        let components = line.split(" ").collect_vec();
        if components.len() != 3 {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Invalid request line: {}", line),
            ));
        }
        let request_type = HttpRequestType::from_str(components[0])?;
        let path = if components[1].is_empty() {
            String::from("/")
        } else {
            components[1].to_string()
        };
        // Ignore http version for now.

        Ok(HttpRequestBuilder::new(request_type, path, context))
    }

    pub fn new(
        request_type: HttpRequestType,
        path: String,
        context: Arc<ServerContext>,
    ) -> HttpRequestBuilder {
        HttpRequestBuilder {
            request_type,
            context,
            path,
            header: None,
            body: None,
        }
    }

    pub fn header(mut self: Self, header: HttpRequestHeader) -> Self {
        self.header = Some(header);
        self
    }

    pub fn body(mut self: Self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }

    pub fn build(self: Self) -> HttpRequest {
        HttpRequest {
            request_type: self.request_type,
            path: self.path,
            header: self
                .header
                .unwrap_or(HttpRequestHeaderBuilder::new().build()),
            body: self.body.unwrap_or(vec![]),
            context: self.context,
        }
    }
}
