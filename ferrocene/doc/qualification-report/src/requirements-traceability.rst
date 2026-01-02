.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _requirements-traceability:

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
