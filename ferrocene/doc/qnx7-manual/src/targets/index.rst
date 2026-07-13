.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Compilation targets overview
============================

Ferrocene has support for multiple compilation targets and host platforms.
Targets are categorized into :doc:`levels of support <user-manual:targets/index>`.

This page lists the current support status for QNX7 targets, and individual
pages with more details are provided for QNX7 Qualified targets.

.. _qualified-targets:

Qualified QNX7 targets
----------------------


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

Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
