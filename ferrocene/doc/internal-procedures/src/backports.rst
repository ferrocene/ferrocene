.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _handling-backports:

Handling Backports
==================

All feature development of Ferrocene happens in the ``main`` branch, but occasionally
we have to backport PRs from ``main`` or from upstream ``rust-lang/rust`` into a release branch.
This usually happens when preparing the documentation for an upcoming release, or when a bug fix
also affects a supported release.

We developed tooling to aid backporting: this page explains how it works and
how to use it.

Automated Backports
-------------------

In most cases, backporting a PR doesn't require running any command locally or
manually opening PRs. The automated backport tooling allows creating backports
from the GitHub UI, thanks to labels.

.. _backport-phase-candidates:

Backport Phase 1: Marking PRs as Backport Candidate
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Whenever a PR is merged, the automation will add the ``backport:maybe`` label
to the PR, marking it as a candidate for backport. This is only done when no
other backport label is present:

* PRs from different automations often have the ``backport:never`` label,
  instructing the backport automation not to mark it as a candidate.

* PRs already marked with a backport target (like the ``backport:1.76``) are
  not marked as a candidate, since during code review they were already deemed
  to be backported.

.. _backport-phase-triage:

Backport Phase 2: Triaging Backport Candidates
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Periodically, a Ferrocene developer must go through `the list of backport
candidates <https://github.com/ferrocene/ferrocene/labels/backport:maybe>`_ and
make a decision on whether to backport the PRs, and to which branches to
backport them to.

If a PR should not be backported, the developer must remove the
``backport:maybe`` label.

If a PR should be backported, the developer must decide the releases to
backport it to, and apply the relevant backport target label, creating one if
it doesn't exist yet. Backport target labels are called ``backport:1.NN``, and
will cause a backport into the ``release/1.NN`` branch.

.. _backport-phase-automation:

Backport Phase 3: Automated Backport Creation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Every day, the automation will collect all PRs with a backport target label
(added in phase 2) and will try to backport them all into the target branch. It
will then open a PR with the changes, that must go through the
:doc:`qualification-plan:development`.

If there are conflicts, the automation will skip just the conflicting PRs, and
add the ``backport:manual`` label to them. This indicates a developer must
follow the manual backporting process.

.. _backport-phase-cleanup:

Backport Phase 4: Automated Backport Labels Removal
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

All backport commits created with the backport tooling, either manually or
through the automation, contain metadata in the commit message identifying
which PR they are a backport of.

Thanks to those labels, the automation will remove the relevant ``backport:NN``
label from a PR as soon as a backport of it is merged into the target branch,
leaving the other labels intact. Thanks to this, developers will never have to
manually remove backport target labels!

Manual Backports
----------------

If backporting would result in a conflict or the backport source is in the ``rust-lang/rust``
repository, a manual backport is required, where it's up to the developer to manually fix the
conflict. Ferrocene PRs with backport conflicts have the ``backport:manual`` label.

To backport a PR, create a branch targeting the ``release/1.*`` branch you want to
backport to, and run:

.. code-block:: console

   # When backporting a rust-lang/rust PR, add a `--rust` flag to the invocation
   ferrocene/tools/backport/one.py PR_NUMBER

The command will configure and execute a git rebase to backport the PR's
commits into the current branch. As the rebase progresses, you will encounter
conflicts: resolve them, and then run ``git rebase --continue`` to proceed.
Once the rebase finishes, you can backport other PRs or push the branch to
GitHub for review.

In the case of the backported PR fixing a known problem, the backport must be
accompanied by at least one `tests/ui` test if the backported PR itself did not
add a respective test.

Note that the manual backport script also adds the correct metadata needed for
:ref:`backport-phase-cleanup` to run: as long as you use the script, you will
not need to manually remove the backport labels from the PR.
