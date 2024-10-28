#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Helper script to decide which tasks should be executed by each job.
#
# We're using a script instead of manually passing job names and --exclude
# flags to x.py to avoid accidentally skipping tests. Otherwise we might pass
# an --exclude without actually executing the test in another job.


from typing import Iterable, TypeAlias
import itertools
import shlex
import sys
import pathlib


REPOSITORY_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent


def find_all_compiletests(exclude: list[str] | None, deprioritize: list[str] | None):
    if exclude is None:
        exclude = []
    if deprioritize is None:
        deprioritize = []

    found = []
    for entry in (REPOSITORY_ROOT / "tests").iterdir():
        if not entry.is_dir():
            continue
        elif entry.name in exclude:
            continue

        weight = 1 if entry.name in deprioritize else 0

        found.append(Task(entry.relative_to(REPOSITORY_ROOT).as_posix(), weight))
    return found


class Task:
    def __init__(self, path: str, weight=0):
        self.path = path
        self.weight = weight

    def __lt__(self, other):
        if self.weight == other.weight:
            return self.path < other.path
        return self.weight < other.weight


# TODO: this should be `type Job = list[str]` etc., but this requires CI to update
# to python 3.12 or above
Job: TypeAlias = list[str]
Kind: TypeAlias = dict[str, Job]
JobsDefinition: TypeAlias = dict[str, Kind]

# If you are trying to exclude a step from being executed at all in our CI,
# avoid changing the script to pass --exclude for it. Rather, change the
# bootstrap source code to prevent the step from being invoked at all.
#
# This way, customers building from source will also avoid executing the step,
# even if they don't rely on this script.
#
# See this commit for an example of how to do it:
#
#    https://github.com/ferrocene/ferrocene/commit/feb061293a968b0cd7122cb9e00a2409be3e3a39
#

# fmt: off
JOBS_DEFINITION: JobsDefinition = {
    "dist": {
        # Build the documentation on a different jobs, since building it takes
        # a while and with a separate job we can run dist inside the same job
        # as linkchecker (which also needs to generate docs).
        "docs": ["ferrocene-docs", "ferrocene-test-outcomes", "ferrocene-docs-doctrees"],

        # Build the source code tarball on a different job, since that requires
        # a (slower) clone of the whole LLVM submodule, not just the subset,
        # which would greatly increase the time of existing jobs.
        "src": ["rust-src", "ferrocene-src"],

        "oxidos": ["ferrocene-oxidos"],

        "tools": ["rust-analyzer", "clippy", "rustfmt", "flip-link", "probe-rs"],
    },

    "test": {
        # Tests to execute in the initial "misc-checks" job. These tests should
        # be fairly quick to complete, and they must not require the compiler
        # to be built.
        "misc-checks": ["src/tools/tidy", "src/bootstrap"],

        "docs": [
            # Linkchecker requires the documentation, which takes a while to build.
            # The separate job will also allow to run dist with the same artifacts.
            "src/tools/linkchecker",
            # Signature verification requires building the documentation that
            # needs to be verified. To avoid wasting resources, run it in the
            # docs builder along with the rest of the steps that build docs.
            "ferrocene-check-document-signatures",
        ],

        # Compiletests require custom tooling to be built (compiletest itself),
        # take a while to execute and are highly parallelizable, so running
        # them separately allows for increased concurrency and reduces the CI
        # wall clock time.
        "compiletest": find_all_compiletests(
            exclude=[
                # `auxiliary` is not a compiletest suite.
                "auxiliary",
                # `rustdoc-gui` requires nodejs, npm and chromium to be installed
                # in the container, which is currently not the case.
                "rustdoc-gui",
            ],
            deprioritize=[
                # This test suite interferes with the others (notably the UI
                # suite), so run it last to avoid side effects.
                #
                # See https://github.com/rust-lang/rust/issues/92644
                "run-make-fulldeps",
            ],
        ),

        # Library tests are the second slowest part of a test run, so we run
        # them in a separate job to reduce the CI wall clock time. Note that
        # stdlib tests are run in a separate job, as those require IPv6 and
        # thus can't be executed in containers due to CircleCI limitations.
        "library": ["library/core", "library/alloc", "library/test"],

        # The standard library tests require IPv6, which is not available in
        # containers. Run them separately in a VM.
        "library-std": ["library/std"],
    },

    "qnx": {
        # like 'test:compiletest' minus the ONLY_HOSTS test groups
        # the ONLY_HOSTS tests will run on the host even when
        # `--target $NOT_THE_HOST` is passed to `x.py test`
        # this avoids re-running tests on the same host triple across
        # different CI jobs
        #
        # this is in a separate category and not under `test:` because that
        # would make repeated arguments appear in the output of
        # `split-tasks.py test`
        "compiletest-no-only-hosts": find_all_compiletests(
            exclude=[
                # see `test:compiletest`
                "auxiliary",
                "rustdoc-gui",

                # ONLY_HOST tests
                "coverage-run-rustdoc",
                "pretty",
                "rustdoc",
                "rustdoc-js",
                "rustdoc-js-std",
                "rustdoc-json",
                "rustdoc-ui",
                "ui-fulldeps",
            ],
            deprioritize=[
                # see `test:compiletest`
                "run-make-fulldeps",
            ],
        ),
    },
}
# fmt: on

def sorted_tasks(tasks: Iterable[str | Task]) -> list[Task]:
    tasks = list(tasks)

    # Convert everything to Task first
    for i in range(len(tasks)):
        if type(tasks[i]) == str:
            tasks[i] = Task(tasks[i])

    return sorted(tasks)


def get_flags_for_job(config: JobsDefinition, kind: str, job: str):
    try:
        tasks = config[kind][job]
    except KeyError:
        err(f"job '{kind}:{job}' not supported")

    return [shlex.quote(task.path) for task in sorted_tasks(tasks)]


def get_flags_for_default_job(config: JobsDefinition, kind: str):
    try:
        jobs = config[kind].values()
    except KeyError:
        err(f"job '{kind}' not supported")

    tasks = set(itertools.chain.from_iterable(jobs))
    return [shlex.quote(f"--exclude={task.path}") for task in sorted_tasks(tasks)]


def cli():
    args = sys.argv[1:]
    if len(args) != 1:
        err(f"usage: {sys.argv[0]} <job>")
    split = args[0].split(":", 1)

    flags = []
    if len(split) == 2:
        flags += get_flags_for_job(JOBS_DEFINITION, split[0], split[1])
    else:
        flags += get_flags_for_default_job(JOBS_DEFINITION, split[0])

    flag_arg = " ".join(flags)
    print(f"Split tasks: {flag_arg}", file=sys.stderr)
    print(flag_arg)


def err(message):
    print("error:", message, file=sys.stderr)
    exit(1)


if __name__ == "__main__":
    cli()
