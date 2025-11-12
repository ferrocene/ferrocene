.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

.. note::

   flip-link is known to work with ARM Cortex-M programs that link to version 0.6.x of the
   ``cortex-m-rt`` `crate <https://crates.io/crates/cortex-m-rt>`_ and are linked using the
   linker shipped with the toolchain (LLD).

   At this time, it hasn't been tested with other architectures or runtime crates.

``flip-link`` is used as a linker during the build process. To use it, configure your build
environment in one of the following ways.

Using ``cargo``
---------------

To use with ``cargo``, modify the ``.cargo/config.toml``:

.. code-block:: toml
   
   [target.'cfg(all(target_arch = "arm", target_os = "none"))']
   # (..)
   linker = "flip-link"

Using ``rustc``
---------------

To use with ``rustc``, pass ``-C linker=flip-link``:

.. code-block:: bash

   rustc src/main.rs -C panic=abort --target thumbv7em-none-eabihf -C linker=flip-link

Logging
-------

If you want to see what ``flip-link`` is up to, you can set these environment variables:

.. code-block:: bash

   export RUSTC_LOG=rustc_codegen_ssa::back::link=info
   export RUST_LOG=info


This will produce something like:

.. code-block:: bash

   $ cargo build
   ...
   INFO rustc_codegen_ssa::back::link linker stderr:
   [INFO  flip_link] found MemoryEntry(line=3, origin=0x20000000, length=0x10000) in ./target/thumbv7em-none-eabi/debug/build/lm3s6965-3b7087c63b161e04/out/memory.x
   [INFO  flip_link] used RAM spans: origin=0x20000000, length=12, align=4
   [INFO  flip_link] new RAM region: ORIGIN=0x2000fff0, LENGTH=16
   INFO rustc_codegen_ssa::back::link linker stdout:
   INFO rustc_codegen_ssa::back::link linker stdout:
      Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.08s

You can see even more detail about how expressions are parsed using ``RUST_LOG=debug``.