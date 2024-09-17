# Simple HTTP 1.0 Webserver
> u1F98E

## Building & Running
This program requires the Rust toolchain, you can install it here: https://rustup.rs

After installing, navigate to the project directory in a terminal and run:

```
cargo build
```

The only dependency this project has is on a cli argument parsing library (clap).
It will be automatically downloaded and built when using `cargo build`.

To start the server using the default port `8080`, use the following:

```
# Run server
cargo run

# To pass command line arguments with cargo, separate arguments with `--`
cargo run -- --help
cargo run -- --port 8686 --root www

# Or run the executable directly
./target/debug/simple-http-server --port 8686 --root www
```

## Usage
```
Usage: simple-http-server [OPTIONS]

Options:
  -p, --port <PORT>  The port the application should listen on [default: 8080]
      --root <ROOT>  The directory the server will serve files from. Defaults to the current directory [default: .]
  -h, --help         Print help
```
