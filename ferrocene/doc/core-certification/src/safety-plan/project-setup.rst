.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Project Setup
=============

Release cadence
---------------

Due to the use of internal apis, the core library versions are only compatible with one matching Ferrocene release. The core library will be recertified for every Ferrocene release. That is approximately every three months.

The first Ferrocene release to include the certified core library is "Ferrocene 25.11.0". Ferrocene distributes an uncertified version of the core library since the first Ferrocene release.

Roles and responsibilities
--------------------------

Roles and responsibilities (e.g. Safety Manager, Product Manager) are documented at :doc:`qualification-plan:organization`.

Lifecycle Phases Overview
-------------------------

1. Changes to the core library are pulled from the Upstream Rust project, as part of daily upstream pulls (see :doc:`internal-procedures:upstream-pulls`).
2. On a regular cadence, latest before the release, the changes are examined and it is ensured the certification requirements are upheld.

The requirements are:

- No uncertified code is used from certified code
- Every public function of the certified subset has a requirement with sufficient quality
- The certified code adheres to the coding standard
- The certified subset is tested with 100% statement coverage

Internal procedures
-------------------

The :doc:`qualification-plan:index` describes how the Ferrocene organisation works, among others: Infrastructure, and the Development, Build, Testing and Release process. It is based on software engineering best practices, to be updated upon detection of shortcomings in the development process.
