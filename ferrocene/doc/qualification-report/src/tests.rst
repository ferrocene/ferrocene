.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Tests Tesults
=============

Ferrocene for ARMv8-A bare metal and x86-64 Linux
-------------------------------------------------

Test Environment
^^^^^^^^^^^^^^^^

For this qualification, testing is restricted to the following environment:

.. list-table:: 
   :align: left
   :stub-columns: 1

   * - Host
     - x86_64 Linux
   * - Target
     - ARMv8-A bare metal (aarch64)
   * - Supported languages
     - Rust

.. end of table

.. note::

   For the Rust language, only the Rust standard as described in the
   Ferrocene :doc:`Language Specification document <specification:index>`
   is verified.

Bare metal testing
******************

The ARMv8-A bare metal Ferrocene target is meant to be used in an environment
without any operative system. Consequently, it does not include APIs relying on
one (as part of the ``std`` crate).

Rust's test suites require those APIs to be available in order to invoke the
tests themselves and to report the execution results. To solve the issue, a new
target, based on the existing Rust ``arch64-unknown-none`` target, was created
called ``arch64-unknown-ferrocenecoretest``.

This target is strictly internal, and will not be released to customers. It has
the same configuration as the ``aarch64-unknown-none`` target, with the only
exception being enabling the operating system bindings for Linux (the OS used
to execute the test suite).

Since the only difference between the two targets is the APIs in the ``std``
crate, which is not present in the ``aarch64-unknown-none`` target (and
consequently not shipped to customers), we can conclude that the test results
of ``aarch64-unknown-ferrocenecoretest`` are also valid for
``aarch64-unknown-none``.

Release Notes
^^^^^^^^^^^^^

The |ferrocene_version| version of the Ferrocene toolset contains the following
tools:

.. csv-table:: Tools and their versions
   :align: left
   :header: **Tool**, **Version**
   :widths: 8, 9
   :delim: !
   :class: longtable

   cargo!|rust_version|
   rustc!|rust_version|
   rustdoc!|rust_version|

Results
^^^^^^^

The following are the results of the Ferrocene test suites. For further
details of each test suite, refer to
:doc:`Evaluation Report : Qualification Method <evaluation-report:method>`.

Compiletest Test Suite
**********************

The ``Debuginfo`` test suite is not verified on ``aarch64`` because the test
suite does not support that target.

The ``Pretty`` test suite is not verified on ``aarch64`` because it does not
make sense to exercise the pretty-printing functionality of the Ferrocene
compiler on non-host platforms.

The ``RunMakeFullDeps`` and ``UiFullDeps`` test suites are not verified on
``aarch64`` because the Ferrocene toolset does not provide a **host**
``aarch64`` compiler.

The ``Rustdoc``, ``RustdocJson``, and ``RustdocUi`` test suites are not
verified on ``aarch64`` because the ``rustdoc`` tool does not run on
``aarch64`` hosts.

.. suite-summary:: bootstrap::test::Compiletest

Library Test Suite
******************

The library test suite uses the ``stage1`` library because a ``stage2`` library
is never built, and the ``stage2`` Ferrocene compiler reuses the ``stage1``
library.

The standard library is not tested on ``aarch64`` because it is not available
for that target.

.. FIXME: due to how the bootstrap code works, we're forced to match only the
   root node, otherwise we'd also match other tests invoking Crate down their
   dependency chain.

.. suite-summary:: bootstrap::test::Crate
   :only_match_root_node:

Crates Test Suite
*****************

The crates test suite is not tested on ``aarch64`` because the Ferrocene
toolset does not provide a **host** ``aarch64`` compiler.

.. suite-summary:: bootstrap::test::CrateLibrustc

Linkchecker Test Suite
**********************

The linkchecker test suite is a pass/fail test suite integrated into the
Ferrocene CI infrastructure.

The linkchecker test suite is verified as part of
:ref:`testing:Test Phase 2: Full Testing and Merge`. As indicated in 
:doc:`Qualification Plan : Development Process <qualification-plan:development>`
, a PR is merged into the repository only when it passes full testing.

As a result, the linkchecker test suite reports a **pass** for this
qualification.

Documentation Test Suite
************************

The documentation test suite is integrated directly into the crates test suite
and the library test suite.

Build System Test Suite
***********************

.. suite-summary:: bootstrap::test::Bootstrap

Tidy Test Suite
***************

The tidy test suite is a pass/fail test suite integrated into the Ferrocene
CI infrastructure.

The tidy test suite is verified as part of
:ref:`testing:Test Phase 2: Full Testing and Merge`. As indicated in 
:doc:`Qualification Plan : Development Process <qualification-plan:development>`
, a PR is merged into the repository only when it passes full testing.

As a result, the tidy test suite reports a **pass** for this qualification.

Ferrocene Compiletest Test Suite
********************************

The Ferrocene compiletest test suite is directly integrated into the
compiletest test suite.

Known Problems
^^^^^^^^^^^^^^

KPs identified through the lifecycle of Ferrocene for ARMv8-A bare metal and
x86-64 Linux are tracked in the :doc:`safety-manual:known-problems`. This
document is made available to customers for consulting.

Ignored Tests
^^^^^^^^^^^^^

The following table presents all ignored tests, along with reasons as to why
they were ignored.

.. ignored-tests::

.. note::

   Ignored documentation tests are **not** included in the table above.

   Rust's documentation and testing tooling considers all snippets of code in
   the generated documentation to be a test by default, and snippets that are
   not meant to be valid Rust code (or Rust code at all) would cause test
   failures if not ignored explicitly.

   Because of that, we don't consider ignored documentation tests to be valid
   tests that were skipped by our testing infrastructure: we consider them not
   to be tests at all.
