<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# Lockfile management tooling

This directory contains all the tooling needed to update the dependencies tracked in the
`Cargo.lock` and `src/bootstrap/Cargo.lock` files.

## Running the automation

The automation is configured to run every day at 5:00 UTC. You can also
start it manually by going to the [GitHub Actions workflow page][dispatch] and
clicking "Run workflow".

You can also run it locally by running the `update.py` script (no arguments are required).

[dispatch]: https://github.com/ferrocene/ferrocene/actions/workflows/automation-update-lockfiles.yml
