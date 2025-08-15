.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Norm mapping of IEC-61508 standard requirements
===============================================

IEC-61508-1
-----------

5.2.1
~~~~~

Covered by :ref:`Safety Plan - Lifecycle Phases Overview <safety-plan:Lifecycle Phases Overview>`.

5.2.2
~~~~~

Covered by

- :ref:`Safety Plan - Lifecycle Phases Overview <safety-plan:Lifecycle Phases Overview>`
- :ref:`Safety Plan - Roles and Responsibilities <safety-plan:Roles and Responsibilities>`

5.2.3
~~~~~

Covered by

- :ref:`Safety Plan - Lifecycle Phases Overview <safety-plan:Lifecycle Phases Overview>`
- :ref:`Safety Plan - Release Cadence <safety-plan:Release Cadence>`

5.2.4
~~~~~

Covered by

- :doc:`Certified core library API docs <subset-api-docs>`
- :doc:`Code coverage report <code-coverage>`
- :doc:`Requirements review <requirement-review>`

5.2.5
~~~~~

All docs available in the :doc:`Ferrocene documentation package <index>`.

5.2.6
~~~~~

Covered by :ref:`Qualification Plan - Documentation Validation <qualification-plan:documentation-validation>`.

5.2.7
~~~~~

The core library certification documentation follows the structure of the other documents in the :doc:`Ferrocene documentation package <index>`.

5.2.8
~~~~~

Processes from the Rustc qualification are used, unless specific needs for the core library certification require otherwise.

5.2.9
~~~~~

Covered by :doc:`qualification-plan:change-tracking`.

5.2.10
~~~~~~

Changes to the certified core library are included in the Ferrocene release notes. When new releases of the certified core library, which is released with Ferrocene, include new functionality such as an expanded subset, this will be covered in the release notes.

5.2.11
~~~~~~

Covered by

- :ref:`Qualification Plan - Documentation Validation <qualification-plan:documentation-validation>`
- :ref:`safety-plan:Roles and Responsibilities`

6.2
~~~

See subsections.

6.2.1
~~~~~

Covered by :ref:`Safety Plan - Roles and responsibilities <safety-plan:Roles and Responsibilities>`.

6.2.2
~~~~~

Ferrous Systems is ISO 9001-2015 certified. See :doc:`Qualification Plan - Ferrocene Organization <qualification-plan:organization>`.

6.2.3
~~~~~

Covered by :ref:`safety-plan:Roles and Responsibilities`.

6.2.4
~~~~~

Covered by :doc:`safety-manual:customer-interactions`.

6.2.5
~~~~~

Covered by :doc:`qualification-plan:patching`.

6.2.6
~~~~~

Covered by :ref:`safety-plan:Internal Procedures`.

6.2.7
~~~~~

Covered by :ref:`safety-plan:Release Cadence`.

6.2.8
~~~~~

Covered by :doc:`qualification-plan:development`.

6.2.9
~~~~~

Covered by :doc:`qualification-plan:kp-tracking`.

6.2.10
~~~~~~

Covered by :doc:`qualification-plan:infrastructure`.

6.2.11
~~~~~~

N/A; No emergency services involved.

6.2.12
~~~~~~

Covered by :doc:`qualification-plan:organization`.

6.2.13
~~~~~~

Ferrous Systems maintains a database of staff competencies consisting of staff CVs as well as any training provided by Ferrous Systems. Prior to assigning major tasks, leadership verifies the competencies of the respective staff.
Ferrous Systems's ISO-9001 managed internal handbook details how staff may undertake training for new skills, or re-training for existing skills.

6.2.14
~~~~~~

When assigning staff to projects, leadership verifies that staff experience, training, decision making authority, responsibilities, and level of supervision required are a fit. Where appropriate, Ferrous Systems assigns more experienced staff to work alongside less experienced staff to facilitate hands-on training.

6.2.15
~~~~~~

Ferrous Systems maintains a database of staff competencies consisting of their CVs as well as any training provided by Ferrous Systems.

6.2.16
~~~~~~

Detailed in Ferrous Systems's ISO-9001 managed internal handbook and re-checked for each renewal of certification.

6.2.17
~~~~~~

N/A; No suppliers involved.

6.2.18
~~~~~~

Covered by

- :ref:`Safety Plan - Release Cadence <safety-plan:Release Cadence>`

7.4.2
~~~~~

Covered by :ref:`safety-plan:Failure analysis`.

7.5.2
~~~~~

Covered by

- :doc:`safety-manual:known-problems`

IEC-61508-3
-----------

4
~

1. The :ref:`Safety Plan - Certification Scope <safety-plan:Certification Scope>` specifies and justifies the SIL level.
2. The core library certification excludes the requirement of having an architecture.

   .. include:: ./partials/simple-design.rst

5
~

Covered by :ref:`5.2.1 to 5.2.11 of IEC-61508-1 <norm-mapping:5.2.1>`.

6.2.1
~~~~~

Covered by :ref:`6.2 of IEC-61508-1 <norm-mapping:6.2>`.

6.2.2
~~~~~

Covered by :doc:`safety-plan`.

6.2.3
~~~~~

Covered by

- :doc:`qualification-plan:development`
- :doc:`qualification-plan:change-tracking`
- :doc:`qualification-plan:infrastructure`

7.1.2.1-3
~~~~~~~~~

Covered by :ref:`Safety Plan - Lifecycle Phases Overview <safety-plan:Lifecycle Phases Overview>`.

7.1.2.4-6
~~~~~~~~~

We diverge from the V-Model, because we are certifying an existing project, not developing the code from scratch.

To explain it in V-model-terms: The Rust project, who is maintaining the upstream core library, performs the requirement specification, the software architecture, the software design, the module design and the coding. Ferrous Systems, consumes the output of those activities from upstream and performs module testing, integration testing, and validation testing on the code received from upstream pull requests.

7.1.2.7
~~~~~~~

Covered by :ref:`norm-mapping:IEC-61508-3 Annex A`.

7.1.2.8-9
~~~~~~~~~

The following relevant artefacts are included in the deliverables:

- :doc:`Code coverage report <code-coverage>`
- :doc:`Test results <qualification-report:rustc/index>`
- :doc:`Doc-comment review <requirement-review>`
- :doc:`Certified core library API docs <subset-api-docs>`

7.2.2.1-3
~~~~~~~~~

Covered by :ref:`safety-plan:Requirements management`.

7.2.2.4
~~~~~~~

N/A

.. include:: ./partials/simple-design.rst

All functions included in the subset are certified SIL2.

7.2.2.5
~~~~~~~

.. include:: ./partials/no-system.rst

7.2.2.6-9
~~~~~~~~~

N/A; the core library is a pure software library. Hardware constraints should be taken into consideration when integrating the certified core library into a hardware environment.

7.2.2.11
~~~~~~~~

N/A; There is no way to configure the core library binary after it is compiled.

7.2.2.12-13
~~~~~~~~~~~

N/A; the core library does not use, and therefore does not configure, any pre-existing software.

7.3.2.1-5
~~~~~~~~~

.. include:: ./partials/core-testing.rst

7.4.2.1
~~~~~~~

The upstream Rust project is responsible for developing the core library.

Ferrous Systems performs verification activities to prove that the core library can be used in safety critical applications up to SIL 2.

Integration into hardware and into a broader system must be performed by the user of the core library.

7.4.2.2-6
~~~~~~~~~

Covered by :ref:`safety-plan:Doc-comments in the core library`.

7.4.2.7
~~~~~~~

.. include:: ./partials/na-library.rst

7.4.2.8-11
~~~~~~~~~~

All functions in the certified core library are deemed to be of the same SIL.

7.4.2.12-14
~~~~~~~~~~~

.. include:: ./partials/na-library.rst

7.4.3
~~~~~

N/A, therefore no architecture is needed

.. include:: ./partials/simple-design.rst

7.4.4.1-9
~~~~~~~~~

No online tools are used for the core library certification, only offline tools. In :ref:`Safety Plan - Tool Safety Assessments <safety-plan:Tool Safety Assessments>` all used tools are specified and justified.

7.4.4.10-11
~~~~~~~~~~~

The certified core library is being build with the safety Qualified Ferrocene compiler, which uses Rust as defined by the Ferrocene language specification. Rust is well matched to the needs of the core library.

7.4.4.12-13
~~~~~~~~~~~

.. include:: ./partials/implicit-coding-standard.rst

7.4.4.14
~~~~~~~~

N/A; Rust macros are not automatic code generation, since they are written in source code.

7.4.4.15-18
~~~~~~~~~~~

All testing infrastructure, including offline support tools, and related configuration, is stored in the GitHub repository, versioned, and subject to the same quality control process as other code.

Infrastructure of Ferrocene is detailed in :doc:`qualification-plan:infrastructure`.

A record of all packages used by the build and test environment of each version of Ferrocene, including the core library, is contained in the ``ferrocene-src`` component, which contains:

* The root directory contains the entire Ferrocene source.
* The ``vendor/rust`` folder contains a copy of the source of each Rust dependency for Ferrocene in a format suitable for use with ``x.py``.
* The ``vendor/uv`` folder contains a copy of the source of each Python dependency for Ferrocene in a format suitable for using with ``uv``.
* The ``vendor/build-environment`` folder contains a comprehensive list of all distribution provided packages and their versions, as well as the hashes and URLs of all additional packages used (versions included).

This component is available to all customers and contains everything necessary to reproduce releases of Ferrocene.

7.4.4.19
~~~~~~~~

The "Technical Lead" is responsible for making or approving technical decisions, including which tools to use and how they are going to be configured.

7.4.5.1-2
~~~~~~~~~

See :ref:`history:contributing to upstream` for the upstream development and quality management process.

Ferrous Systems monitors upstream doc-comments, used as requirements and design, and verifies them for each pull request by running the full test suite.

7.4.5.3-5
~~~~~~~~~

.. include:: ./partials/simple-design.rst

7.4.6
~~~~~

All upstream Rust code is reviewed by a documented team of appointed Rust experts, and heavily tested, before being merged. Changes are reviewed by an expert who was not involved in the change. Test results and review evidence are public. Ferrous Systems tests that code for correctness on all qualified targets.

7.4.7-8
~~~~~

.. include:: ./partials/core-testing.rst

7.5
~~~

.. include:: ./partials/core-testing.rst

7.6
~~~

See 7.8.

7.7.1
~~~~~

Objective met.

7.7.2.1-4
~~~~~~~~~

.. include:: ./partials/core-testing.rst

7.7.2.5-6
~~~~~~~~~

The ``corestests`` test suite specifies all test cases and expected results in source code.

7.7.2.7-9
~~~~~~~~~

.. include:: ./partials/core-testing.rst

7.8
~~~

Covered by

- :doc:`qualification-plan:development`
- :doc:`internal-procedures:upstream-pulls`

7.9.1
~~~~~

Objective met.

7.9.2.1-7
~~~~~~~~~

.. include:: ./partials/core-testing.rst

7.9.2.8
~~~~~~~

N/A; There is no system, only software requirements.

7.9.2.9
~~~~~~~

N/A; There is no architecture design.

7.9.2.10-13
~~~~~~~~

.. include:: ./partials/core-testing.rst

7.9.2.14
~~~~~~~~

N/A; Timing performance depends on the system requirements, which are unknown during the certification phase.

8.1-3
~~~~~

Certification is carried out by TÜV SÜD, an independent assessment body.

IEC-61508-3 Annex A
-------------------

Table A.1
~~~~~~~~~

1a
""

Covered by :ref:`safety-plan:Requirements Management`.

Table A.2
~~~~~~~~~

7
"

.. include:: ./partials/simple-design.rst

8
"

N/A; the core library does not use external software elements.

11a
"""

N/A

.. include:: ./partials/simple-design.rst

13a
"""

N/A; core is a library.

Table A.3
~~~~~~~~~

1-2
"""

Rust has strong typing and assertions, is memory safe, and is well suited to structured and defensive programming. It is fully defined by the Ferrocene language specification and is a widely used general purpose programming language.

4a
""

The certified core library uses Ferrocene, the fully qualified Rust compiler according to IEC-61508.

Table A.4
~~~~~~~~~

1a
""

N/A

.. include:: ./partials/simple-design.rst

4
"

The core library is highly modularized.

5
"

N/A

.. include:: ./partials/implicit-coding-standard.rst

As such, the certified core library does not have a coding standard.

6
"

The Rust programming language encourages structured programming. It has support for modular designs and does not support goto jumps. Rust has complex return types and therefore the use of out parameters is not common.

7
"

N/A; the core library does not use external software elements.

Table A.5
~~~~~~~~~

2
"

N/A; core is a library.

3
"

.. include:: ./partials/core-testing.rst

4
"

.. include:: ./partials/core-testing.rst

8
"

Tests are managed and automated by the libtest tool. It compiles a test runner binary which executes all tests and collects and visualises all test results. Coretests is run by CI for every PR.

Table A.6
~~~~~~~~~

N/A; No electronics or other hardware.

Table A.7
~~~~~~~~~

4
"

.. include:: ./partials/core-testing.rst

Table A.8
~~~~~~~~~

1-3, 4a
"""""""

The complete toolchain is reverified for every change. For every change, all tests are run, and all release artifacts are built.

5
"

Covered by :doc:`qualification-plan:ci`.

6
"

Covered by :doc:`qualification-plan:development`.

Table A.9
~~~~~~~~~

3
"

The Ferrocene compiler performs various kinds of static analysis and lints when compiling the core library.

4
"

.. include:: ./partials/core-testing.rst

Table A.10
~~~~~~~~~~

3
"

Covered by :doc:`evaluation-report:rustc/tool-analysis`.

IEC-61508-3 Annex B
-------------------

Table B.1 - Design and coding standards
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

.. include:: ./partials/implicit-coding-standard.rst

2
"

The core library does not use heap-allocation.

7
"

Rust does not allow unstructured control flow (i.e. goto statements), except in inline assembly.

8
"

Type conversions in Rust are exclicit (``.into`` or ``as``), except for convenience (e.g. `& &mut T` to `&T`) or method dispatch.

Table B.2 - Dynamic analysis and testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

.. include:: ./partials/boundary-value-analysis.rst

7b
""

Covered by :doc:`code-coverage`.

Table B.3 - Functional and black-box testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

4
"

.. include:: ./partials/boundary-value-analysis.rst

Table B.4 - Failure analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

3
"

Covered by :ref:`safety-plan:Failure analysis`.


Table B.5 - Modelling
~~~~~~~~~~~~~~~~~~~~~

3
"

N/A

.. include:: ./partials/no-system.rst


Table B.6 - Performance testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

2-3
"""

N/A

.. include:: ./partials/no-system.rst

Table B.7 - Semi-formal methods
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Covered by :ref:`safety-plan:Requirements Management`.

Table B.8 - Static analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~

3-4
"""

The Ferrocene rustc compiler performs thorough control flow and data flow analysis. The data flow analysis is usually referred to as "Borrow Checker" and is one of the core features of Rust. One example for the outstanding control flow analysis is that rustc detects when some variants of an enum are not handled and throws a hard error.

Table B.9 - Modular approach
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

The core library does not impose a module size limit, but instead structures modules according to their function.

3
"

All fields and methods of a data structure are private by default and must be made public explicitly.

5
"

Functions in Rust can only be called through one interface (i.e. no overloading).

6
"

All items within a module are private by default and must be made public explicitly.
