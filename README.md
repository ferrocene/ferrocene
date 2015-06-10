# backtrace-rs

A library for acquiring backtraces at runtime for Rust. This library aims to
enhance the support given by the standard library at `std::rt` by providing a
more stable and programmatic interface.

## Install

```toml
[dependencies]
backtrace = "0.1"
```

```rust
extern crate backtrace;
```

## Usage

```rust
extern crate backtrace;

fn main() {
    backtrace::trace(&mut |frame| {
        let ip = frame.ip();
        let symbol_address = frame.symbol_address();

        // Resolve this instruction pointer to a symbol name
        backtrace::resolve(ip, &mut |symbol| {
            if let Some(name) = symbol.name() {
                // ...
            }
            if let Some(filename) = symbol.filename() {
                // ...
            }
        });

        true // keep going to the next frame
    });
}
```
