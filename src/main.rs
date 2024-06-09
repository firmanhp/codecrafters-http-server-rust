mod request;
mod response;
mod routing;
mod server;
mod encoding;

use std::io::Result;
use std::net::TcpListener;
use std::path::PathBuf;
use std::sync::Arc;
use std::thread;

use clap::Parser;

use routing::handle_connection;
use server::ServerContext;

/// Simple HTTP server based on codecrafters.io project.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Directory to host files.
    #[arg(short, long)]
    directory: Option<String>,
}

fn make_server_context() -> ServerContext {
    let args = Args::parse();
    println!("directory: {:?}", args.directory);
    ServerContext {
        host_files_path: args.directory.and_then(|s| Some(PathBuf::from(s))),
    }
}

fn main() -> Result<()> {
    let server_context = Arc::new(make_server_context());
    let listener: TcpListener = TcpListener::bind("127.0.0.1:4221").unwrap();

    println!("Logs from your program will appear here!");
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted new connection");
                let context = Arc::clone(&server_context);
                thread::spawn(move || match handle_connection(_stream, context) {
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
