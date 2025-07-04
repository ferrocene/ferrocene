.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Norm mapping of IEC-61508 standard requirements
===============================================

IEC-61508-1
-----------

5.2.1
~~~~~

Covered by “Lifecycle phases overview” of the libcore safety and certification plan.

5.2.2
~~~~~

Covered by “Lifecycle phases overview” and “Roles and responsibilities” of libcore safety and certification plan.

5.2.3
~~~~~

Covered by “Lifecycle phases overview” and “Release Cadence” of the libcore safety and certification plan.

5.2.4
~~~~~

TODO

Covered by
    • certified libcore API documentation. This has not yet been published.
    • libcore code coverage report. This has not yet been published.
        ◦ Here is the current code coverage report for whole libcore, including uncertified code: https://public-docs.ferrocene.dev/main/coverage/library-x86_64-unknown-linux-gnu/index.html 
    • libcore subset quality review. This is work in progress.

5.2.5
~~~~~

Docs will be available in a libcore subsection of the public ferrocene docs. This is work in progress.

5.2.6
~~~~~

https://public-docs.ferrocene.dev/main/qualification/plan/validation.html#documentation-validation

5.2.7
~~~~~

As the other docs in https://public-docs.ferrocene.dev/main/index.html

5.2.8
~~~~~

We use processes from the rustc qualification, unless specific needs for libcore certification require otherwise.

5.2.9
~~~~~

https://public-docs.ferrocene.dev/main/qualification/plan/change-tracking.html

5.2.10
~~~~~~

Changes to libcore are included in the Ferrocene release notes. When new releases of libcore, which is released with Ferrocene, include new functionality such as an expanded subset, this will be covered in the release notes.

5.2.11
~~~~~~

Covered by
    • https://public-docs.ferrocene.dev/main/qualification/plan/validation.html#documentation-validation
    • And “Roles and Responsibilities” in the libcore safety plan

6.2
~~~

See subsections.

6.2.1
~~~~~

Covered by “Roles and Responsibilities” in the libcore safety plan

6.2.2
~~~~~

Ferrous Systems is ISO 9001-2015 certified https://public-docs.ferrocene.dev/main/qualification/plan/organization.html#ferrocene-organization

6.2.3
~~~~~

Covered by Roles and Responsibilities in libcore safety and certification plan

6.2.4
~~~~~

Specified in https://public-docs.ferrocene.dev/main/safety-manual/customer-interactions.html 

6.2.5
~~~~~

Specified in https://public-docs.ferrocene.dev/main/qualification/plan/patching.html

6.2.6
~~~~~

See Internal Procedures in libcore safety and certification plan

6.2.7 
~~~~~

Covered by Release cadence in libcore safety and certification plan

6.2.8
~~~~~

https://public-docs.ferrocene.dev/main/qualification/plan/development.html#development-process

6.2.9
~~~~~

https://problems.ferrocene.dev/ will be updated to contain issues relevant to libcore. Existing issues are already tracked and explicitly ignored. A developer on the Ferrous team will process and review them.

6.2.10
~~~~~~

https://public-docs.ferrocene.dev/main/qualification/plan/infrastructure.html

6.2.11
~~~~~~

N/A

6.2.12
~~~~~~

https://public-docs.ferrocene.dev/main/qualification/plan/organization.html 

6.2.13
~~~~~~

Ferrous Systems maintains a database of staff competencies consisting of their CVs as well as any training provided by Ferrous Systems. Prior to assigning major tasks, leadership verifies the competencies of the respective staff.
Ferrous Systems’s ISO-9001 managed internal handbook details how staff may undertake training for new skills, or re-training for existing skills. 

6.2.14
~~~~~~

When assigning staff to projects, leadership verifies that their experience, training, decision making authority, responsibilities, and level of supervision required are a fit. Where appropriate, Ferrous Systems assigns more experienced staff to work alongside less experienced staff to facilitate hands-on training.

6.2.15
~~~~~~

Ferrous Systems maintains a database of staff competencies consisting of their CVs as well as any training provided by Ferrous Systems.

6.2.16
~~~~~~

Detailed in Ferrous Systems’s ISO-9001 managed internal handbook and re-checked for each renewal of certification.

6.2.17
~~~~~~

N/A

6.2.18
~~~~~~

See Release cadence in Libcore Safety and certification Plan

7.4.2
~~~~~

We will analyze failure modes of libcore, potentially through a HazOp.

7.5.2
~~~~~

https://public-docs.ferrocene.dev/main/safety-manual/known-problems.html

IEC-61508-3
-----------

4
~

The “Certification scope” in the libcore safety plan specifies and justifies the SIL level.

The libcore certification excludes the requirement of having an architecture, due to the simple design of libcore. It is a library of independent functions with no internal state management. 

Quoting the libcore API docs: “The core library is minimal: it isn’t even aware of heap allocation, nor does it provide concurrency or I/O. These things require platform integration, and this library is platform-agnostic.”

5
~

Covered in sections 5.2.1 to 5.2.11 of chapter IEC 61508-1.

6.2.1
~~~~~

Covered in section 6.2 of chapter IEC 61508-1.

6.2.2
~~~~~

Covered in the libcore safety plan.

6.2.3
~~~~~

Covered in
    • https://public-docs.ferrocene.dev/main/qualification/plan/development.html 
    • https://public-docs.ferrocene.dev/main/qualification/plan/change-tracking.html 
    • https://public-docs.ferrocene.dev/main/qualification/plan/infrastructure.html

7.1.2.1-7.1.2.3
~~~~~~~~~~~~~~~

“Lifecycle Phases Overview” of the libcore safety plan

7.1.2.4-7.1.2.6
~~~~~~~~~~~~~~~

We diverge from the V-Model, because we are certifying an existing project, not developing the code from scratch.

To explain it in V-model-terms: The Rust project, who is maintaining the upstream libcore, performs the requirement specification, the software architecture, the software design, the module design and the coding. Ferrous Systems, consumes the output of those activities from upstream and performs module testing, integration testing, and validation testing on the code received from upstream pull requests.

7.1.2.7
~~~~~~~

See section “IEC-61508-3 Annex A”.

7.1.2.8-7.1.2.9
~~~~~~~~~~~~~~~

The following relevant artefacts are included in the deliverables:

* Code coverage report
* Test results
* Doc-comment review
* Libcore subset API docs

7.2.2.1-7.2.2.3
~~~~~~~~~~~~~~~

See Requirements management in the Libcore safety plan.

7.2.2.4
~~~~~~~

Libcore is a library of independent functions with no internal state management. And all functions included in the subset are certified SIL2.

7.2.2.5
~~~~~~~

Ferrous Systems certifies libcore as a library, to be used in other systems whose requirements are unknown. Users of certified libcore should consider their specific system safety requirements when developing safety related software with certified libcore.

7.2.2.6-7.2.2.9
~~~~~~~~~~~~~~~

N/A to libcore as a pure software library. Hardware constraints should be taken into consideration when integrating the certified libcore subset into a hardware environment.

7.2.2.11
~~~~~~~~

N/A, there is no way to configure the libcore binary after it is compiled.

7.2.2.12-7.2.2.13
~~~~~~~~~~~~~~~~~

“The Rust Core Library is the dependency-free [...]. It links to no upstream libraries, no system libraries, and no libc.” (from the libcore API docs)

N/A, because libcore is not using, and therefore not configuring, any pre-existing software.

7.3.2.1
~~~~~~~

See the Testing plan.

7.3.2.2
~~~~~~~

See the Testing plan.

7.3.2.3-7.3.2.4
~~~~~~~~~~~~~~~

See the Testing plan.

7.3.2.5
~~~~~~~

See the Testing plan.

7.4.2.1
~~~~~~~

Ferrous Systems is performing verification activities to prove that libcore can be used in safety critical applications up to SIL 2. Integration into hardware and into a broader system has to be performed by the user of libcore.

7.4.2.2-4
~~~~~~~~~

See “Doc-comments in libcore” in the libcore safety plan.

7.4.2.5-6
~~~~~~~~~

See “Doc-comments in libcore” in the libcore safety plan.

7.4.2.7
~~~~~~~

The design of the function, which is the combination of doc-comment and function signature, is specifying how the function handles failures. This can either be a certain return type, e.g. a Result::Err or Option::None, or a panic on runtime. Unsafe functions will specify the requirements a user of that function must uphold, but will not check if the user does so, and therefore results in undefined behavior when a requirement is not met.

7.4.2.8-11
~~~~~~~~~~

All functions in the certified libcore subset are deemed to be of the same SIL.

7.4.2.12
~~~~~~~~

Libcore is certified under route 3S.

The safety manual for libcore is added to the existing Ferrocene safety manual.

7.4.2.13
~~~~~~~~

Covered by this document.

7.4.2.14
~~~~~~~~

N/A, Libcore does not consist of pre-existing functionality and therefore does not configure any.

“The Rust Core Library is the dependency-free [...]. It links to no upstream libraries, no system libraries, and no libc.” (libcore API docs)

7.4.3
~~~~~

N/A due to the simple design of libcore. It is a library of independent functions with no internal state management. 

Quoting the libcore API docs: “The core library is minimal: it isn’t even aware of heap allocation, nor does it provide concurrency or I/O. These things require platform integration, and this library is platform-agnostic.”

7.4.4.1-9
~~~~~~~~~

There are only offline, and no online tools used for libcore certification. In “Tool safety assessments” of the libcore safety plan all used tools are specified and justified.

7.4.4.10-11
~~~~~~~~~~~

Libcore uses the rustc safety Qualified Ferrocene compiler, which uses rust as defined by the Rust language specification. Rust is well matched to the needs of libcore.

7.4.4.12-13
~~~~~~~~~~~

The Rust project has extensive measures (lints and tests) in place to assure quality and consistency of the codebase. Certified Libcore uses the same implicit standards as are ensured in the upstream codebase, to minimize divergence. Increased divergence from upstream leads to a higher maintenance burden and is a source of potential bugs.

7.4.4.14
~~~~~~~~

Rust uses macros for code generation. The validity of the output is guaranteed by the safety-qualified compiler and the correctness is validated by the test suite. 

7.4.4.15-18
~~~~~~~~~~~

All testing infrastructure, including offline-support tools, and related configuration, is stored in the GitHub respository, versioned, and subject to the same quality control process as other code.

https://public-docs.ferrocene.dev/main/qualification/plan/infrastructure.html#

7.4.4.19
~~~~~~~~

Ferrous Systems monitors upstream configuration but maintains and verifies the testing configuration with each pull request.

7.4.5.1-2
~~~~~~~~~

Ferrous Systems monitors upstream doc-comments, used as design, and verifies the design for each pull request by running the full test suite.

7.4.5.3-5
~~~~~~~~~

libcore has a very simple and modular architecture, consisting of 38 active modules, which are all independent functions with no internal state management. Each module provides functions and data structures around a single well-defined topic. All modules are documented and contain the design of the module, which is tested.

7.4.6
~~~~~

All upstream rust code is reviewed by a documented team of appointed Rust experts, and heavily tested, before being merged into libcore. Changes are reviewed by an expert who was not involved in the change. Test results and review evidence are public. Ferrous Systems tests that code for correctness on all qualified targets. 

7.4.7
~~~~~

All libcore modules are tested, with 100% test coverage. See the coretest results https://public-docs.ferrocene.dev/main/qualification/report/rustc/x86_64-unknown-linux-gnu.html#library-test-suite  TODO check into double listing

7.4.8
~~~~~

N/A as no integration testing is required as libcore has a very simple and modular architecture, consisting of 38 active modules, which are all independent functions with no internal state management.

7.5
~~~

Not applicable as libcore is a pure software library. Libcore is tested on all qualified targets. Hardware integration should be taken into consideration when integrating the certified libcore subset into a hardware environment.

7.6
~~~

Not applicable as libcore is a pure software library. Libcore is tested on all qualified targets. Hardware integration should be taken into consideration when integrating the certified libcore subset into a hardware environment.

7.7.1
~~~~~

Objective met.

7.7.2.1-4
~~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.7.2.5-6
~~~~~~~~~

Corestest test suite specifies all test cases and expected results.

7.7.2.7-9
~~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.8.1
~~~~~

Objective met.

7.8.2.1-5
~~~~~~~~~

Covered in the Upstream pulls section of https://public-docs.ferrocene.dev/main/qualification/internal-procedures/upstream-pulls.html 

7.8.2.6-7
~~~~~~~~~

Upstream pulls are performed on a regular basis, verified by running the full test suite, and then approved by an independent developer. See https://public-docs.ferrocene.dev/main/qualification/internal-procedures/upstream-pulls.html 

7.8.2.8-9
~~~~~~~~~

Documented in each pull request. As shown in https://public-docs.ferrocene.dev/main/qualification/internal-procedures/upstream-pulls.html

7.8.2.10
~~~~~~~~

The full test suite must pass for every change before it is merged. 

7.9.1
~~~~~

Objective met.

7.9.2.1-3
~~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.4-6
~~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.7
~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.8
~~~~~~~

N/A because there is no system, only software requirements.

7.9.2.9
~~~~~~~

N/A because there is no architecture design.

7.9.2.10
~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.11
~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.12
~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.13
~~~~~~~~

Libcore is tested as laid out in the testing plan, and those test results, for all qualified targets, are available in the qualification report.

7.9.2.14
~~~~~~~~

N/A as timing performance depends on the system requirements, which are unknown during the certification phase.

8.1-3
~~~~~

Certification is carried out by TÜV SÜD, an independent assessment body.

IEC-61508-3 Annex A
-------------------

Table A.1
~~~~~~~~~

1a
""

Covered by Requirements Management in the libcore safety and certification plan.

Table A.2
~~~~~~~~~

7
"

libcore has a very simple and modular architecture, consisting of 38 active modules, which are all independent functions with no internal state management. Each module provides functions and data structures around a single well-defined topic. All modules are documented and contain the design of the module.

https://doc.rust-lang.org/core/index.html#modules 

8
"

N/A, because libcore does not use external software elements.

11a
"""

N/A due to the simplicity of the design of libcore, as it is a library of independent functions with no internal state management.

13a
"""

N/A to libcore as a library.

Table A.3
~~~~~~~~~

1-2
"""

Rust has strong typing and assertions, is memory safe, and is well suited to structured and defensive programming. It is fully defined by the Ferrocene language specification and is a widely used general purpose programming language.

4a
""

Certified Libcore used Ferrocene, the fully qualified Rust compiler according to IEC-61508.

Table A.4
~~~~~~~~~

1a
""

N/A due to the simplicity of the design of libcore, as it is a library of independent functions with no internal state management.

4
"

Libcore is highly modularized.

5
"

The Rust project has extensive measures (lints and tests) in place to assure quality and consistency of the codebase. Certified Libcore uses the same implicit standards as are ensured in the upstream codebase, to minimize divergence. Increased divergence from upstream leads to a higher maintenance burden and is a source of potential bugs.

As such, Certified Libcore does not have a coding standard, and this requirement is not applicable.

6
"

The Rust programming language encourages structured programming. It has support for modular designs and does not support goto jumps. Rust has complex return types and therefore the use of out parameters is not common.

7
"

N/A, because libcore does not use external software elements.

Table A.5
~~~~~~~~~

2
"

N/A to libcore as a library.

3
"

Functions that cannot be tested by coretests will be tested by other means, including manually, and this will be documented.

4
"

Libcore is tested by the coretests test suite. TODO link

8
"

Tests are managed and automated by the libtest tool. It compiles a test runner binary which executes all tests and collects and visualises all test results. Coretests is run by CI for every PR.

Table A.6
~~~~~~~~~

N/A, no electronics or other hardware.

Table A.7
~~~~~~~~~

4
"

The coretests test suite runs on all hardware platforms that Ferrocene is qualified for.

Table A.8
~~~~~~~~~

1-3, 4a
"""""""

The complete system is reverified for every change. For every change, all tests are run, and all release artifacts are built. 

5
"

https://public-docs.ferrocene.dev/main/qualification/plan/ci.html#build-and-testing-process 

6
"

https://public-docs.ferrocene.dev/main/qualification/plan/development.html#merge-requirements

Table A.9
~~~~~~~~~

3
"

The Ferrocene compiler performs various kinds of static analysis when compiling libcore.

4
"

Libcore is tested by the coretests test suite. See testing plan.

Table A.10
~~~~~~~~~~

3
"

Covered in Tool Analysis of the Libcore Evaluation Report https://public-docs.ferrocene.dev/main/qualification/evaluation-report/rustc/tool-analysis.html# (when the docs are public)
