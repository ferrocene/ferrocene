.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Documentation Process
=====================

The Documentation Process is very similar to the Development Process because the
documentation is managed by the same Ferrocene CI infrastructure and subjected
to the same level of verification.

This section presents the documentation-specific phases that differ from the
general purpose development phases.


Doc Phase 1: Document Development
---------------------------------

This phase corresponds roughly to :ref:`development:Dev Phase 1: Code
Development` of the Development Process.

When a new document needs to be created or an existing document updated, a
GitHub issue is created to track the requirement. This issue is then addressed
by the appropriate team (Product Engineering Team for technical documents such
as the User Manual; and Qualification and Certification Team for qualification
and certification documentation).

As previously mentioned, each time a new commit is pushed to the Ferrocene
GitHub repository, the Ferrocene CI infrastructure runs a subset of the test
suite as part of the commit workflow. These checks only serve as an aid for the
authors to catch the most common mistakes before and during documentation
review.

A PR is then opened and initiates :ref:`documentation:Doc Phase 2: Document
Review`.


Doc Phase 2: Document Review
----------------------------

This phase corresponds roughly to :ref:`development:Dev Phase 2: Code Review` of
the Development Process.

During this phase, the latest commit in a PR must receive a positive review
through GitHub Reviews by an appropriate Ferrocene Reviewer and there must be
no outstanding request for changes. Technical documents (e.g. User Manual) are
reviewed by an independent Technical Reviewer from the same technical team. An
independent Technical Reviewer is any team member who has not contributed to
the redaction of the document.

Qualification and certification documents are reviewed by the Verification
Engineer. The validation performed by the Verification Engineer is a mandatory
requirement for compliance with the QMS and ISO 26262.

The role of the Verification Engineer is to check that the document's contents
are valid, up-to-date, and complete, that all applicable requirements have been
successfully covered, and that the language, syntax, and grammar are correct.

During this phase, the Ferrocene CI infrastructure performs quick checks on the
pushed code as part of the commit workflow to ensure that the documentation can
be built without any issues. The PR author can opt to run the full test suite
(which includes the Ferrocene test suites, not only document-related tests) by
sending a command to the merge bot before the approval.

Once the commit workflow successfully executes and the necessary positive
approval is left by a Reviewer, any Ferrocene developer can send a command to
the bot to queue the PR, thus initiating remaining Development Process phases
:ref:`development:Dev Phase 3: Queue` and :ref:`development:Dev Phase 4: Test`.
