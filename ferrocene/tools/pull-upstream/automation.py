#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Script to automatically create pull requests for pulling from upstream, and
# to open issues if the pulling fails due to a merge conflict. The script is
# meant to be run on GitHub Actions.
#
# Required enviroment variables:
# - `GITHUB_TOKEN`: API token with access to the repo contents, issues and RPs
# - `GITHUB_REPOSITORY`: name of the GitHub repository to run this script on

from typing import Optional
from dataclasses import dataclass
import os
import sys
import generate_pr_body

sys.path.insert(0, os.path.join(os.path.dirname(__file__), "..", "common"))

from automated_prs import AutomatedPR, AutomationResult


class PullUpstreamPR(AutomatedPR):
    def __init__(self, upstream_branch, base_branch):
        self._upstream_branch = upstream_branch
        self._base_branch = base_branch
        super().__init__()

    def run(self):
        """
        Shell out to the script actually pulling from upstream.
        """
        res = self.cmd(
            [
                os.path.join(self.repo_root, "ferrocene/tools/pull-upstream/pull.sh"),
                self._upstream_branch,
                self._base_branch,
            ],
            check=False,
        )
        if res.returncode == 0:
            return AutomationResult.SUCCESS
        elif res.returncode == 42:
            return AutomationResult.NO_CHANGES
        else:
            self.conflict_diff = self.cmd_capture(
                ["git", "diff", "--diff-filter=U", "--no-color"],
            )
            self.cmd(["git", "merge", "--abort"], check=False)
            return AutomationResult.FAILURE

    def automation_name(self):
        return "pull-upstream"

    def pr_title(self):
        return f"Automated pull from upstream `{self._upstream_branch}`"

    def pr_labels(self):
        return {"automation", "backport:never"}

    def pr_body(self, branch_name):
        changes = generate_pr_body.render_changes(
            self.origin, self.base_branch(), branch_name
        )
        return f"""
This PR pulls the following changes from the upstream repository:

{changes}
"""

    def error_issue_title(self):
        return f"Repository diverged from upstream `{self._upstream_branch}`"

    def error_issue_labels(self):
        return {"automation"}

    def error_issue_body(self):
        return """
While trying to pull the latest changes from the upstream \
`{{upstream_branch}}` branch, the automation failed due to the following \
merge conflict:

```diff
{{diff}}
```

The automation will not open any more pull requests pulling from upstream \
until the merge conflict is fixed. There are multiple ways to fix the merge \
conflict:

* If ***all*** the conflicts should be resolved by keeping the Ferrocene changes, \
  [start the workflow][dispatch] with the "ours" merge strategy.
* If ***all*** the conflicts should be resolved by using the new code introduced \
  by Rust upstream, [start the workflow][dispatch] with the "theirs" merge \
  strategy.
* Otherwise, you'll have to fix the merge conflict locally (see below for \
  instructions).

[dispatch]: https://github.com/ferrocene/ferrocene/actions/workflows/automation-pull-upstream.yml

<details>
<summary><i>How to fix the merge conflict manually</i></summary><br>

To fix the conflict, create a new branch and execute the following command:

```
ferrocene/tools/pull-upstream/pull.sh {{upstream_branch}}
```

The command will try pull the latest changes from upstream into the branch, \
and exit when the merge conflict is detected. Fix the merge conflict and then \
run:

```
git merge --continue
```

The command will finish the merge. You'll then be able to push the \
local branch to GitHub and open a PR for it.

</details>
""".replace(
            "{{diff}}", self.conflict_diff
        ).replace(
            "{{upstream_branch}}", self._upstream_branch
        )

    def error_issue_fixed_comment(self, pull_request_url):
        return f"""
The automation successfully pulled the latest changes from upstream in \
{pull_request_url}! Closing this issue.
"""

    def error_issue_repeated_comment(self):
        return "The automation failed to pull the latest changes from upstream again."

    def base_branch(self):
        return self._base_branch


if __name__ == "__main__":
    if len(sys.argv) != 3:
        print(f"usage: {sys.argv[0]} <upstream-branch> <base-branch>")
        exit(1)
    PullUpstreamPR(sys.argv[1], sys.argv[2]).create()
