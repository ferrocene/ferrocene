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

Compiletest Test Suite
^^^^^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS1_COMP

**Compiletest** is the main compiler test suite. It contains a set of Rust
programs that verify the *robustness* and *behavior* of the compiler and
compiler arguments.

Robustness is checked by invoking the compiler with illegal or malformed
arguments and by compiling illegal Rust programs, where the error diagnostics
emitted by the compiler are compared against expected results.

Behavior is checked by invoking the compiler with legal arguments and by
compiling legal Rust programs, and comparing  all of the output artifacts
against expected results. If the Rust program produces an executable, the
behavior of the executable is compared against expected results. Behavior tests
may employ various compiler arguments, and be exercised on all or selected
targets.

In addition, the majority of tests are annotated with unique "ferrocene
annotation" tags that establish traceability between a test and a section from
the Ferrocene Language Specification. The ferrocene annotations are applied at
the directory level, where all tests in that directory inherit the annotation,
and also applied at the level of an individual test.

Library Test Suite
^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS2_LIBR

The **library test suite** contains tests that cover the ``core`` library, and on
supported platforms, the libraries ``alloc``, ``std``, and ``test``. The tests are a
mix of unit tests, integration tests, and documentation tests (code snippets
within the documentation which are built and run).

Compiler Test Suite
^^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS3_CRAT

The **compiler test suite** covers the crates that comprise the compiler
codebase. The tests are a mix of unit tests, integration tests and
documentation tests (code snippets within the documentation which are built and
run). The compiler test suite does not have good coverage because the upstream
Rust project prefers to use compiletest to check the compiler functionality, and
in addition, unit tests are only used on a subset of the compiler codebase.

Linkchecker Test Suite
^^^^^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS4_LINK

The **linkchecker** test suite checks that none of the links in the generated
documentation are broken.

Build System Test Suite
^^^^^^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS6_BSYS

The **build system test** suite checks that major components such as x.py and
bootstrapping operate as expected. According to our analysis, the test suite is
not exhaustive, but still covers some basic mistakes in the build system
implementation.

Tidy Test Suite
^^^^^^^^^^^^^^^

.. id:: RUSTC_TS7_TIDY

The **tidy test suite** checks for consistency in the compiler codebase. It
includes a variety of miscellaneous checks.

Self-Test Test Suite
^^^^^^^^^^^^^^^^^^^^

.. id:: RUSTC_TS8_SELF

The **self-test test suite** installs the packaged release in a temporary
directory, and executes the ``ferrocene-self-test`` tool on it. The tool
ensures that packages contain the correct files, that files are installed in
the correct places, and that the installed toolchain can successfully compile a
small number of example programs.

.. _rustc-cli-testing-categories:

Compiler arguments affecting the compilation outcome
----------------------------------------------------

The compiler supports :doc:`multiple command-line arguments
<user-manual:rustc/cli>` affecting its behavior. Not all of them affect the
compilation process the same way, so different testing strategies are required.
For the purpose of testing, we categorize command-line arguments in:

- **Informational:** these arguments do not affect the compilation process, but
  instead configure the compiler to output information. An example of such
  arguments is ``--version``, showing the compiler version number.

- **Narrow impact:** these arguments do affect the compilation process, but
  their effect is narrow and well scoped, and they can be tested independently.
  An example is ``-C debuginfo``, to configure the compiler to emit debug
  information alongside executable code.

- **Wide impact:** these arguments affect the compilation process, and their
  effects can influence all parts of the compilation process. An example is ``-C
  opt-level``, to configure the code generation optimization level.

For informational and narrow impact arguments it is sufficient to add tests in
the test suite verifying their effects. For wide impact arguments, since every
test could be affected by them, we deem it necessary to re-execute every test
suite with each combination of wide impact argument values.
