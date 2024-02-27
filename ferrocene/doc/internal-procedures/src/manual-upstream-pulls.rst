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

Upstream Pull without merge conflicts
-------------------------------------

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

    git fetch
    git switch automation/pull-upstream/<ID>

Fix conflict
^^^^^^^^^^^^

To find the conflicting files run following command::

  python ferrocene/ci/scripts/detect-conflict-markers.py

There are multiple kinds of merge conflicts:

General conflict
""""""""""""""""

For general conflicts the output will look similar to this::

  compiler/rustc_abi/Cargo.toml: conflict between lines 18 and 20

Fix the conflict manually and stage the changed files::

  git add <FILE_1> <FILE_2> # stages file(s)

Deletion conflict
"""""""""""""""""

For deletion conflicts the output will look similar to this::

  compiler/rustc_abi/src/layout.rs: file deleted by one side of the merge

This conflict happens when we made changes to a file and it gets deleted by
upstream.

Usually this is solved by removing the file::

  git rm <FILE_1> <FILE_2> # removes file(s) and stages that change

Nevertheless it is necessary to make sure that the purpose we changed the file
for is preserved. For example if this file was a test case used in the
traceability matrix, we might need to find a new testcase. Or if we made
changes to a bootstrap file, we might need to port this change to another file.
These changes should happen in a separate PR though.

Commit and push
^^^^^^^^^^^^^^^

After having fixed the conflicts, commit your changes, push them to the branch
and follow `Upstream Pull without merge conflicts`_.
