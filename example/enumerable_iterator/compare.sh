#!/bin/bash -e
cargo build --manifest-path ./rust_fib_stream/Cargo.toml --release --quiet
printf "Ruby says: %d\n" $(ruby ./ruby_fib_stream/enumberable.rb)
printf "Rust says: %d\n" $(./rust_fib_stream/target/release/rust_fib_stream)