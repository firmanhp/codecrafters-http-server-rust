mod files;

use crate::encoding::types::EncodedContent;
use crate::request;
use crate::response;
use crate::server;

use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;

use request::HttpRequest;
use response::{builder::HttpResponseBuilder, HttpResponse, HttpResponseType};
use server::ServerContext;

fn route(request: HttpRequest) -> HttpResponse {
    if request.path.starts_with("/files/") {
        return files::handle(request);
    } else if request.path.starts_with("/echo/") {
        let to_echo = &request.path["/echo/".len()..];
        return HttpResponseBuilder::new(HttpResponseType::Ok)
            .body(EncodedContent::from(String::from(to_echo).into_bytes()))
            .build();
    } else if request.path.starts_with("/user-agent") {
        return HttpResponseBuilder::new(HttpResponseType::Ok)
            .body(EncodedContent::from(request.header.user_agent.into_bytes()))
            .build();
    } else if &request.path == "/" {
        return HttpResponseBuilder::new(HttpResponseType::Ok)
            .body(EncodedContent::from(String::from("").into_bytes()))
            .build();
    } else {
        return HttpResponseBuilder::new(HttpResponseType::NotFound).build();
    }
}

pub fn handle_connection(stream: TcpStream, server_context: Arc<ServerContext>) -> Result<()> {
    let request = HttpRequest::read_from_stream(&stream, server_context)?;
    println!("request: {}", request);

    let requested_encodings = request.header.accept_encoding.clone();
    let mut response = route(request);

    // Match response's encoding with request's.
    if response.has_body() && !requested_encodings.contains(&response.body.encoding_type) {
        let valid_encoding = requested_encodings.iter().next().unwrap().clone();
        // Reencode if they differ
        // println!("requested encoding {:?}", requested_encoding);
        response = HttpResponseBuilder::from(response)
            .encode_body(valid_encoding)?
            .build();
    }
    HttpResponse::respond(stream, &response)
}
