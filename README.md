# WebAssembly
Code for the examples from the WebAssembly talk given at the Webmontag Meetup in Kassel.

## Requirements

- Rust Installation (https://www.rust-lang.org/learn/get-started)
- Node.js installation (https://nodejs.org)
- Simple HTTP server installation (e.g. https://www.npmjs.com/package/http-server)
- WASM runtime installation (e.g. https://github.com/wasm3/wasm3)

## Getting started

- Clone this repo.
- Open a Terminal at the position where you've cloned the repo.
- If not already done, install the Rust compiler's WASM target: `rustup target add wasm32-unknown-unknown`

## Run the JS-API example

- Go to the `js-api` folder
- Run `cargo build`
- Run `http-server`
- In a browser open http://localhost:8080
- Output should be similar to this:

![Bildschirmfoto 2023-11-21 um 18 43 35](https://github.com/jens-siebert/webassembly/assets/143868460/621508cc-cf31-4721-b176-fb45a7e86cc1)

## Run the wasm-pack web example

- Go to the `wasm-pack` folder
- If not already done, run `cargo install wasm-pack`
- Run `wasm-pack build --target web`
- Run `http-server`
- In a browser open http://localhost:8080
- Output should be similar to this:

![Bildschirmfoto 2023-11-21 um 19 18 31](https://github.com/jens-siebert/webassembly/assets/143868460/87e9623e-2c62-4359-af56-6e41549d4f9d)

## Run the wasm-pack node.js example

- Go to the `wasm-pack` folder
- If not already done, run `cargo install wasm-pack`
- Run `wasm-pack build --target nodejs`
- Run `node index.js`
- Output should be similar to this:

![Bildschirmfoto 2023-11-22 um 18 28 06](https://github.com/jens-siebert/webassembly/assets/143868460/43405ab2-df44-4ea3-b67c-fe5d093d31a2)

## Run the WASM runtime example

- Go to the `js-api` folder
- Run `cargo build`
- Run `wasm3 --func factorial target/wasm32-unknown-unknown/debug/js_api.wasm 10`
- Output should be similar to this:

![Bildschirmfoto 2023-11-22 um 18 34 11](https://github.com/jens-siebert/webassembly/assets/143868460/a5fe40b1-4cbf-4126-b493-861fc3d15df6)
