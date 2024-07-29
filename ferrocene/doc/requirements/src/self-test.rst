.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

``ferrocene-self-test`` tool
============================

General checks
--------------

Checks for executables
^^^^^^^^^^^^^^^^^^^^^^

.. note::

   **executable** refers to the following:

   - ``rustc``
   - ``rustdoc``

The executable should exist in the ``bin`` directory, relative to the root of the installation.

The metadata of the executable should be fetchable.

The executable should be a regular file.

The executable should have read and execute permissions set for all users.

When executed with ``-vV`` command-line options, the following apply:

- The executable should exit successfully.

- The output of the executable should be UTF-8 text.

- The output of the executable should have at least 3 values: host, commit-hash, and release.

- The value of ``host`` from the output of the executable should match the target-triple
  of the executing environment.

- The value of ``commit-hash`` from the output of the executable should match the 
  commit hash of the Ferrocene release corresponding to the installed Ferrocene toolchain.

- The value of ``release`` from the output of the executable should match the
  version of the Ferrocene release corresponding to the installed Ferrocene toolchain.

Checks for targets
------------------

.. list-table::
   :header-rows: 1

   * - Target
     - Has a standard library (``std``)
   * - :target-with-triple:`x86_64-unknown-linux-gnu`
     - ✔
   * - :target-with-triple:`aarch64-unknown-none`
     -
   * - :target-with-triple:`x86_64-pc-windows-msvc`
     - ✔
   * - :target-with-triple:`aarch64-unknown-linux-gnu`
     - ✔
   * - :target-with-triple:`aarch64-apple-darwin`
     - ✔
   * - :target-with-triple:`x86_64-apple-darwin`
     - ✔
   * - :target-with-triple:`aarch64-unknown-nto-qnx710`
     - ✔
   * - :target-with-triple:`x86_64-pc-nto-qnx710`
     - ✔
   * - :target-with-triple:`armv8r-none-eabihf`
     -
   * - :target-with-triple:`thumbv7em-none-eabi`
     -
   * - :target-with-triple:`thumbv7em-none-eabihf`
     -
   * - :target-with-triple:`armv7r-none-eabihf`
     -
   * - :target-with-triple:`armebv7r-none-eabihf`
     -

.. note::

   In the subsections below, we have these substitutions:

   - ``$target`` refers to any of the targets listed
   - ``$hash`` is a 16 character hexadecimal string

All targets
^^^^^^^^^^^

- Inside of ``lib/rustlib/$target/lib`` directory,
  relative to the root of the installation,
  should exist these regular files:

  - ``libcore-$hash.rlib``
  - ``liballoc-$hash.rlib``

- Inside of ``lib/rustlib/$target/lib`` directory,
  relative to the root of the installation,
  should not exist duplicates of these regular files:

  - ``libcore-$hash.rlib``
  - ``liballoc-$hash.rlib``

Targets with ``std``
^^^^^^^^^^^^^^^^^^^^

- Inside of ``lib/rustlib/$target/lib`` directory,
  relative to the root of the installation,
  should exist these regular files:

  - ``libstd-$hash.rlib``
  - ``libtest-$hash.rlib``
  - ``libproc_macro-$hash.rlib``

- Inside of ``lib/rustlib/$target/lib`` directory,
  relative to the root of the installation,
  should not exist duplicates of these regular files:
    
  - ``libstd-$hash.rlib``
  - ``libtest-$hash.rlib``
  - ``libproc_macro-$hash.rlib``

Checks for linkers
------------------

.. note:: The following checks apply to these targets:

   - :target:`x86_64-unknown-linux-gnu`

   ``$target`` refers to any of the targets listed above

Check if we can create temporary directories.

Inside of ``lib/rustlib/$target/bin`` directory,
relative to the root of the installation,
should exist the regular file named ``rust-lld``.

Inside of ``lib/rustlib/$target/bin/gcc-ld`` directory,
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

.. list-table::
   :header-rows: 1

   * - Target
     - Host target
   * - :target-with-triple:`x86_64-unknown-linux-gnu`
     - ✔
   * - :target-with-triple:`aarch64-unknown-none`
     -
   * - :target-with-triple:`x86_64-pc-windows-msvc`
     - ✔
   * - :target-with-triple:`aarch64-unknown-linux-gnu`
     - ✔
   * - :target-with-triple:`aarch64-apple-darwin`
     - ✔
   * - :target-with-triple:`x86_64-apple-darwin`
     -
   * - :target-with-triple:`aarch64-unknown-nto-qnx710`
     -
   * - :target-with-triple:`x86_64-pc-nto-qnx710`
     -
   * - :target-with-triple:`armv8r-none-eabihf`
     -
   * - :target-with-triple:`thumbv7em-none-eabi`
     -
   * - :target-with-triple:`thumbv7em-none-eabihf`
     -
   * - :target-with-triple:`armv7r-none-eabihf`
     -
   * - :target-with-triple:`armebv7r-none-eabihf`
     -

All targets
^^^^^^^^^^^

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

Host targets
^^^^^^^^^^^^

Check if a sample program that ``rustc`` produced can be executed.

Check if the output of the program is as expected.
