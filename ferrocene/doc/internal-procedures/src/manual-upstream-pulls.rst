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
Sometimes merge conflicts may occur in which case automation opens an issue
describing the conflict and requiring the upstream pull to be done manually.

Manual intervention when the automation fails
---------------------------------------------

Performing a manual pull differs slightly depending on whether the pull is done
for the `master` or `beta` branch. The following text uses a few angle
bracketed terms that have to be substituted as follows:

* `<BASE_BRANCH>`: The branch of the Ferrocene repository to pull the changes into.
  This is `main` if doing a `master` upstream pull and `release/1.70` for a
  `beta` upstream pull.
* `<UPSTREAM_BRANCH>`: The upstream branch to pull changes from. This is `master`
  if doing a `master` upstream pull and `beta` for a `beta` upstream pull.
* `<NEW_PULL_BRANCH>`: This represents the branch you will be making the pull
  with. Replace it with a branch name of your choice, e.g.
  `pull-upstream-master-2023-05-22`

Initiating the pull
^^^^^^^^^^^^^^^^^^^

Run the following git commands to set up a new local `<NEW_PULL_BRANCH>`
branch to do the pull on::

  git checkout <BASE_BRANCH> # check out the ferrocene branch to do the pull for
  git pull --rebase # make sure your local branch is up to date
  git submodule update # update any submodules that might've changed
  git checkout -b <NEW_PULL_BRANCH> # checkout a new branch to do the pull with

Performing the pull
^^^^^^^^^^^^^^^^^^^

Now run::

  ferrocene/tools/pull-upstream/pull.sh <UPSTREAM_BRANCH>

The script will fetch the latest changes from `rust-lang/rust`'s
`<UPSTREAM_BRANCH>` branch, create a commit to remove the excluded files
(see below) from the upstream branch, and finally try to merge upstream into
the current branch.

The script is able to resolve some kinds of merge conflicts on its own. If
there are still remaining merge conflicts after the automatic resolution the
script will exit. You'll have to fix the merge conflicts yourself in that case:

There are two kinds of conflicts you will encounter in the output that the
script produces:.

* Submodule conflict ::

    Please manually handle the merging of each conflicted submodule.
    This can be accomplished with the following steps:
    - go to submodule (src/tools/cargo), and either merge commit <COMMIT_HASH>
      or update to an existing commit which has merged those changes
    - come back to superproject and run:

          git add src/tools/cargo

      to record the above merge or update
    - resolve any other conflicts in the superproject
    - commit the resulting index in the superproject

  The message already describes how to fix this conflict. The commands required
  for this example are::

    cd src/tools/cargo
    git merge <COMMIT_HASH>
    cd ../../..
    git add src/tools/cargo

* General conflicts::

    CONFLICT (content): Merge conflict in src/bootstrap/flags.rs

  These are more common conflicts and will require manual fixing depending on
  the conflict in source.

After having fixed the conflicts, finish up the merge by doing the following::

  git add . # add your changes to the staging area
  git merge --continue # finish the merge
  git push --set-upstream origin <NEW_PULL_BRANCH> # push out the pull request branch

Finally generate the PR description with::

  ferrocene/tools/pull-upstream/generate_pr_body.py origin <BASE_BRANCH> <NEW_PULL_BRANCH>

This will output a markdown list of the pull requests that have been pulled
from the upstream Rust repository. Paste that into the PR description of
the pull request you create from your `<NEW_PULL_BRANCH>` branch.
You may omit `<NEW_PULL_BRANCH>` in which case the script will use the currently checked
out branch.
