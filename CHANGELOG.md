# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.76](https://github.com/rust-lang/backtrace-rs/compare/backtrace-v0.3.75...backtrace-v0.3.76) - 2025-09-26

### Other

- Expand `take_nhdr` doc-comment
- Fix missing alignment check for ELF note header in fuchsia support
- Upgrade `ruzstd` `object`, and `addr2line` to the latest version (rust-lang/backtrace-rs#718)
- Merge of rust-lang/backtrace-rs#705: Add `optimize(size)` to some particularly large functions
