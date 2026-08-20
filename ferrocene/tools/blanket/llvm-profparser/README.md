# llvm-profparser

[![Build Status](https://github.com/xd009642/llvm-profparser/workflows/Build/badge.svg)](https://github.com/xd009642/llvm-profparser/actions)
[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Coverage Status](https://coveralls.io/repos/github/xd009642/llvm-profparser/badge.svg?branch=master)](https://coveralls.io/github/xd009642/llvm-profparser?branch=master)

This is a reasonably complete to parse the llvm instrumentation profraw file
format and avoid the need to install and use the llvm-profdata/llvm-cov
binaries. It aims to be backwards compatible with as many llvm versions that
could be used for coverage data in Rust projects and currently supports the
following llvm versions: 11-21. Support for LLVM 11 is only partial.

This parser supports the "instrumentation" profile format, but not the
"sampling" or "memory access" profile format. For those, use `llvm-profdata`
directly.

**This project is not affilated with the llvm-project in anyway!** It is merely
a parser for some of their file formats to aid interoperability in Rust.

## Contributing

All of the functionality required has been implemented, however there are areas
to improve in handling unexpected or invalid files. To start finding issues
there's a fuzz directory which will undoubtedly reveal some issues that can be
fixed. Go into the fuzz directory for guides on how to run. 

## License

llvm\_profparser is currently licensed under the terms of the Apache License
(Version 2.0). See LICENSE for details. Test data included from the llvm-project
residing in `tests/data` retains the llvm license. See the llvm-project for 
details. 
