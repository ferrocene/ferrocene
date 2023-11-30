# SPDX-License-Identifier: MIT OR Apache-2.0
# SPDX-FileCopyrightText: The Ferrocene Developers

from .parse_debug_repr import DebugReprParser
from collections import OrderedDict
from dataclasses import dataclass, field
from pathlib import Path
from typing import Dict, Optional, List, Union
import json
import os


TEST_EXECUTED = object()
TEST_IGNORED_NO_REASON = object()

SUPPORTED_FORMAT_VERSION = 1


class Outcomes:
    def __init__(self):
        self.suites = OrderedDict()

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

        for invocation in contents["invocations"]:
            for step in invocation["children"]:
                name = step["type"]
                try:
                    suite = self.suites[name]
                except KeyError:
                    suite = Suite(name=name)

                collector = InvocationCollector(suite, step["debug_repr"])
                collector.collect(step)

                if collector.invocations:
                    suite.invocations += collector.invocations
                    self.suites[name] = suite

    def finalize(self):
        for suite in self.suites.values():
            for name in list(suite.ignored_tests):
                if suite.ignored_tests[name] is TEST_EXECUTED:
                    del suite.ignored_tests[name]

        self.suites = OrderedDict(sorted(self.suites.items(), key=lambda kv: kv[0]))
        for suite in self.suites.values():
            suite.invocations.sort(key=lambda invocation: invocation.debug_repr)
            suite.ignored_tests = OrderedDict(
                sorted(suite.ignored_tests.items(), key=lambda kv: kv[0])
            )


class InvocationCollector:
    def __init__(self, suite, suite_debug_repr):
        self._suite = suite
        self._suite_debug_repr = suite_debug_repr
        self._bootstrap_types = []
        self.invocations = []

    def collect(self, step):
        pop_bootstrap_type = False
        if step["kind"] == "rustbuild_step":
            self._bootstrap_types.append(step["type"])
            pop_bootstrap_type = True

        invocation = Invocation(
            bootstrap_types=list(self._bootstrap_types),
            debug_repr=self._suite_debug_repr,
            parsed_debug_repr=DebugReprParser(self._suite_debug_repr).parse_item()
        )
        for child in step["children"]:
            if child["kind"] == "rustbuild_step":
                self.collect(child)
            elif child["kind"] == "test_suite":
                for test in child["tests"]:
                    outcome = test["outcome"]
                    if outcome == "ignored":
                        invocation.ignored_tests += 1
                        if test["name"] not in self._suite.ignored_tests:
                            reason = test["ignore_reason"]
                            if reason is None:
                                reason = TEST_IGNORED_NO_REASON
                            self._suite.ignored_tests[test["name"]] = reason
                    elif outcome == "failed":
                        invocation.failed_tests += 1
                        self._suite.ignored_tests[test["name"]] = TEST_EXECUTED
                    elif outcome == "passed":
                        invocation.passed_tests += 1
                        self._suite.ignored_tests[test["name"]] = TEST_EXECUTED
                    else:
                        raise RuntimeError(f"unexpected outcome: {outcome}")

        if invocation.total_tests() > 0:
            self.invocations.append(invocation)

        if pop_bootstrap_type:
            self._bootstrap_types.pop()


@dataclass
class Invocation:
    bootstrap_types: List[str]
    debug_repr: str
    parsed_debug_repr: object

    passed_tests: int = 0
    failed_tests: int = 0
    ignored_tests: int = 0

    def total_tests(self):
        return self.passed_tests + self.ignored_tests + self.failed_tests


@dataclass
class Suite:
    name: str
    invocations: List[Invocation] = field(default_factory=list)
    # Store the list of ignored tests. Unfortunately, to properly process the
    # data from multiple files, until the finalize method is called on the
    # Outcomes class there will be multiple possible values for the items of
    # this dictionary:
    #
    # - If the entry is missing, the test was not already executed. It can
    #   either be set to TEST_EXECUTED if the test was executed,
    #   TEST_IGNORED_NO_REASON if the test was ignored without a reason, or the
    #   ignore reason if the test was ignored with a reason.
    #
    # - If the entry is TEST_EXECUTED, the test was executed at least once. It
    #   can never be replaced by other states.
    #
    # - If the entry is a string or TEST_IGNORED_NO_REASON, the test was always
    #   ignored, and the string the reason why the test was ignored. It can be
    #   replaced by TEST_EXECUTED if another execution of the test did run the
    #   test. That might happen for architecture-specific tests for example.
    #
    # I miss Rust enums :( -pietro
    ignored_tests: OrderedDict[str, Union[str, object]] = field(
        default_factory=OrderedDict
    )


def builder_inited(app):
    # Cache the test outcomes at the start of the build
    app.env.ferrocene_test_outcomes = _load_outcomes()


def _load_outcomes():
    try:
        directory = Path(os.environ["FERROCENE_TEST_OUTCOMES_DIR"])
    except KeyError:
        return None

    files_to_load = []
    for entry in directory.iterdir():
        if entry.is_file() and entry.suffix == ".json":
            files_to_load.append(entry)
    files_to_load.sort()

    # HACK: when we signed the qualification report, we didn't sort the list
    # of metrics files we ingested. This caused some nondeterminism, breaking
    # signatures. To ensure we are using the same order as when the signatures
    # were generated, we reverse the list of files.
    files_to_load.reverse()

    outcomes = Outcomes()
    for file_to_load in files_to_load:
        outcomes.load_file(file_to_load)
    outcomes.finalize()

    return outcomes


def setup(app):
    app.connect("builder-inited", builder_inited)
