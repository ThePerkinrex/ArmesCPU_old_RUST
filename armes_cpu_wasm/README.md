To compile to wasm you need [`make`](https://www.gnu.org/software/make/) & [`wasm-pack`](https://rustwasm.github.io/wasm-pack/).
To test it you can use `python3`

Then, do `make build` to compile, `make serve` to start a `python3` web server on port `8000` & `make watch` to watch for changes and compile the wasm when changes are made