use std::{
    io::{BufReader, BufWriter, Write}, net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream}, str::FromStr
};

static HTTP_VERSION: &str = "HTTP/1.0";

pub mod request;
pub mod response;

use clap::Parser;
use request::HttpRequest;
use response::HttpResponse;

fn handle_request<W: Write>(request: HttpRequest, writer: W) -> std::io::Result<()> {
    // Print the request using the HttpRequest's `fmt` function
    println!("{request}");

    // Create a generic response as this assignment doesn't need more than that
    // currently.
    let response = HttpResponse {
        status: 200,
        body: "Hello world!".as_bytes().to_owned(),
        ..Default::default()
    };

    response.write(writer)?;
    Ok(())
}

/// Handle an incoming http client connection
fn handle_connection(stream: TcpStream) {
    // Get a copy of the stream so we can create separate read and write buffers
    let Ok(read_stream) = stream.try_clone() else {
        eprintln!("Error cloning stream for request. Skipping request.");
        return;
    };

    let reader = BufReader::new(read_stream);
    let writer = BufWriter::new(stream);

    // Try to read an http request from the reader
    match HttpRequest::read(reader) {
        Ok(request) => {
            if let Err(e) = handle_request(request, writer) {
                eprintln!("Error while processing request: {e:?}");
            }
        }
        Err(e) => eprintln!("Error while reading request: {e:?}"),
    }
}

// CLI Argument Parsing
#[derive(Parser)]
struct CliArgs {
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
}

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let addr =
        Ipv4Addr::from_str("127.0.0.1").expect("Listening address was not a valid ipv4 address");
    let port = args.port;
    let listener = TcpListener::bind(SocketAddrV4::new(addr, port))?;

    // Listen for new connections
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // If a TCP stream for the request was successfully created, spawn a thread to handle it
                std::thread::spawn(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to get client connection: {e:?}");
            }
        }
    }

    Ok(())
}
