.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Release process overview
========================

This section describes where the release process is implemented, how to change
it, and how to publish new releases. Note that the high-level overview of the
release process is present :doc:`in the qualification plan
<qualification-plan:release>`. Make sure to read the overview before diving
into this section.

Release tooling
---------------

Ferrocene is developed using a custom tool, `publish-release`_, which is
responsible for all of the release steps. The tool is written in Rust and
developed in the ``ferrocene/publish-release`` repository, and it's executed on
GitHub Actions in the ``ferrocene/ferrocene`` repository.

When GitHub Actions invokes the release tool, the
`calculate-release-job-matrix.py`_ script is used to determine which commit(s)
to instruct the release process to release. The script does little when
starting a manual release, but it's fully responsible for determining which
scheduled releases to run.

.. _manual-release:

Publishing a manual release
---------------------------

Our release tooling also allows to manually start the release process. This can
be useful to start releases that are not on a schedule, to restart a scheduled
release that failed, or for testing (in the ``dev`` environment). Only starting
the release is manual, all the actual steps are fully automated.

To manually start a release, go to the `release workflow page`_ and click the
"Run workflow" button. You need to input the git reference you want to release
(it can be a branch name, a short commit hash, or a full commit hash), and the
environment to release in. Note that anyone can publish a release in the
``dev`` environment, while approval from a :ref:`release manager
<qualification-plan:organization:Release Managers>` is required for the
``prod`` environment.

.. note::

   The git reference needs to be a merge commit authored by bors.

   This is because the Release workflow does not actually build anything, but
   just copies the release artifacts to the appropriate storage location.

It's also possible to override some of the startup checks the release process
performs, and to treat the git reference as verbatim instead of trying to
resolve it. The latter option is useful if you need to release a commit which
is not present in the ``ferrocene/ferrocene`` repository.

Once you click the green "Run workflow" button a new job will be queued.

If the release targets the prod environment an approval from a release manager
will be required. The release manager will need to approve the release in the
GitHub UI before the release can proceed. Once it's done, the release process
will start automatically.

If there is a service outage preventing you from manually starting a release
follow the instructions in :doc:`internal-procedures:release/during-outages`.

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

.. _publish-release: https://github.com/ferrocene/publish-release
.. _calculate-release-job-matrix.py: https://github.com/ferrocene/ferrocene/blob/main/ferrocene/ci/scripts/calculate-release-job-matrix.py
.. _release workflow page: https://github.com/ferrocene/ferrocene/actions/workflows/release.yml
