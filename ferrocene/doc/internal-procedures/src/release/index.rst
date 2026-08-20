.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Release process overview
========================

This section describes where the release process is implemented, how to change
it, and how to publish new releases. Note that the high-level overview of the
release process is present :doc:`in the qualification plan
<qualification-plan:release>`. Make sure to read the overview before diving
into this section.

Channel names
-------------

The release channel is determined automatically by the tooling, based on both
the ``src/ci/channel`` file (managed by upstream) and the
``ferrocene/ci/channel`` file (managed by Ferrocene):

.. list-table::
   :header-rows: 1

   * - Release channel
     - ``src/ci/channel``
     - ``ferrocene/ci/channel``
   * - **nightly**
     - nightly
     - rolling
   * - **pre-rolling**
     - beta
     - rolling
   * - **rolling**
     - stable
     - rolling
   * - **beta-${version}**
     - stable
     - beta
   * - **stable-${version}**
     - stable
     - stable

We rely on this approach of combining the two files to determine the channel
(rather than storing the actual channel in ``ferrocene/ci/channel``) to reduce
the maintenance efforts.

This way, as long as we set ``ferrocene/ci/channel`` to "rolling" on the main
branch, we don't need to make any change ourselves to promote a branch from
"nightly" to "pre-rolling" to "rolling", as upstream does that for us when they
change ``src/ci/channel``.

.. _release-checklist:

Release Checklist
-----------------

This checklist is targeted towards release managers who are running a release.

12 weeks before release
~~~~~~~~~~~~~~~~~~~~~~~

* :ref:`Determine which upstream version will be branched <determine-baseline>`

6 weeks before release
~~~~~~~~~~~~~~~~~~~~~~

In any order:

* [release/1.NN] :ref:`Feature freeze and backports <handling-backports>`.
  If there are any remaining ``backport:manual`` PRs, backport them now.
  After this point, the ``release/1.NN`` branch is in **feature freeze**: no new PRs can be backported unless approved by a release manager.

* :ref:`Validate the documentation <documentation-validation>`.

Simultaneously, do the following *in order*:

#. [release/1.NN] :ref:`Branch beta <branch-beta>`.
#. [KPs repo] :ref:`Add the new version to the Known Problems database <add-known-problems>`.
#. :ref:`release-note-bump`.

   #. [main] Update the version in the release notes.
   #. [release/1.NN] Backport the release notes update, and remove `:upcoming-release:` from beta's release notes.
   #. [main] Create a release note named ``next.rst``.

5 weeks before release
~~~~~~~~~~~~~~~~~~~~~~

In order:

#. Perform another :ref:`documentation-validation`.
#. Create a :ref:`semantic-diff`.
#. [release/1.NN] Instruct the :ref:`Safety Manager <qualification-plan:leadership-roles>` to
   :ref:`sign the release branch <internal-procedures:signing-all-documents>`.

#. :ref:`Deliver the documentation to the assessor <deliver-docs>`.

At this point, the branch is in **documentation freeze**:
no new PRs can be merged unless they do not affect the documentation signatures.
In practice, only changes to CI are allowed from this point forward.

.. note::

   If you do make a documentation change after this point,
   you MUST re-sign the docs and re-send them to the assessor.

When technical reports are received from the assessor
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#. :ref:`Upload the technical reports <release-technical-reports>`
#. [release/1.NN] :ref:`Publish a dev release <promote-stable>`.
#. :ref:`Perform release validation <release-validation>`.

At this point, the branch is in **commit freeze**:
No commits at all can be made until the release is published.

Day of the release
~~~~~~~~~~~~~~~~~~

* :ref:`Publish a manual release <manual-release>` in the ``prod`` environment.

After the release
~~~~~~~~~~~~~~~~~

* [release/1.NN] :ref:`prepare-patch-release`.
* [main] :ref:`remove-upcoming`.
* [main] :ref:`forward-ports`.
* [main] Update the release docs to reflect current reality.
