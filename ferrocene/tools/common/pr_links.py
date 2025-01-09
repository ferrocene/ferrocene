#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Utility to generate references to PRs in issue bodies. We cannot simply use a
# GitHub reference (like #12345), as that'd generate backlinks in the target
# PR, which is unnecessary noise. We thus fetch the PR title and construct the
# link manually, taking care of avoiding to generate backlinks.

import os
import requests
import sys


# Backlinks are not inserted if www. is prefixed to the domain, even though in
# the end it redirects to the www-less domain.
DOMAIN_WITHOUT_BACKLINKS = "www.github.com"


class PRLinker:
    def __init__(self):
        self.client = requests.Session()
        if "GITHUB_TOKEN" in os.environ:
            self.client.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"
            self.has_token = True
        else:
            self.has_token = False

    def link(self, repo, number):
        if type(number) == str and "#" in number:
            number = int(number.rsplit("#", 1)[1])
        url = f"https://{DOMAIN_WITHOUT_BACKLINKS}/{repo}/issues/{number}"

        try:
            details = self._get(f"https://api.github.com/repos/{repo}/issues/{number}")
        except requests.exceptions.HTTPError as e:
            # In rare cases it happens that PRs upstream get deleted by GitHub itself due to being
            # created by spam accounts. Avoid crashing the script in those cases.
            #
            # This happened in https://github.com/rust-lang/rust/pull/134884 for example.
            if e.response.status_code == 404:
                return f"`{number}`: [*!!! Could not retrieve the PR name !!!*]({url})"
            else:
                raise e

        return f"`{number}`: [{details['title']}]({url})"

    def _get(self, url):
        resp = self.client.get(url)
        if resp.status_code == 429:
            print("error: failed to retrieve PR names: rate limited", file=sys.stderr)
            if not self.has_token:
                print(
                    "The rate limit for unauthenticated requests is low, set a GitHub token"
                    "in $GITHUB_TOKEN to increase it.",
                    file=sys.stderr,
                )
        resp.raise_for_status()
        return resp.json()
