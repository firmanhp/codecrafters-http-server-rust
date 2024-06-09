use crate::encoding::types::EncodedContent;
use crate::request;
use crate::response;
use crate::response::builder::HttpResponseBuilder;
use crate::server;

use request::{HttpRequest, HttpRequestType};
use response::{HttpResponse, HttpResponseType};
use server::ServerContext;
use std::fs;
use std::fs::File;
use std::io::Write;

pub fn handle(request: HttpRequest) -> HttpResponse {
    let context: &ServerContext = request.context.as_ref();
    if context.host_files_path.is_none() {
        return HttpResponseBuilder::new(HttpResponseType::ServiceUnavailable).build();
    }

    if request.request_type == HttpRequestType::Get {
        return handle_get(request);
    } else if request.request_type == HttpRequestType::Post {
        return handle_post(request);
    }

    HttpResponseBuilder::new(HttpResponseType::NotFound).build()
}

fn handle_post(request: HttpRequest) -> HttpResponse {
    let context = request.context.as_ref();
    let filename = &request.path["/files/".len()..];
    let path = context.host_files_path.as_ref().unwrap().join(filename);
    if path.exists() {
        return HttpResponseBuilder::new(HttpResponseType::Conflict).build();
    }

    let file = File::create(path);
    match file.and_then(|mut file| file.write_all(request.body.as_slice())) {
        Ok(_) => HttpResponseBuilder::new(HttpResponseType::Created).build(),
        Err(err) => HttpResponseBuilder::new(HttpResponseType::InternalServerError)
            .body(EncodedContent::from(
                format!("Error when writing: {}", err).into_bytes(),
            ))
            .build(),
    }
}

fn handle_get(request: HttpRequest) -> HttpResponse {
    let context = request.context.as_ref();
    let filename = &request.path["/files/".len()..];
    let path = context.host_files_path.as_ref().unwrap().join(filename);
    if !path.exists() {
        return HttpResponseBuilder::new(HttpResponseType::NotFound).build();
    }

    match fs::read(path) {
        Ok(contents) => HttpResponseBuilder::new(HttpResponseType::Ok)
            .content_type(String::from("application/octet-stream"))
            .body(EncodedContent::from(contents))
            .build(),
        Err(err) => {
            return HttpResponseBuilder::new(HttpResponseType::InternalServerError)
                .body(EncodedContent::from(format!("Error: {}", err).into_bytes()))
                .build()
        }
    }
}
