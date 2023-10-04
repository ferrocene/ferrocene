#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script is responsible for calculating the list of release jobs we should
# start, as part of the .github/workflows/release.yml GitHub Actions workflow.

from dataclasses import dataclass
import base64
import boto3
import json
import os
import re
import requests
import sys


# Regex matching all branches that should be released by our tooling.
RELEASE_BRANCHES_RE = re.compile(r"^(main|release\/1\.[0-9]+)$")

S3_BUCKET = "ferrocene-ci-artifacts"
S3_PREFIX = "ferrocene/dist"


def commits_in_release_branches(ctx):
    commits = []

    # This is an unconditional note that's always emitted. We do that to give
    # insights to someone looking at the logs to figure out why a branch isn't
    # being picked up.
    print("note: only protected branches are considered here", file=sys.stderr)

    url = f"https://api.github.com/repos/{ctx.repo}/branches?protected=true&per_page=100"
    while url is not None:
        response = ctx.http.get(url)
        response.raise_for_status()

        # Handle pagination
        if "next" in response.links and response.links["next"]["url"] is not None:
            url = response.links["next"]["url"]
        else:
            url = None

        for branch in response.json():
            name = branch["name"]
            if RELEASE_BRANCHES_RE.search(name) is None:
                print(
                    f"note: branch `{name}` doesn't seem to be a release branch",
                    file=sys.stderr,
                )
            else:
                commits.append(branch["commit"]["sha"])

    return commits


def resolve_ref(ctx, ref):
    response = ctx.http.get(f"https://api.github.com/repos/{ctx.repo}/commits/{ref}")
    response.raise_for_status()
    return response.json()["sha"]


def commits_to_releases(ctx, all_commits):
    for commit in all_commits:
        metadata = build_metadata(ctx, commit)
        rust = metadata.rust_channel
        ferrocene = metadata.ferrocene_channel

        if metadata.ferrocene_version == "rolling":
            ferrocene_major = metadata.ferrocene_version
        else:
            ferrocene_major = ".".join(metadata.ferrocene_version.split(".")[:2])

        if ferrocene == "rolling":
            if rust == "nightly":
                yield PendingRelease(commit, metadata, "nightly")
            elif rust == "beta":
                yield PendingRelease(commit, metadata, "pre-rolling")
            elif rust == "stable":
                yield PendingRelease(commit, metadata, "rolling")
            else:
                raise RuntimeError(f"unknown rust channel `{rust}` (in `{branch}`)")
        elif ferrocene == "beta":
            yield PendingRelease(commit, metadata, f"beta-{ferrocene_major}")
        elif ferrocene == "stable":
            yield PendingRelease(commit, metadata, f"stable-{ferrocene_major}")
        else:
            raise RuntimeError(
                f"unknown ferrocene channel `{ferrocene}` (in `{branch}`)"
            )


def filter_automated_channels(releases):
    rolling_releases = []
    for release in releases:
        if release.channel == "nightly":
            yield release
        elif release.channel == "pre-rolling":
            yield release
        elif release.channel.startswith("beta-"):
            yield release
        elif release.channel == "rolling":
            version = [int(num) for num in release.metadata.rust_version.split(".")]
            rolling_releases.append((version, release))
        else:
            print(
                "note: channel {release.channel} cannot be released automatically",
                file=sys.stderr,
            )

    rolling_releases.sort(key=lambda vr: vr[0])
    # When starting from a squashed repo with no release branches yielding the
    # latest rolling release would crash the script.
    if rolling_releases:
        yield rolling_releases.pop()[1]
    for discarded in rolling_releases:
        print(
            f"note: version {discarded[1].metadata.rust_version} is not the latest "
            "in the rolling channel",
            file=sys.stderr,
        )


def prepare_github_actions_output(ctx, pending_releases):
    if ctx.event_name == "schedule":
        environment = "release-prod-automated"
        name_suffix = "automated"
        command_suffix = ""
    elif ctx.event_name == "workflow_dispatch":
        inputs = ctx.event_data["inputs"]

        environment = f"release-{inputs['env']}-manual"
        name_suffix = "manual"
        command_suffix = ""

        if inputs["override-existing"] == "true":
            command_suffix += " --allow-overriding-existing-releases"
            name_suffix += ", allow overriding"
        if inputs["allow-duplicate"] == "true":
            command_suffix += " --allow-duplicate-releases"
            name_suffix += ", allow duplicates"

    jobs = []
    for release in pending_releases:
        jobs.append(
            {
                "name": f"{release.channel} ({name_suffix})",
                "environment": environment,
                "command": f"ferrocene/ci/scripts/publish-release.sh {release.commit}{command_suffix}",
            }
        )
    return jobs


def build_metadata(ctx, commit):
    response = ctx.s3.get_object(
        Bucket=S3_BUCKET, Key=f"{S3_PREFIX}/{commit}/ferrocene-ci-metadata.json"
    )
    metadata = json.loads(response["Body"].read())

    if metadata["metadata_version"] == 1:
        return BuildMetadata(
            rust_channel=metadata["channel"],
            rust_version=metadata["version"],
            # Version 1 of the build metadata did not have the Ferrocene
            # channel in it, but on the other hand all of its releases were for
            # the "rolling" channel, so we hardcode it.
            ferrocene_channel="rolling",
            ferrocene_version="rolling",
        )
    elif metadata["metadata_version"] == 2:
        return BuildMetadata(
            rust_version=metadata["rust_version"],
            rust_channel=metadata["rust_channel"],
            ferrocene_channel=metadata["ferrocene_channel"],
            ferrocene_version=metadata["ferrocene_version"],
        )
    else:
        raise RuntimeError(
            "unexpected ferrocene-ci-metadata.json version "
            f"`{metadata['metadata_version']}` (for commit `{commit}`)"
        )


def run():
    s3 = boto3.client("s3")

    http = requests.Session()
    http.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"

    repo = os.environ["GITHUB_REPOSITORY"]
    event_name = os.environ["GITHUB_EVENT_NAME"]
    event_path = os.environ["GITHUB_EVENT_PATH"]

    with open(event_path) as f:
        event_data = json.load(f)

    ctx = Context(
        s3=s3, http=http, repo=repo, event_name=event_name, event_data=event_data
    )

    if ctx.event_name == "schedule":
        commits = commits_in_release_branches(ctx)
        releases = filter_automated_channels(commits_to_releases(ctx, commits))
    elif ctx.event_name == "workflow_dispatch":
        commit = resolve_ref(ctx, ctx.event_data["inputs"]["ref"])
        releases = commits_to_releases(ctx, [commit])
    else:
        raise RuntimeError(f"unsupported event name: {event_name}")


    output = prepare_github_actions_output(ctx, releases)
    print(f"jobs={json.dumps(output)}")


@dataclass
class BuildMetadata:
    rust_version: str
    rust_channel: str
    ferrocene_channel: str
    ferrocene_version: str


@dataclass
class PendingRelease:
    commit: str
    metadata: BuildMetadata
    channel: str


@dataclass
class Context:
    s3: object
    http: object
    repo: str
    event_name: str
    event_data: object


if __name__ == "__main__":
    run()
