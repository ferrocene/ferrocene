.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Safety Plan
===========

Purpose of this document
------------------------

This document describes the approach and the activities to be performed to achieve safety certification according to the safety standard(s) described later in this document.

Certification scope
-------------------

The core library shall be suitable to be used in safety applications according to IEC 61508, up to SIL 2.

Therefore, we are evaluating the core library through an "assessment of non-compliant development” (according to Route 3S of 7.4.2.12). This assessment targets a full compliance statement to IEC 61508, SIL2, as far as it is applicable for a Software Safety Element out of Context.

The targeted version of the core library is |rust_version|, and will apply to all qualified targets of Ferrocene :doc:`user-manual:targets/index`.

Project Setup
-------------

Timeline
~~~~~~~~

Our first milestone is to be "certification-ready” by July 2025. Because of the short timeline we will focus on solutions that prioritize feasibility over user convenience. The second milestone will focus on convenience, tooling, and automation.

Release cadence
~~~~~~~~~~~~~~~

Due to the use of internal apis, the core library versions are only compatible with one matching Ferrocene release. The core library will be recertified for every Ferrocene release.

Roles and responsibilities
~~~~~~~~~~~~~~~~~~~~~~~~~~

Roles and responsibilities (e.g. Safety Manager, Product Manager) are documented at :doc:`qualification-plan:organization`.

Lifecycle Phases Overview
~~~~~~~~~~~~~~~~~~~~~~~~~

1. Pull changes to the core library codebase, as part of daily upstream pulls (see :doc:`internal-procedures:upstream-pulls`).
2. On a regular cadence, latest before the release, examine the diff and follow the code-review checklist (see below)

The first time this is executed it will be a lot of work because we go from zero to the first subset of the core library. In the future the changes will be smaller because they will only be changes to the existing subset and potentially new functions added to the subset.

Code-Review checklist
"""""""""""""""""""""

On every upstream pull we need to review:

- no uncertified code is called from certified code
- quality of doc-comments
- changes to doc-comments
- tests the adherence to the coding standard (rustc lints + rustfmt)
- 100% statement code coverage

In the long run we want to automate as much as possible. In the beginning many of the checks will be manual.

Internal procedures
~~~~~~~~~~~~~~~~~~~

Describes the internal engineering procedures for the Ferrocene project, based on software engineering best practices, to be updated upon detection of shortcomings in the development process.

See :doc:`internal-procedures:index`.

Deliverables and Documents
--------------------------

We are delivering following documents to the assessor and to customers:

- Product Documentation
   - User Manual
   - The core library API docs, which include
      - Requirements (the function doc-comments)
      - Software Design (the module doc-comments)
- Functional Safety Management
   - Safety Plan
   - Norm Mapping
   - Review Evidence for requirements and software design
   - Safety Manual
   - Test Plan, Test cases and Test results
- Binaries in the delivery

Requirements Management
-----------------------

Structure of Requirements
~~~~~~~~~~~~~~~~~~~~~~~~~

The Doc-comments described below cover the single level of functional requirements for each function. The documentation for each module also covers the purpose and overview, and as such is suitable for the design requirement. We don't have an architecture document due to the small size of the core library, as well as the fact that it is pre-existing software.

Doc-comments in general
"""""""""""""""""""""""

Rust has a concept called "doc-comments” also known as documentation comments. They are denoted by triple-slashes, while normal comments are denoted by double-slashes. They support markdown, and code inside code blocks is automatically run as tests, to ensure the code and docs strings do not get out of sync.

For example:

.. code-block:: rust
  :linenos:

  /// Add two `u32`s.
  /// ```
  /// assert_eq!(add(1, 5), 6);
  /// ```
  /// This is a doc-comment
  //
  // This is not a doc-comment
  fn add(x: u32, y: u32) -> u32 { /* */ }

Those doc-comments are picked up by Rust tooling and used to generate documentation with the rustdoc tool. Every crate on [crates.io](http://crates.io/), the standard Rust crate registry, automatically gets this documentation built.

See [the heapless documentation](https://docs.rs/heapless/latest/heapless/) as an example.

Read more about doc comments here: <https://doc.rust-lang.org/rust-by-example/meta/doc.html>.

Doc-comments in the core library
""""""""""""""""""""""""""""""""

The core library makes heavy use of those doc-comments. Modules contain doc-comments that describe the functionality and structure in that module. Functions contain doc-comments that include a description of the behaviour, usage examples and safety comments. The Ferrocene compiler automatically enforces that all publicly exposed functions in the core library have a doc-comment through the missing_docs lint.

The doc-comments of both modules and functions are compiled, together with the function signatures, into the core library API docs.

Overall the doc-comments in the core library are very extensive, very high-quality and a lot of work has been and continues to be put into them.

That's why we want to rely on them for multiple purposes of the certification, after making sure the following conditions are met:

- Each method must have a description of what it does.
- Each method must state the return type of the method, and the types of each argument it takes.
- Each method should list one or more useful examples as verified doctests.
- Where applicable, each method should reference safety information.
- Where applicable, each method should reference panic information.

Architecture and software design
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The core library does not need a software architecture (see above).

But it needs a software design document. Here we are going to build upon the doc-comments of the modules. Those doc-comments already describe the design of those modules.

Requirements
~~~~~~~~~~~~

For requirements we will rely on the doc-comments plus the signature of the function. The signature describes the types of the input and output parameters, which are enforced by the compiler. The doc-comments describe the expected behaviour, which is tested by unit tests.

Quality of the doc-comments
~~~~~~~~~~~~~~~~~~~~~~~~~~~

We will assess the quality of both the module and function doc-comments and their fitness for usage as software design and requirements respectively, via a design standard.

Note that only functions, methods, and trait methods are reviewed according to the standard, as trait method definitions do not have independent documentation from their defining trait method.

If we find gaps we will upstream the solutions, which has the advantage of getting additional reviews by Rust experts and creating a consensus in the Rust community.

We will track if the doc-comments change, because that could mean our requirements change and tests need to be updated. Paying close attention to changes to doc-comments is part of the review checklist.

Requirements and tests
----------------------

One of the major pieces of work is going to be having requirements and tests for the certified core library subset.

Subset of the core library
~~~~~~~~~~~~~~~~~~~~~~~~~~

The subset included in the safety certification is defined and documented in the Safety Manual.

Requirements
~~~~~~~~~~~~

As described above we are going to build upon the doc-comments as requirements.

Tracing
"""""""

Firstly, our requirements are doc comments which are on top of functions, therefore the requirements are already traced to functions. Secondly, we are relying on code coverage to ensure that functions are covered by tests. Combining this, if all functions are covered by tests, also all requirements are covered by tests. Therefore we do not need to manually trace tests to requirements.

Requirement identifier
""""""""""""""""""""""

Each function has one doc-comment aka. one requirement. The module path of a function is unique, which is ensured by the compiler, and can therefore be used as an identifier for that requirement. Doc comments might change between versions, so to ensure uniqueness across versions, that requirement id is the combination of version and module path of the function.

Requirement status
""""""""""""""""""

A requirement is in one of three statuses: draft, approved, retired. If a requirement gets proposed via a pull request, it is in draft status. As soon as it is merged, the status is approved. If a pull request changes an existing requirement, the old requirement becomes retired. If a function gets marked as deprecated the requirement becomes retired as well.

Verification of Requirements
""""""""""""""""""""""""""""

All requirements must fulfill the basic properties of good requirements:

- Atomic
- Unambiguous
- Complete
- Accurate
- Free from vague terms like "some”, "several”, "many”, "sufficient”, "reasonable", "any” etc.
- Technically and logically feasible

The adherence to these basic properties are checked during diff review.

Tests
-----

For tests we will rely on the existing coretests test suite. Gaps in code coverage will be filled by adding tests to the coretests test suite.

Tests must cover all requirements specified for the safety certification scope and the defined reactions to unexpected inputs or behaviour. If functionality or failure reactions cannot be tested, the code will be inspected by a code review that will be documented.

Code with SIL2 systematic capability
------------------------------------

All public functions of the certified subset are considered "software safety functions” and are going to be certified for SIL 2. That means our customers can use all of those functions for SIL 2 use cases. Since we consider all of them safety relevant we do not consider independence. Usually for independence we would have to prove that non-safety functions do not impact safety functions, but since all functions in the subset are safety functions this is not a problem.

The systematic capability of these functions is based on:

- The requirements and the documented completeness of these requirements and their implementation in the code and test
- The absence of any undocumented and untested code in the safety certification scope
- The required test coverage
- The adherence of the code within the safety scope to the Coding Guidelines

Private functions
-----------------

We will first and foremost specify and test the public functions that are part of our subset. Functionality of a private function is usually included in the functionality described for the public function and is covered by overall statement test coverage.

Uncertified code
----------------

We need to make sure no uncertified code is being used. This means for us, code that is not part of the safety certification effort where we do not yet provide evidence for requirements and/or sufficient test coverage.

We achieve this by instructing customers to verify that they only call functions from the certified subset in their source code. Initially via providing a list of certified functions, and moving on to tooling and automation in the second half of the year 2025 for the next release and its certification.

We will ensure that all certified functions, and functions called by those certified functions, are 100% statement-covered by tests and described by requirements and design. In best case through tests of the certified functions, but maybe also through tests of the private functions (see "Private functions” section).

All uncertified functions and certified functions that are not called are unused code (see "Unused code”).

Unused code
-----------

We rely on the qualified Ferrocene compiler to ensure that only functions that are intentionally called by the customer and functions called by those intentionally called functions are used. If the compiler fails to do that correctly that is a problem with the compiler qualification and not the core library certification.

The compiler usually removes unused functions, but that behavior is not specified and can therefore not be relied upon.

Tool safety assessments
-----------------------

All offline tools we use to develop the core library are listed below. The compiler is T3, all other tools are T2 or T1.

There are no online tools used to develop the core library.

For each tool there is a description of the usage of the tool for the core library, the version of the tool used as well as a safety assessment.

Code coverage
~~~~~~~~~~~~~

Version
"""""""

- grcov: 0.8.20
- llvm-cov: LLVM version 19.1.6-rust-1.86.0-nightly
- rustc: |ferrocene_version|

Usage
"""""

1. ``rustc`` is instructed to instrument the binary by passing ``-Cinstrument-coverage``.
2. The ``coretests`` test suite is executed. Due to the instrumentation, this will create ``.profraw`` files that contain the coverage information.
3. ``llvm-cov`` is used to merge the multiple raw coverage files into one ``info`` file with all the coverage information.
4. ``grcov`` is used to generate the HTML report from the ``info`` file.

Developer usage is described in :doc:`internal-procedures:code-coverage`.

Code coverage is measured only on one platform, x86_64-unknown-linux-gnu. This is sufficient because the the code of the core library is largely platform independent and code coverage is only a measure for the quality of the test suite, the correctness is still tested by running the tests on all qualified targets.

Safety Assessment
"""""""""""""""""

- Tool Classification: T2
- Level of reliance: Low, it is not involved in ensuring correctness, but only a measure of quality of the test suite.

The instrumentation mechanism using ``-Cinstrument-coverage`` and ``llvm-cov`` is the standard mechanism of collecting code coverage information in Rust. But, since it is part of the LLVM suite of tools, it is not only used in Rust but also widely used in the C++ ecosystem. This widespread usage gives us confidence in the quality and robustness of the tooling.

``grcov`` is a tool that builds on top of ``llvm-cov`` and adds functionality to simplify the generation of a coverage report. It developed by Mozilla to collect code coverage information for the Firefox browser, and is widely used in the Rust ecosystem. The widespread usage and that it is developed by Mozilla, a trustworthy vendor, gives us confidence it its usage.

Failure modes
'''''''''''''

- False-positive A function is reported as covered, although it is not covered
  - Risk: Overreporting, could result in testing gap.
  - Mitigation: No mitigation, since we assume the likeliehood of such an error low.
- False-negative: A function is reported as not covered, although it is covered
  - Risk: Underreporting, will not result in testing gap.
  - Mitigation: Since we want to achieve 100% line coverage this would stand out and be manually investigated.
- The code coverage instrumentation introduces bugs into the test runner
  - Risk: That results in failing tests being reported as successful or successful tests being reported as failing
  - Mitigation: Running the test suite once with and once without code coverage instrumentation and ensuring both report the same result.

Compiler
~~~~~~~~

Version
"""""""

- rustc: |ferrocene_version|

Usage
"""""

The qualified Ferrocene compiler is used to build the core library, which gives high confidence in its quality.

Nightly features
''''''''''''''''

The core library relies on a few so-called "nightly features" of the compiler. Regular users of Ferrocene are not allowed to use them, therefore they are not part of the compiler qualification.
This is because they are either "experimental" or "internal”. They do work well, but they can change between compiler versions and do not fall under the usual Rust stability guarantees.
This is not a problem for the core library, because rustc and the core library are developed and tested together.

Nightly features are activated by setting the ``RUSTC_BOOTSTRAP`` environment variable when executing ``rustc``.

Nightly features used by the core library are listed as ``#![feature(name_of_the_feature)]`` in ``library/core/src/lib.rs``.

Nightly features are tested by the ``compiletest`` test suite, by tests that activate that feature explicitly. E.g. ``tests/ui/unknown-language-item.rs`` tests ``#![feature(lang_items)]``.

Compiler built-in functions
'''''''''''''''''''''''''''

There are functions in the core library that are "compiler built-in”. That means they are not implemented in the library codebase but in the compiler codebase. They can be found by searching for "compiler built-in” in the ferrocene repository (e.g. ``rg "compiler built-in" library/core``).

All of those functions are macros. They generate different code on every use. Customers have to ensure the generated code is correct. This is documented in the safety manual.

At the time of writing there are 59 such functions. An example of such a function is ` ``pub macro Clone`` <https://github.com/ferrocene/ferrocene/blob/c711094a96c03fc27f98d58e2bf85a1ab6996940/library/core/src/clone.rs#L184>`_.

Safety Assessment
"""""""""""""""""

- Tool Classification: T3

No assessment necessary, since the compiler is pre-qualified.

Linting
~~~~~~~

Version
"""""""

- clippy: |ferrocene_version|
- rustc: |ferrocene_version|
- rustfmt: |ferrocene_version|

Usage
"""""

Upstream already has very good coding practices for the core library, which are enforced by the tidy test suite.
The "tidy” test suite executes rustc and clippy lints to enforce consistency in semantics and rustfmt to enforce consistency in syntax.

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
   - Mitigation: None. If found, report issue upstream.
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

- False-positive: Introduce changes, that were not made
   - Risk: Erroneous code, documentation, configuration
   - Mitigation: Code review.
- False-negative: Do not track changes, that were made
   - Risk: Loose time invested.
   - Mitigation: Code review.

``rustdoc``
~~~~~~~~~~~

Version
"""""""

- rustdoc: |ferrocene_version|

Usage
"""""

``rustdoc`` is used to generate the API documentation from source code as well as generating the spreadsheet of all functions in the subset.

Safety Assessment
"""""""""""""""""

- Tool classification: T2
- Level of reliance: Medium

``rustdoc`` is the standard tool to generate documentation of Rust libraries and is very widely used. Each version of each crate published on <https://crates.io> automatically gets its documentation build by ``rustdoc`` and published on <https://doc.rs>. This means it is executed hundreds of times per day for a wide variety of crates and documentations. This wide and diverse usage gives high confidence in its quality and robustness.

Failure modes
~~~~~~~~~~~~~

- Modify generated documentation
   - Risk: Erroneous documentation
   - Mitigation: If detected, report error.

Qualification targets
---------------------

We certify the core library for all compilation targets rustc is qualified for. We already run the core library test suite for all qualified targets in our CI. So there is no additional work that needs to be done here.

Failure analysis
----------------

The HazOp was re-evaluated with the core library in mind and core library-specific additions were made. See :doc:`evaluation-report:rustc/tool-analysis`.
