.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Compilation targets overview
============================

Ferrocene has support for multiple compilation targets and host platforms.
Targets are categorized as either "supported" or "experimental" depending on
the level of support. This page lists the current support status for all
targets, and individual pages with more details are provided for supported
targets.

There are two kinds of targets available:

* **Host platform**: can be used by developers and build systems to compile
  Rust code from that platform, either targeting the same platform or
  cross-compiling to a different one. Host platforms cannot neccessarily
  target other host platforms.

* **Cross-compilation**: can only be used to cross-compile to the platform from
  any host platform.

There are also two variants of the standard library available:

* **Bare-metal**: provides the ``core`` and ``alloc`` built-in crates.
* **Full**: provides the ``core``, ``alloc``, ``std``, ``test`` and
  ``proc_macro`` built-in crates.

Qualified targets
-----------------

Qualified targets are provided with the highest level of assurance. They are
qualified for use in safety-critical contexts (when used with a qualified
rustc), and the full Ferrocene test suite is executed on the target for every
code change.

Only stable releases of qualified targets are qualified. Other releases, such
as beta, should be considered Quality Managed. Such releases can be
qualified upon request.

.. list-table::
   :header-rows: 1

   * - Target
     - Triple
     - Kind
     - Standard library
     - Notes

   * - :ref:`x86_64-unknown-linux-gnu`
     - ``x86_64-unknown-linux-gnu``
     - Host platform
     - Full
     - \-

   * - :ref:`aarch64-unknown-none`
     - ``aarch64-unknown-none``
     - Cross-compilation
     - Bare-metal
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.


   * - :ref:`aarch64-unknown-nto-qnx710`
     - ``aarch64-unknown-nto-qnx710``
     - Cross-compilation
     - Full
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.

   * - :ref:`x86_64-pc-nto-qnx710`
     - ``x86_64-pc-nto-qnx710``
     - Cross-compilation
     - Full
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.

Quality managed targets
-----------------------

Quality managed targets are targets which are suitable for use in production 
outside of safety critical contexts. Support is available. The Ferrocene test
suite successfully executed on the target. Known problems are tracked and
available.

For any of the following reasons, the target is not qualified:

* The target is deemed unlikely to be used in a safety critical context.
* The target is in the process of qualification, but is not completed yet.

Quality managed targets are not qualified, but can usually be qualified on request.

.. list-table::
   :header-rows: 1

   * - Target
     - Triple
     - Kind
     - Standard library
     - Notes

   * - :ref:`aarch64-apple-darwin`
     - ``aarch64-apple-darwin``
     - Host platform
     - Full
     - \-

Experimental targets
--------------------

Experimental targets cannot be used in safety-critical contexts, and there is
no guarantee that the Ferrocene test suite is successfully executed on the
target. They are provided as a preview, with limited support available. They
should not be used in production.

.. list-table::
   :header-rows: 1

   * - Target
     - Triple
     - Kind
     - Standard library
     - Notes

   * - :target:`thumbv7em-none-eabi`
     - ``thumbv7em-none-eabi``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`thumbv7em-none-eabihf`
     - ``thumbv7em-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-
  
   * - :target:`armv8r-none-eabihf`
     - ``armv8r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`armv7r-none-eabihf`
     - ``armv7r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`armebv7r-none-eabihf`
     - ``armebv7r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`wasm32-unknown-unknown`
     - ``wasm32-unknown-unknown``
     - Cross-compilation
     - Full
     - The full standard library is available, but unsupported functions in ``std`` will panic.

   * - :target:`x86_64-apple-darwin`
     - ``x86_64-apple-darwin``
     - Cross-compilation
     - Full
     - Available as a cross-compile target on :ref:`aarch64-apple-darwin`.

   * - :ref:`x86_64-pc-windows-msvc`
     - ``x86_64-pc-windows-msvc``
     - Host platform
     - Full
     - \-

   * - :target:`aarch64-unknown-linux-gnu`
     - ``aarch64-unknown-linux-gnu``
     - Host platform
     - Full
     - \-

   * - :target:`riscv64gc-unknown-linux-gnu`
     - ``riscv64gc-unknown-linux-gnu``
     - Cross-compilation
     - Full
     - \-


If your project needs support for one of these targets, please reach out to the
Ferrocene support team.

Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
