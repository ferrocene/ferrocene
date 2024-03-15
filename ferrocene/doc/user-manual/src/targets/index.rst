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
  cross-compiling to a different one.

* **Cross-compilation**: can only be used to cross-compile to the platform from
  another host platform.

There are also two variants of the standard library available:

* **Bare-metal**: provides the ``core`` and ``alloc`` built-in crates.
* **Full**: provides the ``core``, ``alloc``, ``std``, ``test`` and
  ``proc_macro`` built-in crates.

Supported targets
-----------------

Supported targets are provided with the highest level of assurance. They are
qualified for use in safety-critical contexts (when used with a qualified
Ferrocene compiler), and the full Ferrocene test suite is executed on the
target for every code change.

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

   * - :target:`wasm32-unknown-unknown`
     - ``wasm32-unknown-unknown``
     - Cross-compilation
     - Full
     - The full standard library is available, but unsupported functions in ``std`` will panic.

If your project needs support for one of these targets, please reach out to the
Ferrocene support team.

Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
