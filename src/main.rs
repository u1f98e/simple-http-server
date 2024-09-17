use std::{
    collections::HashMap,
    fs,
    io::{BufReader, BufWriter, Write},
    net::{Ipv4Addr, SocketAddrV4, TcpListener, TcpStream},
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};

static HTTP_VERSION: &str = "HTTP/1.0";
static MAX_REQUEST_BODY_SIZE: usize = 2_000_000; // 2 MB

pub mod error;
pub mod request;
pub mod response;
pub mod status;

use clap::Parser;
use error::{HttpError, HttpResult};
use request::{HttpRequest, RequestMethod};
use response::HttpResponse;
use status::HttpStatus;

struct ServerContext {
    root: PathBuf,
}

impl ServerContext {
    fn root_dir(&self) -> &PathBuf {
        &self.root
    }
}

fn serve_file(ctx: &ServerContext, request: HttpRequest) -> HttpResult<HttpResponse> {
    let uri = if request.uri() == "/" {
        Path::new("index.html")
    } else {
        Path::new(request.uri())
    };

    // Strip the uri of any leading '/' as .join() will treat absolute
    // paths as a replacement for the source path.
    let stripped_uri = uri.strip_prefix("/").unwrap_or(uri);
    // Make the uri relative to the served directory.
    let path = ctx.root_dir().join(stripped_uri);
    if !fs::exists(&path)? {
        return Err(HttpError::NotFound(request.uri().to_string()));
    }

    let body = fs::read(&path)?;

    Ok(HttpResponse {
        status: HttpStatus::Ok,
        headers: HashMap::new(),
        body,
    })
}

/// Handle an http request
fn handle_request<W: Write>(
    ctx: &ServerContext,
    request: HttpRequest,
    writer: W,
) -> std::io::Result<()> {
    // Print the request using the HttpRequest's `fmt` function
    println!("{request}");

    // Currently, we only support 
    let result = match request.method() {
        RequestMethod::Get => serve_file(ctx, request),
        RequestMethod::Head => Err(HttpError::NotImplemented),
        RequestMethod::Post => Err(HttpError::NotImplemented)
    };

    let response = match result {
        Ok(response) => response,
        Err(err) => err.into(),
    };

    response.write(writer)?;
    Ok(())
}

/// Handle an incoming http client connection
fn handle_connection(ctx: &ServerContext, stream: TcpStream) {
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
            if let Err(e) = handle_request(ctx, request, writer) {
                eprintln!("Error while processing request: {e:?}");
            }
        }
        Err(e) => {
            // An error occured while parsing the request, report an error
            // to the client.
            eprintln!("Error while reading request: {e:?}");
            let err_response: HttpResponse = e.into();
            err_response.write(writer).unwrap_or_else(|e| {
                eprintln!("Failed to write error response for previous error: {e:?}")
            })
        }
    }
}

// CLI Argument Parsing
#[derive(Parser)]
struct CliArgs {
    /// The port the application should listen on.
    #[arg(short, long, default_value_t = 8080)]
    port: u16,

    /// The directory the server will serve files from. Defaults to
    /// the current directory.
    #[arg(long, default_value = ".")]
    root: PathBuf,
}

fn main() -> std::io::Result<()> {
    let args = CliArgs::parse();

    let addr =
        Ipv4Addr::from_str("127.0.0.1").expect("Listening address was not a valid ipv4 address");
    let port = args.port;
    let listener = TcpListener::bind(SocketAddrV4::new(addr, port))?;

    let ctx = Arc::new(ServerContext { root: args.root });

    // Listen for new connections
    eprintln!("Listening on port {port}.");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // If a TCP stream for the request was successfully created, spawn a thread to handle it
                let ctx = ctx.clone();
                std::thread::spawn(move || {
                    handle_connection(&ctx, stream);
                });
            }
            Err(e) => {
                eprintln!("Failed to get client connection: {e:?}");
            }
        }
    }

    Ok(())
}
