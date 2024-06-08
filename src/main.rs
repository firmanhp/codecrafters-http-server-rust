use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::net::{TcpListener, TcpStream};
use std::thread;

mod request;
use request::HttpRequest;

mod response;
use response::Response;

fn handle_client(mut stream: TcpStream) -> Result<()> {
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

    process(stream, &total_buffer.as_slice())
}

fn process(stream: TcpStream, buf: &[u8]) -> Result<()> {
    println!("buffer: {}", String::from_utf8_lossy(&buf));
    let request = HttpRequest::from_bytes(buf)?;
    println!("request: {}", request);

    if request.path.starts_with("/echo/") {
        return Response::respond_200(stream, &request.path["/echo/".len()..]);
    } else if request.path.starts_with("/user-agent") {
        return Response::respond_200(
            stream,
            request.header.user_agent.unwrap_or(String::new()).as_str(),
        );
    } else if &request.path == "/" {
        return Response::respond_200(stream, "");
    } else {
        return Response::respond_404(stream);
    }
}

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted new connection");
                thread::spawn(|| match handle_client(_stream) {
                    Err(e) => {
                        println!("Error in connection: {}", e);
                    }
                    Ok(()) => {}
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    Ok(())
}
