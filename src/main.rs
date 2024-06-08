use std::io::ErrorKind;
use std::io::Read;
use std::io::Result;
use std::io::Write;
use std::net::{TcpListener, TcpStream};

mod request;
use request::HttpRequest;

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

fn process(mut stream: TcpStream, buf: &[u8]) -> Result<()> {
    println!("buffer: {}", String::from_utf8_lossy(&buf));
    let request = HttpRequest::from_bytes(buf)?;
    println!("request: {}", request);

    if &request.path == "/" {
        stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
    } else {
        stream.write(b"HTTP/1.1 404 Not Found\r\n\r\n")?;
    }
    return Ok(());
}

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_client(_stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
    Ok(())
}
