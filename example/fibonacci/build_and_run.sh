#!/bin/bash
cargo build --manifest-path ./fibonacci_rust/Cargo.toml --release
make -C ./fibonacci_c
go build -buildmode=c-shared -o ./fibonacci_go/fibonacci.dylib ./fibonacci_go/fibonacci.go
bundle exec ruby ./fibonacci.rb
