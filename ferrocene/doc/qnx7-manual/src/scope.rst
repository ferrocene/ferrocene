.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Qualification scope
===================

This qualification applies to Ferrocene |ferrocene_version|, limited to:

* rustc |rust_version|, limited to the Rust language as described in the
  :doc:`specification:index`, and according to the requirements and
  restrictions outlined in this safety manual.

This qualification is restricted to the following environment:

.. list-table::
   :align: left
   :header-rows: 1

   * - Host
     - Target
     - Certified Libraries
     - Uncertified Libraries
     - Qualified Libraries

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`aarch64-unknown-nto-qnx710`
     - ``core``
     - ``alloc``, ``std``, ``test``
     - ``proc_macro``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`x86_64-pc-nto-qnx710`
     - ``core``
     - ``alloc``, ``std``, ``test``
     - ``proc_macro``
