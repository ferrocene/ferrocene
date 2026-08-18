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
