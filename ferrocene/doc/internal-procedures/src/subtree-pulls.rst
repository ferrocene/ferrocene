.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Subtree Pulls
==============

What is a Subtree
-----------------

The most common way to include a repository inside another, git submodules,
works by adding metadata on what repository and revision should be included as
a submodule, along with the path the submodule should be cloned into. Git
clients then can be instructed to clone all the submodules of a repository.

A big drawback of submodules when dealing with private repositories is that the
git client must be authorized to access the repositories of all submodules,
otherwise it will have an incomplete repository. This is problematic to setup
securely on GitHub and the CI systems we use.

A different approach to including repositories inside another is subtrees.
While with submodules each repository is still a fully separate git repository,
subtrees merge the history of the included repository inside the parent
repository's history, resulting in a single bigger repository.

Subtrees provide builtin tooling to keep those inclusions up to date as the
merged repository changes.

What is a Subtree Pull
----------------------

Similar to upstream pulls, a subtree pull pulls in new commits from a corresponding
subtree into the Ferrocene repository to keep them in sync. Likewise this
procedure is usually performed by a periodic Github Action that opens a pull
request automatically, only requiring sign off of a reviewer to get merged.
Sometimes merge conflicts may occur in which case automation opens an issue
describing the conflict and requiring the upstream pull to be done manually.

Backtrace subtree pull
^^^^^^^^^^^^^^^^^^^^^^

When `rust-lang/backtrace-rs` is updated, the modifications to its `Cargo.toml`
file must be copied over to the relevant section in `library/std/Cargo.toml` instead,
where a comment marks the start of the `backtrace` crate dependencies.

This is due to `std` including the sources of backtrace-rs directly, instead of
depending on it as a cargo dependency.

In addition, for any new dependency added this way, `<new-crate>/rustc-dep-of-std` must be
added to the `backtrace` array in `[features]`.

Finally, the crate must be added to `PERMITTED_STDLIB_DEPENDENCIES` in `tidy/src/deps.rs`.

Manual intervention when the automation fails
---------------------------------------------

Performing a manual pull works similar to doing a manual upstream pull.
The following text will make use of the term `<NEW_PULL_BRANCH>` to refer
to the new branch created by the pull initiator to perform the pull on.

Initiating the pull
^^^^^^^^^^^^^^^^^^^

Run the following git commands to set up a new local `<NEW_PULL_BRANCH>`
branch to do the pull on::

  git checkout origin/main # check out the ferrocene branch to do the pull for
  git pull --rebase # make sure your local branch is up to date
  git submodule update # update any submodules that might've changed
  git checkout -b <NEW_PULL_BRANCH> # checkout a new branch to do the pull with

Performing the pull
^^^^^^^^^^^^^^^^^^^

Now run::

  ferrocene/tools/pull-subtrees/pull.py --target main

The script will attempt to update all the registered subtrees in order
until all succeed or the first one fails.

The `--subtree` argument can be used to perform a manual pull for a single subtree::
  ferrocene/tools/pull-subtrees/pull.py --target main --subtree <SUBTREE_REPO>

Note that `<SUBTREE_REPO>` must be the repository of the subtree and not its path.

If merging fails for one of the subtrees the script will stop and report which
one failed as well as the conflicts that have occured.

After having manually fixed the conflicts, finish up the merge by doing the following::

  git add . # add your changes to the staging area
  git merge --continue # finish the merge
  git push --set-upstream origin <NEW_PULL_BRANCH> # push out the pull request branch

Finally open up a pull request from the `<NEW_PULL_BRANCH>`.
