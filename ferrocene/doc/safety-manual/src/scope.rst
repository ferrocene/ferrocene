.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Qualification scope
===================

This qualification applies to Ferrocene |ferrocene_version|, limited to:

* rustc |rust_version|, limited to the Rust language as described in the
  :doc:`specification:index`, and according to the requirements and
  restrictions outlined in this safety manual.

* rustfmt |rustfmt_version|, according to the requirements and restrictions
  outlined in this safety manual.

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

The libraries provided are evaluated and tested within the scope of
Ferrocene qualification for compiler use only. The use of these libraries by
end-use code is outside the scope of the current Ferrocene qualification. It
is the end-user responsibility to certify these libraries if they are used in
their code.

User responsibility
-------------------

According to clause 11.4.2 of [|iso_ref|] part 8, the user shall verify the
validity of the predetermined TCL prior to the use of this software tool in a
safety-related development.

Furthermore, according to clause 11.4.3 of [|iso_ref|] part 8, the user
shall ensure the usage, the environment, and the functional constraints of this
software tool comply with its evaluation criteria or its qualification.

Ferrocene is accompanied by the appropriate documentation to support this
process.
