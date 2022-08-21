rust-cli-skeleton
===================

This is a simple CLI skeleton written in Rust with some basic code to run HTTP and gRPC servers.

```bash
$ cargo run help
    Finished dev [unoptimized + debuginfo] target(s) in 0.19s
     Running `target/debug/rust-cli-skeleton help`

rust-cli-skeleton 0.0.1
Vincent Composieux
A simple Rust CLI skeleton

USAGE:
    rust-cli-skeleton [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -v, --verbose    Enable verbose mode
    -V, --version    Print version information

SUBCOMMANDS:
    grpc:server    Runs the gRPC server.
    hello          This is an example command.
    help           Print this message or the help of the given subcommand(s)
    http:server    Runs the HTTP server.
```
