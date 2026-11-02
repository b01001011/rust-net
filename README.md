# rust-net

A network library written in Rust.

## Prerequisites
```bash
brew install protobuf
```

## Examples

The repository contains several network programming examples.

## Run an example

To run the Discovery query example:

```bash
RUST_LOG=trace cargo run -p examples --example discovery query _my_app._udp
```

Replace `stun` with the name of another example to run it.
