<<<<<<< HEAD:README.md
<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: Ferrous Systems GmbH -->

<br>
<p align="center">
    <img height="75" src="https://github.com/ferrocene/ferrocene/raw/main/ferrocene/logo.svg" alt="Ferrocene Logo: A circle between two planes, representing the chemical compound of the name Ferrocene, with the name written next to it.">
</p>
<br>

Ferrocene is a toolchain to enable the use of the Rust programming language in
safety-critical environments. It is a proper downstream of the main Rust
compiler - rustc, maintained by the Rust project on [`rust-lang/rust`].

The mission of Ferrocene is to bring open source practices to safety-critical
industries and improve the Rust open source ecosystem through safety-critical
practices.

Ferrocene is maintained and supported by the world-renowed experts at Ferrous
Systems. Both standard and long-term support are available. Check [our
website][website] for details.

## Current status

Ferrocene is currently in the final stages of qualification for ISO 26262
ASIL-D and IEC 61508 SiL4. Qualification for other standards and areas, such as
railway and aerospace, are planned.

## Installation

Prebuilt Ferrocene binaries are available for customers and partners. You can
visit [releases.ferrocene.dev] to download the release archives after logging
in with your Ferrocene account.

## Documentation and Procedures

The documentation of the Ferrocene toolchain and its source can be found
in the [`ferrocene/doc` directory]. The documentation contains the projects
procedures, its quality management measures and current testing coverage.

Rendered versions of the documentation are also available:
=======
# Installing from Source

**Note: This document describes _building_ Rust _from source_.
This is _not recommended_ if you don't know what you're doing.
If you just want to install Rust, check out the [README.md](README.md) instead.**
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:INSTALL.md

* [docs.ferrocene.dev]: available to customers and partners, contains the
  documentation for all release channels.
* [public-docs.ferrocene.dev/main]: publicly available, contains the
  documentation for the latest commit merged on the `main` branch.

## Support

Multiple levels of support are available for paying customers, provided by
[Ferrous Systems]. You can log into [customers.ferrocene.dev] to learn about
your support plan, and how to send support requests.

## Contribution policy

As a downstream of the Rust project, Ferrocene prefers to keep the compiler
unmodified. This means that general, contributions to the compiler or its tools
and discussions should be kept to [`rust-lang/rust`]. However, Ferrocene does
serve as a community of peers to propose and produce changes useful in
safety-critical changes towards the project.

Contributions to qualification activities and manuals are welcome, but
generally gated. Contribution is open to industry and academic partners,
customers and project employees.

<<<<<<< HEAD:README.md
You can use the [Ferrocene issue tracker][issues] to file an issue for the
materials provided by the Ferrocene developers. Please note that the issue
tracker is not a support channel.
=======
## Dependencies
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:INSTALL.md

Please note that Ferrocene is governed under the Apache-2.0 license and
contribution policies apply to the issue tracker as well as the codebase
itself.

## Additional services

[Ferrous Systems] provides services built and tailored around Ferrocene:

* **Trainings:** trainings on Rust and Ferrocene, particularly for teams, are
  available. Trainings can be custom tailored to your needs. [Check out our
  training offerings.][trainings]

* **Inclusion in SDKs:** Ferrocene is available to be integrated in your
  toolchain! Please [get in touch][contact] to learn more.

* **Tailoring, enabling and integration within your system**: We're more than
  happy to enable Rust support in your operating system or tool, including
  porting the Rust compiler to the targets you need and qualifying them in
  Ferrocene. [Get in touch][contact] to learn more.

* **Infrastructure support**: Ferrocene is built for a DevOps world. Rust
  for your builds in the cloud is a first-class citizen for us, and we can
  provide support tailored to you. [Get in touch][contact] for more
  information.

## Security

<<<<<<< HEAD:README.md
Please follow [Ferrocene's security policy][security] if you discover a
security vulnerability affecting Ferrocene.

## License and trademark
=======
## Building on a Unix-like system

### Build steps
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:INSTALL.md

The contents of the repository are primarily licensed under either the MIT or
Apache 2.0 license: users can choose either license, and contributors must
license their changes under both licenses. Note that the repository contains
files authored by third parties and published under different licenses, see the
annotations next to those files.

Ferrocene is a registered trademark of Critical Section GmbH, a subsidiary of
Ferrous Systems. See our [trademark policy][trademark] for the guidelines on
the use of the trademark.

[issues]: https://github.com/ferrocene/ferrocene/issues
[trainings]: https://ferrous-systems.com/training
[contact]: https://ferrous-systems.com/contact#ferrocene
[trademark]: ./TRADEMARK.md
[security]: https://github.com/ferrocene/ferrocene/security/policy
[website]: https://ferrous-systems.com/ferrocene

[Ferrous Systems]: https://ferrous-systems.com
[releases.ferrocene.dev]: https://releases.ferrocene.dev
[docs.ferrocene.dev]: https://docs.ferrocene.dev
[public-docs.ferrocene.dev/main]: https://public-docs.ferrocene.dev/main
[customers.ferrocene.dev]: https://customers.ferrocene.dev

<<<<<<< HEAD:README.md
[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`ferrocene/doc` directory]: ferrocene/doc/
=======
   ```sh
   ./configure
   ```

   If you plan to use `x.py install` to create an installation, it is
   recommended that you set the `prefix` value in the `[install]` section to a
   directory: `./configure --set install.prefix=<path>`

3. Build and install:

   ```sh
   ./x.py build && ./x.py install
   ```

   When complete, `./x.py install` will place several programs into
   `$PREFIX/bin`: `rustc`, the Rust compiler, and `rustdoc`, the
   API-documentation tool. By default, it will also include [Cargo], Rust's
   package manager. You can disable this behavior by passing
   `--set build.extended=false` to `./configure`.

[Cargo]: https://github.com/rust-lang/cargo

### Configure and Make

This project provides a configure script and makefile (the latter of which just
invokes `x.py`). `./configure` is the recommended way to programmatically
generate a `config.toml`. `make` is not recommended (we suggest using `x.py`
directly), but it is supported and we try not to break it unnecessarily.

```sh
./configure
make && sudo make install
```

`configure` generates a `config.toml` which can also be used with normal `x.py`
invocations.

## Building on Windows

On Windows, we suggest using [winget] to install dependencies by running the
following in a terminal:

```powershell
winget install -e Python.Python.3
winget install -e Kitware.CMake
winget install -e Git.Git
```

Then edit your system's `PATH` variable and add: `C:\Program Files\CMake\bin`.
See
[this guide on editing the system `PATH`](https://www.java.com/en/download/help/path.html)
from the Java documentation.

[winget]: https://github.com/microsoft/winget-cli

There are two prominent ABIs in use on Windows: the native (MSVC) ABI used by
Visual Studio and the GNU ABI used by the GCC toolchain. Which version of Rust
you need depends largely on what C/C++ libraries you want to interoperate with.
Use the MSVC build of Rust to interop with software produced by Visual Studio
and the GNU build to interop with GNU software built using the MinGW/MSYS2
toolchain.

### MinGW

[MSYS2][msys2] can be used to easily build Rust on Windows:

[msys2]: https://www.msys2.org/

1. Download the latest [MSYS2 installer][msys2] and go through the installer.

2. Run `mingw32_shell.bat` or `mingw64_shell.bat` from the MSYS2 installation
   directory (e.g. `C:\msys64`), depending on whether you want 32-bit or 64-bit
   Rust. (As of the latest version of MSYS2 you have to run `msys2_shell.cmd
   -mingw32` or `msys2_shell.cmd -mingw64` from the command line instead.)

3. From this terminal, install the required tools:

   ```sh
   # Update package mirrors (may be needed if you have a fresh install of MSYS2)
   pacman -Sy pacman-mirrors

   # Install build tools needed for Rust. If you're building a 32-bit compiler,
   # then replace "x86_64" below with "i686". If you've already got Git, Python,
   # or CMake installed and in PATH you can remove them from this list.
   # Note that it is important that you do **not** use the 'python2', 'cmake',
   # and 'ninja' packages from the 'msys2' subsystem.
   # The build has historically been known to fail with these packages.
   pacman -S git \
               make \
               diffutils \
               tar \
               mingw-w64-x86_64-python \
               mingw-w64-x86_64-cmake \
               mingw-w64-x86_64-gcc \
               mingw-w64-x86_64-ninja
   ```

4. Navigate to Rust's source code (or clone it), then build it:

   ```sh
   python x.py setup user && python x.py build && python x.py install
   ```

### MSVC

MSVC builds of Rust additionally require an installation of Visual Studio 2017
(or later) so `rustc` can use its linker. The simplest way is to get
[Visual Studio], check the "C++ build tools" and "Windows 10 SDK" workload.

[Visual Studio]: https://visualstudio.microsoft.com/downloads/

(If you're installing CMake yourself, be careful that "C++ CMake tools for
Windows" doesn't get included under "Individual components".)

With these dependencies installed, you can build the compiler in a `cmd.exe`
shell with:

```sh
python x.py setup user
python x.py build
```

Right now, building Rust only works with some known versions of Visual Studio.
If you have a more recent version installed and the build system doesn't
understand, you may need to force rustbuild to use an older version.
This can be done by manually calling the appropriate vcvars file before running
the bootstrap.

```batch
CALL "C:\Program Files (x86)\Microsoft Visual Studio\2019\Community\VC\Auxiliary\Build\vcvars64.bat"
python x.py build
```

### Specifying an ABI

Each specific ABI can also be used from either environment (for example, using
the GNU ABI in PowerShell) by using an explicit build triple. The available
Windows build triples are:
- GNU ABI (using GCC)
    - `i686-pc-windows-gnu`
    - `x86_64-pc-windows-gnu`
- The MSVC ABI
    - `i686-pc-windows-msvc`
    - `x86_64-pc-windows-msvc`

The build triple can be specified by either specifying `--build=<triple>` when
invoking `x.py` commands, or by creating a `config.toml` file (as described in
[Building on a Unix-like system](#building-on-a-unix-like-system)), and passing
`--set build.build=<triple>` to `./configure`.

## Building Documentation

If you'd like to build the documentation, it's almost the same:

```sh
./x.py doc
```

The generated documentation will appear under `doc` in the `build` directory for
the ABI used. That is, if the ABI was `x86_64-pc-windows-msvc`, the directory
will be `build\x86_64-pc-windows-msvc\doc`.

## Notes

Since the Rust compiler is written in Rust, it must be built by a precompiled
"snapshot" version of itself (made in an earlier stage of development).
As such, source builds require an Internet connection to fetch snapshots, and an
OS that can execute the available snapshot binaries.

See https://doc.rust-lang.org/nightly/rustc/platform-support.html for a list of
supported platforms.
Only "host tools" platforms have a pre-compiled snapshot binary available; to
compile for a platform without host tools you must cross-compile.

You may find that other platforms work, but these are our officially supported
build environments that are most likely to work.
>>>>>>> pull-upstream-temp--do-not-use-for-real-code:INSTALL.md
