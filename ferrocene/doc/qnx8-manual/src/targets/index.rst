.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Compilation targets overview
============================

Ferrocene has support for multiple compilation targets and host platforms.
.. Targets are categorized into :doc:`levels of support <user-manual:targets/index>`.

This page lists the current support status for QNX8 targets, and individual
pages with more details are provided for QNX8 Qualified targets.

.. _qualified-targets:

Qualified QNX8 targets
----------------------


.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`aarch64-unknown-qnx`
     - ``core``
     - ``alloc``, ``std``, ``test``
     - ``proc_macro``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`x86_64-pc-qnx`
     - ``core``
     - ``alloc``, ``std``, ``test``
     - ``proc_macro``


Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
