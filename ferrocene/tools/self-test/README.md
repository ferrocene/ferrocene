> [!WARNING]
> This file just acts as a todo-list while tracing the requirements. It should be rmeoved before merging to main.

# Requirements

## General

- [ ] REQ_LSGKACM: The tool is used to ensure that the Ferrocene toolchain is installed correctly.
- [ ] REQ_THXJWWD: The tool should be executed on any of the host platforms listed in Compilation targets overview.
- [x] REQ_67EQEVF: The tool should only execute with success if it was distributed with the same Ferrocene toolchain that it is checking.
- [ ] REQ_P2OXII1: The tool should be executed on the command line.
- [x] REQ_9A6740M: The tool accepts no command line arguments, and quits with a fail exit status when they are present.
- [ ] REQ_42QU5MK: The tool displays the checks it performs, and quits with a fail exit status when any of the checks fails.

## Checks for executables

- [ ] REQ_0OAUY3U: These executables should be checked: rustc, rustdoc, cargo (if installed)
    - [x] REQ_N1VBW46: The executable should exist in the bin directory, relative to the root of the installation.
    - [x] REQ_R2UQ8D3: The executable should be a regular file.
    - [x] REQ_NUP1G0D: The executable should have read and execute permissions set for all users.
    - [ ] REQ_GVLWOTQ: Check the behavior of --verbose --version command-line options.
        - [x] REQ_6OAFM70: The executable should exit successfully.
        - [x] REQ_ABPRHHQ: The output of the executable should be UTF-8 text.
        - [x] REQ_SL5USTK: The output of the executable should have at least 3 key-value pairs, with these 3 keys: host, commit-hash, and release [...]

## Checks for targets

### All targets

- [ ] REQ_0640QY8: Inside of lib/rustlib/$target/lib directory, relative to the root of the installation, these regular files should be checked: libcore-$hash.rlib, liballoc-$hash.rlib
    - [ ] The files should exist.
    - [ ] The files should not have duplicates, which can happen if the $hash is different.

### Targets with std

- [ ] REQ_RUCUMJJ: Inside of lib/rustlib/$target/lib directory, relative to the root of the installation, these regular files should be checked: libstd-$hash.rlib, libtest-$hash.rlib, libproc_macro-$hash.rlib
    - [ ] REQ_GAPK9QF: The files should exist.
    - [ ] REQ_IJN9ZPU: The files should not have duplicates, which can happen if the $hash is different.

## Checks for linkers

- [ ] REQ_QQDV24N: Inside of lib/rustlib/$target/bin directory, relative to the root of the installation, should exist the regular file named rust-lld.
- [ ] REQ_J42HAPX: Inside of lib/rustlib/$target/bin/gcc-ld directory, relative to the root of the installation, should exist the regular file named ld.lld, which is the linker wrapper.

### Platforms that need a C compiler

- [ ] REQ_GR1AK1Q: Search for a system C compiler in the PATH environment variable.
- [ ] REQ_FCE5QJ5: Use the system C compiler to compile a sample program, and use the linker wrapper for the linking stage.
- [ ] REQ_5Q3NRL3: Check that the system C compiler passes -Wl,$arg arguments to the linker, where $arg is command line arguments that the system linker accepts.
- [ ] REQ_1MN4JOQ: Ensure that the linker command line arguments that can be accepted are of the form documented in the Safety Manual.

## Checks for compilation

### All targets

- [ ] REQ_99TXVWC: Check if we can compile the following Rust crate types: lib, staticlib, bin
- [ ] REQ_SV3CV3N: Check that only the following artefacts are produced by rustc for each crate type compilation, where $basename is the file name without the extension: [...]

### Host targets

- [ ] REQ_8TNOYG8: Check if a sample program that rustc produced can be executed.
- [ ] REQ_B07M5S2: Check if the output of the program is as expected.


# TODO

- [ ] Make NOTEs into requirements, e.g. "The following checks apply to these Host platforms: x86-64 Linux (glibc), Armv8-A Linux (glibc)"
- [ ] "Test" General requirements with a test that just contains a comment that explains why the requirement is covered?
- [ ] Only use "target" or "platform", but do not use it interchangeably
- [ ] Are we allowed to have tests that are not traced to any requirement?
- [ ] What about "negative tests", tests that test the opposite of a requirement does not work?
- [ ] Permission tests for windows
- [ ] Should the requirements macro go before or after the test macro? Before or after the cfg(windows) macro?
