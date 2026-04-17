.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers
   SPDX-FileCopyrightText: The Rust Project Contributors

Contributing to the FLS
=======================

Contribution Process
--------------------

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
----------------

Whether the changes appear in the FLS will remain the responsibility and
discretion of the `FLS team`_. Changes may be approved as is, or may be edited
to match the tone, style and format of the FLS.

Again, we thank you for your patience as we address the suggested changes and,
last but not least, thank you for your interest and contributions to the FLS!

Line length
-----------

Be sure that the paragraphs in your PRs occupy a single line. If a change modifies existing text that is not formatted to match this rule (*unwrapping*), split the semantic change and the one that unwraps the paragraph into separate commits, to ease review.

.. _FLS team: https://rust-lang.org/governance/teams/lang/#team-fls
