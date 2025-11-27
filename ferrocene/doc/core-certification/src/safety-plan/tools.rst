.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Tool safety assessments
=======================

All offline tools we use to develop the core library are listed below. The compiler is T3, all other tools are T2 or T1.

There are no online tools used to develop the core library.

For each tool there is a description of the usage of the tool for the core library, the version of the tool used as well as a safety assessment.

Code coverage
-------------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``blanket``
     - |ferrocene_version| (in-tree)
   * - ``llvm-profdata``
     - |llvm_version|
   * - ``rustc``
     - |rust_version|
   * - ``symbol-report``
     - |ferrocene_version| (in-tree)

Usage
~~~~~

See :ref:`safety-plan/testing-plan:Code coverage tests` for details of how it works.

Developer usage is described in :doc:`internal-procedures:code-coverage`.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool Classification: T2
- Level of reliance: Low, it is not involved in ensuring correctness, but only a measure of quality of the test suite.

The instrumentation mechanism using ``-Cinstrument-coverage`` is the standard mechanism of collecting code coverage information in Rust. Since it is part of the LLVM suite of tools, it is not only used in Rust but also widely used in the C++ ecosystem. This widespread usage gives us confidence in the quality and robustness of the tooling.

``blanket`` and ``symbol-report`` are tools developed by Ferrous Systems. ``symbol-report`` uses information from the compiler to ensure all the functions from the certified subset are being considered for code coverage. ``blanket`` is built on top of the Rust library called ``llvm-profparser`` and uses the output of ``symbol-report``. This library is developed by the ``cargo-tarpaulin`` project, which is widely used to measure code coverage for Rust projects.

The tools are designed to not overcount code coverage.

Failure modes
"""""""""""""

- False-positive: A function is reported as covered, although it is not covered
  - Risk: Overreporting, could result in testing gap.
  - Mitigation: No mitigation, since we assume the likelihood of such an error low.
- False-negative: A function is reported as not covered, although it is covered
  - Risk: Underreporting, will not result in testing gap.
  - Mitigation: Since we want to achieve 100% line coverage this would stand out and be manually investigated.
- The code coverage instrumentation introduces bugs into the library or the test runner
  - Risk: That results in failing tests being reported as successful or successful tests being reported as failing
  - Mitigation: Running the test suite once with and once without code coverage instrumentation and ensuring both report the same result.
- Undercounting: Total number of functions is too high
  - Risk: A function is being considered, although it is not part of the certified subset
  - Mitigation: Not a risk as it only results in us testing more than necessary
- Overcounting: Total number of functions is too low
  - Risk: A function is not being considered, although it is part of the certified subset
  - Mitigation: Developing ``symbol-report`` which uses exactly the same information as the compiler
- Line that can be executed not being reported as executable
    - Risk: Underreporting, code that should be tested may not being tested
    - Mitigation:
        - ``blanket`` warns if a function has no executable line
        - (Future work) End-to-end test that ensures the correct lines are being reported as executable

Compiler
--------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``rustc``
     - |rust_version|

Usage
~~~~~

The qualified Ferrocene compiler is used to build the core library, which gives high confidence in its quality.

Nightly features
""""""""""""""""

The core library relies on a few so-called "nightly features" of the compiler. Regular users of Ferrocene are not allowed to use them, therefore they are not part of the compiler qualification.
This is because they are either "experimental" or "internal”. They do work well, but they can change between compiler versions and do not fall under the usual Rust stability guarantees.
This is not a problem for the core library, because ``rustc`` and the core library are developed, built, and tested together.

Nightly features are activated by setting the ``RUSTC_BOOTSTRAP`` environment variable when executing ``rustc``.

Nightly features used by the core library are listed as ``#![feature(name_of_the_feature)]`` in ``library/core/src/lib.rs``.

Nightly features are tested by the ``compiletest`` test suite, by tests that activate that feature explicitly. E.g. ``tests/ui/unknown-language-item.rs`` tests ``#![feature(lang_items)]``.

Compiler built-in functions
"""""""""""""""""""""""""""

There are functions in the core library that are "compiler built-in”. That means they are not implemented in the core library codebase but in the compiler codebase.

They fall in two categories:

Macros
''''''

Macros generate different code on every use.

Customers have to ensure the generated code is correct. This is documented in the safety manual.

At the time of writing there are 60 compiler built-in macros (``rg "compiler built-in" library/core``). Not all of them are certified.

An example of such a built-in macro is ``pub macro Clone`` (`<https://github.com/ferrocene/ferrocene/blob/3ab6d2e0eb60057ec912d9619542ab590da45a51/library/core/src/clone.rs#L258-L260>`_).

Intrinsics
''''''''''

Intrinsics are "implementation details of ``core`` and should not be used outside of the standard library" (quote from the intrinsics module doc-comment).

All instrinsic function are in the ``intrinsics`` module and its submodules.

They are not availble in stable Rust and therefore cannot be used directly by customers.

At the time of writing there are 395 intrinsic function (``rg "fn" library/core/src/intrinsics``). Not all of them are certified.

An example of such a intrinsic function is ``fn unaligned_volatile_load<T>(src: *const T) -> T`` (`<https://github.com/ferrocene/ferrocene/blob/3ab6d2e0eb60057ec912d9619542ab590da45a51/library/core/src/intrinsics/mod.rs#L1050>`_).

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool Classification: T3

No assessment necessary, since the compiler is pre-qualified.

Linting
-------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``clippy``
     - |ferrocene_version|
   * - ``llvm-rustc``
     - |rust_version|
   * - ``rustfmt``
     - |ferrocene_version|

Usage
~~~~~

Upstream already has very good coding practices for the core library, which are enforced by the ``tidy`` test suite.
The ``tidy`` test suite executes rustc and clippy lints to enforce consistency in semantics and ``rustfmt`` to enforce consistency in syntax.

It does not make sense for us to come up with a separate coding standard and try to force it upon the upstream core library.
If we would start to come up with new rules from our coding standard we would have to work against upstream and either convince them to refactor their code without a clear benefit for them or we would have to carry a big changeset which has a big potential to introduce bugs.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool Classification: T1
- Level of reliance: Low, the lints are not involved in ensuring correctness, but only a measure of quality of the source code. (Note: ``rustc`` is involved in ensuring correctness, but here we only look at it in its capacity of a linter, not a compiler.)

``clippy``, ``rustc`` and ``rustfmt`` are standard tools in the Rust ecosystem. There are used in virtually every Rust project. This gives high confidence in its quality.

Failure modes
"""""""""""""

- False-negative: Fail to detect non-compliance with the consistency rules
   - Risk: Diverging from consistency rules. This is not critical, because Ferrous Systems only consumes the code from upstream and does not impose additional rules on it.
   - Mitigation: None. If found, report issue upstream.
- False-positive: Report non-compliance, although the code is compliant
   - Risk: None
   - Mitigation: Report issue upstream.

Test runner
-----------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``libtest``
     - |ferrocene_version|

Usage
~~~~~

The libtest test runner compiles all tests specified in the coretests test suite into an executable that executes the tests and reports if the test results are as expected.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool classification: T2
- Level of reliance: High, ensures correctness of the test results.

``libtest`` is used extensively by virtually every user of Rust, since it powers the common ``cargo test`` command. Heavy users of it include the upstream Rust project and Ferrous Systems which uses it in the rustc compiler qualification. Both upstream and Ferrous Systems execute thousands of tests with it, every day. Therefore there is a high chance of a bug in libtest being detected.

Failure modes
"""""""""""""

- False-positive: Report test as successful, although it is failing
   - Risk: Not detect incorrect code.
   - Mitigation: Report issue upstream.
- False-negative: Report test as failing, although it is successful
   - Risk: None
   - Mitigation: Report issue upstream.

Version control system
----------------------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``git``
     - Version 2
   * - GitHub
     - GitHub Enterprise version 3

Usage
~~~~~

``git`` is being used to track changes, with GitHub as a remote repository.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool classification: T2
- Level of reliance: Medium

Git and GitHub are very very widely used tools. This gives us confidence in its quality.

Failure modes
"""""""""""""

- False-positive: Introduce changes that were not made
   - Risk: Erroneous code, documentation, configuration
   - Mitigation: Code review.
- False-negative: Do not track changes that were made
   - Risk: Lose time invested.
   - Mitigation: Code review.

``rustdoc``
-----------

Version
~~~~~~~

.. list-table::
   :align: left
   :header-rows: 1

   * - Tool
     - Version
   * - ``rustdoc``
     - |ferrocene_version|

Usage
~~~~~

``rustdoc`` is used to generate the API documentation from source code as well as generating the symbols for the code coverage report.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Tool classification: T2
- Level of reliance: Medium

``rustdoc`` is the standard tool to generate documentation of Rust libraries and is very widely used. Each version of each crate published on `<https://crates.io>`_ automatically gets its documentation build by ``rustdoc`` and published on `<https://doc.rs>`_. This means it is executed hundreds of times per day for a wide variety of crates and documentations. This wide and diverse usage gives high confidence in its quality and robustness.

Failure modes
"""""""""""""

- Modify generated documentation
   - Risk: Erroneous documentation
   - Mitigation: If detected, report error.
