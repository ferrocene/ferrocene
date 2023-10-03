.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Product engineering procedures
==============================

Development procedures have been described in the
:doc:`qualification-plan:index`. These procedures have been developed to ensure
that the Ferrocene toolchain conforms to the appropriate level of quality.


PE methodology
--------------

See the following chapters for the detailed process:

* :doc:`qualification-plan:development`
* :doc:`qualification-plan:testing`
* :doc:`qualification-plan:packaging`
* :doc:`qualification-plan:release`
* :doc:`qualification-plan:change-tracking`
* :doc:`qualification-plan:kp-tracking`
* :doc:`qualification-plan:validation`

PE workflow
-----------

The workflow outlined below describes the expected interactions between the
engineer and the ``ferrocene/ferrocene`` GitHub repository. 

Create branch
~~~~~~~~~~~~~
Create a personal branch of the ``ferrocene/ferrocene`` repository to work on.

.. Note:: 
   Once pushed to GitHub, this branch can be shared with other people. These
   branches however, will not be released.

Ensure that you are branching off the ``main`` branch by checking it out before
creating your own branch.
::
    
    git checkout main

Create a new branch based off the ``main`` branch and immediately check it out.
::

   git checkout -b branch_name

It is suggested that ``branch_name`` should employ the following format:
::

   <personal_prefix>-<description>

where ``personal_prefix`` identifies the author of the eventual PR, and
``description`` is a short description of the branch. For example:
::

   kirtchev-add-branch-guidelines
   pa-self-test-linkers
   tshepang-unused-config-option

Update branch
~~~~~~~~~~~~~

Ensure that you are on the correct branch:
::

   git checkout branch_name

Obtain new updates from the ``main`` branch.

Enact the new updates:
::

   git rebase origin/main

Work
~~~~

Modify files in the repository. It is recommended to save often.

Build
~~~~~

The Rust build system uses a script called ``x`` on Unix and ``x.ps1`` on
Windows to build, test, and package the compiler, libraries, auxiliary tools,
and documentation. The script lives at the root of the project.

The ``x`` command can be run directly on most Unix systems in the following
format:
::

    ./x <subcommand> [flags]

Commit
~~~~~~

Obtain a list of local changes:
::

   git status

Add specific files to the index:
::

   git add file_name

Takes all files from the index and produces a commit in the staging area.

For initial commit:
::

   git commit

``git`` will open an editor where you will need to add the commit message. The
message should contain the information on the scope and goal for the changes 
made.

.. Note::
   By default, the editor ``nano`` is used by ``git commit``. To use your
   prefered editor you will need to set the ``GIT_EDITOR`` environment variable
   with: ``export GIT_EDITOR=<editor>``

For fixing review feedback or amending the previous commit:
::

   git commit --amend

.. Note::
   If the change you’re making is just a fix for the previous one, use
   ``--amend``, otherwise create a new commit.

Push
~~~~

Once the commit has been done and the message redacted, the following commands
allow pushing the changes to GitHub.

For initial push:
::

   git push -u origin branch_name

This submits the contents of the staging area to GitHub.

For subsequent pushes, use:
::

   git push

It is also possible to force pushes if ``git push`` fails following:

* ``git commit --amend``
* ``git rebase``

In this case, use:
::

   git push --force

Pull Request (PR)
~~~~~~~~~~~~~~~~~

Once the changes are pushed, GitHub will send a link to create a PR.
Follow the link to create the PR, add the relevant :ref:`labels
<issues:Labels>`, the appropriate reviewers, and the branch the changes will be
merged to once approved.
