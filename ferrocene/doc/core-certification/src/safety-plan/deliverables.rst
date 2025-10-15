.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Deliverables and Documents
==========================

The following deliverables are delivered to the assessor and to customers:

Product Documentation
---------------------

- :doc:`user-manual:index`
- The `Certified core library API docs <../../api-docs/core/index.html>`_, which include
   - Requirements (the function doc-comments)
   - Software Design (the module doc-comments)

Functional Safety Documentation
-------------------------------

- Safety Plan (this document)
- :doc:`core-certification:norm-mapping/index` for all safety standards
- :doc:`safety-manual:index`
- :doc:`core-certification:safety-plan/testing-plan`
- Test cases (``coretests`` test suite), and
- :doc:`qualification-report:rustc/index`

Binaries in the delivery
------------------------

The `rust-std-<TARGET>.tar.xz` archives contain the precompiled core library for each certified target.
