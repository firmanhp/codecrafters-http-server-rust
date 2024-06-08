mod request;
mod response;
mod routing;

use std::io::Result;
use std::net::TcpListener;
use std::thread;

use routing::handle_connection;

fn main() -> Result<()> {
    println!("Logs from your program will appear here!");

    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted new connection");
                thread::spawn(|| match handle_connection(_stream) {
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
