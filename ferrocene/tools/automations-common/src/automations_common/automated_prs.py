# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Shared tooling to create automated PRs from CI.
#
# Consumers of this module should create a subclass of `AutomatedPR` and
# implement all the abstract methods, and then call the `create` method.

import abc
import enum
import os
import sys
import random
import requests
import string
import subprocess


DEFAULT_BASE_BRANCH = "main"
ORIGIN = "origin"

#####################
#                   #
#     Utilities     #
#                   #
#####################

# These log to stderr to avoid interleaving output between stdout and stderr.
def log(*args, **kwargs):
    print("info:", *args, file=sys.stderr, **kwargs)
def warn(*args, **kwargs):
    print("warning:", *args, file=sys.stderr, **kwargs)
def err(*args, **kwargs):
    print("error:", *args, file=sys.stderr, **kwargs)

def pretty_print_cmd(*args, **kwargs):
    if isinstance(args[0], list):
        args = args[0]
    # TODO: make this prettier
    return '"' + '" "'.join([str(a) for a in args]) + '"'

def cmd(*args, dry_run=False, **kwargs):
    """
    Run a command and error out if it fails to execute.
    """
    kwargs.setdefault("check", True)
    c = pretty_print_cmd(*args, **kwargs)
    if dry_run:
        log(f"dry_run: not running command: {c}")
    else:
        log(f"run cmd: {c}")
        return subprocess.run(*args, **kwargs)

def cmd_capture(*args, **kwargs):
    """
    Run a command, error out if it fails to execute and return its stdout.
    """
    kwargs.setdefault("stdout", subprocess.PIPE)
    kwargs.setdefault("text", True)
    if (stdout := cmd(*args, **kwargs).stdout) is not None:
        lines = stdout.splitlines()
        if len(lines):
            log("stdout:", lines[0])
        if len(lines) > 1:
            log("stdout: ...")
        print(file=sys.stderr)
        return stdout.strip()


#####################
#                   #
#        API        #
#                   #
#####################

class AutomationResult(enum.Enum):
    SUCCESS = 1
    FAILURE = 2
    NO_CHANGES = 3


class AutomatedPR(abc.ABC):
    # backcompat to avoid runtime errors
    def cmd(self, *args, **kwargs):
        return cmd(*args, **kwargs)
    def cmd_capture(self, *args, **kwargs):
        return cmd_capture(*args, **kwargs)

    #####################
    #                   #
    #    Entrypoints    #
    #                   #
    #####################

    def create(self, dry_run=False):
        """
        Handle the creation of the PR, and open an issue if an error occurs.
        """
        self.dry_run = dry_run
        self.origin = ORIGIN

        self.http = requests.Session()
        self.http.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"

        self.repo_root = self.cmd_capture(["git", "rev-parse", "--show-toplevel"])

        existing_pull = self.__find_open("pulls", self.pr_title(), self.pr_labels())
        if existing_pull is not None:
            log("An automated PR is already open, a new one won't be created.")
            log(f"==> {existing_pull['html_url']}")
            return

        existing_conflict_issue = self.__find_open(
            "issues", self.error_issue_title(), self.error_issue_labels()
        )

        current_branch = self.cmd_capture(["git", "branch", "--show-current"])
        current_hash = self.cmd_capture(["git", "rev-parse", "HEAD"])

        # A random branch name is used to avoid GitHub API errors if two PRs are
        # opened with the same branch (for example if one of the labels is removed).
        branch_name = (
            "automation/"
            + self.automation_name()
            + "/"
            + "".join(random.choices(string.ascii_lowercase + string.digits, k=8))
        )

        result = self.run()
        if result == AutomationResult.SUCCESS:
            self.on_success(branch_name, current_branch, existing_conflict_issue)
        elif result == AutomationResult.FAILURE:
            self.on_failure(existing_conflict_issue)
        else:
            assert(result == AutomationResult.NO_CHANGES)

        # Reset the commit at the state it was before the automation ran.
        # Otherwise, if multiple automations are executed in the same job,
        # the following PRs will also include the changes of the previous PRs.
        self.cmd(["git", "reset", "--hard", current_hash], dry_run=self.dry_run)

    def on_success(self, branch_name, current_branch, existing_conflict_issue):
        self.cmd(["git", "checkout", "-B", branch_name], dry_run=self.dry_run)
        self.cmd(["git", "push", self.origin, branch_name, "-f"], dry_run=self.dry_run)

        # Create the PR
        if self.dry_run:
            log(f"dry_run: not opening PR for {branch_name}->{self.base_branch()}")
        else:
            response = self.http.post(
                self.__repo_api("pulls"),
                json={
                    "title": self.pr_title(),
                    "head": branch_name,
                    "base": self.base_branch(),
                    "body": self.pr_body(branch_name),
                    "maintainers_can_modify": True,
                },
            )
            response.raise_for_status()
            create_json = response.json()

            # Apply the label to the PR
            response = self.http.post(
                self.__repo_api(f"issues/{create_json['number']}/labels"),
                json={
                    "labels": list(self.pr_labels()),
                },
            ).raise_for_status()

        # Close the "there is a conflict" if it's still open
        if existing_conflict_issue is not None:
            if self.dry_run:
                log(f"dry_run: not closing existing issue {existing_conflict_issue['url']}")
            else:
                self.http.post(
                    existing_conflict_issue["comments_url"],
                    json={
                        "body": self.error_issue_fixed_comment(create_json["html_url"]),
                    },
                ).raise_for_status()
                self.http.patch(
                    existing_conflict_issue["url"],
                    json={
                        "state": "closed",
                    },
                ).raise_for_status()

        self.cmd(["git", "checkout", current_branch], dry_run=self.dry_run)
        self.cmd(["git", "branch", "-D", branch_name], dry_run=self.dry_run)

    def on_failure(self, existing_conflict_issue):
        if self.dry_run:
            log("dry_run: not opening remote issue")
            return
        if existing_conflict_issue is None:
            # Create an issue alerting the team that the pull failed
            response = self.http.post(
                self.__repo_api("issues"),
                json={
                    "title": self.error_issue_title(),
                    "body": self.error_issue_body(),
                },
            )
            response.raise_for_status()
            conflict_issue_json = response.json()

            # Apply the label to the issue
            response = self.http.post(
                self.__repo_api(f"issues/{conflict_issue_json['number']}/labels"),
                json={
                    "labels": list(self.error_issue_labels()),
                },
            ).raise_for_status()

        else:
            # If an issue already exists just post a status update
            self.http.post(
                existing_conflict_issue["comments_url"],
                json={
                    "body": self.error_issue_repeated_comment(),
                },
            ).raise_for_status()

    ############################
    #                          #
    #     Abstract methods     #
    #                          #
    ############################

    @abc.abstractmethod
    def automation_name(self):
        pass

    @abc.abstractmethod
    def pr_title(self):
        pass

    @abc.abstractmethod
    def pr_labels(self):
        pass

    @abc.abstractmethod
    def pr_body(self, branch_name):
        pass

    @abc.abstractmethod
    def error_issue_title(self):
        pass

    @abc.abstractmethod
    def error_issue_labels(self):
        pass

    @abc.abstractmethod
    def error_issue_body(self):
        pass

    @abc.abstractmethod
    def error_issue_fixed_comment(self, pull_request_url):
        pass

    @abc.abstractmethod
    def error_issue_repeated_comment(self):
        pass

    @abc.abstractmethod
    def run(self):
        pass

    def base_branch(self):
        return DEFAULT_BASE_BRANCH

    #####################
    #                   #
    #     Internals     #
    #                   #
    #####################

    def __find_open(self, kind, expected_title, expected_labels):
        """
        Return the open pull request with the expected title and label, if present.
        """
        url = self.__repo_api(f"{kind}?state=open&per_page=100")
        while url is not None:
            response = self.http.get(url)
            response.raise_for_status()

            for pr in response.json():
                if pr["title"] == expected_title and expected_labels.issubset(
                    set(label["name"] for label in pr["labels"])
                ):
                    return pr

            if "next" in response.links:
                url = response.links["next"]["url"]
            else:
                url = None

    def __repo_api(self, url):
        """
        Return the API URL for the requested GitHub repository.
        """
        return f"https://api.github.com/repos/{os.environ['GITHUB_REPOSITORY']}/{url}"
