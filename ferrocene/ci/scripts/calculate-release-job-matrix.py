#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# This script is responsible for calculating the list of release jobs we should
# start, as part of the .github/workflows/release.yml GitHub Actions workflow.
#
# The script is meant to be executed inside of GitHub Actions, and relies on
# environment variables set by it. There are helpers to run it locally though:
#
#     calculate-release-job-matrix.py local-test schedule
#     calculate-release-job-matrix.py local-test dispatch key1=value1 key2=value2
#

from dataclasses import dataclass, field
import base64
import boto3
import botocore
import collections
import json
import os
import re
import requests
import sys


# Regex matching all branches that should be released by our tooling.
RELEASE_BRANCHES_RE = re.compile(r"^(main|release\/1\.[0-9]+)$")

S3_BUCKET = "ferrocene-ci-artifacts"
S3_PREFIX = "ferrocene/dist"

LOCAL_TEST_REPO = "ferrocene/ferrocene"

RELEASES_ACCESS_ROLES = {
    "dev": "arn:aws:iam::173318036596:role/release-scheduler",
    "prod": "arn:aws:iam::397686924940:role/release-scheduler",
}


def commits_in_release_branches(ctx):
    commits = []

    # This is an unconditional note that's always emitted. We do that to give
    # insights to someone looking at the logs to figure out why a branch isn't
    # being picked up.
    note("only protected branches are considered here")

    url = (
        f"https://api.github.com/repos/{ctx.repo}/branches?protected=true&per_page=100"
    )
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
                note(f"branch `{name}` doesn't seem to be a release branch")
            else:
                commits.append(branch["commit"]["sha"])

    return commits


def resolve_ref(ctx, ref):
    response = ctx.http.get(f"https://api.github.com/repos/{ctx.repo}/commits/{ref}")
    response.raise_for_status()
    return response.json()["sha"]


def commits_to_releases(ctx, all_commits):
    for commit in all_commits:
        try:
            metadata = build_metadata(ctx, commit)
        except UnsupportedMetadataVersion as e:
            note(f"skipping `{commit}` (unsupported metadata version {e.version})")
            if ctx.manual:
                error("manual mode early exits on unsupported metadata versions")
            continue
        yield PendingRelease(commit, metadata)


def filter_automated_channels(releases):
    rolling_releases = []
    for release in releases:
        if release.metadata.channel == "nightly":
            yield release
        elif release.metadata.channel == "pre-rolling":
            yield release
        elif release.metadata.channel.startswith("beta-"):
            yield release
        elif release.metadata.channel == "rolling":
            version = [int(num) for num in release.metadata.rust_version.split(".")]
            rolling_releases.append((version, release))
        else:
            note("channel {release.metadata.channel} cannot be released automatically")

    rolling_releases.sort(key=lambda vr: vr[0])
    # When starting from a squashed repo with no release branches yielding the
    # latest rolling release would crash the script.
    if rolling_releases:
        yield rolling_releases.pop()[1]
    for discarded in rolling_releases:
        note(
            f"version {discarded[1].metadata.rust_version} is not the latest "
            "in the rolling channel",
        )


# In some cases, it's possible that multiple pending releases target the same
# channel. This could happen for example if a new `release/1.NN` branch is
# created by an automation, and the PR bumping the channel away from nightly
# hasn't been merged yet. In that case, two releases with the same (nightly)
# channel will be attempted.
#
# Multiple pending releases with the same channel results in unpredictable
# behavior though. The release process would be instructed to release both of
# them, but the startup check preventing duplicate releases with the same ID
# would prevent ONE of them from being releases.
#
# This leads to unpredictability, since which commit ends up being released is
# not deterministic and only depends on which GitHub Actions job ran first.
# This could lead for example to a release channel going back in time.
#
# To prevent this from happening, the function discards all releases that have
# a duplicate channel. In the case above, no nightly release would be returned
# by the function, as two of them were pending.
def discard_duplicate_channels(releases):
    # Buffer the iterator as we need to iterate through it multiple times.
    releases = list(releases)

    channels_count = collections.defaultdict(lambda: 0)
    for release in releases:
        channels_count[release.metadata.channel] += 1

    for release in releases:
        if channels_count[release.metadata.channel] > 1:
            note(
                f"discarding {release.commit} on channel {release.metadata.channel} "
                "as multiple releases with that channel exist",
            )
        else:
            yield release


def discard_branches_with_no_changes(ctx, releases):
    # Assume the role on the AWS account containing the published releases,
    # otherwise we cannot determine which commit is already published.
    try:
        role = RELEASES_ACCESS_ROLES[ctx.env()]
    except KeyError:
        error(f"no AWS IAM Role configured to access environment {ctx.env()}")
    sts = boto3.client("sts")
    role = sts.assume_role(RoleArn=role, RoleSessionName="github-actions")
    releases_s3 = boto3.client(
        "s3",
        aws_access_key_id=role["Credentials"]["AccessKeyId"],
        aws_secret_access_key=role["Credentials"]["SecretAccessKey"],
        aws_session_token=role["Credentials"]["SessionToken"],
    )

    for release in releases:
        try:
            response = releases_s3.get_object(
                Bucket=f"ferrocene-{ctx.env()}-releases",
                Key=f"ferrocene/channels/{release.metadata.channel}.json",
            )
        except botocore.exceptions.ClientError as e:
            if e.response["Error"]["Code"] == "AccessDenied":
                # First release in the channel
                yield release
                continue
            else:
                raise e

        channel_metadata = json.loads(response["Body"].read())
        if channel_metadata["metadata_version"] == 2:
            last_commit = channel_metadata["latest_release"]["sha1_full"]
        else:
            note(
                "Channel API metadata version {channel_metadata['metadata_version']} "
                "for channel {metadata.release.channel} is not supported. "
                '"No changes" check will be skipped.'
            )
            last_commit = "Z"  # Will never match :)

        if release.commit != last_commit:
            yield release
        else:
            note(
                f"skipping channel {release.metadata.channel} "
                "as we would release the same commit"
            )


def prepare_github_actions_output(ctx, pending_releases):
    if ctx.manual:
        inputs = ctx.event_data["inputs"]

        environment = f"release-{ctx.env()}-manual"
        name_suffix = "manual"
        command_suffix = ""

        if inputs.get("override-existing") == "true":
            command_suffix += " --allow-overriding-existing-releases"
            name_suffix += ", allow overriding"
        if inputs.get("allow-duplicate") == "true":
            command_suffix += " --allow-duplicate-releases"
            name_suffix += ", allow duplicates"
    else:
        environment = f"release-{ctx.env()}-automated"
        name_suffix = "automated"
        command_suffix = ""

    iterator = discard_duplicate_channels(pending_releases)
    if not ctx.manual:
        iterator = discard_branches_with_no_changes(ctx, iterator)

    jobs = []
    for release in iterator:
        jobs.append(
            {
                "name": f"{release.metadata.channel} ({name_suffix})",
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

    if metadata["metadata_version"] in (3, 4):
        # For the purposes of this script, metadata versions 3 and 4 are
        # equivalent (the only difference is the tarball naming scheme).
        return BuildMetadata(
            rust_version=metadata["rust_version"],
            rust_channel=metadata["rust_channel"],
            ferrocene_channel=metadata["ferrocene_channel"],
            ferrocene_version=metadata["ferrocene_version"],
            channel=metadata["channel"],
        )
    else:
        raise UnsupportedMetadataVersion(metadata["metadata_version"])


def run():
    args = sys.argv[1:]
    if not args:  # CI mode
        with open(os.environ["GITHUB_EVENT_PATH"]) as f:
            event_data = json.load(f)
        event_name = os.environ["GITHUB_EVENT_NAME"]
        if event_name == "workflow_dispatch":
            manual = True
        elif event_name == "schedule":
            manual = False
        else:
            raise RuntimeError(f"unsupported event name: {ctx.event_name}")
        ctx = Context(
            repo=os.environ["GITHUB_REPOSITORY"],
            manual=manual,
            event_data=event_data,
        )
    elif args == ["local-test", "schedule"]:
        ctx = Context(repo=LOCAL_TEST_REPO, manual=False, event_data={})
    elif args[:2] == ["local-test", "dispatch"]:
        event = {"inputs": {}}
        for argument in args[2:]:
            try:
                key, value = argument.split("=")
            except ValueError:
                error(f"input must be key=value: {argument}")
            event["inputs"][key] = value
        ctx = Context(
            repo=LOCAL_TEST_REPO,
            manual=True,
            event_data=event,
        )
    else:
        error("invalid arguments")

    if ctx.manual:
        if ctx.event_data["inputs"].get("verbatim-ref") == "true":
            commit = ctx.event_data["inputs"]["ref"]
        else:
            commit = resolve_ref(ctx, ctx.event_data["inputs"]["ref"])
        releases = commits_to_releases(ctx, [commit])
    else:
        commits = commits_in_release_branches(ctx)
        releases = filter_automated_channels(commits_to_releases(ctx, commits))

    output = prepare_github_actions_output(ctx, releases)
    print(f"jobs={json.dumps(output)}")


def setup_http_client():
    http = requests.Session()
    http.headers["Authorization"] = f"token {os.environ['GITHUB_TOKEN']}"
    return http


def note(message):
    print(f"note: {message}", file=sys.stderr)


def error(message):
    print(f"error: {message}", file=sys.stderr)
    exit(1)


@dataclass
class BuildMetadata:
    rust_version: str
    rust_channel: str
    ferrocene_channel: str
    ferrocene_version: str
    channel: str


@dataclass
class PendingRelease:
    commit: str
    metadata: BuildMetadata


@dataclass
class Context:
    repo: str
    manual: bool
    event_data: dict

    s3: object = field(default_factory=lambda: boto3.client("s3"))
    http: object = field(default_factory=setup_http_client)

    def env(self):
        if self.manual:
            return self.event_data["inputs"]["env"]
        else:
            return "prod"


class UnsupportedMetadataVersion(Exception):
    def __init__(self, version):
        self.version = version


if __name__ == "__main__":
    run()
