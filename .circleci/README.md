<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# CircleCI configuration

This directory contains the configuration for Ferrocene's CI, powered by
CircleCI.

If you need to change what's executed as part of CI you likely want to change
the `workflows.yml` file.

## Configuration structure

The repository relies on CircleCI's [dynamic configuration] feature to
calculate workflow parameters before the workflow can be executed. This is
because we need to calculate the tag of the Docker images dynamically, and it's
not possible to do so as part of the same workflow (since the images are pulled
before any custom code is executed).

The dynamic configuration implementation in this repository tries to stay as
static as possible, with no dynamically generated elements except for the
values of the pipeline parameters. This is intentionally done to reduce the
effort needed to understand and change the configuration, and to be as close as
possible to "standard" CircleCI configuration.

[dynamic configuration]: https://circleci.com/docs/2.0/dynamic-config/
