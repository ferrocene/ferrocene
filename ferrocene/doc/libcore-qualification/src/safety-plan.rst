.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Safety Plan
===========

Purpose of this document
------------------------

This document describes the approach and the activities to be performed to achieve safety certification according to the safety standard(s) described later in this document.

Certification scope
-------------------

Libcore shall be suitable to be used in safety applications according to IEC 61508, up to SIL 2.

Therefore, we are evaluating libcore through an “assessment of non-compliant development” (according to Route 3S of 7.4.2.12). This assessment targets a full compliance statement to IEC 61508, SIL2, as far as it is applicable for a Software Safety Element out of Context.

The targeted libcore version for this first safety certification is 1.87, and will apply to all qualified targets of Ferrocene <https://public-docs.ferrocene.dev/main/user-manual/targets/index.html#qualified-targets> w

Project Setup
-------------

Timeline
~~~~~~~~

Our first milestone is to be “certification-ready” by July 2025. Because of the short timeline we will focus on solutions that prioritize feasibility over user convenience. The second milestone will focus on convenience, tooling, and automation.

Release cadence
~~~~~~~~~~~~~~~

Due to the use of internal apis, libcore versions are only compatible with one matching Ferrocene release. Libcore will be recertified for every Ferrocene release.

Roles and responsibilities
~~~~~~~~~~~~~~~~~~~~~~~~~~

Roles and responsibilities (e.g. Safety Manager, Product Manager) are documented at <https://public-docs.ferrocene.dev/main/qualification/plan/organization.html>.

Lifecycle Phases Overview
~~~~~~~~~~~~~~~~~~~~~~~~~

1. Pull changes to libcore codebase, as part of daily upstream pulls (see <https://public-docs.ferrocene.dev/main/qualification/internal-procedures/upstream-pulls.html>
2. On a regular cadence, latest before the release, examine the diff and follow the code-review checklist (see below)

The first time this is executed it will be a lot of work because we go from zero to the first subset of libcore. In the future the changes will be smaller because they will only be changes to the existing subset and potentially new functions added to the subset.

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

See <https://public-docs.ferrocene.dev/main/qualification/internal-procedures/index.html>

Deliverables and Documents
--------------------------

We are delivering following documents to the assessor and to customers:

- Product Documentation
   - User Manual
   - The libcore API docs, which include
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

Structure of Requirements in Libcore
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

The Doc-comments described below cover the single level of functional requirements for each function. The documentation for each module also covers the purpose and overview, and as such is suitable for the design requirement. We don't have an architecture document due to the small size of libcore, as well as the fact that it is pre-existing software.

Doc-comments in general
"""""""""""""""""""""""

Rust has a concept called “doc-comments” also known as documentation comments. They are denoted by triple-slashes, while normal comments are denoted by double-slashes. They support markdown, and code inside code blocks is automatically run as tests, to ensure the code and docs strings do not get out of sync.

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

Doc-comments in libcore
"""""""""""""""""""""""

The core library makes heavy use of those doc-comments. Modules contain doc-comments that describe the functionality and structure in that module. Functions contain doc-comments that include a description of the behaviour, usage examples and safety comments. libcore automatically enforces that all publicly exposed functions have a doc-comment through the missing_docs lint.

The doc-comments of both modules and functions are compiled, together with the function signatures, into the libcore API docs.

Overall the doc-comments in libcore are very extensive, very high-quality and a lot of work has been and continues to be put into them.

That's why we want to rely on them for multiple purposes of the certification, after making sure the following conditions are met:

- Each method must have a description of what it does.
- Each method must state the return type of the method, and the types of each argument it takes.
- Each method should list one or more useful examples as verfied doctests.
- Where applicable, each method should reference safety information.
- Where applicable, each method should reference panic information.

Architecture and software design
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

libcore does not need a software architecture (see above).

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

One of the major pieces of work is going to be having requirements and tests for the certified libcore subset.

Subset of Libcore
~~~~~~~~~~~~~~~~~

The initial certification will be of a specific subset of libcore, the exact subset still to be determined. We expect the subset to contain around 1000 functions.

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
- Free from vague terms like “some”, “several”, “many”, “sufficient”, "reasonable", “any” etc.
- Technically and logically feasible

The adherence to these basic properties are checked during diff review.

Tests
-----

For tests we will rely on the existing coretests test suite. Additionally, we will fill gaps in the tests with a combination of tests generated by the “Testify” tool from Cryspen, and hand-written tests. With Testify we can generate test cases based on “contracts” the function has to uphold. Their tool combines strategy and fuzzing to generate extensive test cases.

Tests must cover all requirements specified for the safety certification scope and the defined reactions to unexpected inputs or behaviour. If functionality or failure reactions cannot be tested, the code will be inspected by a code review that will be documented.

Code with SIL2 systematic capability
------------------------------------

All public functions of the certified subset are considered “software safety functions” and are going to be certified for SIL 2. That means our customers can use all of those functions for SIL 2 use cases. Since we consider all of them safety relevant we do not consider independence. Usually for independence we would have to prove that non-safety functions do not impact safety functions, but since all functions in the subset are safety functions this is not a problem.

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

We achieve this by instructing customers to verify that they only call functions from the certified subset in their source code. Initially via providing a list of certified functions, and moving on to tooling and automation in the second half of the year for the next release and its certification.

We will ensure that all certified functions, and functions called by those certified functions, are 100% statement-covered by tests and described by requirements and design. In best case through tests of the certified functions, but maybe also through tests of the private functions (see “Private functions” section).

All uncertified functions and certified functions that are not called are unused code (see “Unused code”).

Unused code
-----------

We rely on the qualified Ferrocene compiler to ensure that only functions that are intentionally called by the customer and functions called by those intentionally called functions are used. If the compiler fails to do that correctly that is a problem with the compiler qualification and not the libcore certification.

The compiler usually removes unused functions, but that behavior is not specified and can therefore not be relied upon.

Tool safety assessments
-----------------------

We need to describe how we use the tools we use.

We don't need a generic qualification of the tool, because we do not need to argue why it is safe to use in the general case, but just for our use case(s).

Tools we use and need to describe their usage:

- code coverage tool
- compiler (rustc)
- linter (for coding standard)
- test runner

The compiler is T3, all other tools are T2 or T1.

For each tool there is a safety assessment below with the level of reliance placed upon the tool, potential failure modes, and for each of those the associated risk, mitigation and constraints.

Code coverage
-------------

Tool Classification: T2

We need full statement coverage to show the quality of our test suite.

But we only need coverage for the certified subset and all code called by it.

For the code coverage tool we could add a test that checks that it analyses the correct code coverage for a project with known code coverage.

We only measure code coverage on one platform, x86_64-unknown-linux-gnu. This is sufficient because the libcore code is largely platform independent and code coverage is only a measure for the quality of the test suite, the correctness is still tested by running the tests on all qualified targets.

Usage described in <https://public-docs.ferrocene.dev/main/qualification/internal-procedures/code-coverage.html>

Safety Assessment
~~~~~~~~~~~~~~~~~

- Level of reliance
  - Low, it is not involved in ensuring correctness, but only a measure of quality of the test suite.
- Failure modes
   - A function could be reported as covered, although it is not
      - Risk: Overreporting, could result in testing gap
      - Mitigation: No mitigation. The tool is used widely, including by its developer Mozilla. Additionally, the level of reliance on the tool for libcore certification is low.
      - Constraints: No constraint introduced.
   - A function could be reported as uncovered, although it is
      - Risk: Underreporting, will not result in testing gap
      - Mitigation: Manually annotate the function with the test case it is tested by.
      - Constraints: No constraint introduced.
   - The code coverage instrumentation introduces bugs into the test runner
      - Risk: that results in failing tests being reported as successful or successful tests being reported as failing
      - Mitigation: Running the test suite once with and once without code coverage instrumentation and ensuring both report the same result.
      - Constraints: No constraint introduced.

Compiler
--------

Tool Classification: T3

Tool Qualification: certified compiler

We use the qualified Ferrocene compiler to compile libcore, which gives us high confidence in its quality.

Nightly features
~~~~~~~~~~~~~~~~

libcore relies on a few so-called nightly features of the compiler. Those nightly features are not exposed to normal users because they are “internal”. They work well if used according to their instructions. But they can change between compiler versions and do not fall under the usual Rust stability guarantees. This is not a problem for libcore though, because rustc and libcore are developed by the same group of people and are tested together.

We are activating the nightly features libcore uses via the RUSTC_BOOTSTRAP environment variable.

We qualify RUSTC_BOOTSTRAP and those nightly compiler features as an add-on qualification specific to libcore. Therefore we do not change the rustc qualification, but in the tools section of the libcore certification we specify an additional use case for rustc and provide qualification evidence (aka. specify and test it).

We will list the nightly features that are being used by libcore. If the nightly features that are used change, the list will be updated.

We're going to trace test cases that enable any of those nightly features to that nightly feature and are going to make sure that every nightly feature is covered by at least one test.

Nightly features are not available to customers, and are not present in the version of libcore we provide.

Compiler built-in functions
~~~~~~~~~~~~~~~~~~~~~~~~~~~

There are functions in libcore that are “compiler built-in”. That means they are not implemented in the library codebase but in the compiler codebase. You can find them by searching for “compiler built-in” in the ferrocene repository (e.g. rg "compiler built-in" library/core).

At the time of writing there are 59 such functions. An example of such a function is [clone](https://github.com/ferrocene/ferrocene/blob/c711094a96c03fc27f98d58e2bf85a1ab6996940/library/core/src/clone.rs#L184).

All of those functions are macros. They generate different code on every use. Customers have to ensure the generated code is correct. This will be documented in the safety manual.

Safety Assessment
~~~~~~~~~~~~~~~~~

No assessment necessary, since the compiler is pre-qualified.

Linter/Coding standard
----------------------

Tool Classification: T2

For linting, we qualify what libcore already does. In CI the “tidy” test suite is run, which executes rustc lints to enforce consistency in semantics and rustfmt to enforce consistency in syntax.

libcore already has very good coding practices that are enforced through tooling and tests.

It does not make sense for us to come up with a separate coding standard and try to force it upon upstream libcore.

If we would start to come up with new rules from our coding standard we would have to work against upstream and either convince them to refactor their code without a clear benefit for them or we would have to carry a big changeset which has a big potential to introduce bugs.

If necessary we can create a document to describe what rules libcore enforces regarding syntax and semantics and update that whenever upstream libcore updates their rules.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Level of reliance: Low, it is not involved in ensuring correctness, but only a measure of quality of the source code.
- Failure modes
   - Fail to detect non-compliance with the consistency rules
      - Risk: Diverging from consistency rules. This is not critical, because Ferrous Systems only consumes the code from upstream and does not impose additional rules on it.
      - Mitigation: None. If found, report issue upstream.
      - Constraints: No constraint introduced.
   - Report non-compliance, although the code is compliant
      - Risk: None
      - Mitigation: Report issue upstream
      - Constraints: No constraint introduced.

Test runner
-----------

Tool classification: T2

The libtest test runner compiles all tests specified in the coretests test suite into an executable that executes the tests and reports if the test result is as expected.

Safety Assessment
~~~~~~~~~~~~~~~~~

- Level of reliance: High, ensures correctness of the test results.
- Failure modes
   - False-positive: Report test as successful, although it is failing
      - Risk: Not detect incorrect code. There is a high to medium chance of such a bug in libtest being detected, because libtest is used extensivly by the upstream Rust project, by Ferrous Systems in the rustc compiler qualification and by all users of Rust.
      - Mitigation: None. If found, report issue upstream.
      - Constraints: No constraint introduced.
   - False-negative: Report test as failing, although it is successful
      - Risk: None
      - Mitigation: Report issue upstream.
      - Constraints: No constraint introduced.

Qualification targets
---------------------

We qualify libcore for all compilation targets rustc is qualified for. We already run the libcore test suite for all qualified targets in our CI. So there is no additional work that needs to be done here.
