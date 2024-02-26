.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Upstream Pulls
==============

What is an Upstream Pull
------------------------

An "upstream pull" is a pull request that pulls in commits from the upstream Rust
repository back into the Ferrocene repository to keep them in sync. This
procedure is usually performed by a periodic Github Action that opens a pull
request automatically. Sometimes merge conflicts may occur in which case the conflict
needs to to be resolved manually.

Upstram Pull without merge conflicts
---------------------

To sign off a automatic upstream pull without merge conflicts, the reviewer only
needs to approve the PR and comment `bors merge`. This will trigger the "full" CI
workflow. If the workflow succeeds, the Upstream Pull is getting merged.

Upstream Pull with merge conflicts
----------------------------------

If an automatic upstream pull creates merge conflicts, the automation commits the
merge conflicts and they need to be resolved manually.

Checkout PR
^^^^^^^^^^^

Start by checking out the PR. The name of the branch is
`automation/pull-upstream/<ID>`, e.g. `automation/pull-upstream/dfvjj2s4`::

    git switch automation/pull-upstream/<ID>

Fix conflict
^^^^^^^^^^^^

There are multiple kinds of merge conflicts:

General conflict
""""""""""""""""

In order to find general conflicts search for the git conflict markers (``<<<<<<<``,
``=======`` or ``>>>>>>>``).

Fix the conflict manually and stage the changed files::

  git add <FILE_1> <FILE_2> # stages file(s)

Deletion conflict
"""""""""""""""""

In order to find deletion conflicts search for ``<<<PULL-UPSTREAM>>>``. Automation
inserts this marker into the first line of the conflicting file.

This conflict happens when we made changes to a file and it gets deleted by upstream.
To resolve this, remove the file::

  git rm <FILE_1> <FILE_2> # removes file(s) and stages that change

If this file was a test case used in the traceability matrix, we might need to
update the traceability matrix, if some section is missing a test case. But finding
a new testcase should be done in a separate PR.

Commit and push
^^^^^^^^^^^^^^^

After having fixed the conflicts, add a new commit and push it to the PR::

  git commit
  git push

Review
^^^^^^

Now another developer needs to review and sign off the PR (see "Upstram Pull without
merge conflicts" section above).
