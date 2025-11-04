.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Requirements traceability
=========================

Our infrastructure automatically tracks the traceability between requirements
and tests, and the report is available `as part of the documentation package
<../traceability-matrix.html>`_. The report must show full coverage: when it is
not possible to test a requirement on a target, an exception will be listed on
this page.

Note that the reporting tool only considers tests that were actually executed
as part of the :doc:`test results <rustc/index>`. Ignored tests are not
considered when determining the traceability.

Self-tests
~~~~~~~~~~

We have tests that check the validity of the traceability suite itself.

``diff-upstream``
"""""""""""""""""

This tool verifies that:
1. All `// ferrocene-annotations` in the test suite correspond to a section in the Ferrocene Language Specification or `Traceability Matrix <../traceability-matrix.html>`_.
2. All `// ferrocene-annotations` in the test suite are either followed by the name of that section, or correspond to a CLI-only unnamed section.

It can also be run manually to view all differences between the Ferrocene compiler and the upstream Rust Project compiler.
We plan in the future to inspect this diff on a regular basis to verify that there are no unintended differences from the upstream compiler.
