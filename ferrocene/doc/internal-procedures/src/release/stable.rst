.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Stable release process
======================

This page details the steps required to publish a new stable release of
Ferrocene, from branching off a rolling branch to publishing point releases.

This is a *reference*, not a *checklist*.
It is only vaguely ordered.
For a canonical ordering, see :ref:`release-checklist`.

.. _determine-baseline:

Determine the baseline Rust version
-----------------------------------

Each major version of Ferrocene is based off a Rust version.
Choose the Rust version such that the Ferrocene release is the based off the
latest stable Rust *at the time that docs are sent to the assessor*.
Consult `Rust Forge <https://forge.rust-lang.org/>`_ for a list of Rust release dates.

.. note::

    For example, Ferrocene 26.08 released in August 2028, and sent docs in the last week of July,
    so it was based off of Rust 1.97 (released July 9), **not** 1.98 (released August 20).

Once a version is chosen, all development work will be done in the
``release/1.NN`` branch, where ``NN`` is the minor Rust version number.
Release branches can upgrade to newer patch releases of Rust, but not to new
minor or major Rust versions.

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

.. _branch-beta:

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

.. _add-known-problems:

Add version to Known Problems
-----------------------------

Add the new version to the `Known Problems repository <https://github.com/ferrocene/problems/>`_
by adding the version and branch to the ``src/config.yml`` file then running the tool, following
the instructions in ``README.rst``.

Validate that the locally built site now has the version, and that known problems are tracked for it.
Make a pull request, ensure it gets merged, then validate the new version shows up on the
`Known Problems page <https://problems.ferrocene.dev/>`_.

.. _release-note-bump:

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


.. _semantic-diff:

Semantic diff
-------------

To avoid unnecessary work for the safety assessor and safety manager,
create a high-level overview of what has changed since the last release.
This should include at least:

* Changes to ``ferrocene/doc``
* Changes to the symbol report
* Anything mentioned in the release notes

.. note::

   You can see a list of doc changes since the last release like so:

.. code-block::

    git diff release/1.95 release/1.97 --ignore-all-space --ignore-blank-lines 'ferrocene/doc' ':!*/signature.toml'

.. _deliver-docs:

Delivering the documentation package
------------------------------------

Wait for the nightly beta, or manually cut a beta release onto production. Over email,
send the assessor direct links to the ``ferrocene-docs`` and ``ferrocene-docs-signatures``
packages, as well as the semantic diff.

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

Reviewer guidance
~~~~~~~~~~~~~~~~~

The reviewer of this change has to check the following:

1. Download the reports from S3 and check they are the correct ones. Correct means that the document mentioned in the certificate matches the document uploaded.

   .. code-block::

      $ aws --profile ferrocene-ci s3 cp s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-compiler-technical-report.pdf path/to/compiler-report.pdf
      $ aws --profile ferrocene-ci s3 cp s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-YY.MM.N-core-technical-report.pdf path/to/core-report.pdf

2. Check that the configuration is correctly set in ``ferrocene/ci/configure.sh``.

.. _promote-stable:

Promoting beta to stable
------------------------

To publish a stable release, you need to first open a PR targeting the
``release/1.NN`` branch, changing the contents of ``ferrocene/ci/channel`` to
``stable``.

Once the PR is merged, you need to grab the commit hash of the merge commit and
:ref:`start a manual release <manual-release>` on the ``dev`` environment.

.. _prepare-patch-release:

Prepare for patch releases
--------------------------

Once you've released to ``prod``, you need to send another PR targeting the ``release/1.NN`` branch,
changing ``ferrocene/ci/channel`` back to ``beta`` and incrementing the point
release version in ``ferrocene/version`` by 1. Note that you will need to
remove digital signatures, because they will be invalidated by the version
change. The CI also ensures that the signatures remain valid.

.. _remove-upcoming:

Remove upcoming notes in the ``main`` branch
--------------------------------------------

After publishing the stable release, send a PR to the ``main`` branch to:

* Remove the ``:upcoming-release:`` role at the top of the release notes page
  for this release.

* Remove all mentions of ``:upcoming:`YY.MM``` in the documentation, where
  ``YY.MM`` is the current version number.

.. _forward-ports:

Identify any forward ports
--------------------------

In some cases, releases may have small pull requests of last minute changes
which did not end up yet on the development branch.

If these changes do not have a related pull request to `main` they are
labelled `needs-forward-port`.

Before preparing for a release, ensure any pull requests labelled
`needs-forward-port` have been submitted to the `main` branch.

Typically the process is go through each `needs-forward-port` tagged pull
request and:

* ``git cherry-pick`` the commits onto a branch
* ``git commit --amend`` the last commit and add the appropriate trailer
  like how our backport process works

.. code-block::

    Ferrocene-forwardport-of: {pr_number}
    Ferrocene-forwardported-commits: {commits}

Once done, submit a pull request directly mentioning the forward ported pull requests.
