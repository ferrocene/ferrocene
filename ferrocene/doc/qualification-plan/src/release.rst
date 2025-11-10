.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Release Process
===============

Release List
------------

A record of all past releases is published to `releases.ferrocene.dev <https://releases.ferrocene.dev/ferrocene/index.html>`_. Access to this URL requires a customer account.

Release Channels
----------------

Each Ferrocene release is published in a channel. The category of a channel
indicates the level of readiness, support, and qualification a release has, and
customers are able to either install the latest release from a channel or a
specific (pinned) version released in it.

The following channels are available to customers:

* **nightly**: This channel tracks the latest changes from the ``main`` branch
  and a new release is published on a daily basis on days when changes are done
  to the codebase. There are no stability guarantees, no long term support, no
  warranty, and no qualification. Customers are expected to use nightly only if
  they need to explicitly check a change we just committed.

* **pre-rolling**: This channel tracks the latest beta release *made by
  upstream* and is used to prepare the next rolling release.

* **rolling**: This channel tracks the latest release *made by upstream*, with
  a new release roughly every six weeks. There are no stability guarantees,
  no long term support, no warranty, and no qualification. In practice, this
  channel will likely be more stable than releases in the nightly channel, but
  even this cannot be guaranteed. Customers will likely use this channel when
  trying out new features that will be later included in a stable release, or
  when prototyping.

* **beta-${version}**: This channel is used to prepare the next stable
  release before it is published.

* **stable-${version}:** Each major release of Ferrocene has its own stable
  channel, where both the initial release and all subsequent patch releases are
  published. Releases in this channel will be qualified, and Ferrocene provides
  support and warranty. These releases will also be supported in the long term
  (however long customers are willing to pay for long term support).

All customers have access to the nightly, pre-rolling and rolling channels as
well as access to the stable and beta channels for the versions they subscribe
to. Depending on their plan, they may also have access to just a subset of
releases from the stable and beta channels (for example, only the releases
published in the first five years if that's the duration of their support
contract).

Release Trains
--------------

Ferrocene releases follow the software **release train** **model** used by
upstream. Every time a new release has to be prepared, a commit ("train") is
branched off the ``main`` development branch and gets increasingly stable as
bugs and regressions are fixed and it moves through the channels ("stations").

For Ferrocene, every six weeks a train starts from the nightly channel and
goes to the pre-rolling channel, in which critical bugs and regressions are
fixed. After six weeks, the train moves over to the rolling channel and becomes
available to customers. Whenever Ferrocene developers need to prepare a new
stable release, Ferrocene developers pick the last train on the rolling channel
and move it to the beta channel, where Ferrocene developers perform additional
bug and regression fixes, until it moves to the stable channel as a qualified
version of Ferrocene.

The following diagram shows how the Ferrocene release trains work, with some
sample release numbers:

.. figure:: figures/ferrocene-release-train-diagram.svg

   Release Train Diagram


Release Environments
--------------------

There are two release environments for Ferrocene:

* **dev:** This environment is only available to Ferrocene staff. It allows
  Ferrocene developers to deploy releases or make changes to the release process
  without impacting customers. Here it is allowable to publish broken releases,
  delete or override existing releases, or publish releases for testing
  purposes.

* **prod:** The production environment is the environment Ferrocene customers
  rely on. Releases published in this environment must not be deleted or
  overridden except in extraordinary circumstances (e.g. if proprietary code or
  export control code has been erroneously included in the release). Any release
  published in this environment will be immediately visible to customers.


Scheduled Releases
------------------

Most Ferrocene releases start automatically on a schedule and do not require any
manual intervention. The scheduled jobs are executed in GitHub Actions.

The following releases are currently published on a schedule in the **dev**
environment:

* *None so far.*

The following releases are currently published on a schedule in the **prod**
environment:

* Everyday at 00:00 UTC, the tip of the ``main`` branch is released on the
  nightly channel.
* Everyday at 00:00 UTC, the tip of the branch with the Rust channel of
  ``beta`` is released on the pre-rolling channel.
* Everyday at 00:00 UTC, the tip of the most recent branch with the Rust
  channel of ``stable`` is released on the pre-rolling channel.
* Everyday at 00:00 UTC, the tip of every branch with the Ferrocene channel of
  ``beta`` is released on the beta-${version} channel.


Manually-started Releases
-------------------------

The Ferrocene release tooling also allows Ferrocene developers to manually
initiate the release process to prepare releases that are not currently
scheduled, to restart a scheduled release that failed, or for testing (only for
**dev** environment). Only the initiation of the release process is manual in
this case; all the other steps are fully automated.

If the release targets the **prod** environment, an approval from a Release
Manager is required. A :ref:`release manager <organization:Release Managers>`
needs to approve the release in the GitHub UI before the release can proceed.
Once approved, the release process will start automatically.


Backporting Changes
-------------------

As part of long-term support, Ferrocene customers may request that bug fixes
are backported into stable relases. In addition, Ferrocene engineers may
identify patches that would need to be backported into a release.

If a PR is identified as a candidate for backporting, a GitHub issue is
created for traceability purposes, and also to capture any discussions, impact
assessment, and decisions made concerning the candidate PR.

If the decision is to backport the identified PR, then a Ferrocene engineer
marks the PR with a backport label, such as ``backport:1.68``. Once per day,
the Ferrocene CI infrastructure collects all such labeled PRs into a backport
PR, and performs an automated merge into the appropriate release, following the
Development Process from :ref:`dev-phase-review`. The backport process may be
started manually using a GitHub Action.
