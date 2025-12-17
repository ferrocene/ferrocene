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

.. note::
  ``alloc``, ``std``, ``test``, and ``proc_macro``, and uncertified parts of ``core``
  are  provided are evaluated and tested within the scope of Ferrocene qualification for
  compiler use only. The use of these libraries by end-use code is outside the scope of the
  current Ferrocene qualification.
  It is the end-user responsibility to certify these libraries if they are used in their code.

  A subset of ``core`` is certified against IEC 61508 (SIL 2) on qualified targets; see the
  :doc:`user-manual:core/index` and :doc:`core-certification:index`.

A summary of the assurances and support offered by target levels is summarized immediately below.
For more details, refer to the appropriate section.

================ ========== =============== ======================== ============
Feature          Qualified  Quality managed Supported                Experimental
================ ========== =============== ======================== ============
Binaries         Yes        Yes             Yes                      Best effort
Tested           Per merge  Per release     Best effort              No
Qualified        Yes        No              No                       No
Known Issues     Yes        Yes             Documented only          No
Customer Support Yes        Yes             Yes                      Best effort
Support Patches  2 years    2 years         2 years (critical only)  None
Deprecation      1 year     1 year          1 year (with exceptions) Any time
LTS available    Yes        Yes             Yes (with exceptions)    No
================ ========== =============== ======================== ============

.. _qualified-targets:

Qualified targets
-----------------

Qualified targets are provided with the highest level of assurance and are
qualified for use in safety-critical contexts.

Support is available. Known problems are tracked and available.

The full Ferrocene test suite is executed on the target before any
change is merged.

Qualified targets have a two year support window from their release date.
Qualified targets may be removed from future release after a deprecation period of
1 year after announcement or when circumstances outside our control require removal,
for example when the target vendor no longer supports the target.

Most qualified targets come with a "certified equivalent" target. This
certified target allows using the certified core library. The core library is
not certified when using qualified targets, only when using certified targets.
Refer to :ref:`certified-core-targets`.

Only stable releases of qualified targets are qualified. Other releases, such
as beta, should be considered Quality Managed. Such releases can be
qualified upon request.

.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

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

   * - :ref:`thumbv7em-none-eabi`
     - ``thumbv7em-none-eabi``
     - Cross-compilation
     - Bare-metal
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.

   * - :ref:`thumbv7em-none-eabihf`
     - ``thumbv7em-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.

   * - :ref:`x86_64-unknown-linux-gnu`
     - ``x86_64-unknown-linux-gnu``
     - Host platform
     - Full
     - \-

   * - :ref:`x86_64-pc-nto-qnx710`
     - ``x86_64-pc-nto-qnx710``
     - Cross-compilation
     - Full
     - Only qualified when cross-compiled from :ref:`x86_64-unknown-linux-gnu`.

Quality managed targets
-----------------------

Quality managed targets are targets which are suitable for use in production
outside of safety critical contexts.

Support is available. Known problems are tracked and available.

The Ferrocene test suite is executed on the target prior to release.

Quality managed targets have a two year support window from their release date.
Quality managed targets may be removed from future release after a deprecation period of
1 year after announcement or when circumstances outside our control require removal,
for example when the target vendor no longer supports the target.

Quality managed targets are not qualified, but can usually be qualified on request.


.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

   * - :ref:`aarch64-apple-darwin`
     - ``aarch64-apple-darwin``
     - Host platform
     - Full
     - \-


Supported targets
-----------------

Supported targets are targets which are suitable for use outside of safety-critical
contexts, and may be suitable for production given adequate user testing.

Support is available. Known problems are tracked and documented, but are prioritized
lower than Qualified or Quality Managed targets.

While supported targets are built and provided to customers, the
Ferrocene test suite may not always be fully executed.

Supported targets may be removed from future release after a deprecation period of
1 year after announcement or when circumstances outside our control require removal,
for example when the target vendor no longer supports the target.

Supported targets can often be qualified or quality managed upon request.


.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

   * - :target:`aarch64-unknown-linux-gnu`
     - ``aarch64-unknown-linux-gnu``
     - Host platform
     - Full
     - \-

   * - :target:`aarch64-unknown-linux-musl`
     - ``aarch64-unknown-linux-musl``
     - Cross-compilation
     - Full
     - \-

   * - :target:`armebv7r-none-eabihf`
     - ``armebv7r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`armv7r-none-eabihf`
     - ``armv7r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`armv8r-none-eabihf`
     - ``armv8r-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`riscv64gc-unknown-linux-gnu`
     - ``riscv64gc-unknown-linux-gnu``
     - Cross-compilation
     - Full
     - Available as a cross-compile target on :target:`aarch64-unknown-linux-gnu` and :ref:`x86_64-unknown-linux-gnu`.

   * - :target:`thumbv6m-none-eabi`
     - ``thumbv6m-none-eabi``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`thumbv8m.base-none-eabi`
     - ``thumbv8m.base-none-eabi``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`thumbv8m.main-none-eabi`
     - ``thumbv8m.main-none-eabi``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`thumbv8m.main-none-eabihf`
     - ``thumbv8m.main-none-eabihf``
     - Cross-compilation
     - Bare-metal
     - \-

   * - :target:`wasm32-unknown-unknown`
     - ``wasm32-unknown-unknown``
     - Cross-compilation
     - Full
     - The full standard library is available, but unsupported functions in ``std`` will panic.

   * - :ref:`x86_64-pc-windows-msvc`
     - ``x86_64-pc-windows-msvc``
     - Host platform
     - Full
     - \-

   * - :target:`x86_64-unknown-linux-musl`
     - ``x86_64-unknown-linux-musl``
     - Cross-compilation
     - Full
     - \-


Experimental Targets
--------------------

Experimental targets are provided as a preview and without any assurances.
Use for production is generally not advised.

Limited support available. Known problems are not tracked.

The Ferrocene test suite may have been executed at some point.

Experimental targets may be removed for any reason without warning or deprecation period.

It is sometimes possible for Experimental targets to be qualified or quality managed upon request.

.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

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


Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
