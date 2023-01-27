# libcore statistics (January 2023)

This repository contains the code used in January 2023 to extract statistics
from libcore. It works by walking through the rustdoc JSON output and emitting
TSV files (tab-separated CSV).

To run the tool, execute:

```
cargo run --release path/to/core.json path/to/output/dir/ path/to/rust/checkout/
```

## Obtaining the rustdoc JSON representation

To generate the `core.json` file, you need to clone the [rust-lang/rust]
repository, and add this snippet to the `config.toml`:

```toml
[build]
library-docs-private-items = true
```

> **Note:** you will need to be in a branch containing [#107264].

Then, you can generate the documentation by running:

```
./x doc --json library/core
```

The resulting documentation will be located in:

```
build/$target/json-doc/core.json
```

[rust-lang/rust]: https://github.com/rust-lang/rust
[#107264]: https://github.com/rust-lang/rust/pull/107264
