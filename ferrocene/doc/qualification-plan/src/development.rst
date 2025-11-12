.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Development Process
===================

The Ferrocene toolchain is a complex codebase that contains inputs from
different teams and upstream sources. Ferrocene developers need to ensure that
all changes made to it are properly reviewed, are of adequate quality, and can
be properly assessed through automated testing.

This section describes the process for merging any change into the Ferrocene
codebase while accomplishing the aforementioned goals.

Merge Requirements
------------------

There are four requirements for merging any change into the ``main`` and
``release/*`` branches of the Ferrocene GitHub repository:

1. All changes must be proposed through a PR on the Ferrocene GitHub repository.

2. At least one independent Ferrocene Reviewer must have approved the latest
   revision of the PR, and there must be no requests for changes.
   Positive reviews for older revisions do not count. The following screenshot
   shows PR approval:

   .. figure:: figures/pr-approval-msg.png

3. The full set of tests run by CI must pass on the merge commit between the PR
   and the base branch. The merge commit must be the one being fast-forwarded on
   the base branch. Testing a merge commit and then merging into the base branch
   with a different merge commit (for example if other changes were merged in
   the meantime) does not count.

4. All changes deemed major must have an approved ticket, and must adhere to the
   proposed solution outlined in the ticket. See :ref:`dev-phase-design` for more
   information about tickets.

An independent Ferrocene Reviewer is any Ferrocene developer who has not
committed code as part of the change being reviewed. The only exception occurs
for automated PRs pulling code from upstream. In those cases, any Ferrocene
developer is considered as an independent Reviewer, even if they contributed to
the changes in the upstream Rust project that are being merged. This is
acceptable since we can take credit from the upstream review process, which
establishes confidence in the code.


Merge Strategy
--------------

To avoid merge complications, the Ferrocene CI infrastructure uses a merge
queue to only test and merge one batch of PRs at a time. Whenever a Ferrocene
developer indicates that a PR is ready, the PR is placed into the merge queue.
A bot then takes one batch of PRs, merges all of them together with the ``main``
branch into a single merge commit, and runs the full test suite on that merge
commit. Only one batch at a time is tested so every change is fully tested
against the most recent ``main`` branch.

This strategy also applies when merging changes to ``release/*`` branches. Once
the backport PR is approved, a bot will follow the aforementioned steps to
ensure the batch is fully tested against the ``release/*`` branch to be updated.

Development Phases
------------------

.. _dev-phase-design:

Dev Phase 1: Development Ticket Approval
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

To ensure adequate quality, changes that are deemed *major* by the authors or
reviewers need to have a development ticket written and approved, before
development can begin.

Whether a change is deemed major is at the discretion of the Ferrocene
developers and reviewers assigned to the task. Still, any member of the
Ferrocene team can ask for a change to be deemed major (and thus require a
ticket), even if the developers and reviewers decided it was minor.

Tickets must be created in the project management tool under the respective
project board, they must be appropriately scoped, and they must detail acceptance criteria.

The ticket shall detail which parts of Ferrocene are expected to be impacted.
That could be for example rustc, the core library, the qualification documentation etc.
Also it shall detail which tests have to be added.

Large tickets must be broken down into smaller, independent tickets. A ticket
must be approved by at least one other member of the Ferrocene team without any
concerns.

Once the ticket has been scoped and approved, or if a change was not deemed major,
:ref:`dev-phase-development` can begin.

.. _dev-phase-development:

Dev Phase 2: Code Development
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

During this phase, one or more Ferrocene developers work in a branch, both
locally and in the Ferrocene GitHub repository.
The branch can be called anything except the following:

- ``main``
- ``release/*``
- ``automation/*``
- ``trying``
- ``staging``

To avoid conflicts, it is strongly recommended that the main developer for a
branch adds a prefix to the branch name unique to them (like ``hoverbear/`` or
``skade/``).

During development, the vast majority of code shall be written directly by
humans. Technology like auto-complete (eg. Intellisense) or structured
refactoring (eg. with Rust Analyzer) are permitted. However, bulk
low-effort changes of questionable copyright or provenance, such as those
created with generative AI such as LLMs, will be rejected. The Rust compiler
team pursues a similar policy of rejecting
`low-effort changes <https://github.com/rust-lang/compiler-team/issues/893>`_,
and the Rust project is currently
`evaluating their stance on AI <https://rust-lang.zulipchat.com/#narrow/channel/392734-council/topic/AI.20policy/with/493978958>`_.

Each time a new commit is pushed to the Ferrocene GitHub repository, the
Ferrocene CI infrastructure runs a subset of the test suite as part of the
commit workflow. These checks are not meant to provide any assurance on the
commits being tested, rather they serve as an aid for the Ferrocene developer
to catch the most common mistakes before and during code review. For further
details, see :ref:`ci-phase-spot`.

Once active development is done, a PR is opened with a description of the
change. This initiates :ref:`dev-phase-review`.

.. _dev-phase-review:

Dev Phase 3: Code Review
^^^^^^^^^^^^^^^^^^^^^^^^

During this phase, the latest commit of a PR must receive a positive review
through GitHub Reviews by an independent Ferrocene Reviewer, and there must be
no outstanding request for changes.

Any code change pushed after a positive review invalidates the assessment and
demands that the code be reviewed again. Outstanding requests for changes must
not be automatically dismissed on code push. It is possible for a Ferrocene
developer, other than the original Reviewer, to dismiss it manually in the case
that the person who left the request for changes is unavailable for a followup
review (e.g. if they're out of office). Before dismissing the request, the
Ferrocene developer is required to ensure that the concerns raised were
adequately addressed.

During this phase, CI performs quick checks on the pushed code as part of the
commit workflow, and the PR author can opt to run the full test suite by sending
a command to the merge bot before the approval. When the command is sent, CI
will perform the same checks it would perform before merging the PR but without
performing the merge.

This can be done in the event that the Ferrocene developer wants to run the full
test suite before the PR is approved in order to ensure that the PR will pass
the test suite ahead of the review.

Once the commit workflow successfully executes, and the necessary positive
approval is left by a Ferrocene Reviewer, any Ferrocene developer can send a
command to the bot to queue the PR for :ref:`dev-phase-queue`.

.. _dev-phase-queue:

Dev Phase 4: Queue
^^^^^^^^^^^^^^^^^^

While the PR is in the queue, no new commits should be pushed as any new push
will automatically move the PR out of the queue and back to
:ref:`dev-phase-review`. The PR is also removed from the queue if any merge
conflict arises during this phase to avoid wasting CI time with a broken PR.

Once it is the PR's turn to be tested, either as standalone (if the queue is
otherwise empty) or in a batch (if multiple PRs are waiting to be merged), the
PR goes to :ref:`dev-phase-test`.

.. _dev-phase-test:

Dev Phase 5: Test
^^^^^^^^^^^^^^^^^

Once it is a PR's turn to be tested, the commits in the PR are merged with the
latest commit from the ``main`` branch. That merged commit is then pushed to
the staging branch. This triggers the full test workflow, which executes the
full set of tests to ensure the merge commit does not have issues. For further
details, see :ref:`ci-phase-full`.

If the full test workflow executes successfully, the merge commit is
fast-forwarded to the ``main`` branch and a new PR is pulled out of the queue to
be tested.

If the workflow fails, the current PR is moved back to :ref:`dev-phase-review`.


Integrating Upstream Changes
----------------------------

Every weekday, the Ferrocene CI infrastructure pulls changes made in the
upstream GitHub repository, then creates a PR with those changes. The goal
is to stay consistent with upstream to integrate the latest bug fixes and
features.

The Ferrocene CI infrastructure is configured to avoid considering changes that
are not relevant to Ferrocene, such as the CI configuration of upstream.

Any Ferrocene developer can then approve the PR and instruct the Ferrocene CI
infrastructure to merge it into the Ferrocene GitHub repository. Note that
Ferrocene developers do not perform manual code reviews on the changes, on the
assumption that upstream processes ensure the correct level of quality.


Patch Reversion
---------------

If a patch needs to be reverted, a Ferrocene developer first creates a GitHub
issue for the problem, and then performs a git revert manually, which creates a
change PR. From there, the PR is considered to be in :ref:`dev-phase-review`.

After the patch is reverted, GitHub change tracking is used to determine the
current status of a branch after the change.

The GitHub issue is used to track the analysis performed by either the original
author of the patch or the Release Manager, post mortems and conclusions, and
any additional tests introduced to prevent future recurrences of the problem.
