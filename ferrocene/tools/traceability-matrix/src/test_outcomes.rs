// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::path::Path;

use anyhow::Context;

const EXPECTED_FORMAT_VERSION: usize = 1;

/// Answers which tests were executed and which were ignored
#[derive(Debug, PartialEq, Eq, Default, Clone)]
pub(crate) struct TestOutcomes {
    // key is name of test, represented by a path
    // value is targets on which the tests were executed
    pub(crate) executed_tests: BTreeMap<String, BTreeSet<String>>,
    // key is name of test, also represented by a path
    // value can be any number of ignored tests
    pub(crate) ignored_tests: BTreeMap<String, BTreeSet<String>>,
}

impl TestOutcomes {
    pub(crate) fn load(directory: &Path) -> anyhow::Result<Self> {
        let mut test_outcomes = TestOutcomes::default();

        for entry in std::fs::read_dir(directory)? {
            let path = entry?.path();
            if path.extension().and_then(|e| e.to_str()) != Some("json") {
                continue;
            }

            let metrics: Metrics = serde_json::from_slice(&std::fs::read(&path)?)
                .with_context(|| format!("failed to read metrics from {}", path.display()))?;
            if metrics.format_version != EXPECTED_FORMAT_VERSION {
                anyhow::bail!(
                    "build metrics format version {} is not supported",
                    metrics.format_version
                );
            }

            let mut search_queue = VecDeque::from_iter(
                metrics.invocations.into_iter().flat_map(|invocation| invocation.children),
            );
            while let Some(node) = search_queue.pop_front() {
                match node {
                    MetricsNode::RustbuildStep { children } => search_queue.extend(children),
                    MetricsNode::TestSuite {
                        tests,
                        metadata: TestSuiteMetadata::Compiletest { target },
                    } => {
                        for Test { name, outcome } in tests {
                            let Some(name) = parse_name(name) else { continue };
                            test_outcomes.insert(outcome, name, &target);
                        }
                    }
                    // Ignore test results from Cargo packages, as we don't consider those in the
                    // traceability matrix (yet?).
                    MetricsNode::TestSuite {
                        metadata: TestSuiteMetadata::CargoPackage, ..
                    } => {}
                }
            }
        }

        Ok(test_outcomes)
    }

    fn insert(&mut self, outcome: MetricsTestOutcome, name: String, target: &str) {
        let tests = match outcome {
            MetricsTestOutcome::Ignored => &mut self.ignored_tests,
            MetricsTestOutcome::Passed => &mut self.executed_tests,
        };
        tests.entry(name).or_default().insert(target.into());
    }
}

/// Compiletest test names are in the `[suite] path/to/test.rs#revision`
/// format, with the revision being optional.
fn parse_name(name: String) -> Option<String> {
    let (_, mut name) = name.split_once("] ")?;
    if let Some((name2, _)) = name.rsplit_once('#') {
        name = name2;
    }
    Some(name.into())
}

#[derive(serde::Deserialize)]
struct Metrics {
    #[serde(default)] // The field was not present in version 0
    format_version: usize,
    invocations: Vec<MetricsInvocation>,
}

#[derive(serde::Deserialize)]
struct MetricsInvocation {
    children: Vec<MetricsNode>,
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum MetricsNode {
    RustbuildStep { children: Vec<MetricsNode> },
    TestSuite { metadata: TestSuiteMetadata, tests: Vec<Test> },
}

#[derive(serde::Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
enum TestSuiteMetadata {
    CargoPackage,
    Compiletest { target: String },
}

#[derive(serde::Deserialize)]
struct Test {
    name: String,
    outcome: MetricsTestOutcome,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
// Failed is missing so as to trigger a deserialize failure,
// because we should not reach this far if any test fails.
enum MetricsTestOutcome {
    Passed,
    Ignored,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_load_outcomes() {
        let dir = TempDir::new().unwrap();

        write_to_dir(content_1(), &dir, "runner1");
        write_to_dir(content_2(), &dir, "runner2");

        let outcomes = TestOutcomes::load(dir.path()).unwrap();

        assert_eq!(
            TestOutcomes {
                executed_tests: BTreeMap::from([
                    (
                        "tests/ui/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/ui/bar.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/run-make/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                    (
                        "tests/codegen/foo.rs".into(),
                        BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                    ),
                ]),
                ignored_tests: BTreeMap::from([(
                    "tests/ui/baz.rs".into(),
                    BTreeSet::from(["aarch64-unknown-linux-gnu".into()])
                )]),
            },
            outcomes,
        )
    }

    fn write_to_dir(json: serde_json::Value, dir: &TempDir, file_name: &str) {
        // format for ease of debugging
        let content = serde_json::to_string_pretty(&json).unwrap();
        std::fs::write(dir.path().join(format!("{file_name}.json")), content).unwrap();
    }

    fn content_1() -> serde_json::Value {
        json!({
            "format_version": 1,
            "invocations": [
                {
                    "children": [
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::compile::Assemble",
                            "children": [],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Ui",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "compiletest",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[ui] tests/ui/foo.rs",
                                                    "outcome": "passed",
                                                },
                                                {
                                                    "name": "[ui] tests/ui/bar.rs",
                                                    "outcome": "passed",
                                                },
                                                {
                                                    "name": "[ui] tests/ui/baz.rs",
                                                    "outcome": "ignored",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::RunMake",
                            "children": [
                                {
                                    "kind": "rustbuild_step",
                                    "type": "bootstrap::test::Compiletest",
                                    "children": [
                                        {
                                            "kind": "test_suite",
                                            "metadata": {
                                                "target": "aarch64-unknown-linux-gnu",
                                                "kind": "compiletest",
                                            },
                                            "tests": [
                                                {
                                                    "name": "[run-make] tests/run-make/foo.rs#revision",
                                                    "outcome": "passed",
                                                },
                                            ],
                                        },
                                    ],
                                }
                            ],
                        },
                        {
                            "kind": "rustbuild_step",
                            "type": "bootstrap::test::Crate",
                            "children": [
                                {
                                    "kind": "test_suite",
                                    "metadata": {
                                        "target": "aarch64-unknown-linux-gnu",
                                        "kind": "compiletest",
                                    },
                                    "tests": [
                                        {
                                            "name": "rustc_foo::bar::tests::test_baz",
                                            "outcome": "passed",
                                        },
                                    ],
                                },
                            ],
                        },
                    ],
                },
            ],
        })
    }

    fn content_2() -> serde_json::Value {
        json!({
                "format_version": 1,
                "invocations": [
                    {
                        "children": [
                            {
                                "kind": "rustbuild_step",
                                "type": "bootstrap::test::Codegen",
                                "children": [
                                    {
                                        "kind": "rustbuild_step",
                                        "type": "bootstrap::test::Compiletest",
                                        "children": [
                                            {
                                                "kind": "test_suite",
                                                "metadata": {
                                                    "target": "aarch64-unknown-linux-gnu",
                                                    "kind": "compiletest",
                                                },
                                                "tests": [
                                                    {
                                                        "name": "[codegen] tests/codegen/foo.rs",
                                                        "outcome": "passed",
                                                    },
                                                ],
                                            },
                                        ],
                                    }
                                ],
                            },
                        ],
                    },
                    {
                        "children": [
                            {
                                "kind": "rustbuild_step",
                                "type": "bootstrap::test::Codegen",
                                "children": [
                                    {
                                        "kind": "rustbuild_step",
                                        "type": "bootstrap::test::Compiletest",
                                        "children": [
                                            {
                                                "kind": "test_suite",
                                                "metadata": {
                                                    "target": "aarch64-unknown-linux-gnu",
                                                    "kind": "compiletest",
                                                },
                                                "tests": [
                                                    {
                                                        "name": "[codegen] tests/codegen/foo.rs",
                                                        "outcome": "passed",
                                                    },
                                                ],
                                            },
                                        ],
                                    }
                                ],
                            },
                        ],
                    },
                ],
        })
    }
}
