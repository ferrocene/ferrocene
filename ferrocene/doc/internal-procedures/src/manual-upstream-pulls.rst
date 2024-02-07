.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Upstream Pulls
==============

What is an Upstream Pull
------------------------

An "upstream pull" is a pull request that pulls in commits from the upstream Rust
repository back into the Ferrocene repository to keep them in sync. This
procedure is usually performed by a periodic Github Action that opens a pull
request automatically, only requiring sign off of a reviewer to get merged.
Sometimes merge conflicts may occur in which case the conflict needs to to be
resolved manually.

Sign off Upstram Pull
---------------------

To sign off the automatic upstream pull, the reviewer needs to approve the PR and
comment `bors merge`. This will trigger the "full" CI workflow. If the workflow
succeeds, the Upstream Pull is getting merged.

Manually resolving merge conflicts
----------------------------------

If an automatic upstream pull creates merge conflicts, the automation commits the
merge conflicts and they need to be resolved manually.

Start by checking out the PR. The name of the branch is
`automation/pull-upstream/<ID>`, e.g. `automation/pull-upstream/dfvjj2s4`::

    git switch automation/pull-upstream/<ID>

In order to find the conflicts you can either look at the CI logs, or search for the
git conflict markers (`<<<<<<<`, `=======` and `>>>>>>>`).

There are two kinds of conflicts you will encounter in the output that the
script produces:.

* Submodule conflict
  To fix a submodule conflict run::

    cd src/tools/cargo
    git merge <COMMIT_HASH>
    cd ../../..
    git add src/tools/cargo

  <COMMIT_HASH> is ...

* General conflict
  These are more common conflicts and will require manual fixing depending on
  the conflict in source.

After having fixed the conflicts, add a new commit and push it to the PR::

  git add . # add your changes to the staging area
  git commit
  git push

Now another developer needs to sign off the PR (see "Sign off Upstream Pull" section above).
