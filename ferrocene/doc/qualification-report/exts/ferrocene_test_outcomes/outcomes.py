# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from collections import OrderedDict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Optional, List, Union
import json
import os
import re


TEST_EXECUTED = object()
TEST_IGNORED_NO_REASON = object()

SUPPORTED_FORMAT_VERSION = 1

_DOCTEST_RE = re.compile(r"^[a-zA-Z0-9\-_\.\/]+ - [a-zA-Z0-9_:'<>,]+ \(line [0-9]+\)$")


@dataclass(order=True)
class IgnoredTest:
    name: str
    reason: Optional[str]

    def is_doc_test(self):
        return _DOCTEST_RE.search(self.name) is not None


@dataclass(order=True)
class CompiletestInvocation:
    suite: str
    mode: Optional[str]


@dataclass(order=True)
class CargoPackageInvocation:
    crates: list[str]


@dataclass(order=True)
class Invocation:
    bootstrap_types: list[str]
    host: str
    target: str
    stage: int
    kind: Union[CompiletestInvocation, CargoPackageInvocation]

    passed_tests: int = 0
    failed_tests: int = 0
    ignored_tests: int = 0

    def total_tests(self):
        return self.passed_tests + self.ignored_tests + self.failed_tests

    def is_compiletest(self):
        return isinstance(self.kind, CompiletestInvocation)

    def is_cargo_package(self):
        return isinstance(self.kind, CargoPackageInvocation)


@dataclass
class Platform:
    invocations: list[Invocation] = field(default_factory=list)
    ignored_tests: list[IgnoredTest] = field(default_factory=list)
    ignored_doc_tests: list[IgnoredTest] = field(default_factory=list)

    def filter_invocations(self, bootstrap_type, *, only_match_root_node=False):
        for invocation in self.invocations:
            if only_match_root_node:
                if invocation.bootstrap_types[0] == bootstrap_type:
                    yield invocation
            elif bootstrap_type in invocation.bootstrap_types:
                yield invocation


@dataclass
class Outcomes:
    platforms: OrderedDict[(str, str), Platform] = field(default_factory=OrderedDict)

    def load_file(self, file):
        with open(file) as f:
            contents = json.load(f)

        if "format_version" not in contents:
            raise RuntimeError(f"test outcomes {file} does not have a format_version")
        elif contents["format_version"] != SUPPORTED_FORMAT_VERSION:
            raise RuntimeError(
                f"test outcomes {file} "
                f"use format version {contents['format_version']} "
                f"while only {SUPPORTED_FORMAT_VERSION} is supported",
            )

        loader = FileLoader(self)
        for invocation in contents["invocations"]:
            loader.load_invocation(invocation)

    def sort(self):
        for platform in self.platforms.values():
            platform.ignored_tests.sort()
            platform.ignored_doc_tests.sort()
            platform.invocations.sort()

    def platform(self, host, target):
        key = (host, target)
        if key not in self.platforms:
            self.platforms[key] = Platform()
        return self.platforms[key]


class FileLoader:
    def __init__(self, outcomes):
        self.outcomes = outcomes
        self.bootstrap_type_stack = []

    def load_invocation(self, invocation):
        for step in invocation["children"]:
            self.load_step(step)

    def load_step(self, step):
        if step["kind"] == "rustbuild_step":
            self.bootstrap_type_stack.append(step["type"])
            for child in step["children"]:
                self.load_step(child)
            self.bootstrap_type_stack.pop()
        elif step["kind"] == "test_suite":
            self.load_test_suite(step)
        else:
            raise RuntimeError(f"unknown step kind: {step['kind']}")

    def load_test_suite(self, suite):
        metadata = suite["metadata"]
        platform = self.outcomes.platform(metadata["host"], metadata["target"])

        if metadata["kind"] == "compiletest":
            mode = metadata["mode"] if metadata["mode"] != metadata["suite"] else None
            kind = CompiletestInvocation(suite=metadata["suite"], mode=mode)
        elif metadata["kind"] == "cargo_package":
            kind = CargoPackageInvocation(crates=metadata["crates"])
        else:
            raise RuntimeError(f"unknown test suite kind: {metadata['kind']}")

        invocation = Invocation(
            bootstrap_types=list(self.bootstrap_type_stack),
            host=metadata["host"],
            target=metadata["target"],
            stage=metadata["stage"],
            kind=kind,
        )

        for test in suite["tests"]:
            if test["outcome"] == "ignored":
                invocation.ignored_tests += 1
                ignored = IgnoredTest(name=test["name"], reason=test["ignore_reason"])
                if ignored.is_doc_test():
                    platform.ignored_doc_tests.append(ignored)
                else:
                    platform.ignored_tests.append(ignored)
            elif test["outcome"] == "passed":
                invocation.passed_tests += 1
            elif test["outcome"] == "failed":
                invocation.failed_tests += 1
            else:
                raise RuntimeError(f"unknown test outcome: {test['outcome']}")

        if invocation.total_tests():
            platform.invocations.append(invocation)


def builder_inited(app):
    # Cache the test outcomes at the start of the build
    app.env.ferrocene_test_outcomes = _load_outcomes()


def _load_outcomes():
    try:
        directory = Path(os.environ["FERROCENE_TEST_OUTCOMES_DIR"])
    except KeyError:
        return None

    # Sort the file names before loading, to prevent nondeterminism.
    files_to_load = []
    for entry in directory.iterdir():
        if entry.is_file() and entry.suffix == ".json":
            files_to_load.append(entry)
    files_to_load.sort()

    outcomes = Outcomes()
    for file_to_load in files_to_load:
        outcomes.load_file(file_to_load)
    outcomes.sort()

    return outcomes


def setup(app):
    app.connect("builder-inited", builder_inited)
