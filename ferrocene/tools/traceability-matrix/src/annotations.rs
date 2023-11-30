// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::test_outcomes::TestOutcomes;
use anyhow::Error;
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) struct AnnotatedFile {
    pub(crate) test: PathBuf,
    pub(crate) source: AnnotationSource,
}

impl std::fmt::Display for AnnotatedFile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.source {
            AnnotationSource::TestItself => write!(f, "{}", self.test.display()),
            AnnotationSource::Makefile => write!(f, "{} (from its Makefile)", self.test.display()),
            AnnotationSource::ParentDirectory { .. } => {
                write!(f, "{} (from its parent directory)", self.test.display())
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(crate) enum AnnotationSource {
    TestItself,
    ParentDirectory { bulk_file: PathBuf },
    Makefile,
}

#[derive(Debug)]
pub(crate) struct Annotations {
    pub(crate) paragraphs: HashMap<String, HashSet<AnnotatedFile>>,
    pub(crate) ignored_tests: HashSet<PathBuf>,
    pub(crate) considers_ignored_tests: bool,
}

impl Annotations {
    pub(crate) fn new() -> Self {
        Annotations {
            paragraphs: HashMap::new(),
            ignored_tests: HashSet::new(),
            considers_ignored_tests: true,
        }
    }

    pub(crate) fn load_directory(
        &mut self,
        dir: &Path,
        src_base: &Path,
        test_outcomes: Option<&TestOutcomes>,
    ) -> Result<(), Error> {
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

    pub(crate) fn load_file(
        &mut self,
        file: &Path,
        src_base: &Path,
        test_outcomes: Option<&TestOutcomes>,
    ) -> Result<(), Error> {
        #[derive(serde::Deserialize)]
        struct JsonOutput {
            bulk_annotations_file_name: String,
            tests: Vec<JsonTestFile>,
        }

        #[derive(serde::Deserialize)]
        struct JsonTestFile {
            // TODO: sha256: HashMap<String, String>,
            file: PathBuf,
            annotations: Vec<JsonAnnotation>,
        }

        #[derive(Debug, serde::Deserialize)]
        struct JsonAnnotation {
            annotation: String,
            file: PathBuf,
        }

        // Mark the annotations as not considering ignored tests as soon as test outcomes are not
        // provided once, even if they were provided before.
        if test_outcomes.is_none() {
            self.considers_ignored_tests = false;
        }

        let output: JsonOutput = serde_json::from_slice(&std::fs::read(file)?)?;

        let shrink_path = |path: &Path| path.strip_prefix(src_base).unwrap_or(path).to_path_buf();
        let annotated_in_parent = |annotation: &JsonAnnotation, file: &JsonTestFile| {
            annotation.file.parent() == file.file.parent()
                && annotation.file.file_name().and_then(|f| f.to_str())
                    == Some(&output.bulk_annotations_file_name)
        };

        for file in &output.tests {
            if let Some(outcomes) = test_outcomes {
                let relative_file = shrink_path(&file.file)
                    .to_str()
                    .ok_or_else(|| anyhow::anyhow!("non-utf8 path: {}", file.file.display()))?
                    .to_string();
                // Do not consider annotations from ignored tests.
                if !outcomes.executed_tests.contains(&relative_file) {
                    // Only mark as ignored tests that actually have annotations.
                    if !file.annotations.is_empty() {
                        self.ignored_tests.insert(shrink_path(&file.file));
                    }
                    continue;
                }
            }

            for annotation in &file.annotations {
                let source = if annotation.file == file.file {
                    AnnotationSource::TestItself
                } else if annotated_in_parent(&annotation, &file) {
                    AnnotationSource::ParentDirectory { bulk_file: shrink_path(&annotation.file) }
                } else if annotation.file == file.file.join("Makefile") {
                    AnnotationSource::Makefile
                } else {
                    anyhow::bail!(
                        "bug: annotation {annotation:?} doesn't come from the file itself, \
                        its parent directory, or a Makefile. \
                        If you updated compiletest to accept annotations from other sources, \
                        you also need to update traceability-matrix."
                    );
                };

                self.paragraphs
                    .entry(annotation.annotation.clone())
                    .or_insert_with(HashSet::new)
                    .insert(AnnotatedFile { test: shrink_path(&file.file), source });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::{hashmap, hashset};
    use tempfile::{NamedTempFile, TempDir};

    #[test]
    fn test_load_file() -> Result<(), Error> {
        let file = NamedTempFile::new()?;
        std::fs::write(file.path(), annotations_file_1()?)?;

        let outcomes = Some(TestOutcomes {
            executed_tests: hashset![
                "example/foo.rs".into(),
                "example/bar.rs".into(),
                "example/foobar.rs".into()
            ],
            ignored_tests: hashset![
                "example/ignored.rs".into(),
                "example/ignored-without-annotations.rs".into(),
            ],
        });

        let mut annotations = Annotations::new();
        annotations.load_file(file.path(), Path::new("/base"), outcomes.as_ref())?;

        assert_eq!(
            hashmap! {
                "foo".into() => hashset![test_itself("example/foo.rs")],
                "bar".into() => hashset![
                    test_itself("example/foo.rs"),
                    test_itself("example/bar.rs"),
                ],
                "foobar".into() => hashset![
                    AnnotatedFile {
                        test: "example/foobar.rs".into(),
                        source: AnnotationSource::ParentDirectory {
                            bulk_file: "example/ferrocene-annotations".into(),
                        },
                    },
                ],
            },
            annotations.paragraphs,
        );
        assert_eq!(hashset!["example/ignored.rs".into()], annotations.ignored_tests,);

        Ok(())
    }

    #[test]
    fn test_load_directory() -> Result<(), Error> {
        let dir = TempDir::new()?;
        std::fs::write(dir.path().join("foo.json"), annotations_file_1()?)?;
        std::fs::write(dir.path().join("bar.json"), annotations_file_2()?)?;
        // Should not be loaded, as it's not a JSON file:
        std::fs::write(dir.path().join("baz.txt"), b"not JSON\n")?;
        // Should not be loaded, as it's in a subdirectory:
        std::fs::create_dir(dir.path().join("sub"))?;
        std::fs::write(dir.path().join("sub/quux.json"), annotations_file_3()?)?;

        let outcomes = Some(TestOutcomes {
            executed_tests: hashset![
                "example/foo.rs".into(),
                "example/bar.rs".into(),
                "example/baz.rs".into(),
                "example/foobar.rs".into()
            ],
            ignored_tests: hashset![
                "example/ignored.rs".into(),
                "example/ignored-without-annotations.rs".into(),
            ],
        });

        let mut annotations = Annotations::new();
        annotations.load_directory(dir.path(), Path::new("/base"), outcomes.as_ref())?;

        assert_eq!(
            hashmap! {
                "foo".into() => hashset![test_itself("example/foo.rs")],
                "bar".into() => hashset![
                    test_itself("example/foo.rs"),
                    test_itself("example/bar.rs"),
                    test_itself("example/baz.rs"),
                ],
                "baz".into() => hashset![test_itself("example/baz.rs")],
                "foobar".into() => hashset![
                    AnnotatedFile {
                        test: "example/foobar.rs".into(),
                        source: AnnotationSource::ParentDirectory {
                            bulk_file: "example/ferrocene-annotations".into(),
                        },
                    },
                ],
                // quux is not loaded as it's in a subdirectory
            },
            annotations.paragraphs,
        );
        assert_eq!(hashset!["example/ignored.rs".into()], annotations.ignored_tests,);

        Ok(())
    }

    fn test_itself(path: impl AsRef<Path>) -> AnnotatedFile {
        AnnotatedFile { test: path.as_ref().into(), source: AnnotationSource::TestItself }
    }

    fn annotations_file_1() -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/foo.rs",
                    "annotations": [
                        {
                            "annotation": "foo",
                            "file": "/base/example/foo.rs",
                        },
                        {
                            "annotation": "bar",
                            "file": "/base/example/foo.rs",
                        },
                    ],
                },
                {
                    "file": "/base/example/bar.rs",
                    "annotations": [
                        {
                            "annotation": "bar",
                            "file": "/base/example/bar.rs",
                        },
                    ],
                },
                {
                    "file": "/base/example/foobar.rs",
                    "annotations": [
                        {
                            "annotation": "foobar",
                            "file": "/base/example/ferrocene-annotations",
                        },
                    ],
                },
                {
                    "file": "/base/example/ignored.rs",
                    "annotations": [
                        {
                            "annotation": "ignored",
                            "file": "/base/example/ignored.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))?)
    }

    fn annotations_file_2() -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/baz.rs",
                    "annotations": [
                        {
                            "annotation": "baz",
                            "file": "/base/example/baz.rs",
                        },
                        {
                            "annotation": "bar",
                            "file": "/base/example/baz.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))?)
    }

    fn annotations_file_3() -> Result<Vec<u8>, Error> {
        Ok(serde_json::to_vec(&serde_json::json!({
            "tests": [
                {
                    "file": "/base/example/quux.rs",
                    "annotations": [
                        {
                            "annotation": "quux",
                            "file": "/base/example/quux.rs",
                        },
                    ],
                },
            ],
            "bulk_annotations_file_name": "ferrocene-annotations",
        }))?)
    }
}
