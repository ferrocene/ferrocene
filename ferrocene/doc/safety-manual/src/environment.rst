.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Environment
===========

This qualification is restricted to the following environment:

.. list-table:: 
   :align: left
   :stub-columns: 1

   * - Host
     - x86_64 Linux
   * - Target
     - ARMv8-A bare metal (aarch64)
   * - Target specific libraries
     - libcore and liballoc
   * - Supported languages
     - Rust

.. end of table

The libraries provided are evaluated and tested within the scope of
Ferrocene qualification for compiler use only. The use of these libraries by
end-use code is outside the scope of the current Ferrocene qualification. It
is the end-user responsibility to qualify these libraries if they are used in
their code.

.. Note::
   For the Rust language, only the Rust standard as described in the
   Ferrocene :doc:`Language Specification document <specification:index>`.

The qualification scope is limited to the set of supported compilation options
described in the :doc:`Tool Options <safety-manual:options>`.
