# Bevy Starter Project

A basic bevy project with common libs and code snippets, that builds to webassembly.

## Prerequisites

```bash
rustup target install wasm32-unknown-unknown +nightly
cargo install wasm-server-runner
cargo install wasm-bindgen-cli
```

To manage itch.io uploads from the command line, install [butler](https://itch.io/docs/butler/installing.html).

## Run locally

```bash
cargo run
```

## Deploy
Does not work for me on Firefox for some reason, but works on Chrome.

### Build
```bash
cargo build --release
wasm-bindgen --out-dir out/pkg --target web target/wasm32-unknown-unknown/release/bevy_starter_project.wasm
rsync -a assets/ out/assets/
```

### Deploy locally
Build, then run this:
```bash
cd out
python3 -m http.server <port>
```

### Deploy to [itch.io](https://itch.io)
Build, then run this:
```bash
butler login # follow the instructions
zip -r bevy-starter-project.zip out
butler push bevy-starter-project.zip <user>/bevy-starter-project:html
```
If this is the first time uploading to the html channel,
you need to go to the game page on itch.io and click on _Edit game_,
then set it to playable in the browser.
