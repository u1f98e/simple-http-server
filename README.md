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

To start the server using the default port `8080`, use:

```
cargo run
```

Or, pass additional arguments to the program with `--`:

```
# Use `--` to pass additional arguments with cargo
cargo run -- --help
cargo run -- --port 8686

# Or run the executable directly
./target/debug/simple-http-server --port 8686
```
