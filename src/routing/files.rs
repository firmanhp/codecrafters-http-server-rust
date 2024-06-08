use crate::request;
use crate::response;
use crate::server;

use request::HttpRequest;
use response::{HttpResponse, HttpResponseType};
use server::ServerContext;
use std::fs;

pub fn handle(request: HttpRequest) -> HttpResponse {
    let context: &ServerContext = request.context.as_ref();
    if context.host_files_path.is_none() {
        return HttpResponse::of(HttpResponseType::NotFound);
    }

    let filename: &str = &request.path["/files/".len()..];
    let path = context.host_files_path.as_ref().unwrap().join(filename);
    if !path.exists() {
        return HttpResponse::of(HttpResponseType::NotFound);
    }

    match fs::read(path) {
        Ok(contents) => {
            return HttpResponse::from_bytes(HttpResponseType::Ok, contents.as_slice());
        }
        Err(err) => {
            return HttpResponse::from_str(
                HttpResponseType::InternalServerError,
                format!("Error: {}", err).as_str(),
            )
        }
    }
}
