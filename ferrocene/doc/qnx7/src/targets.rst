.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Compilation targets overview
============================

Ferrocene has support for multiple compilation targets and host platforms.
Targets are categorized into levels of support. This page lists
the current support status for all targets, and individual pages with more
details are provided for all Qualified and Quality managed targets, along
with some other selected targets.

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
  are evaluated and tested within the scope of Ferrocene qualification for
  compiler use only. The use of these libraries by end-use code is outside the scope of the
  current Ferrocene qualification.
  It is the end-user responsibility to certify these libraries if they are used in their code.

  A subset of ``core`` is certified against IEC 61508 (SIL 2) and ISO 26262 (ASIL B) on qualified targets;
  see the :doc:`user-manual:core/index` and :doc:`core-certification:index`.

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
