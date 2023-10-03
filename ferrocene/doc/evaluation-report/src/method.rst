.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Qualification Method
====================

According to the Tool Confidence Level determined previously and the
recommendation of the Table 4 of the [|iso_ref|] part 8, the chosen
qualification method is "Validation of the software tool in accordance with
11.4.9". To ensure the absence of potential issues in Ferrocene, automated
jobs are carried out for all targets. These jobs launch several tests suites,
including a non-regression test suite, in a correctly configured environment to
be sure of the tool conformance. In particular, the following test suites are run
daily:

Upstream Test Suites
--------------------

The following test suites are developed and maintained by the upstream Rust
project.

Compiletest Test Suite
^^^^^^^^^^^^^^^^^^^^^^

.. id:: TS1_COMP

**Compiletest** is the main compiler test suite. It contains a set of Rust
programs that verify the *robustness* and *behavior* of the compiler.

Robustness is checked by compiling illegal Rust programs, where the error
diagnostics emitted by the compiler are compared against expected results.

Behavior is checked by compiling legal Rust programs, and comparing  all of the
output artifacts against expected results. If the Rust program produces an
executable, the behavior of the executable is compared against expected results.
Behavior tests may employ various compiler arguments, and be exercised on all or
selected targets.

Compiletest has the following statistics:

* 7,080 test cases

* 28,589 files

* 909,823 lines of code

Library Test Suite
^^^^^^^^^^^^^^^^^^

.. id:: TS2_LIBR

The **library test suite** contains tests that cover library libcore, and on
supported platforms, libraries liballoc, libstd, and libtest. The tests are a
mix of unit tests, integration tests, and documentation tests (code snippets
within the documentation and built and run).

The library test suite has the following statistics:

* 1,000 test cases

* 100 files

* 17,500 lines of code

Crates Test Suite
^^^^^^^^^^^^^^^^^

.. id:: TS3_CRAT

The **crates test suite** covers the crates that comprise the compiler codebase.
The tests are a mix of unit tests and integration tests. The crates test suite
does not have good coverage because the upstream Rust project prefers to use
compiletest to check the compiler functionality, and in addition, unit tests are
only used on a subset of the compiler codebase.

The crates test suite has the following statistics:

.. Approximation of the following command, as of 2023-02-24:
   rg "^ *#\[test\]" compiler/ -c --no-filename | paste -sd+ | bc

* 500 test cases for the compiler

.. Approximatrion of the following command, as of 2023-02-24:
   rg "^ *#\[test\]" library/ -c --no-filename | paste -sd+ | bc

* 3,200 test cases for the standard library

Linkchecker Test Suite
^^^^^^^^^^^^^^^^^^^^^^

.. id:: TS4_LINK

The **linkchecker** test suite checks that none of the links in the generated
documentation are broken.

The linkchecker test suite has the following statistics:

.. Approximation of the number of links checked minus the number of links
   ignored, as of 2023-02-24:

* 3,000,000 links checked

Documentation Test Suite
^^^^^^^^^^^^^^^^^^^^^^^^

.. id:: TS5_DOCS

The **documentation test suite** checks that any code snippet found in narrative
and generated documentation properly compiles and executes.

The documentation test suite has the following statistics:

.. Approxiamtion of the Doc-tests executed in CI, as of 2023-02-24:

* 6,000 test cases

Build System Test Suite
^^^^^^^^^^^^^^^^^^^^^^^

.. id:: TS6_BSYS

The **build system test** suite checks that major components such as x.py and
bootstrapping operate as expected. According to our analysis, the test suite is
not exhaustive, but still covers some basic mistakes in the build system
implementation.

Tidy Test Suite
^^^^^^^^^^^^^^^

.. id:: TS7_TIDY

The **tidy test suite** checks for consistency in the compiler codebase. It
includes a variety of miscellaneous checks.

Ferrocene Test Suites
---------------------

The following test suites are developed and maintained by Ferrous Systems.

Ferrocene Compiletest
^^^^^^^^^^^^^^^^^^^^^

.. id:: TS8_FCOMP

**Ferrocene Compiletest** is the augmented version of the upstream
*Compiletest* test suite. Ferrocene Compiletest contains additional tests that
check the robustness and behavior of the compiler arguments that have been
selected for qualification.

Robustness is checked by invoking the compiler with illegal or malformed
arguments, where the error diagnostics emitted by the compiler are compared
against expected results.

Behavior is checked by invoking the compiler with legal arguments, and if the
compiler generates output artifacts, the artifacts are compared against expected
results.

In addition, the majority of tests are annotated with unique "ferrocene
annotation" tags that establish traceability between a test and a section from
the Ferrocene Language Specification. The ferrocene annotations are applied at
the directory level, where all tests in that directory inherit the annotation,
and also applied at the level of an individual test.

Ferrocene Compiletest has the following statistics:

* 7,507 test cases

* 29,267 files

* 915,125 lines of code
