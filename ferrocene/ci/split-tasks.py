#!/usr/bin/env python3
# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

# Helper script to decide which tasks should be executed by each job.
#
# We're using a script instead of manually passing job names and --exclude
# flags to x.py to avoid accidentally skipping tests. Otherwise we might pass
# an --exclude without actually executing the test in another job.


import itertools
import shlex
import sys
import pathlib


REPOSITORY_ROOT = pathlib.Path(__file__).resolve().parent.parent.parent


def find_all_compiletests(*, exclude=None, deprioritize=None):
    if exclude is None:
        exclude = []
    if deprioritize is None:
        prioritize = []

    found = []
    for entry in (REPOSITORY_ROOT / "tests").iterdir():
        if not entry.is_dir():
            continue
        if entry.name in exclude:
            continue

        weight = 0
        if entry.name in deprioritize:
            weight = 1

        found.append(Task(str(entry.relative_to(REPOSITORY_ROOT)), weight))
    return found


class Task:
    def __init__(self, path, weight=0):
        self.path = path
        self.weight = weight

    def __lt__(self, other):
        if self.weight == other.weight:
            return self.path < other.path
        return self.weight < other.weight


JOBS_DEFINITION = {
    "dist": {
        # Build the documentation on a different jobs, since building it takes
        # a while and with a separate job we can run dist inside the same job
        # as linkchecker (which also needs to generate docs).
        "docs": ["ferrocene-docs", "ferrocene-test-outcomes"],

        # Build the source code tarball on a different job, since that requires
        # a (slower) clone of the whole LLVM submodule, not just the subset,
        # which would greatly increase the time of existing jobs.
        "src": ["rust-src", "ferrocene-src"],

        "oxidos": ["ferrocene-oxidos"],

        "tools": ["rust-analyzer", "clippy", "rustfmt"],

        # The "None" job contains the tasks that should never be executed,
        # regardless of which job is requested.
        None: [
            # Combined tarballs: we can't produce those, since different
            # customers will only be allowed to access a subset of Ferrocene
            # components (for example due to proprietary targets).
            "extended",
            # Upstream's plain source tarball is replaced by the "ferrocene-src"
            # step, so we avoid executing it.
            "rustc-src",
            # Upstream's documentation tarball is replaced by the
            # "ferrocene-docs" step, so we avoid executing it.
            "rust-docs",
            "rustc-docs",
        ],
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
            ]
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
}

GLOBAL_EXCLUDE = [
    # The "standalone" documentation includes upstream's index page, which
    # often overrides our own custom index page. It also includes all the
    # redirects used by upstream that we don't need. Avoid building it.
    "doc::standalone",
]


def sorted_tasks(tasks):
    tasks = list(tasks)

    # Convert everything to Task first
    for i in range(len(tasks)):
        if type(tasks[i]) == str:
            tasks[i] = Task(tasks[i])

    return sorted(tasks)


def get_flags_for_job(config, kind, job):
    try:
        tasks = config[kind][job]
    except KeyError:
        err(f"job '{kind}:{job}' not supported")

    return [shlex.quote(task.path) for task in sorted_tasks(tasks)]


def get_flags_for_default_job(config, kind):
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

    flags += [shlex.quote(f"--exclude={path}") for path in GLOBAL_EXCLUDE]

    print(" ".join(flags))


def err(message):
    print("error:", message, file=sys.stderr)
    exit(1)


if __name__ == "__main__":
    cli()
