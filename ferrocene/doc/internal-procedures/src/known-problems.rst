.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Known Problems
==============

What are known problems
-----------------------

Known problems are what we call the currently discovered soundness issues in
the Rust compiler.
We need to list and describe these in a manner that we
can show them to customers to make them aware of these issues.

The database and tooling for these live in https://github.com/ferrocene/problems.

Classifying and describing a problem
------------------------------------

When describing a new issue, the first step is to check if the issue applies to
one of our supported targets.
If that is not the case, we can discard it and record it in the `ignored-issues.yml` list with the
target as its ignore reason as it won't be relevant to our customers.

If it does apply, copy the `template.md` into the src folder, rename it to `KP-R<ISSUE-NUMBER>.md`
and fill out the sections appropriately.

frontmatter `title`
^^^^^^^^^^^^^^^^^^^

The title summarizes the gist of the issue in a single phrase in terms of
what went wrong.
It's easiest to look at another issue of the same category for inspiration.

frontmatter `references`
^^^^^^^^^^^^^^^^^^^^^^^^

The `references` frontmatter links to the corresponding rust-lang/rust issue on github.

frontmatter `tag`
^^^^^^^^^^^^^^^^^

The `tag` frontmatter describes the category this issue belongs to. You can see the current
categories here https://github.com/ferrocene/problems/blob/main/config.yml.

frontmatter `introduced-commit`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The `introduced-commit` frontmatter lists the hash of the commit that
introduced the issue.
The hash is used to figure out from what Ferrocene version the issue applies.
If it is not known since when the issue exists, set the field to `unknown`,
marking it applicable on all releases.

frontmatter `fixed-commit`
^^^^^^^^^^^^^^^^^^^^^^^^^^

The `fixed-commit` frontmatter lists the hash of the commit(s) that fixed the
issue if it has already been fixed.
Leave it blank if the issue is still open.

frontmatter `duplicate`
^^^^^^^^^^^^^^^^^^^^^^^

The duplicate field is optional and marks whether a new problem is a duplicate
of an older one. `introduced-commit` and/or `fixed-commit` may be different.
The problem is marked as duplicate if
it is considered as duplicate upstream.

frontmatter `test-paths`
^^^^^^^^^^^^^^^^^^^^^^^^

The `test-paths` field references paths of all tests that test the absence of the fixed known problem.
This field is only populated for known problems that have their `fixed-commit` filled out.

heading `Description`
^^^^^^^^^^^^^^^^^^^^^

The `Description` section gives a general description of the issue, optionally
with a corresponding source code snippet that shows the issue.

heading `Workaround`
^^^^^^^^^^^^^^^^^^^^

The `Workaround` section describes how to avoid running into the issue.
This usually consists of rules that forbid certain code patterns.

heading `Detection`
^^^^^^^^^^^^^^^^^^^

The `Detection` section describes how to detect occurrences of this issue.
This usually consists of instructions like looking for certain code patterns or
strings.

heading `Mitigation`
^^^^^^^^^^^^^^^^^^^^

The `Mitigation` section describes how to fix detected occurrences of the issue
in a codebase.
This usually consists of instructions on either removing the offending code
patterns or replacing them with similar behaving constructs that do not invoke
the issue.
