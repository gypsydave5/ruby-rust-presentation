#!/bin/bash
cargo build --manifest-path ./fibonacci_rust/Cargo.toml --release
make -C ./fibonacci_c
bundle exec ruby ./fibonacci.rb
