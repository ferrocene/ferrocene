// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::annotations::{AnnotatedFile, Annotations};
use crate::documentations::Documentation;
use anyhow::Error;
use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::ops::Deref;

pub(crate) const ELEMENT_KIND_SECTION: ElementKind = ElementKind {
    singular: "section",
    plural: "sections",
    hide_in_annotation_mode: false,
    include_title_when_copying: true,
};

pub(crate) const ELEMENT_KIND_PARAGRAPH: ElementKind = ElementKind {
    singular: "paragraph",
    plural: "paragraphs",
    hide_in_annotation_mode: true,
    include_title_when_copying: false,
};

pub(crate) const ELEMENT_KIND_CLI_OPTION: ElementKind = ElementKind {
    singular: "command line option",
    plural: "command line options",
    hide_in_annotation_mode: false,
    include_title_when_copying: false,
};

pub(crate) fn prepare(
    documentations: &[Documentation],
    annotations: &Annotations,
) -> Result<TraceabilityMatrix, Error> {
    let mut matrix = TraceabilityMatrix {
        paragraphs: MatrixAnalysis::new(&ELEMENT_KIND_PARAGRAPH),
        sections: MatrixAnalysis::new(&ELEMENT_KIND_SECTION),
        cli_options: MatrixAnalysis::new(&ELEMENT_KIND_CLI_OPTION),
        unknown_annotations: Vec::new(),
    };

    let mut seen_ids = HashSet::new();
    for documentation in documentations {
        let to_url = |url: &str| format!("{}/{url}", documentation.url);

        for page in &documentation.ids.documents {
            let matrix_page = Page {
                documentation: documentation.name.clone(),
                name: page.title.clone(),
                link: to_url(&page.link),
            };

            for section in &page.sections {
                seen_ids.insert(&section.id);

                let mut extra_section_tests = Vec::new();
                if section.paragraphs.is_empty() {
                    extra_section_tests.push(LinkTest::NoParagraphsInSection);
                }
                if page.informational || section.informational {
                    extra_section_tests.push(LinkTest::Informational);
                }
                let has_annotations = matrix.sections.add(
                    annotations,
                    &extra_section_tests,
                    Element {
                        kind: &ELEMENT_KIND_SECTION,
                        number: Some(ElementNumber(section.number.clone())),
                        title: Some(section.title.clone()),
                        id: section.id.clone(),
                        link: to_url(&section.link),
                        page: matrix_page.clone(),
                    },
                );

                let mut extra_paragraph_tests = Vec::new();
                if has_annotations {
                    extra_paragraph_tests.push(LinkTest::InheritFromSection {
                        section_id: section.id.clone(),
                        section_number: ElementNumber(section.number.clone()),
                    });
                }

                for paragraph in &section.paragraphs {
                    seen_ids.insert(&paragraph.id);
                    matrix.paragraphs.add(
                        annotations,
                        &extra_paragraph_tests,
                        Element {
                            kind: &ELEMENT_KIND_PARAGRAPH,
                            id: paragraph.id.clone(),
                            number: Some(ElementNumber(paragraph.number.clone())),
                            title: None,
                            link: to_url(&paragraph.link),
                            page: matrix_page.clone(),
                        },
                    );
                }
            }

            for option in &page.options {
                seen_ids.insert(&option.id);
                matrix.cli_options.add(
                    annotations,
                    &[],
                    Element {
                        kind: &ELEMENT_KIND_CLI_OPTION,
                        id: option.id.clone(),
                        number: None,
                        title: Some(format!("{} {}", option.program, option.option)),
                        link: to_url(&option.link),
                        page: matrix_page.clone(),
                    },
                );
            }
        }
    }

    // Detect which tests link to unknown paragraphs.
    for (annotation, files) in &annotations.ids {
        if seen_ids.contains(annotation) {
            continue;
        }
        for file in files {
            matrix
                .unknown_annotations
                .push(UnknownAnnotation { annotation: annotation.clone(), file: file.clone() });
        }
    }

    matrix.unknown_annotations.sort_unstable();

    Ok(matrix)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TraceabilityMatrix {
    pub(crate) sections: MatrixAnalysis,
    pub(crate) paragraphs: MatrixAnalysis,
    pub(crate) cli_options: MatrixAnalysis,
    pub(crate) unknown_annotations: Vec<UnknownAnnotation>,
}

impl TraceabilityMatrix {
    pub(crate) fn analyses_by_kind(&self) -> impl Iterator<Item = &MatrixAnalysis> {
        [&self.sections, &self.paragraphs, &self.cli_options].into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UnknownAnnotation {
    pub(crate) annotation: String,
    pub(crate) file: AnnotatedFile,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MatrixAnalysis {
    pub(crate) kind: &'static ElementKind,
    pub(crate) linked: BTreeSet<Link>,
    pub(crate) partially_linked: BTreeSet<Link>,
    pub(crate) unlinked: BTreeSet<Element>,
}

impl MatrixAnalysis {
    pub(crate) fn new(kind: &'static ElementKind) -> Self {
        MatrixAnalysis {
            kind,
            linked: BTreeSet::new(),
            partially_linked: BTreeSet::new(),
            unlinked: BTreeSet::new(),
        }
    }

    fn add(
        &mut self,
        annotations: &Annotations,
        extra_tests: &[LinkTest],
        element: Element,
    ) -> bool {
        let mut tests = extra_tests.to_vec();
        if let Some(files) = annotations.ids.get(&element.id) {
            tests.extend(files.iter().map(|file| LinkTest::File(file.clone())));
        }

        if tests.is_empty() {
            self.unlinked.insert(element);
            false
        } else {
            tests.sort();
            let mut untested_targets = BTreeSet::new();
            for link_test in &tests {
                if let LinkTest::File(annotated) = link_test {
                    for target in &annotated.targets.ignored.0 {
                        untested_targets.insert(target.to_owned());
                    }
                }
            }
            // another loop, to capture targets that are not tested at all
            for link_test in &tests {
                if let LinkTest::File(annotated) = link_test {
                    for target in &annotated.targets.executed.0 {
                        untested_targets.remove(target);
                    }
                }
            }
            if untested_targets.is_empty() {
                self.linked.insert(Link { element, tests: tests.clone(), untested_targets });
            } else {
                self.partially_linked.insert(Link {
                    element,
                    tests: tests.clone(),
                    untested_targets,
                });
            }
            true
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Link {
    pub(crate) element: Element,
    pub(crate) tests: Vec<LinkTest>,
    pub(crate) untested_targets: BTreeSet<String>,
}

impl Deref for Link {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.element
    }
}

impl Link {
    pub(crate) fn hide_in_annotation_mode(&self) -> bool {
        self.tests.iter().all(|t| t.hide_in_annotation_mode())
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(crate) enum LinkTest {
    File(AnnotatedFile),
    NoParagraphsInSection,
    Informational,
    InheritFromSection { section_id: String, section_number: ElementNumber },
}

impl LinkTest {
    fn hide_in_annotation_mode(&self) -> bool {
        match self {
            LinkTest::File(_) => false,
            LinkTest::NoParagraphsInSection => true,
            LinkTest::Informational => true,
            LinkTest::InheritFromSection { .. } => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Element {
    pub(crate) kind: &'static ElementKind,
    pub(crate) number: Option<ElementNumber>,
    pub(crate) page: Page,
    pub(crate) title: Option<String>,
    pub(crate) id: String,
    pub(crate) link: String,
}

impl Element {
    pub(crate) fn name(&self) -> String {
        match (&self.number, &self.title) {
            (Some(number), Some(title)) => format!("{number} {title}"),
            (Some(number), None) => number.0.clone(),
            (None, Some(title)) => title.clone(),
            (None, None) => "no name".into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(crate) struct ElementKind {
    pub(crate) singular: &'static str,
    pub(crate) plural: &'static str,
    pub(crate) hide_in_annotation_mode: bool,
    pub(crate) include_title_when_copying: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(crate) struct Page {
    pub(crate) documentation: String,
    pub(crate) name: String,
    pub(crate) link: String,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub(crate) struct ElementNumber(String);

impl Ord for ElementNumber {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Custom ordering is implemented to provide the intuitive order people expect.
        //
        // "1.2.3:4.5" is converted to ([1, 2, 3], [4, 5]), and then *that* is compared. This
        // results in the paragraph numbers inside a section to be sorted before the following
        // section (due to the two different array in a tuple), and in numbers being properly
        // ordered (compared to ordering the string representation of numbers).

        #[derive(PartialEq, Eq, PartialOrd, Ord)]
        enum Number<'a> {
            Int(i32),
            Letter(&'a str),
        }

        fn parse_number(raw: &str) -> Number<'_> {
            if raw.chars().all(|c| c.is_ascii_digit()) {
                Number::Int(raw.parse().unwrap())
            } else {
                Number::Letter(raw)
            }
        }

        fn segments(input: &str) -> (Vec<Number<'_>>, Vec<Number<'_>>) {
            fn parsed(arg: &str) -> Vec<Number<'_>> {
                arg.split('.').filter(|number| !number.is_empty()).map(parse_number).collect()
            }
            let (section, paragraph) = input.split_once(':').unwrap_or((input, ""));
            (parsed(section), parsed(paragraph))
        }

        segments(&self.0).cmp(&segments(&other.0))
    }
}

impl PartialOrd for ElementNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for ElementNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for ElementNumber {
    fn from(f: T) -> Self {
        Self(f.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotations::AnnotationSource;
    use crate::documentations::{CliOption, Document, Paragraph, Section, TraceabilityIds};
    use std::collections::BTreeMap;
    use std::path::Path;

    #[test]
    fn test_prepare() -> Result<(), Error> {
        let documentations = [Documentation {
            name: "FLS".into(),
            url: "../fls".into(),
            ids: TraceabilityIds {
                documents: vec![
                    Document {
                        title: "Example document".into(),
                        link: "example.html".into(),
                        informational: false,
                        sections: vec![
                            Section {
                                id: "fls_01".into(),
                                number: "12".into(),
                                title: "Example section".into(),
                                link: "example.html#example-section".into(),
                                informational: false,
                                paragraphs: vec![Paragraph {
                                    id: "fls_02".into(),
                                    number: "12:4".into(),
                                    link: "example.html#fls_02".into(),
                                }],
                            },
                            Section {
                                id: "fls_03".into(),
                                number: "12.2".into(),
                                title: "Example subsection".into(),
                                link: "example.html#example-subsection".into(),
                                informational: false,
                                paragraphs: vec![
                                    Paragraph {
                                        id: "fls_04".into(),
                                        number: "12.2:3".into(),
                                        link: "example.html#fls_04".into(),
                                    },
                                    Paragraph {
                                        id: "fls_04b".into(),
                                        number: "12.2:4".into(),
                                        link: "example.html#fls_04b".into(),
                                    },
                                ],
                            },
                            Section {
                                id: "fls_05".into(),
                                number: "12.2.1".into(),
                                title: "Example empty section".into(),
                                link: "example.html#example-empty-section".into(),
                                informational: false,
                                paragraphs: Vec::new(),
                            },
                            Section {
                                id: "fls_08".into(),
                                number: "12.2.2".into(),
                                title: "Example informational section".into(),
                                link: "example.html#example-informational-section".into(),
                                informational: true,
                                paragraphs: vec![Paragraph {
                                    id: "fls_09".into(),
                                    number: "12.2.2:1".into(),
                                    link: "example.html#fls_09".into(),
                                }],
                            },
                        ],
                        options: vec![CliOption {
                            id: "fls_rustc_crate_name".into(),
                            program: "rustc".into(),
                            option: "--crate-name <name>".into(),
                            link: "example.html#fls_rustc_crate_name".into(),
                        }],
                    },
                    Document {
                        title: "Information".into(),
                        link: "information.html".into(),
                        informational: true,
                        sections: vec![Section {
                            id: "fls_06".into(),
                            number: "A.1".into(),
                            title: "Example information".into(),
                            link: "information.html#example-information".into(),
                            informational: false,
                            paragraphs: vec![Paragraph {
                                id: "fls_07".into(),
                                number: "A.1:1".into(),
                                link: "information.html#fls_07".into(),
                            }],
                        }],
                        options: vec![],
                    },
                ],
            },
        }];

        let annotations = Annotations {
            ids: BTreeMap::from([
                ("fls_03".into(), BTreeSet::from([test_itself("/example/foobar.rs")])),
                (
                    "fls_04".into(),
                    BTreeSet::from([
                        test_itself("/example/foo.rs"),
                        test_itself("/example/bar.rs"),
                    ]),
                ),
                (
                    "fls_99".into(),
                    BTreeSet::from([
                        test_itself("/example/baz.rs"),
                        test_itself("/example/quux.rs"),
                    ]),
                ),
                (
                    "fls_rustc_crate_name".into(),
                    BTreeSet::from([test_itself("/example/foobar.rs")]),
                ),
            ]),
            ignored_tests: BTreeMap::new(),
            considers_ignored_tests: true,
        };

        assert_eq!(
            TraceabilityMatrix {
                paragraphs: MatrixAnalysis {
                    kind: &ELEMENT_KIND_PARAGRAPH,
                    linked: BTreeSet::from([
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_PARAGRAPH,
                                id: "fls_04".into(),
                                number: Some("12.2:3".into()),
                                title: None,
                                link: "../fls/example.html#fls_04".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into()
                                },
                            },
                            tests: vec![
                                link_test_itself("/example/bar.rs"),
                                link_test_itself("/example/foo.rs"),
                                LinkTest::InheritFromSection {
                                    section_id: "fls_03".into(),
                                    section_number: "12.2".into()
                                },
                            ],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_PARAGRAPH,
                                id: "fls_04b".into(),
                                number: Some("12.2:4".into()),
                                title: None,
                                link: "../fls/example.html#fls_04b".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into()
                                },
                            },
                            tests: vec![LinkTest::InheritFromSection {
                                section_id: "fls_03".into(),
                                section_number: "12.2".into()
                            }],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_PARAGRAPH,
                                id: "fls_09".into(),
                                number: Some("12.2.2:1".into()),
                                title: None,
                                link: "../fls/example.html#fls_09".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into()
                                }
                            },
                            tests: vec![LinkTest::InheritFromSection {
                                section_id: "fls_08".into(),
                                section_number: "12.2.2".into()
                            }],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_PARAGRAPH,
                                id: "fls_07".into(),
                                number: Some("A.1:1".into()),
                                title: None,
                                link: "../fls/information.html#fls_07".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Information".into(),
                                    link: "../fls/information.html".into(),
                                },
                            },
                            tests: vec![LinkTest::InheritFromSection {
                                section_id: "fls_06".into(),
                                section_number: "A.1".into()
                            }],
                            untested_targets: Default::default(),
                        },
                    ]),
                    unlinked: BTreeSet::from([Element {
                        kind: &ELEMENT_KIND_PARAGRAPH,
                        id: "fls_02".into(),
                        number: Some("12:4".into()),
                        title: None,
                        link: "../fls/example.html#fls_02".into(),
                        page: Page {
                            documentation: "FLS".into(),
                            name: "Example document".into(),
                            link: "../fls/example.html".into()
                        },
                    }]),
                    partially_linked: Default::default(),
                },
                sections: MatrixAnalysis {
                    kind: &ELEMENT_KIND_SECTION,
                    linked: BTreeSet::from([
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_SECTION,
                                id: "fls_03".into(),
                                number: Some("12.2".into()),
                                title: Some("Example subsection".into()),
                                link: "../fls/example.html#example-subsection".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into()
                                },
                            },
                            tests: vec![link_test_itself("/example/foobar.rs")],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_SECTION,
                                id: "fls_05".into(),
                                number: Some("12.2.1".into()),
                                title: Some("Example empty section".into()),
                                link: "../fls/example.html#example-empty-section".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into()
                                },
                            },
                            tests: vec![LinkTest::NoParagraphsInSection],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_SECTION,
                                id: "fls_08".into(),
                                number: Some("12.2.2".into()),
                                title: Some("Example informational section".into()),
                                link: "../fls/example.html#example-informational-section".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Example document".into(),
                                    link: "../fls/example.html".into(),
                                },
                            },
                            tests: vec![LinkTest::Informational],
                            untested_targets: Default::default(),
                        },
                        Link {
                            element: Element {
                                kind: &ELEMENT_KIND_SECTION,
                                id: "fls_06".into(),
                                number: Some("A.1".into()),
                                title: Some("Example information".into()),
                                link: "../fls/information.html#example-information".into(),
                                page: Page {
                                    documentation: "FLS".into(),
                                    name: "Information".into(),
                                    link: "../fls/information.html".into(),
                                },
                            },
                            tests: vec![LinkTest::Informational],
                            untested_targets: Default::default(),
                        },
                    ]),
                    unlinked: BTreeSet::from([Element {
                        kind: &ELEMENT_KIND_SECTION,
                        id: "fls_01".into(),
                        number: Some("12".into()),
                        title: Some("Example section".into()),
                        link: "../fls/example.html#example-section".into(),
                        page: Page {
                            documentation: "FLS".into(),
                            name: "Example document".into(),
                            link: "../fls/example.html".into()
                        },
                    }]),
                    partially_linked: Default::default(),
                },
                cli_options: MatrixAnalysis {
                    kind: &ELEMENT_KIND_CLI_OPTION,
                    linked: BTreeSet::from([Link {
                        element: Element {
                            kind: &ELEMENT_KIND_CLI_OPTION,
                            id: "fls_rustc_crate_name".into(),
                            number: None,
                            title: Some("rustc --crate-name <name>".into()),
                            link: "../fls/example.html#fls_rustc_crate_name".into(),
                            page: Page {
                                documentation: "FLS".into(),
                                name: "Example document".into(),
                                link: "../fls/example.html".into(),
                            },
                        },
                        tests: vec![link_test_itself("/example/foobar.rs")],
                        untested_targets: Default::default(),
                    }]),
                    unlinked: BTreeSet::default(),
                    partially_linked: Default::default(),
                },
                unknown_annotations: vec![
                    UnknownAnnotation {
                        annotation: "fls_99".into(),
                        file: test_itself("/example/baz.rs"),
                    },
                    UnknownAnnotation {
                        annotation: "fls_99".into(),
                        file: test_itself("/example/quux.rs"),
                    },
                ],
            },
            prepare(&documentations, &annotations)?,
        );

        Ok(())
    }

    #[test]
    fn test_element_numbers_ordering() {
        fn numberize(numbers: &[&str]) -> Vec<ElementNumber> {
            numbers.iter().map(|s| (*s).into()).collect()
        }

        let mut numbers =
            numberize(&["1:10", "1.1:1", "A.1", "1:1.1", "10", "1", "2:3", "1:2", "1:1"]);
        numbers.sort();
        assert_eq!(
            numberize(&["1", "1:1", "1:1.1", "1:2", "1:10", "1.1:1", "2:3", "10", "A.1"]),
            numbers
        )
    }

    fn link_test_itself(path: impl AsRef<Path>) -> LinkTest {
        LinkTest::File(test_itself(path))
    }

    fn test_itself(path: impl AsRef<Path>) -> AnnotatedFile {
        AnnotatedFile {
            test: path.as_ref().into(),
            source: AnnotationSource::TestItself,
            targets: Default::default(),
        }
    }
}
