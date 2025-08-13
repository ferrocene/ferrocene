.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Patching process
================

As described in the :doc:`kp-tracking`, Ferrous Systems is responsible for
disclosing any new known problem to customers, and the mitigations they must
apply. From time to time, Ferrous Systems might decide to publish a patch
release addressing a subset of the known problems, to remove the need of manual
mitigation from our customers.

This document outlines the procedure that must be followed to publish a patch
release without undergoing review by our assessors. If the procedure outlined
in this document cannot be followed, the patch release must undergo assessor
review like any feature release.

.. _patching-scope:

Scope of changes
----------------

Only changes explicitly listed here can be included in a patch release that
does not need to undergo assessor review:

* Fixing a rustc bug or security vulnerability, if the source code change is
  self-contained and does not require widespread changes in the rustc codebase.
* Qualifying an existing compiler flag that we have confidence mitigates a
  known problem, optionally updating the Safety Manual to mandate its use.
* Fixing bugs or security vulnerabilities in other components shipped as part
  of the Ferrocene toolchain that are not qualified.
* Updating the Safety Manual to clarify or restrict the use of certain
  facilities.

.. _patching-review:

Review checklist
----------------

The developer reviewing a change slated to be included in a patch release must
ensure that:

* All the requirements in the :doc:`development` are followed.
* The change falls within one of the clauses in :ref:`patching-scope`.
* The patch is the minimal set of changes needed to address and test the fix,
  and does not include any change used by unrelated features.

Release checklist
-----------------

The release manager in charge of publishing the release must ensure that:

* All the steps in the :doc:`validation` are followed.
* The :ref:`patching-review` was followed for all changes included in the patch
  release.
* The traceability matrix's coverage is equal or better compared to the
  previous release.

Once those steps are followed, the Safety Manager must contact the assessor to
request updated certificates with the new version number and document IDs,
providing the evidence that all the steps described in this document were
applied. Once the certificates are :ref:`included in the built documentation
<internal-procedures:release-technical-reports>`, the release can be published.
