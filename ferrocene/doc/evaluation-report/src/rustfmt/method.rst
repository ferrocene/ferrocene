.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Qualification Method
====================

According to the Tool Confidence Level determined previously and the
recommendation of Table 4 of [|iso_ref|] part 8, the chosen
qualification method is "Validation of the software tool in accordance with
11.4.9". To ensure the absence of potential issues in Ferrocene, automated
jobs are carried out for all targets. These jobs launch several test suites,
including a non-regression test suite, in a correctly configured environment to
be sure of the tool conformance. In particular, the following test suites are
executed before any change is merged.

Test Suites
-----------

The following test suites are developed and maintained by both the upstream
Rust project and Ferrocene.

rustfmt Test Suite
^^^^^^^^^^^^^^^^^^^^^^

.. id:: RUSTFMT_TS1

The **library test suite** contains tests that cover rustfmt. The tests are a
mix of unit tests, integration tests, and documentation tests (code snippets
within the documentation which are built and run).
