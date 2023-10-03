<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Subtree management tooling

This directory contains all the tooling needed to manage the subtrees added by
Ferrocene.

## Introduction: what are subtrees?

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
merged repository changes. This directory contains wrappers and enhancements to
the builtin tooling, improving the experience for Ferrocene maintainers.

## Adding a new subtree

Subtrees are configured in the `ferrocene/tools/pull-subtrees/subtrees.yml` file.
When you want to add a new subtree, merge a PR adding a line to that file: the
next time the automation is run the subtree will be imported automatically.

If the repository you added is private, you'll also need to [install the
`ferrocene-pull-subtrees` app][install-ferrocene-pull-subtrees] to enable the
automation to clone the contents of the private repository.

## Running the automation

The automation is configured to run every work day at 5:00 UTC. You can also
start it manually by going to the [GitHub Actions workflow page][dispatch] and
clicking "Run workflow".

If you have access to all the subtree repositories, you can also run it locally
by running the `pull.py` script (no arguments are required).

[install-ferrocene-pull-subtrees]: https://github.com/organizations/ferrocene/settings/apps/ferrocene-pull-subtrees/installations
[dispatch]: https://github.com/ferrocene/ferrocene/actions/workflows/automation-pull-subtrees.yml
