use crate::request;
use crate::response;
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
        return HttpResponse::of(HttpResponseType::ServiceUnavailable);
    }

    if request.req_type == HttpRequestType::Get {
        return handle_get(request);
    } else if request.req_type == HttpRequestType::Post {
        return handle_post(request);
    }

    HttpResponse::of(HttpResponseType::NotFound)
}

fn handle_post(request: HttpRequest) -> HttpResponse {
    let context = request.context.as_ref();
    let filename = &request.path["/files/".len()..];
    let path = context.host_files_path.as_ref().unwrap().join(filename);
    if path.exists() {
        return HttpResponse::of(HttpResponseType::Conflict);
    }

    let file = File::create(path);
    match file.and_then(|mut file| file.write_all(request.body.as_slice())) {
        Ok(_) => HttpResponse::of(HttpResponseType::Created),
        Err(err) => HttpResponse::from_str(
            HttpResponseType::InternalServerError,
            format!("Error when writing: {}", err).as_str(),
        ),
    }
}

fn handle_get(request: HttpRequest) -> HttpResponse {
    let context = request.context.as_ref();
    let filename = &request.path["/files/".len()..];
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
