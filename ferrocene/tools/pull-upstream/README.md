<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Pulling upstream changes

This directory contains the tooling needed to pull the changes Rust upstream
made into Ferrocene. The tooling is meant to be run by a periodic scheduled
job, but it's possible to also run it locally (which is needed if there is a
conflict the periodic job can't resolve on its own).

## Automation

GitHub Actions is configured to run the automation built around the tool
overnight during business days. The automation will:

* If there are no merge conflicts and no other PR opened by the automation,
  open a new PR pulling the latest changes from upstream.
* If there are merge commits, open an issue detailing the merge commit.

You can also manually run the automation by going to [the workflow
page][workflow] and clicking "Run workflow".

[workflow]: https://github.com/ferrocene/ferrocene/actions/workflows/automation-pull-upstream.yml

## Running the tool

To run the tool, switch to a new branch and run the script:

```
ferrocene/tools/pull-upstream/pull.sh
```

The script will fetch the latest changes from `rust-lang/rust`'s `master`
branch, create a commit to remove the excluded files (see below) from the
upstream branch, and finally try to merge upstream into the current branch.

The script is able to resolve some kinds of merge conflicts on its own. If
there are still remaining merge conflicts after the automatic resolution the
script will exit. You'll have to fix the merge conflicts yourself and then run:

```
git merge --continue
```

## Excluding files from the pull

There are some files in upstream's repository that we don't want to pull. Some
of the categories of files we don't want to pull are:

* Configuration files for tooling used by upstream, like the `.github`
  directory or `triagebot.toml`. We don't need those configuration files and
  they might actually interfere with Ferrocene's tooling.

* Upstream-specific files that don't apply to Ferrocene. There are no files in
  this category as of July 2021.

* Files we can't ship due to licensing reasons. There are no files in this
  category as of July 2021.

You can exclude a file (or multiple files matching a pattern) by adding the
`ferrocene-avoid-pulling-from-upstream` attribute to it in the `.gitattributes`
file. Once the attribute is in place the tool will avoid pulling changes of
those files into the repository, and you can safely delete or change them
without worrying about upstream messing with them.
