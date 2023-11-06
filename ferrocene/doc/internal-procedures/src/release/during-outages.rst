.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Releasing during outages
========================

Our release process relies on third-party infrastructure services, and their
outages might prevent releases from being published. In most cases, delaying
the release until the service in question is back available is the best
approach. These services include GitHub, Github Actions, AWS.

There are certain situations when a release must not wait. If you believe
something requires an emergency release, communicate this to the team and wait
for a decision. Release managers will be responsible for the decision, and at
least two of them need to approve the release. Note that performing the release
during the outage should only be the option of last resort.

Note that the instructions in this page are meant to be executed on a Linux
system, and only release managers have the permissions required to follow these
steps.

Required service dependencies
-----------------------------

Some of the third party services we rely on are a strict dependency of our
release process, and their outage will prevent a release from being published
even when this guide is followed:

* AWS IAM and STS in the ``us-east-1`` and ``eu-central-1`` regions, to
  authenticate. `(AWS Status)`_
* AWS S3 in the ``us-east-1`` region, to download artifacts. `(AWS Status)`_
* AWS S3 in the ``eu-central-1`` region, to store release files. `(AWS
  Status)`_

Step by step instructions
-------------------------

1. If not already present, install `AWS CLI v2`_. Note that most Linux
   distributions only provide v1.

2. If not already present, add this to ``~/.aws/config``:

   .. code-block:: ini

      [profile ferrocene-prod-break-glass-release]
      sso_start_url = https://ferrous-systems.awsapps.com/start
      sso_region = eu-central-1
      sso_account_id = 397686924940
      sso_role_name = FerroceneProdBreakGlassRelease

      [profile ferrocene-prod-break-glass-release-role]
      role_arn = arn:aws:iam::397686924940:role/publish-release
      role_session_name = break-glass
      source_profile = ferrocene-prod-break-glass-release

3. Authenticate with the production AWS account using the company SSO::

      aws sso login --profile ferrocene-prod-break-glass-release

4. Download and compile the release tooling::

      git clone git@github.com:ferrocene/publish-release
      cd publish-release
      cargo build --release

5. Configure the environment (in the current shell):

   .. code-block:: bash

      export AWS_PROFILE=ferrocene-prod-break-glass-release-role
      export PUBLISHRELEASE_ARTIFACTS_ROLE=arn:aws:iam::886866542769:role/publish-release
      export PUBLISHRELEASE_ARTIFACTS_BUCKET=ferrocene-ci-artifacts
      export PUBLISHRELEASE_RELEASES_BUCKET=ferrocene-prod-releases

6. In the same shell as before, perform the release::

      cargo run --release -- $sha

.. _(AWS Status): https://health.aws.amazon.com/health/status
.. _AWS CLI v2: https://docs.aws.amazon.com/cli/latest/userguide/getting-started-install.html
