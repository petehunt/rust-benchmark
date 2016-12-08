# Benchmark of native Rust, Rust compiled to JS and "native" JS

Unscientific, half incorrect unrepresentative benchmark of Rust compiled to JS.

## Setting up

Install rustup and run:

```
rustup install nightly
rustup override set nightly
rustup target add asmjs-unknown-emscripten
rustup target add wasm32-unknown-emscripten
```

And install emscripten:

```
curl -O https://s3.amazonaws.com/mozilla-games/emscripten/releases/emsdk-portable.tar.gz
tar -xzf emsdk-portable.tar.gz
source emsdk_portable/emsdk_env.sh
emsdk update
emsdk install sdk-incoming-64bit
emsdk activate sdk-incoming-64bit
```

## Build

Build native:

```
cargo build --release
```

Build asm.js:

```
cargo build --target=asmjs-unknown-emscripten --release
```

## Run the benchmarks

"Native" JS:
```
node sort-benchmark.js
```

Asm.js:
```
node target/asmjs-unknown-emscripten/release/sort-benchmark.js
```

True native:
```
target/release/sort-benchmark
```

## Results I got on my machine

```
[petehunt@petehunt-mbp sort-benchmark{master}]$ node sort-benchmark.js
Sorted 500000 ints in 142 ms
[petehunt@petehunt-mbp sort-benchmark{master}]$ node target/asmjs-unknown-emscripten/release/sort-benchmark.js
Sorted 500000 ints in 134 ms
[petehunt@petehunt-mbp sort-benchmark{master}]$ target/release/sort-benchmark
Sorted 500000 ints in 40 ms
```
