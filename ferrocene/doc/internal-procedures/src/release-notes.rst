.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Release notes maintenance
=========================

Ferrocene maintains its own set of release notes in the
:doc:`release-notes:index` document. This page includes information on the
writing and maintenance of the release notes.

Sections to use
---------------

To standardize the release notes, items should be categorized within the
following sections:

* New features
* New experimental features
* Changes
* Bug fixes
* Deprecations
* Compatibility notes

If a version doesn't have any content it can be omitted.

Marking releases as upcoming
----------------------------

Writing the release notes after all the development finished is error-prone, as
someone has to manually go through all the changes and identify the relevant
ones, with the risk of missing some of them.

To prevent that we add items to the release notes as they are developed,
maintaining a working draft of the release notes of the next release. Those
working drafts should be marked as "upcoming", to alert users the release is
not out yet and the contents are subject to change.

You can mark a release as upcoming by adding this snippet to the `file-wide
metadata
<https://www.sphinx-doc.org/en/master/usage/restructuredtext/field-lists.html#file-wide-metadata>`_
of the document, *before* any non-comment content of the page:

.. code-block:: rst

   :upcoming-release:

Including Rust's release notes
------------------------------

Major releases of Ferrocene bump the bundled Rust version, and when that happens,
we should mention what changed since the last major release of Ferrocene. The
``rust-changelog`` directive takes care of that, automatically including the
release notes of all Rust versions within a semver range.

To use it, add the directive to a section of the changelog, with the ``:from:``
and ``:to:`` options defining the (inclusive) range of versions to include:

.. code-block:: rst

   .. rust-changelog::
      :from: 1.69.0
      :to: 1.76.0
