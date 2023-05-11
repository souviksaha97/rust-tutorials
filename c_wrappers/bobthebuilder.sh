#!/bin/sh
cargo clean
cargo build
cbindgen --config cbindgen.toml --crate rusty --output target/debug/rusty.h
cmake .
make