.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers
   SPDX-FileCopyrightText: The Rust Project Contributors

====================================================
Contributing to the FLS
====================================================

The FLS is released publicly under an open source license, specifically MIT and
Apache 2.0.

The specification is still a work in progress, and while we're open to
contributions, we want to manage your expectations on how much we'll be able to
interact with the community in this repository:

* Any change made to the specification text causes extra work for us behind the
  scenes, as we have other tooling required for qualification that consumes the
  specification text.

* Our resources are limited when it comes to reviewing PRs, especially large
  ones.

We'll try our best to review changes proposed by the community, but we might not
be able to review all of them (or they might be out of date once we get to
them). If there are changes you'd like to make, we recommend opening an issue
beforehand, so that we can provide feedback on whether we'll be able to merge
the changes.

We've all dealt with those open source projects that feel open in name only, and
have big patches and history-free source drops appearing from behind the walls
of some large organization. We don't like that, and we're not going to do that.
But please bear with us until we have the capacity to accept all external
contributions.

   This introduction was inspired by Oxide Computer Company's `Hubris
   contribution guidelines
   <https://github.com/oxidecomputer/hubris/blob/master/CONTRIBUTING.md>`_.

Contribution Process
====================

Before contributing, it would be helpful to familiarize yourself with the
grammar and structure of the FLS. You'll find everything you need to in `Chapter
1: General <https://rust-lang.github.io/fls/general>`_ of the specification.

There are three kinds of contribution that can be made:

* **Flags:** Here the contributor makes the `FLS team`_ aware of changes that
  were made upstream, but leaves the changes up to the team. This
  helps the team "keep the moving parts safe" and ensures
  that the team can respond to upstream changes in a timely manner. Please `open an
  issue <https://github.com/rust-lang/fls/issues>`_ for this.

* **Fixes:** These are defined as small errors or suggestions for improvements
  within one or two sentences. The structure of the content still makes sense,
  but the contribution makes the concept more precise or better defined. To
  contribute a fix to the FLS, please open a pull request!

* **Rewrites:** These are bigger changes in which a contributor rewrites an
  entire chapter, section or subsection. Please open an issue to discuss the
  rewrite you want to make, and if we have the capacity to accept and review it
  we'll coordinate on how to best do it.

PRs and approval
================

Whether the changes appear in the FLS will remain the responsibility and
discretion of the `FLS team`_. Changes may be approved as is, or may be edited
to match the tone, style and format of the FLS.

Again, we thank you for your patience as we address the suggested changes and,
last but not least, thank you for your interest and contributions to the FLS!

.. _FLS team: https://rust-lang.org/governance/teams/lang/#team-fls
