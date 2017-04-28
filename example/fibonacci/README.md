# Rust For Rubyists

## Example: Calling Rust from Ruby

This example demonstrates the performance gain of calling Rust code from Ruby.
It uses the `Fiddle` library to import a function from dylib file, which is
built within a Cargo project. It runs a benchmark of the imported function and
compares it with the same implementation in Ruby and C.

The function being compared generates the nth Fibonacci number.

### To run

`./build_and_run.sh`

### Dependencies
- OSX
- Rust
- Ruby
- Cargo