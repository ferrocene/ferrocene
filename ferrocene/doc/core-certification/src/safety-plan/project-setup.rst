.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Project Setup
=============

Release cadence
---------------

Due to the use of internal apis, the core library versions are only compatible with one matching Ferrocene release. The core library will be recertified for every Ferrocene release. That is approximately every three months.

The first Ferrocene release to include the certified core library is "Ferrocene 25.11.0".
Ferrocene has distributed an uncertified version of the core library since the first Ferrocene release.

Release notes
-------------

Changes to the certified core library are included in the :doc:`release-notes:index`.

When new releases of the certified core library, which are released with Ferrocene, include new functionality such as an expanded subset, this will be covered in the release notes.

Roles and responsibilities
--------------------------

Roles and responsibilities (e.g. Safety Manager, Product Manager) are documented at :doc:`qualification-plan:organization`.

The "Technical Lead" is responsible for making or approving technical decisions, including which tools to use and how they are going to be configured.

Lifecycle Phases Overview
-------------------------

1. Changes to the core library are pulled from the Upstream Rust project, as part of daily upstream pulls (see :doc:`internal-procedures:upstream-pulls`).
2. On a regular cadence, latest before the release, the changes are examined and it is ensured the certification requirements are upheld.

The requirements are:

- No uncertified code is used from certified code
- Every public function of the certified subset has a requirement with sufficient quality
- The certified code adheres to the coding standard
- The certified subset is tested with 100% statement coverage.
    - There are a few lines that cannot be covered by automatic tests. These are annotated with an explanation.

ISO 26262
~~~~~~~~~

The lifecycle is in accordance with ISO 26262-8:2018, Clause 10.

Internal procedures
-------------------

The :doc:`qualification-plan:index` describes how the Ferrocene organisation works, among others: Infrastructure, and the Development, Build, Testing and Release process. It is based on software engineering best practices, to be updated upon detection of shortcomings in the development process.

V-Model
-------

We diverge from the V-Model, because we are certifying an existing project, not developing the code from scratch.

To explain it in V-model-terms: The Rust project, who is maintaining the upstream core library, performs the requirement specification, the software architecture, the software design, the module design and the coding. Ferrous Systems consumes the output of those activities from upstream and performs module testing, integration testing, and validation testing on the code received from upstream pull requests.

Responsibility split
--------------------

Upstream
~~~~~~~~

The upstream Rust project is responsible for developing the core library.

All upstream Rust code is reviewed by a documented team of appointed Rust experts and heavily tested before being merged. Changes are reviewed by an expert who was not involved in the change. Test results and review evidence are public.

See :ref:`history:contributing to upstream` for more details on the upstream development and quality management process.

Ferrous Systems
~~~~~~~~~~~~~~~

Ferrous Systems monitors code changes and performs verification activities to prove that the core library can be used in safety critical applications up to the specified safety level.

Customer
~~~~~~~~

Integration into hardware and into a broader system must be performed by the user of the core library.

Configuration
-------------

All testing infrastructure, including offline support tools, and related configuration, is stored in the GitHub repository, versioned, and subject to the same quality control process as other code.

Infrastructure of Ferrocene is detailed in :doc:`qualification-plan:infrastructure`.

A record of all packages used by the build and test environment of each version of Ferrocene, including the core library, is contained in the ``ferrocene-src`` component, which contains:

* The root directory contains the entire Ferrocene source.
* The ``vendor/rust`` folder contains a copy of the source of each Rust dependency for Ferrocene in a format suitable for use with ``x.py``.
* The ``vendor/uv`` folder contains a copy of the source of each Python dependency for Ferrocene in a format suitable for using with ``uv``.
* The ``vendor/build-environment`` folder contains a comprehensive list of all distribution provided packages and their versions, as well as the hashes and URLs of all additional packages used (versions included).

This component is available to all customers and contains everything necessary to reproduce releases of Ferrocene.

Safety Assessment
-----------------

Any modifiction of the certification documentation or safety plan undergoes review as described in the :doc:`qualification-plan:development` which serves as confirmation of the safety plan.

Certification is carried out by TÜV SÜD, an independent assessment body.

Safety Case
-----------

The Safety case according to ISO 26262 consists of three steps:

1. CI passes
    CI runs all tests, builds all release artifacts and builds all the documentation for qualification and certification. It stores all evidence that prove safety, most notably the test results of all test suites, on AWS S3, which is available for the latest development version at ``public-docs.ferrocene.dev``. This evidence is being reviewed in order to make a release decision.
2. Sufficient code coverage
    CI does not ensure sufficient code coverage but instead continously measures it. This is to allow more flexibility during development, and focus on code coverage in a coordinated effort. Sufficient code coverage is ensured by the Release Manager before each release.
3. Semantic diff
    For each release the Release Manager creates a semantic diff between the current and previous release. During the creation all changes to the documentation are validated to be complete and correct.
