.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

EN 50716 Annex A
----------------

Table A.1 — Lifecycle issues and documentation (5.3)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1-5
"""

Covered by the table of documents in :ref:`section 5.3.2.4 of the EN 50716 norm mapping <norm-mapping/en-50716/en-50716:5.3.2.4>`.

6, 8
""""

Covered by

- :doc:`core-certification:safety-plan/requirements`
- `Certified standard library API docs <../../../../index.html#library-apis>`_

7
"

Covered by :doc:`core-certification:safety-plan/testing-plan`.

9, 10
"""""

N/A; no architecture and no separate design specification is written.

.. include:: ../../partials/simple-design.rst

11
""

Covered by the `Certified standard library API docs <../../../../index.html#library-apis>`_ (function signatures and doc-comments).

12
""

Covered by :ref:`safety-plan/testing-plan:integration tests`.

13
""

N/A; there is no hardware in the certification scope.

.. include:: ../../partials/no-system.rst

14
""

Covered by :doc:`qualification-plan:development` and :ref:`safety-plan/project-setup:safety case`.

15-17
"""""

Covered by the module doc-comments (see :ref:`safety-plan/requirements:architecture and software design`) and by :doc:`qualification-plan:development`.

18
""

The source code and its configuration history are provided with the ``ferrocene-src`` component, see :ref:`safety-plan/project-setup:configuration`.

19-20
"""""

Covered by

- :doc:`qualification-report:rustc/index`
- :doc:`core-certification:safety-report/code-coverage`

21-24
"""""

Covered by :doc:`qualification-report:rustc/index`.

25
""

Covered by :doc:`qualification-plan:validation` and :ref:`safety-plan/project-setup:safety case`.

26
""

Covered by :ref:`safety-plan/tools:Tool safety assessments` and :doc:`evaluation-report:rustc/tool-analysis`.

27
""

Covered by :doc:`release-notes:index`.

28-38
"""""

N/A; the certified libraries are not configurable by application data.

.. include:: ../../partials/na-no-config.rst

39-41
"""""

Covered on the Ferrocene side by :doc:`safety-manual:procedures`; the deployment into the target system is performed by the customer, see 9.1 of :doc:`core-certification:norm-mapping/en-50716/en-50716`.

42-45
"""""

Covered by :doc:`qualification-plan:patching`, :doc:`qualification-plan:change-tracking` and :doc:`qualification-plan:kp-tracking`.

46-47
"""""

Written by the Assessor, see :ref:`safety-plan/project-setup:safety assessment`.

Table A.2 — Software Requirements Specification (7.2)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1,3
"""

Not used; the requirements are written in natural language (doc-comments) together with the function signature, which is a formal and machine-checked description of the interface.

2
"

Applied by the structure of doc-comments and the overall structure of the core library.

Table A.3 — Software architecture (7.3)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Covered by :doc:`core-certification:safety-plan/programming-language` (memory safety, bounds checking, strong typing).

2-4, 6-14, 16
"""""""""""""

Not used; the certified libraries are single-channel software without recovery, reconfiguration or diversity. Detecting and handling failures of the enclosing system is the responsibility of the user of the certified libraries.

5
"

Applied by the language itself: out-of-range indexing, overflow and similar violations terminate in a documented panic, see :ref:`safety-plan/requirements:doc-comments in the certified libraries`.

15
""

Covered by :doc:`core-certification:safety-plan/failure-analysis`.

17
""

Covered by the `Certified standard library API docs <../../../../index.html#library-apis>`_ and the Rust type system, see 7.3.4.18-19 of :doc:`core-certification:norm-mapping/en-50716/en-50716`.

18
""

N/A; no modelling is used, see Table A.17.

19
""

Covered by the modular structure of the certified libraries:

.. include:: ../../partials/simple-design.rst

The selected set is 1, 5, 15, 17 and 19, which is an approved combination for |en_50716_core_sil| (requirement 2 of this table).

Table A.4 — Software design and implementation (7.3, 7.4 and 7.5)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

N/A; no modelling is used, see Table A.17.

2-5
"""

Covered by

- :ref:`safety-plan/requirements:architecture and software design` (modular decomposition into modules with doc-comments)
- Table A.20 below, for measure 4
- the coding standard, see Table A.12 below, for measure 5

6
"

Covered by :doc:`core-certification:safety-plan/programming-language` and :doc:`core-certification:safety-report/code-coverage`.

7, 8
""""

Covered by :doc:`core-certification:safety-plan/programming-language`.

The selected set is 2, 3, 4, 5, 7 and 8, which is an approved combination for |en_50716_core_sil| (requirement 2 of this table).

Table A.5 — Software component analysis and testing (6.2 and 7.4)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Not used; no formal proof is performed. The static guarantees of the Rust language (type checking, borrow checking) provide an equivalent assurance, see :doc:`core-certification:safety-plan/programming-language`.

2
"

Covered by :ref:`safety-plan/testing-plan:rustc lints` and the ``tidy`` test suite, see Table A.19.

3
"

.. include:: ../../partials/core-testing.rst

The selected techniques of Table A.13 are documented in that table.

4
"

Not used; no further metrics are measured. Size and complexity are limited by the language and the tooling, see :ref:`safety-plan/programming-language:recursion limit` and :ref:`safety-plan/testing-plan:rustc lints`.

5
"

Covered by :doc:`core-certification:safety-report/code-coverage`, see Table A.21.

6
"

N/A; timing performance depends on the system requirements, which are unknown during the certification phase.

7
"

Covered by :ref:`safety-plan/testing-plan:unit tests`: every public function is tested through its interface, including its panic and error paths.

The selected set is 2, 3 and 5, which is an approved combination for SIL 1 to SIL 4 (requirements 1 and 2 of this table).

Table A.6 — Software integration analysis and testing (7.3 and 7.6)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Covered by :doc:`core-certification:safety-plan/testing-plan` (the complete test suite is executed for every change on all qualified targets).

2
"

N/A; timing performance depends on the system requirements, which are unknown during the certification phase.

Table A.7 — Overall software analysis and testing (6.2 and 7.2)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

N/A; timing performance depends on the system requirements, which are unknown during the certification phase.

2
"

.. include:: ../../partials/core-testing.rst

The "overall software" of a library is its complete public API; the black-box tests are the doc-tests and the integration tests of the Ferrocene components that use the certified libraries.

3
"

N/A; no modelling is used, see Table A.17.

Table A.8 — Intentionally left blank
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Intentionally left blank in the standard.

Table A.9 — Software quality assurance (6.5)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1, 3
""""

Ferrous Systems is ISO 9001:2015 certified, see :doc:`Qualification Plan - Ferrocene Organization <qualification-plan:organization>` and :ref:`safety-plan/project-setup:internal procedures`.

2
"

Not used as method 1 is already applied.

4
"

Covered by :doc:`qualification-plan:ci`, :doc:`qualification-plan:change-tracking` and :ref:`safety-plan/project-setup:configuration`.

5
"

Covered by the merge requirements of :doc:`qualification-plan:development` and the review checklists of :doc:`qualification-plan:patching`.

6
"

Covered by :doc:`Requirements traceability report <qualification-report:requirements-traceability>` and :ref:`safety-plan/requirements:tracing requirements to tests`.

7
"

Covered by :ref:`Validation Strategy <qualification-plan:validation:Validation Strategy>`: the CI infrastructure keeps logs and artifacts of every activity, and the Known Problems database records field data, see :doc:`qualification-plan:kp-tracking`.

Table A.10 — Software maintenance (9.2)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Deviation: no per-change impact analysis is performed. Every change triggers a complete rebuild and a full execution of all test suites of the certified libraries, therefore the impact of a change is always the whole certified subset, see :ref:`safety-plan/testing-plan:re-verification scope`.

2
"

Covered by :doc:`qualification-plan:kp-tracking` and :doc:`qualification-plan:change-tracking`.

Table A.11 — Data preparation techniques (8.4)
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

N/A; the certified libraries are not configurable by application data.

.. include:: ../../partials/na-no-config.rst

Table A.12 — Coding standards
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1, 2
""""

.. include:: ../../partials/implicit-coding-standard.rst

As such, the certified libraries do not have an additional coding standard of their own. The upstream style and consistency rules are enforced by the ``tidy`` test suite (``rustfmt``, ``clippy``, ``rustc`` lints), see :ref:`safety-plan/testing-plan:tidy test suite` and :ref:`safety-plan/testing-plan:rustc lints`.

3
"

Covered by :doc:`core-certification:safety-plan/programming-language` (recursion limit, functions with many parameters are discouraged). Requirement 1 of this table is applied: further modularization of some complex generic functions is not sensible.

4
"

Covered by the Rust language itself: a function has exactly one entry point and no ``goto`` jumps are available outside of inline assembly, see :ref:`safety-plan/programming-language:rust`.

5
"

Covered by the Rust language itself: mutable global state requires ``unsafe`` code and explicit ``static mut`` or synchronization primitives; the certified subset avoids it, and the lints flag violations, see :ref:`safety-plan/testing-plan:rustc lints`.

Table A.13 — Dynamic analysis and testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Covered by :ref:`safety-plan/testing-plan:unit tests`: the upstream test suites contain a large number of hand-written test cases, which are reviewed by upstream reviewers and by Ferrous Systems.

2
"

Not used; no error seeding is performed.

3
"

Covered by :doc:`core-certification:safety-report/code-coverage`: test execution is measured against the code structure for every release.

4, 5
""""

Not used; no cause consequence diagrams and no prototyping are used.

6
"

.. include:: ../../partials/boundary-value-analysis.rst

7
"

Covered by :ref:`safety-plan/testing-plan:unit tests` and the doc-tests: the equivalence classes of each function are documented in its doc-comment (including the panicking conditions) and tested.

8
"

N/A; no process simulation is used. The tests run natively on the qualified targets or on emulators, see :doc:`qualification-plan:infrastructure`.

Table A.14 — Intentionally left blank
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Intentionally left blank in the standard.

Table A.15 — Suitable Programming Languages
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1-7
"""

Covered by

- :doc:`core-certification:safety-plan/programming-language` (all measures of this table)
- :doc:`specification:index`, which documents the operational semantics of the language used
- :ref:`safety-plan/testing-plan:code coverage tests` for measure 7

Table A.16 — Intentionally left blank
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Intentionally left blank in the standard.

Table A.17 — Modelling
~~~~~~~~~~~~~~~~~~~~~~

N/A; no modelling techniques are used. The design of each module is described in its doc-comment , see :ref:`safety-plan/requirements:architecture and software design`. As there is no state managed or complex interdependencies of modules present inside of the library, no further modelling based approach or graphical representation is required to fully define the functions of the library.

Table A.18 — Performance testing
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

N/A; timing and memory performance depend on the target system and on the application, which are unknown during the certification phase (see 7.9.2.14 of :doc:`core-certification:norm-mapping/iec-61508/iec-61508-3`).

Table A.19 — Static analysis
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1, 2
""""

Covered by the Ferrocene compiler, which performs thorough control flow and data flow analysis (including the borrow checker), see :doc:`core-certification:safety-plan/programming-language`.

3
"

Covered by :doc:`core-certification:safety-plan/failure-analysis` and :doc:`evaluation-report:rustc/tool-analysis`.

4
"

Covered by :ref:`safety-plan/project-setup:responsibility split`: upstream changes are reviewed by appointed Rust experts who were not involved in the change, and Ferrous Systems reviews every pulled change.

Table A.20 — Components
~~~~~~~~~~~~~~~~~~~~~~~

1
"

Covered by :doc:`core-certification:safety-plan/programming-language`: all items of a module and all fields and methods of a data structure are private unless explicitly made public.

2
"

Covered by :doc:`core-certification:safety-plan/programming-language` (``clippy::too_many_arguments``).

3
"

Covered by the `Certified standard library API docs <../../../../index.html#library-apis>`_ and the Rust type system.

Table A.21 — Test coverage for code
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

1
"

Covered by :ref:`safety-plan/project-setup:lifecycle phases overview` (100 % statement coverage of the certified subset) and by :doc:`core-certification:safety-report/code-coverage`.

2-5
"""

Not measured; the code coverage report measures region/statement coverage.

Requirement 4 of this table is applied: lines which cannot be covered by automated tests are annotated with ``#[ferrocene::annotation(REASON)]`` and reasoned there, see :ref:`safety-plan/testing-plan:manual test coverage`. Additionally, unused code cannot be executed, see :doc:`core-certification:safety-plan/uncertified-unused`.
