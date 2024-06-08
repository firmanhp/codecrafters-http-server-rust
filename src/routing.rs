mod files;

use crate::request;
use crate::response;
use crate::server;

use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::net::TcpStream;
use std::sync::Arc;

use request::HttpRequest;
use response::{HttpResponse, HttpResponseType};
use server::ServerContext;

fn route(request: HttpRequest) -> HttpResponse {
    if request.path.starts_with("/files/") {
        return files::handle(request);
    } else if request.path.starts_with("/echo/") {
        return HttpResponse::from_str(HttpResponseType::Ok, &request.path["/echo/".len()..]);
    } else if request.path.starts_with("/user-agent") {
        return HttpResponse::from_str(
            HttpResponseType::Ok,
            request.header.user_agent.unwrap_or(String::new()).as_str(),
        );
    } else if &request.path == "/" {
        return HttpResponse::from_str(HttpResponseType::Ok, "");
    } else {
        return HttpResponse::of(HttpResponseType::NotFound);
    }
}

fn get_request_buffer(mut stream: &TcpStream) -> Result<Vec<u8>> {
    let mut total_buffer: Vec<u8> = Vec::new();
    let mut ask_for_byte: bool = true;
    let stop_condition: &[u8] = b"\r\n\r\n";
    while ask_for_byte {
        let mut single_byte: [u8; 1] = [0];
        let result = stream.read(&mut single_byte);
        if let Err(error) = result {
            match error.kind() {
                ErrorKind::Interrupted => continue,
                _ => return Err(error),
            }
        }
        if let Ok(n) = result {
            if n == 0 {
                continue;
            }
        }

        total_buffer.push(single_byte[0]);
        // Check for stop condition.
        let needs_to_stop: bool = single_byte[0] == stop_condition[stop_condition.len() - 1]
            && total_buffer.len() >= stop_condition.len()
            && &total_buffer[(total_buffer.len() - stop_condition.len())..] == stop_condition;
        ask_for_byte = !needs_to_stop;
        // println!("current buffer: {}", String::from_utf8_lossy(total_buffer.as_slice()));
    }

    Ok(total_buffer)
}

pub fn handle_connection(stream: TcpStream, server_context: Arc<ServerContext>) -> Result<()> {
    let buf = get_request_buffer(&stream)?;
    println!("buffer: {}", String::from_utf8_lossy(&buf));

    let request = HttpRequest::from_bytes(&buf, server_context)?;
    println!("request: {}", request);

    let response = route(request);
    HttpResponse::respond(stream, &response)
}
