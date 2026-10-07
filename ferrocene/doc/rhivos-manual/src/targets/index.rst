.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Compilation targets overview
============================

Ferrocene has support for multiple compilation targets and host platforms.
Targets are categorized into :doc:`levels of support <user-manual:targets/index>`.

This page lists the current support status for Red Hat In-Vehicle Operating System 2 targets, and individual
pages with more details are provided for Red Hat In-Vehicle Operating System 2 Qualified targets.

.. _qualified-targets:

Qualified Red Hat In-Vehicle Operating System targets
-----------------------------------------------------

.. list-table::
   :header-rows: 1

   * - Target
     - Tuple
     - Kind
     - Standard library
     - Notes

   * - :ref:`aarch64-unknown-linux-gnu`
     - ``aarch64-unknown-linux-gnu``
     - Host platform
     - Full
     - Only qualified when cross-compiling to :ref:`aarch64-rhivos2-linux-gnu`.

   * - :ref:`aarch64-rhivos2-linux-gnu`
     - ``aarch64-rhivos2-linux-gnu``
     - Cross-compilation
     - Full
     - This is a variant of the generic :target:`aarch64-unknown-linux-gnu` target that specifically targets Red Hat In-Vehicle Operating System 2 automotive Linux (RHIVOS 2). As per the RHIVOS 2 guidelines, qualified use requires compilation on the matching host platform RedHat Enterprise Linux 10 using the :ref:`aarch64-unknown-linux-gnu` host compiler.


Unsupported targets
-------------------

The Rust compiler includes support for additional targets that are not yet
included in Ferrocene. If you need support for them please reach out to the
Ferrocene support team.
