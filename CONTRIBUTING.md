# Contributing

## Workspace Setup

- See rust version pinned in top-level `Cargo.toml`
- For most things, you can use the `bin` commands, but it's important to note that some of these commands rely on [Overmind](https://github.com/DarthSim/overmind)
  - You can use Foreman or some other Process manager, but the `bin` commands might not work seamlessly
- Unless you have advanced setup you're desiring, you can probably just run the `bin/easy_setup.sh` script and then run `bin/dev.sh` and you'll be good to go

## Client

The client is written in Yew.rs

### Setup

- Up to date verison of rust
- `cargo install trunk`, for running server
- Add WASM target
  - `rustup target add wasm32-unknown-unknown`

### Running dev server

- `trunk serve --open`

## Server

- The server is written in Actix

### Setup

- `cargo install cargo-watch`
