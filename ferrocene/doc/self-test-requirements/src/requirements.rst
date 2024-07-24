.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

General checks
==============

Checks for executables
----------------------

.. note::

   **executable** refers to the following:

   - ``rustc``
   - ``rustdoc``

The executable should exist in the ``bin`` directory, relative to the root of the installation.

The metadata of the executable should be fetchable.

The executable should be a regular file.

The executable should have read and execute permissions set for all users.

When executed with ``-vV`` command-line options,
the executable should exit successfully.

When executed with ``-vV`` command-line options,
the output of the executable should be UTF-8 text.

When executed with ``-vV`` command-line options,
the output of the executable should have at least 3 values: host, commit-hash, and release.

When executed with ``-vV`` command-line options,
the value of ``host`` from the output of the executable should match the target-triple
of the executing environment.

When executed with ``-vV`` command-line options,
the value of ``commit-hash`` from the output of the executable should match the 
commit hash of the Ferrocene release corresponding to the installed Ferrocene toolchain.

When executed with ``-vV`` command-line options,
the value of ``release`` from the output of the executable should match the
version of the Ferrocene release corresponding to the installed Ferrocene toolchain.

Checks for targets
------------------

.. note::

   **target** refers to the following:

   - ``x86_64-unknown-linux-gnu`` triple, which refers to _x86-64 Linux (glibc)_
   - ``aarch64-unknown-none`` triple, which refers to *Armv8-A bare-metal*

x86_64-unknown-linux-gnu
^^^^^^^^^^^^^^^^^^^^^^^^

Inside of ``lib/rustlib/x86_64-unknown-linux-gnu/lib`` directory,
relative to the root of the installation,
should exist these regular files,
where ``$hash`` is a 16 character hexadecimal string:

- ``libcore-$hash.rlib``
- ``liballoc-$hash.rlib``
- ``libstd-$hash.rlib``
- ``libtest-$hash.rlib``
- ``libproc_macro-$hash.rlib``

.. note:: Other files in that directory are ignored.

Inside of ``lib/rustlib/x86_64-unknown-linux-gnu/lib`` directory,
relative to the root of the installation,
should not exist duplicates of these regular files,
where ``$hash`` is a 16 character hexadecimal string:

- ``libcore-$hash.rlib``
- ``liballoc-$hash.rlib``
- ``libstd-$hash.rlib``
- ``libtest-$hash.rlib``
- ``libproc_macro-$hash.rlib``

.. note:: Other files in that directory are ignored.

aarch64-unknown-none
^^^^^^^^^^^^^^^^^^^^

Inside of ``lib/rustlib/aarch64-unknown-none/lib`` directory,
relative to the root of the installation,
should exist these regular files,
where ``$hash`` is a 16 character hexadecimal string:

- ``libcore-$hash.rlib``
- ``liballoc-$hash.rlib``

Inside of ``lib/rustlib/aarch64-unknown-none/lib`` directory,
relative to the root of the installation,
should not exist duplicates of these regular files,
where ``$hash`` is a 16 character hexadecimal string:

- ``libcore-$hash.rlib``
- ``liballoc-$hash.rlib``

Checks for linkers
------------------

x86_64-unknown-linux-gnu
^^^^^^^^^^^^^^^^^^^^^^^^

Check if we can create temporary directories.

Inside of ``lib/rustlib/x86_64-unknown-linux-gnu/bin`` directory,
relative to the root of the installation,
should exist the regular file named ``rust-lld``.

Inside of ``lib/rustlib/x86_64-unknown-linux-gnu/bin/gcc-ld`` directory,
relative to the root of the installation,
should exist the regular file named ``ld.lld``,
which is the linker wrapper.

Search for a system C compiler in the ``PATH`` environment variable.

Use the system C compiler to compile a sample program,
and use the linker wrapper for the linking stage.

Check that the system C compiler passes ``-Wl,$arg`` arguments to the linker,
where ``$arg`` is command line arguments that the system linker accepts.

Checks for compilation
----------------------

Check if we can create temporary directories.

Check if we can compile the following Rust crate types:

- ``lib``
- ``staticlib``
- ``bin``

Check that *only* the following artefacts are produced by ``rustc`` for each crate type compilation,
where ``$basename`` is the file name without the extension:

- ``$basename.rlib`` for ``lib`` crate type
- ``$basename.a`` for ``staticlib`` crate type
- ``$basename`` for ``bin`` crate type

x86_64-unknown-linux-gnu
^^^^^^^^^^^^^^^^^^^^^^^^

Check if a sample program that ``rustc`` can be executed on the host platform.

Check if the output of the program is as expected.
