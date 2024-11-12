// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use crate::test_outcomes::TestOutcomes;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AnnotatedFile {
    pub(crate) test: PathBuf,
    pub(crate) source: AnnotationSource,
    pub(crate) targets: Targets,
}

impl std::fmt::Display for AnnotatedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            AnnotationSource::TestItself => write!(f, "{}", self.test.display()),
            AnnotationSource::Makefile => write!(f, "{} (from its Makefile)", self.test.display()),
            AnnotationSource::Rmake => write!(f, "{} (from its rmake.rs)", self.test.display()),
            AnnotationSource::ParentDirectory { .. } => {
                write!(f, "{} (from its parent directory)", self.test.display())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct Targets {
    pub(crate) executed: DisplayCommaSeparatedSet,
    pub(crate) ignored: DisplayCommaSeparatedSet,
}

// created only so as to impl Display for Targets fields
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub(crate) struct DisplayCommaSeparatedSet(pub BTreeSet<String>);

impl std::fmt::Display for DisplayCommaSeparatedSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let targets: Vec<_> = self.0.clone().into_iter().collect();
        write!(f, "{}", targets.as_slice().join(", "))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AnnotationSource {
    TestItself,
    ParentDirectory { bulk_file: PathBuf },
    Makefile,
    Rmake,
}

#[derive(Debug)]
pub(crate) struct Annotations {
    pub(crate) ids: BTreeMap<String, BTreeSet<AnnotatedFile>>,
    pub(crate) ignored_tests: BTreeMap<String, BTreeSet<String>>,
    pub(crate) considers_ignored_tests: bool,
}

impl Annotations {
    pub(crate) fn new() -> Self {
        Annotations {
            ids: BTreeMap::new(),
            ignored_tests: BTreeMap::new(),
            considers_ignored_tests: true,
        }
    }

    pub(crate) fn load_directory(
        &mut self,
        dir: &Path,
        src_base: &Path,
        test_outcomes: Option<&TestOutcomes>,
    ) -> anyhow::Result<()> {
        // Mark the annotations as not considering ignored tests as soon as test
        // outcomes are not provided once, even if they were provided before.
        if test_outcomes.is_none() {
            self.considers_ignored_tests = false;
        }

        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            if !path.is_file() {
                continue;
            }
            if !path.extension().map(|e| e.to_str() == Some("json")).unwrap_or(false) {
                continue;
            }
            self.load_file(&path, src_base, test_outcomes)?;
        }
        Ok(())
    }

    fn load_file(
        &mut self,
        file: &Path,
        src_base: &Path,
        test_outcomes: Option<&TestOutcomes>,
    ) -> anyhow::Result<()> {
        #[derive(serde::Deserialize)]
        struct JsonOutput {
            bulk_annotations_file_name: String,
            tests: Vec<JsonTestFile>,
        }

        #[derive(serde::Deserialize)]
        struct JsonTestFile {
            file: PathBuf,
            annotations: Vec<JsonAnnotation>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct JsonAnnotation {
            id: String,
            file: PathBuf,
        }

        let output: JsonOutput = serde_json::from_slice(&std::fs::read(file)?)?;

        let shrink_path = |path: &Path| path.strip_prefix(src_base).unwrap_or(path).to_path_buf();
        let annotated_in_parent = |annotation: &JsonAnnotation, file: &JsonTestFile| {
            annotation.file.parent() == file.file.parent()
                && annotation.file.file_name().and_then(|f| f.to_str())
                    == Some(&output.bulk_annotations_file_name)
        };

        let mut unknown_tests = BTreeSet::new();
        for file in &output.tests {
            let relative_file = shrink_path(&file.file)
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("non-utf8 path: {}", file.file.display()))?
                .to_string();
            let tmp_binding;
            let outcomes = if let Some(outcomes) = test_outcomes {
                // Do not consider annotations from ignored tests.
                if !outcomes.executed_tests.contains_key(&relative_file) {
                    if file.annotations.is_empty() {
                        unreachable!("source files should already be annotated")
                    }
                    if let Some(tests) = outcomes.ignored_tests.get(&relative_file) {
                        self.ignored_tests.insert(relative_file.clone(), tests.clone());
                    } else {
                        unknown_tests.insert(relative_file);
                    }
                    continue;
                }
                outcomes
            } else {
                tmp_binding = TestOutcomes::default();
                &tmp_binding
            };

            for annotation in &file.annotations {
                let source = if annotation.file == file.file {
                    AnnotationSource::TestItself
                } else if annotated_in_parent(annotation, file) {
                    AnnotationSource::ParentDirectory { bulk_file: shrink_path(&annotation.file) }
                } else if annotation.file == file.file.join("Makefile") {
                    AnnotationSource::Makefile
                } else if annotation.file == file.file.join("rmake.rs") {
                    AnnotationSource::Rmake
                } else {
                    anyhow::bail!(
                        "bug: annotation {annotation:?} doesn't come from the file itself, \
                        its parent directory, or a Makefile. \
                        If you updated compiletest to accept annotations from other sources, \
                        you also need to update traceability-matrix."
                    );
                };

                let ignored = DisplayCommaSeparatedSet(
                    outcomes
                        .ignored_tests
                        .get(&relative_file)
                        .unwrap_or(&BTreeSet::new())
                        .to_owned(),
                );
                let executed = DisplayCommaSeparatedSet(
                    outcomes
                        .executed_tests
                        .get(&relative_file)
                        .unwrap_or(&BTreeSet::new())
                        .to_owned(),
                );
                let targets = Targets { executed, ignored };
                let annotated_file =
                    AnnotatedFile { test: shrink_path(&file.file), source, targets };
                self.ids.entry(annotation.id.clone()).or_default().insert(annotated_file);
            }
        }
        if !unknown_tests.is_empty() {
            eprintln!(
                "The following tests have no entries in test_outcomes, \
                likely due to test_outcomes being stale:"
            );
            for test in unknown_tests {
                eprintln!("  - {test}");
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_load_file() -> anyhow::Result<()> {
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), annotations_file_1()?)?;

        let outcomes = Some(TestOutcomes {
            executed_tests: BTreeMap::from([
                ("example/foo.rs".into(), BTreeSet::default()),
                ("example/bar.rs".into(), BTreeSet::default()),
                ("example/foobar.rs".into(), BTreeSet::default()),
            ]),
            ignored_tests: BTreeMap::from([
                ("example/ignored.rs".into(), BTreeSet::default()),
                ("example/ignored-without-annotations.rs".into(), BTreeSet::default()),
            ]),
        });

        let mut annotations = Annotations::new();
        annotations.load_file(file.path(), Path::new("/base"), outcomes.as_ref())?;

        assert_eq!(
            BTreeMap::from([
                ("foo".into(), BTreeSet::from([test_itself("example/foo.rs")])),
                (
                    "bar".into(),
                    BTreeSet::from([test_itself("example/foo.rs"), test_itself("example/bar.rs"),])
                ),
                (
                    "foobar".into(),
                    BTreeSet::from([AnnotatedFile {
                        test: "example/foobar.rs".into(),
                        source: AnnotationSource::ParentDirectory {
                            bulk_file: "example/ferrocene-annotations".into(),
                        },
                        targets: Default::default(),
                    }])
                ),
            ]),
            annotations.ids,
        );
        let expected = BTreeMap::from([("example/ignored.rs".into(), BTreeSet::default())]);
        assert_eq!(expected, annotations.ignored_tests);

        Ok(())
    }

    #[test]
    fn test_load_directory() -> anyhow::Result<()> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("foo.json"), annotations_file_1()?)?;
        std::fs::write(dir.path().join("bar.json"), annotations_file_2()?)?;
        // Should not be loaded, as it's not a JSON file:
        std::fs::write(dir.path().join("baz.txt"), b"not JSON\n")?;
        // Should not be loaded, as it's in a subdirectory:
        std::fs::create_dir(dir.path().join("sub"))?;
        std::fs::write(dir.path().join("sub/quux.json"), annotations_file_3()?)?;

        let outcomes = Some(TestOutcomes {
            executed_tests: BTreeMap::from([
                ("example/foo.rs".into(), BTreeSet::default()),
                ("example/bar.rs".into(), BTreeSet::default()),
                ("example/baz.rs".into(), BTreeSet::default()),
                ("example/foobar.rs".into(), BTreeSet::default()),
            ]),
            ignored_tests: BTreeMap::from([
                ("example/ignored.rs".into(), BTreeSet::default()),
                ("example/ignored-without-annotations.rs".into(), BTreeSet::default()),
            ]),
        });

        let mut annotations = Annotations::new();
        annotations.load_directory(dir.path(), Path::new("/base"), outcomes.as_ref())?;

        assert_eq!(
            BTreeMap::from([
                ("foo".into(), BTreeSet::from([test_itself("example/foo.rs")])),
                (
                    "bar".into(),
                    BTreeSet::from([
                        test_itself("example/foo.rs"),
                        test_itself("example/bar.rs"),
                        test_itself("example/baz.rs"),
                    ])
                ),
                ("baz".into(), BTreeSet::from([test_itself("example/baz.rs")])),
                (
                    "foobar".into(),
                    BTreeSet::from([AnnotatedFile {
                        test: "example/foobar.rs".into(),
                        source: AnnotationSource::ParentDirectory {
                            bulk_file: "example/ferrocene-annotations".into(),
                        },
                        targets: Default::default(),
                    }])
                ),
                // quux is not loaded as it's in a subdirectory
            ]),
            annotations.ids,
        );
        let expected = BTreeMap::from([("example/ignored.rs".into(), BTreeSet::default())]);
        assert_eq!(expected, annotations.ignored_tests);

        Ok(())
    }

    fn test_itself(path: impl AsRef<Path>) -> AnnotatedFile {
        AnnotatedFile {
            test: path.as_ref().into(),
            source: AnnotationSource::TestItself,
            targets: Default::default(),
        }
    }

    fn annotations_file_1() -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/foo.rs",
                    "annotations": [
                        {
                            "id": "foo",
                            "file": "/base/example/foo.rs",
                        },
                        {
                            "id": "bar",
                            "file": "/base/example/foo.rs",
                        },
                    ],
                },
                {
                    "file": "/base/example/bar.rs",
                    "annotations": [
                        {
                            "id": "bar",
                            "file": "/base/example/bar.rs",
                        },
                    ],
                },
                {
                    "file": "/base/example/foobar.rs",
                    "annotations": [
                        {
                            "id": "foobar",
                            "file": "/base/example/ferrocene-annotations",
                        },
                    ],
                },
                {
                    "file": "/base/example/ignored.rs",
                    "annotations": [
                        {
                            "id": "ignored",
                            "file": "/base/example/ignored.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))
    }

    fn annotations_file_2() -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/baz.rs",
                    "annotations": [
                        {
                            "id": "baz",
                            "file": "/base/example/baz.rs",
                        },
                        {
                            "id": "bar",
                            "file": "/base/example/baz.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))
    }

    fn annotations_file_3() -> serde_json::Result<Vec<u8>> {
        serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/quux.rs",
                    "annotations": [
                        {
                            "id": "quux",
                            "file": "/base/example/quux.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))
    }
}
