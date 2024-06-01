use std::net::{TcpListener, TcpStream};
use std::io::Write;
use std::io::Result;

fn handle_client(mut stream: TcpStream) -> Result<()> {
    stream.write(b"HTTP/1.1 200 OK\r\n\r\n")?;
    Ok(())
}


fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
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
