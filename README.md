# Game Of Life implementation!

This project was done as an introduction to Rust & WebAssembly (WASM). It is heavily inspired from the Let's code series from [yishn](https://www.youtube.com/playlist?list=PLtTT8p-gjGEdGzZ0ET2bwNnA6iP_mmmrv)

The GameOfLife is a struct that contains only the width, height and alive_cells. It implements the constructor and a tick function that serves as next-state generator based on the rules

# How to use

You will need to:

- Clone the project
- Build the project with cargo build
- Use wasm-pack to build (`wasm-pack build --target web`)
- Serve it (I used `npx http-server` to serve it easily)

# Possible Improvements

- Expose the parameters (width & height & fill percentage) and
- [x] implement a START/STOP mechanism instead of ticking indefinitely
- [x] Add a way to "place" your own alive cells
