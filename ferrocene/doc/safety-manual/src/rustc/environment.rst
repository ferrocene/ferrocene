.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Environment
===========

This qualification is restricted to the following environment:

.. list-table::
   :align: left
   :header-rows: 1

   * - Host
     - Target
     - Uncertified Libraries

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`aarch64-unknown-none`
     - ``core``, ``alloc``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`x86_64-unknown-linux-gnu`
     - ``core``, ``alloc``, ``std``, ``proc_macro``, ``test``

.. end of table

The libraries provided are evaluated and tested within the scope of
Ferrocene qualification for compiler use only. The use of these libraries by
end-use code is outside the scope of the current Ferrocene qualification. It
is the end-user responsibility to qualify these libraries if they are used in
their code.

.. note::

   Only the Rust language,
   as described in the :doc:`specification:index`,
   is verified.

The qualification scope is limited to the set of supported compilation options
described in the :doc:`Tool Options </rustc/options>`.
