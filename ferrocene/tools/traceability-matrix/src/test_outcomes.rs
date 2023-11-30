// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use anyhow::{Context, Error};
use std::collections::{HashSet, VecDeque};
use std::path::Path;

const COMPILETEST_TYPE: &str = "bootstrap::test::Compiletest";

const EXPECTED_FORMAT_VERSION: usize = 1;

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TestOutcomes {
    pub(crate) executed_tests: HashSet<String>,
    pub(crate) ignored_tests: HashSet<String>,
}

impl TestOutcomes {
    pub(crate) fn load(directory: &Path) -> Result<Self, Error> {
        let mut executed_tests = HashSet::new();
        let mut ignored_tests = HashSet::new();

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

            let mut search_queue = VecDeque::new();
            for invocation in &metrics.invocations {
                search_queue.extend(invocation.children.iter().map(|c| (c, false)));
            }
            while let Some((node, mut inside_compiletest)) = search_queue.pop_front() {
                match node {
                    MetricsNode::RustbuildStep { type_, children } => {
                        if type_ == COMPILETEST_TYPE {
                            inside_compiletest = true;
                        }
                        for child in children {
                            search_queue.push_back((child, inside_compiletest));
                        }
                    }
                    MetricsNode::TestSuite(TestSuite { tests }) => {
                        if !inside_compiletest {
                            continue;
                        }

                        for Test { name, outcome } in tests {
                            // Compiletest test names are in the "[suite] path/to/test.rs#revision"
                            // format, with the revision being optional.
                            let Some(name) = name.split_once("] ").map(|(_, n)| n) else { continue };
                            let name = name.rsplit_once('#').map(|(n, _)| n).unwrap_or(name);

                            if let MetricsTestOutcome::Ignored = outcome {
                                ignored_tests.insert(name.into());
                            } else {
                                executed_tests.insert(name.into());
                            }
                        }
                    }
                }
            }
        }

        Ok(TestOutcomes { executed_tests, ignored_tests })
    }
}

#[derive(serde::Deserialize)]
struct Metrics {
    #[serde(default)] // For version 0 the field was not present.
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
    RustbuildStep {
        #[serde(rename = "type")]
        type_: String,
        children: Vec<MetricsNode>,
    },
    TestSuite(TestSuite),
}

#[derive(serde::Deserialize)]
struct TestSuite {
    // metadata field is not deserialized
    tests: Vec<Test>,
}

#[derive(serde::Deserialize)]
struct Test {
    name: String,
    outcome: MetricsTestOutcome,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
enum MetricsTestOutcome {
    Passed,
    Failed,
    Ignored, // ignore reason is not deserialized
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::hashset;
    use serde_json::json;
    use tempfile::TempDir;

    #[test]
    fn test_load_outcomes() {
        let dir = TempDir::new().unwrap();

        std::fs::write(
            dir.path().join("runner1.json"),
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
                                                "tests": [
                                                    {
                                                        "name": "[ui] tests/ui/foo.rs",
                                                        "outcome": "passed",
                                                    },
                                                    {
                                                        "name": "[ui] tests/ui/bar.rs",
                                                        "outcome": "failed",
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
            .to_string(),
        )
        .unwrap();

        std::fs::write(
            dir.path().join("runner2.json"),
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
            .to_string(),
        )
        .unwrap();

        let outcomes = TestOutcomes::load(dir.path()).unwrap();
        assert_eq!(
            TestOutcomes {
                executed_tests: hashset![
                    "tests/ui/foo.rs".into(),
                    "tests/ui/bar.rs".into(),
                    "tests/run-make/foo.rs".into(),
                    "tests/codegen/foo.rs".into(),
                ],
                ignored_tests: hashset!["tests/ui/baz.rs".into()],
            },
            outcomes,
        )
    }
}
