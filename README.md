# Bevy Starter Project

A basic bevy project with common libs and code snippets, that builds to webassembly.

## Prerequisites

```bash
rustup target install wasm32-unknown-unknown +nightly
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
```

## Run locally

```bash
cargo run --target wasm32-unknown-unknown
```

## Deploy
Does not work for me on Firefox for some reason, but works on Chrome.

Build:
```bash
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-dir out/pkg --target web target/wasm32-unknown-unknown/release/bevy_starter_project.wasm
```

Deploy locally:
```bash
cd out
python3 -m http.server <port>
```
