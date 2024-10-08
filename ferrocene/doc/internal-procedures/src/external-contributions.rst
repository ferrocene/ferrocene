.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

External contributions
======================

External contributions are desired, but CI needs to be protected against
arbitrary code execution. Therefore CI does not automatically execute for PRs
from forks. Before executing, confidence in the code needs to be established.
To establish confidence, the code must be reviewed by a Ferrocene Developer and
marked as trusted.

The workflow to establish confidence is as follows:

1. Review the changes. The main threats are leaking credentials and arbitrary
   code execution. If the PR includes any suspicious changes, talk to the PR
   author or close the PR. Only continue if the changes are trustworthy.

2. Fetch the PR commits locally. ``$ID`` is the PR number.

   .. code-block:: console

      $ git fetch origin pull/$ID/head:$BRANCHNAME
      $ git switch $BRANCHNAME

3. Push the PR commits to origin. This marks the commits as trusted and
   triggers the CI commit checks.

   .. code-block:: console

      $ git push origin $BRANCHNAME

4. If new commits are pushed to the PR, steps 1-3 need to be repeated. 

5. After the commit checks pass, approve the PR and "bors merge" (the normal
   workflow).

6. After the PR is merged, delete the branch

   .. code-block:: console

      $ git branch -d --remotes origin/$BRANCHNAME
      $ git branch -D $BRANCHNAME

See also
^^^^^^^^

- https://circleci.com/blog/triggering-trusted-ci-jobs-on-untrusted-forks/
- https://stackoverflow.com/questions/27567846/how-can-i-check-out-a-github-pull-request-with-git
