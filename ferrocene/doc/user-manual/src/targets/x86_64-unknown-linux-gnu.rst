.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-unknown-linux-gnu:

x86_64 Linux (glibc)
====================

The ``x86_64-unknown-linux-gnu`` Ferrocene target provides support for Linux 3.2
or higher on x86_64 using glibc 2.17 or higher.

Prerequisites
-------------

This target uses the LLVM ``ld.lld`` linker and is qualfied for use with the
version of ``ld.lld`` bundled with Ferrocene. To locate the system C libraries
required to link a functional Linux binary, this target drives the ``ld.lld``
linker using your system's C compiler as a linker-driver.

The ``ld.lld`` binary is a small wrapper around the LLVM ``rust-lld`` linker;
it's job is to provide an additional argument to put ``rust-lld`` into a GNU
``ld`` compatible mode.

You must run the Ferrocene Self Test (`ferrocene-self-test`) to determine
whether you have a system C compiler that supports using the Ferrocene
``ld.lld`` linker in place of your system's default linker (usually GNU ``ld``).
This means a C compiler which supports both the ``-fuse-ld=lld`` and ``-B/path``
options. Your C compiler must also be configured to not cause the GCC LTO
plugins (or any other plugins) to be loaded into the linker.

.. code-block::

   $ /path/to/ferrocene/bin/ferrocene-self-test

Ferrocene Self Test will try `cc`, `gcc` and then `clang` in that order whilst
searching for a suitable C compiler to act as a linker-driver. It will then
determine whether any plugins were loaded into the linker, and attempt to
identify a command-line option which will disable those plugins.

Finally it will report which, if any, compiler flags are required each target.

.. code-block::

   $ /path/to/ferrocene/bin/ferrocene-self-test
       Info: using sysroot /path/to/ferrocene
    Success: binary rustc is valid
    Success: binary rustdoc is valid
    Skipped: optional binary cargo (not present)
    Success: target installed correctly: x86_64-unknown-linux-gnu
    Success: target installed correctly: aarch64-unknown-none
    Success: bundled linker detected
    Success: bundled linker-wrapper detected
    Success: Found C compiler `cc` for target `x86_64-unknown-linux-gnu`
    Skipped: Target `aarch64-unknown-none` does not require a C compiler
    Success: compiled sample program `addition.rs` for target x86_64-unknown-linux-gnu
    Success: compiled sample program `subtraction.rs` for target x86_64-unknown-linux-gnu
    Success: compiled sample program `subtraction-sys.rs` for target x86_64-unknown-linux-gnu
    Success: compiled sample program `assertion.rs` for target x86_64-unknown-linux-gnu
    Success: compiled sample program `addition.rs` for target aarch64-unknown-linux-gnu
       Info: Target 'x86_64-unknown-linux-gnu' requires special linker flags:
       Info: 	-Clinker=cc
       Info: 	-Clink-arg=-fno-lto
       Info: Target 'aarch64-unknown-none' requires no special linker flags
    Success: Ferrocene self-check completed!

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=x86_64-unknown-linux-gnu``

In addition, you must also supply to ``rustc`` any additional flags identified
by ``ferrocene-self-test``.
