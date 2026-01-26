.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Stable release process
======================

This page details the steps required to publish a new stable release of
Ferrocene, from branching off a rolling branch to publishing point releases.

Determine the baseline Rust version
-----------------------------------

Each major version of Ferrocene is based off a Rust version, usually the latest
one available at the point the release was branched off the rolling branches.
Once the baseline is chosen, it will be possible to upgrade to newer patch
releases of Rust, but not to new minor or major Rust versions.

The decision of the baseline Rust version for a new Ferrocene major version
is made within the team. Once chosen, all development work will be done in the
``release/1.NN`` branch, where ``NN`` is the minor Rust version number.

Determine the Ferrocene version number
--------------------------------------

Each major version of Ferrocene's version number is ``YY.MM``, where ``YY`` and
``MM`` are the planned year and month of the first release of that major
version. For example, the first release of Ferrocene 23.11 would be published
on November 2023.

Each major version can have multiple point releases, identified by an
incrementing number starting from 0 (the initial release of that major
version). The format of a release's version number is ``YY.MM.N``, where
``YY`` and ``MM`` are the year and month of the *major version's first release*
(not the year and month when the point release was published), and ``N`` is
the number of the point release.

For example, Ferrocene 23.11.3 is the fourth release of Ferrocene 23.11, first
released in November 2023.

If this is the first release of a major Ferrocene version, schedule when the
release will be published: use that as the major version, and use ``YY.MM.0``
as the release's version number.

For any following point release, keep the same year and month as the previous
release, and increment the previous point release number.

Branching from rolling into beta
--------------------------------

The first step to prepare a stable release is to promote a rolling branch into
the beta channel of the release. To do so, open a new PR targeting the
``release/1.NN`` branch, replacing the content of the following files:

* ``ferrocene/version`` with the full version number of the release.
* ``ferrocene/ci/channel`` with ``beta``.

Once the PR is merged, the release process will start to automatically publish
the latest commit on that branch into the ``beta-${major_version}`` channel
every night. You can continue landing changes into the branch until you
are ready to release it as a stable release.

Add version to Known Problems
-----------------------------

Add the new version to the `Known Problems repository <https://github.com/ferrocene/problems/>`_
by adding the version and branch to the ``src/config.yml`` file then running the tool, following
the instructions in ``README.rst``.

Validate that the locally built site now has the version, and that known problems are tracked for it.
Make a pull request, ensure it gets merged, then validate the new version shows up on the
`Known Problems page <https://problems.ferrocene.dev/>`_.

Version Bump Release Notes
--------------------------

Rename ``ferrocene/doc/release-notes/src/next.rst`` to
``ferrocene/doc/release-notes/src/${version}.rst``.
Backport this PR according to :ref:`handling-backports`. Then, on the
``release/1.NN`` branch, remove the ``:upcoming-release:`` from the version.

Create a new ``ferrocene/doc/release-notes/src/next.rst`` on the `main` branch with the following content: 

.. code-block::

   .. SPDX-License-Identifier: MIT OR Apache-2.0
      SPDX-FileCopyrightText: The Ferrocene Developers

   :upcoming-release:

   Next Ferrocene release
   ======================

   This page contains the changes to be introduced in the upcoming Ferrocene
   release.


Precautionary validation
------------------------

Perform :ref:`qualification-plan:release-validation` on the ``release/1.NN`` branch before signing.

Signing
-------

Request the :ref:`Safety Manager <qualification-plan:leadership-roles>` to perform the
:ref:`documentation signatures <internal-procedures:signing-all-documents>`.

Delivering the documentation package
------------------------------------

Wait for the nightly beta, or manually cut a beta release onto production. Over email,
send the assessor direct links to the ``ferrocene-docs`` and ``ferrocene-docs-signatures``
packages, as well as a *semantic diff* of what changed since the last release.  

.. _release-technical-reports:

Uploading the technical reports
-------------------------------

Once qualification and certification are achieved for the Ferrocene major version,
the technical reports provided by the assessors needs to be uploaded to our AWS
account with::

   aws --profile ferrocene-ci s3 cp path/to/compiler-report.pdf s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-compiler-technical-report.pdf
   aws --profile ferrocene-ci s3 cp path/to/core-report.pdf s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-core-technical-report.pdf

In the command above, ``path/to/the/component-report.pdf`` is the local path to the
downloaded file, ``YYYY-MM-DD`` is the current date (**not** the version
number), and ``YY.MM.N`` is the version number.

Once the files are uploaded, open a new PR targeting the ``release/1.NN`` branch
changing ``ferrocene/ci/configure.sh``. In that file, find the lines setting the
``compiler-technical-report-url`` and ``core-technical-report`` options, if
commented uncomment them, and replace the URL with the ``s3://`` URLs of the
reports you just uploaded.

.. _publish-stable:

Publishing a stable release
---------------------------

To publish a stable release, you need to first open a PR targeting the
``release/1.NN`` branch, changing the contents of ``ferrocene/ci/channel`` to
``stable``.

Once the PR is merged, you need to grab the commit hash of the merge commit,
:ref:`start a manual release <manual-release>` on the ``dev`` environment, and
perform the :ref:`qualification-plan:release-validation`.

Once the release validation succeeded, :ref:`start a manual release
<manual-release>` on the ``prod`` environment. The release will require
approval from the release managers.

Finally, you need to send another PR targeting the ``release/1.NN`` branch,
changing ``ferrocene/ci/channel`` back to ``beta`` and incrementing the point
release version in ``ferrocene/version`` by 1. Note that you will need to
remove digital signatures, because they will be invalidated by the version
change. The CI also ensures that the signatures remain valid.

Remove upcoming notes in the ``main`` branch
--------------------------------------------

After publishing the stable release, send a PR to the ``main`` branch to:

* Remove the ``:upcoming-release:`` role at the top of the release notes page
  for this release.

* Remove all mentions of ``:upcoming:`YY.MM``` in the documentation, where
  ``YY.MM`` is the current version number.
