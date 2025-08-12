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

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`aarch64-unknown-none`
     - ``core``
     - ``alloc``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`aarch64-unknown-nto-qnx710`
     - ``core``
     - ``alloc``, ``std``, ``proc_macro``, ``test``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`thumbv7em-none-eabi`
     - ``core``
     - ``alloc``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`thumbv7em-none-eabihf`
     - ``core``
     - ``alloc``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`x86_64-unknown-linux-gnu`
     - ``core``
     - ``std``, ``proc_macro``, ``test``

   * - :target:`x86_64-unknown-linux-gnu`
     - :target:`x86_64-pc-nto-qnx710`
     - ``core``
     - ``alloc``, ``std``, ``proc_macro``, ``test``


The uncertified libraries provided are evaluated and tested within the scope of
Ferrocene qualification for compiler use only. The use of these libraries by
end-use code is outside the scope of the current Ferrocene qualification. It
is the end-user responsibility to certify these libraries if they are used in
their code.

The certified libraries provided are evaluated and tested to be used in other
projects by users of the Ferrocene compiler.

User responsibility
-------------------

According to clause 11.4.2 of [|iso_ref|] part 8, the user shall verify the
validity of the predetermined TCL prior to the use of this software tool in a
safety-related development.

Furthermore, according to clause 11.4.3 of [|iso_ref|] part 8, the user
shall ensure the usage, the environment, and the functional constraints of this
software tool comply with its evaluation criteria or its qualification.

Additionally, according to [|iec_med_ref|], a manufacturer of medical device software shall
consider the used tools in the software development plan (clause 5.1.4) and in their configuration item control (clauses 5.1.10 and 5.1.11).
Although [|iec_med_ref|] does not come with its own scheme of tool classification and validation, it recommends in Annex C.7
the usage of techniques, tools and methods as defined in [|iec_ref|].


Ferrocene is accompanied by the appropriate documentation to support this
process.
