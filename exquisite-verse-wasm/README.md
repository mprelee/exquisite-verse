# Exquisite Verse WASM

This crate provides a WebAssembly (WASM) implementation of the Exquisite Verse application.

## Prerequisites

- Rust and Cargo
- wasm-pack (`cargo install wasm-pack`)
- A web browser
- Python 3 (for the development server)

## Building and Running

1. Make sure you're in the `exquisite-verse-wasm` directory
2. Run the build script:
   ```
   ./build.sh
   ```
3. Open your browser and navigate to `http://localhost:8000`

## Development

For development, you can use cargo-watch to automatically rebuild when files change:

```
cargo watch -x 'run --target wasm32-unknown-unknown'
```

## Project Structure

- `src/lib.rs`: The main entry point for the WASM application
- `index.html`: The HTML file that loads the WASM application
- `build.sh`: Script to build and serve the application

## Troubleshooting

If you encounter issues with the build:

1. Make sure wasm-pack is installed: `cargo install wasm-pack`
2. Check that you're using a compatible version of Rust
3. Ensure all dependencies are up to date: `cargo update` 