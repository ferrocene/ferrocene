.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Safety Plan
===========

Purpose of this document
------------------------

This document describes the approach and the activities to be performed to achieve safety certification according to the safety standard(s) described later in this document.

Certification scope
-------------------

The core library shall be suitable to be used in safety applications according to following safety standards up to the specified safety level:

.. list-table::
   :align: left
   :header-rows: 1

   * - Safety Standard
     - Safety Level
   * - |iec_ref|
     - |iec_sil|

The core library is evaluated as an "assessment of non-compliant development” (according to Route 3S of |iec_ref| section 7.4.2.12). This assessment targets a full compliance statement to the standards above, as far as it is applicable for a Software Safety Element out of Context.

Certified version
~~~~~~~~~~~~~~~~~

The certified version of the core library is |rust_version|.

Certified targets
~~~~~~~~~~~~~~~~~

The core library is certified for all compilation targets Ferrocene rustc is qualified for. See :doc:`user-manual:targets/index` for a full list.

Certified subset
~~~~~~~~~~~~~~~~

The certification does not cover the entirety of the core library, but instead a subset. This is to reduce the effort of the certification.

The subset included in the safety certification is defined and documented in the :doc:`Safety Manual <safety-manual:core/subset>`.

Systematic capabilities
"""""""""""""""""""""""

All public functions of the certified subset are considered "software safety functions” and are going to be certified for all safety standards up to the safety level specified. That means our customers can use all of those functions for use cases up to the highest safety level specified. Since we consider all of them safety relevant we do not consider independence.

The systematic capability of these functions is based on:

- The requirements and the documented completeness of these requirements and their implementation in the code and tests
- The absence of any undocumented and untested code in the safety certification scope
- The required test coverage
- The adherence of the code within the safety scope to the Coding Guidelines

Project Setup
-------------

Release cadence
~~~~~~~~~~~~~~~

Due to the use of internal apis, the core library versions are only compatible with one matching Ferrocene release. The core library will be recertified for every Ferrocene release. That is approximately every three months.

The first Ferrocene release to include the certified core library is "Ferrocene 25.11.0". Ferrocene distributes an uncertified version of the core library since the first Ferrocene release.

Roles and responsibilities
~~~~~~~~~~~~~~~~~~~~~~~~~~

Roles and responsibilities (e.g. Safety Manager, Product Manager) are documented at :doc:`qualification-plan:organization`.

Lifecycle Phases Overview
~~~~~~~~~~~~~~~~~~~~~~~~~

1. Changes to the core library are pulled from the Upstream Rust project, as part of daily upstream pulls (see :doc:`internal-procedures:upstream-pulls`).
2. On a regular cadence, latest before the release, the changes are examined and it is ensured the certification requirements are upheld.

The requirements are:

- No uncertified code is used from certified code
- Every public function of the certified subset has a requirement with sufficient quality
- The certified code adheres to the coding standard
- The certified subset is tested with 100% statement coverage

Internal procedures
~~~~~~~~~~~~~~~~~~~

The :doc:`qualification-plan:index` describes how the Ferrocene organisation works, among others: Infrastructure, and the Development, Build, Testing and Release process. It is based on software engineering best practices, to be updated upon detection of shortcomings in the development process.

Deliverables and Documents
--------------------------

The following deliverables are delivered to the assessor and to customers:

Product Documentation
~~~~~~~~~~~~~~~~~~~~~

- :doc:`user-manual:index`
- The `Certified core library API docs <../../api-docs/core/index.html>`_, which include
   - Requirements (the function doc-comments)
   - Software Design (the module doc-comments)

Functional Safety Documentation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

- Safety Plan (this document)
- :doc:`core-certification:norm-mapping/index` for all safety standards
- :doc:`safety-manual:index`
- :doc:`core-certification:testing-plan`
- Test cases (``coretests`` test suite), and
- :doc:`qualification-report:rustc/index`

Binaries in the delivery
~~~~~~~~~~~~~~~~~~~~~~~~

The `rust-std-<TARGET>.tar.xz` archives contain the precompiled core library for each certified target.

Requirements Management
-----------------------

Doc-comments
~~~~~~~~~~~~

Requirements are implemented as doc-comments.

The Doc-comments described below cover the single level of functional requirements for each function.

The documentation for each module covers the purpose and overview, and as such is suitable for the design requirement.

Doc-comments in general
"""""""""""""""""""""""

Rust has a concept called "doc-comments” also known as documentation comments. They are denoted by triple-slashes, while normal comments are denoted by double-slashes. They support markdown, and code inside code blocks is automatically run as tests, to ensure the code and documentation do not get out of sync.

.. code-block:: rust
  :linenos:

  /// Add two `u32`s.
  ///
  /// ```
  /// assert_eq!(add(1, 5), 6);
  /// ```
  /// This is a doc-comment
  //
  // This is not a doc-comment
  fn add(x: u32, y: u32) -> u32 { /* */ }

In the exampe above, the function ``add(x: u32, y: u32) -> u32`` has a six-line doc comment and directly after a two-line comment which is not a doc-comment.

Those doc-comments are picked up by Rust tooling and used to generate documentation with the rustdoc tool. Every crate on `<http://crates.io/>`_, the standard Rust crate registry, automatically gets this documentation built for every release.

See `the heapless documentation <https://docs.rs/heapless/latest/heapless/>`_ as an example.

Read more about doc comments here: `<https://doc.rust-lang.org/rust-by-example/meta/doc.html>`_.

Doc-comments in the core library
""""""""""""""""""""""""""""""""

The core library makes heavy use of those doc-comments. Modules contain doc-comments that describe the functionality and structure in that module. Functions contain doc-comments that include a description of the behaviour, usage examples and safety comments. The Ferrocene compiler automatically enforces that all publicly exposed functions in the core library have a doc-comment through the missing_docs lint.

The doc-comments of both modules and functions are compiled, together with the function signatures, into the core library API docs.

Overall the doc-comments in the core library are very extensive, very high-quality and a lot of work has been and continues to be put into them.

That's why we want to rely on them for multiple purposes of the certification.

Requirements
~~~~~~~~~~~~

For requirements we will rely on the doc-comments plus the signature of the function. The signature describes the types of the input and output parameters, which are enforced by the compiler. The doc-comments describe the expected behaviour, which is tested by unit tests.

Doc-comments used as requirements must:

- Describe what the function does.
- Include one or more examples, which will be executed as doctests.
- Where applicable, include safety information.
- Where applicable, include panic information.

Architecture and software design
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The core library does not need a software architecture, due to its small size.

The core library uses the doc-comments of the modules as their module design.

Doc-comments used as module design must:

- Describe the purpose of the module.
- Describe the functionality included in it.

Quality of the doc-comments
~~~~~~~~~~~~~~~~~~~~~~~~~~~

The requirements for doc-comments used as requirements or module design are regularly checked. If gaps are found, the fixes will be upstreamed, which has the advantage of getting additional reviews by Rust experts and creating a consensus in the Rust community.

Requirement to test tracing
~~~~~~~~~~~~~~~~~~~~~~~~~~~

1. Firstly, the requirement of a function is the doc comment which is on top of that functions. Therefore the requirements is traced to the function.
2. Secondly, the certification relies on code coverage to ensure that each function is sufficiently covered by tests.
3. Combining one and two, if all functions are covered by tests, also all requirements are covered by tests. Therefore tests do not need to be manually traced to requirements.

Requirement identifier
~~~~~~~~~~~~~~~~~~~~~~

Each function has one doc-comment aka. one requirement. The module path of a function is unique, which is ensured by the compiler, and can therefore be used as an identifier for that requirement. Doc comments might change between versions, so to ensure uniqueness across versions, that requirement id is the combination of the version of Ferrocene and the module path of the function.

Requirement status
~~~~~~~~~~~~~~~~~~

A requirement is in one of three statuses: draft, approved, retired. If a requirement gets proposed via a pull request, it is in draft status. As soon as it is merged, the status is approved. If a pull request changes an existing requirement, the old requirement becomes retired. If a function gets marked as deprecated the requirement becomes retired as well.

Verification of Requirements
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

All requirements must fulfill the basic properties of good requirements:

- Atomic
- Unambiguous
- Complete
- Accurate
- Free from vague terms like "some”, "several”, "many”, "sufficient”, "reasonable", "any” etc.
- Technically and logically feasible

Private functions
~~~~~~~~~~~~~~~~~

Only public functions that are part of the certified subset must have an associated requirement. Functionality of a private function is usually included in the functionality described for the public function. Private function still must have full statement test coverage.

Testing
-------

See the :doc:`core-certification:testing-plan` for how the certified core library is tested.

Uncertified code
----------------

It has to be ensured that no uncertified code from the core library is being used in a customer project.

This is achieved in two steps.

Firstly, it is ensured that the certified subset only contains certified code.

Secondly, customers must ensure they only use code from the certified subset.

All uncertified code and certified code that is not called, is unused code.

Unused code
-----------

The qualified Ferrocene compiler ensures that no code that is not used in source code is being executed.

Additionally the compiler usually removes unused functions from the final binary. But this behavior is not specified and can therefore not be relied upon.

Tool safety assessments
-----------------------

All offline tools we use to develop the core library are listed below. The compiler is T3, all other tools are T2 or T1.

There are no online tools used to develop the core library.

For each tool there is a description of the usage of the tool for the core library, the version of the tool used as well as a safety assessment.

Code coverage
~~~~~~~~~~~~~

Version
"""""""

- blanket: |ferrocene_version| (in-tree)
- llvm-profdata: |llvm_version|
- rustc: |rust_version|
- symbol-report: |ferrocene_version| (in-tree)

Usage
"""""

See :ref:`testing-plan:Code coverage tests` for details of how it works.

Developer usage is described in :doc:`internal-procedures:code-coverage`.

Safety Assessment
"""""""""""""""""

- Tool Classification: T2
- Level of reliance: Low, it is not involved in ensuring correctness, but only a measure of quality of the test suite.

The instrumentation mechanism using ``-Cinstrument-coverage`` is the standard mechanism of collecting code coverage information in Rust. Since it is part of the LLVM suite of tools, it is not only used in Rust but also widely used in the C++ ecosystem. This widespread usage gives us confidence in the quality and robustness of the tooling.

``blanket`` and ``symbol-report`` are tools developed by Ferrous Systems. ``symbol-report`` uses information from the compiler to ensure all the functions from the certified subset are being considered for code coverage. ``blanket`` is built on top of the Rust library called ``llvm-profparser`` and uses the output of ``symbol-report``. This library is developed by the ``cargo-tarpaulin`` project, which is widely used to measure code coverage for Rust projects.

The tools are designed to not overcount code coverage.


Failure modes
'''''''''''''

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
~~~~~~~~

Version
"""""""

- rustc: |rust_version|

Usage
"""""

The qualified Ferrocene compiler is used to build the core library, which gives high confidence in its quality.

Nightly features
''''''''''''''''

The core library relies on a few so-called "nightly features" of the compiler. Regular users of Ferrocene are not allowed to use them, therefore they are not part of the compiler qualification.
This is because they are either "experimental" or "internal”. They do work well, but they can change between compiler versions and do not fall under the usual Rust stability guarantees.
This is not a problem for the core library, because rustc and the core library are developed, build and tested together.

Nightly features are activated by setting the ``RUSTC_BOOTSTRAP`` environment variable when executing ``rustc``.

Nightly features used by the core library are listed as ``#![feature(name_of_the_feature)]`` in ``library/core/src/lib.rs``.

Nightly features are tested by the ``compiletest`` test suite, by tests that activate that feature explicitly. E.g. ``tests/ui/unknown-language-item.rs`` tests ``#![feature(lang_items)]``.

Compiler built-in functions
'''''''''''''''''''''''''''

There are functions in the core library that are "compiler built-in”. That means they are not implemented in the core library codebase but in the compiler codebase.

They fall in two categories:

Macros
++++++

Macros generate different code on every use.

Customers have to ensure the generated code is correct. This is documented in the safety manual.

At the time of writing there are 60 compiler built-in macros (``rg "compiler built-in" library/core``). Not all of them are certified.

An example of such a built-in macro is ``pub macro Clone`` (`<https://github.com/ferrocene/ferrocene/blob/3ab6d2e0eb60057ec912d9619542ab590da45a51/library/core/src/clone.rs#L258-L260>`_).

Intrinsics
++++++++++

Intrinsics are "implementation details of ``core`` and should not be used outside of the standard library" (quote from the intrinsics module doc-comment).

All instrinsic function are in the ``intrinsics`` module and its submodules.

They are not availble in stable Rust and therefore cannot be used directly by customers.

At the time of writing there are 395 intrinsic function (``rg "fn" library/core/src/intrinsics``). Not all of them are certified.

An example of such a intrinsic function is ``fn unaligned_volatile_load<T>(src: *const T) -> T`` (`<https://github.com/ferrocene/ferrocene/blob/3ab6d2e0eb60057ec912d9619542ab590da45a51/library/core/src/intrinsics/mod.rs#L1050>`_).

Safety Assessment
"""""""""""""""""

- Tool Classification: T3

No assessment necessary, since the compiler is pre-qualified.

Linting
~~~~~~~

Version
"""""""

- clippy: |ferrocene_version|
- rustc: |rust_version|
- rustfmt: |ferrocene_version|

Usage
"""""

Upstream already has very good coding practices for the core library, which are enforced by the ``tidy`` test suite.
The ``tidy`` test suite executes rustc and clippy lints to enforce consistency in semantics and ``rustfmt`` to enforce consistency in syntax.

It does not make sense for us to come up with a separate coding standard and try to force it upon the upstream core library.
If we would start to come up with new rules from our coding standard we would have to work against upstream and either convince them to refactor their code without a clear benefit for them or we would have to carry a big changeset which has a big potential to introduce bugs.

Safety Assessment
"""""""""""""""""

- Tool Classification: T1
- Level of reliance: Low, the lints are not involved in ensuring correctness, but only a measure of quality of the source code. (Note: ``rustc`` is involved in ensuring correctness, but here we only look at it in its capacity of a linter, not a compiler.)

``clippy``, ``rustc`` and ``rustfmt`` are standard tools in the Rust ecosystem. There are used in virtually every Rust project. This gives high confidence in its quality.

Failure modes
~~~~~~~~~~~~~

- False-negative: Fail to detect non-compliance with the consistency rules
   - Risk: Diverging from consistency rules. This is not critical, because Ferrous Systems only consumes the code from upstream and does not impose additional rules on it.
   - Mitigation: None. If found, report issue upstream.
- False-positive: Report non-compliance, although the code is compliant
   - Risk: None
   - Mitigation: Report issue upstream.

Test runner
~~~~~~~~~~~

Version
"""""""

- libtest: |ferrocene_version|

Usage
"""""

The libtest test runner compiles all tests specified in the coretests test suite into an executable that executes the tests and reports if the test results are as expected.

Safety Assessment
"""""""""""""""""

- Tool classification: T2
- Level of reliance: High, ensures correctness of the test results.

``libtest`` is used extensively by virtually every user of Rust, since it powers the common ``cargo test`` command. Heavy users of it include the upstream Rust project and Ferrous Systems which uses it in the rustc compiler qualification. Both upstream and Ferrous Systems execute thousands of tests with it, every day. Therefore there is a high chance of a bug in libtest being detected.

Failure modes
~~~~~~~~~~~~~

- False-positive: Report test as successful, although it is failing
   - Risk: Not detect incorrect code.
   - Mitigation: Report issue upstream.
- False-negative: Report test as failing, although it is successful
   - Risk: None
   - Mitigation: Report issue upstream.

Version control system
~~~~~~~~~~~~~~~~~~~~~~

Version
"""""""

- git: version 2
- GitHub: GitHub Enterprise version 3

Usage
"""""

``git`` is being used to track changes, with GitHub as a remote repository.

Safety Assessment
"""""""""""""""""

- Tool classification: T2
- Level of reliance: Medium

Git and GitHub are very very widely used tools. This gives us confidence in its quality.

Failure modes
~~~~~~~~~~~~~

- False-positive: Introduce changes that were not made
   - Risk: Erroneous code, documentation, configuration
   - Mitigation: Code review.
- False-negative: Do not track changes that were made
   - Risk: Lose time invested.
   - Mitigation: Code review.

``rustdoc``
~~~~~~~~~~~

Version
"""""""

- rustdoc: |ferrocene_version|

Usage
"""""

``rustdoc`` is used to generate the API documentation from source code as well as generating the symbols for the code coverage report.

Safety Assessment
"""""""""""""""""

- Tool classification: T2
- Level of reliance: Medium

``rustdoc`` is the standard tool to generate documentation of Rust libraries and is very widely used. Each version of each crate published on `<https://crates.io>`_ automatically gets its documentation build by ``rustdoc`` and published on `<https://doc.rs>`_. This means it is executed hundreds of times per day for a wide variety of crates and documentations. This wide and diverse usage gives high confidence in its quality and robustness.

Failure modes
~~~~~~~~~~~~~

- Modify generated documentation
   - Risk: Erroneous documentation
   - Mitigation: If detected, report error.

Failure analysis
----------------

The HazOp was re-evaluated with the core library in mind and core library-specific additions were made. See :doc:`evaluation-report:rustc/tool-analysis`.
