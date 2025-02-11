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

Each major version will have multiple point releases, identified by an
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

The first step to prepare a stable releases is to promote a rolling branch into
the beta channel of the release. To do so, open a new PR targeting the
``release/1.NN`` branch changing:

* ``ferrocene/version`` with the full version number of the release.
* ``ferrocene/ci/channel`` with ``beta``.

Once the PR is merged, the release process will start automatically publishing
the latest commit on that branch into the ``beta-${major_version}`` channel
every night. You can then continue landing changes into the branch until you
are ready to release it as a stable release.

.. _release-technical-report:

Uploading the qualification technical report
--------------------------------------------

Once qualification is achieved for the Ferrocene major version, the technical
report provided by the assessors needs to be uploaded to our AWS account with::

   aws --profile ferrocene-ci s3 cp path/to/report.pdf s3://ferrocene-ci-mirrors/manual/tuv-technical-reports/YYYY-MM-DD-ferrocene-VERSION-technical-report.pdf

In the command above, ``path/to/the/report.pdf`` is the local path to the
downloaded file, ``YYYY-MM-DD`` is the current date (**not** the version
number), and ``VERSION`` is the version number.

Once the file is uploaded, open a new PR targeting the ``release/1.NN`` branch
changing ``ferrocene/ci/configure.sh``. In that file, find the line setting the
``technical-report-url`` option, if commented uncomment it, and replace the URL
with the ``s3://`` URL you just uploaded.

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
release version in ``ferrocene/version`` by 1. Note that you might need to
remove some digital signatures when you increment the version number.

Remove upcoming notes in the ``main`` branch
--------------------------------------------

After publishing the stable release, send a PR to the ``main`` branch to:

* Remove the ``:upcoming-release:`` role at the top of the release notes page
  for this release.

* Remove all mentions of ``:upcoming:`YY.MM``` in the documentation, where
  ``YY.MM`` is the current version number.
