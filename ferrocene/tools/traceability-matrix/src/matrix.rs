// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use std::collections::{BTreeSet, HashSet};
use std::fmt::Debug;
use std::ops::Deref;

use crate::annotations::{AnnotatedFile, Annotations};
use crate::documentations::{CliOption, Documentation, Section};

pub(crate) const ELEMENT_KIND_SECTION: ElementKind = ElementKind {
    singular: "specification section",
    plural: "specification sections",
    include_title_when_copying: true,
};

pub(crate) const ELEMENT_KIND_CLI_OPTION: ElementKind = ElementKind {
    singular: "command line option",
    plural: "command line options",
    include_title_when_copying: false,
};

pub(crate) fn prepare(
    documentations: &[Documentation],
    annotations: &Annotations,
) -> anyhow::Result<TraceabilityMatrix> {
    let mut matrix = TraceabilityMatrix {
        sections: MatrixAnalysis::new(&ELEMENT_KIND_SECTION),
        cli_options: MatrixAnalysis::new(&ELEMENT_KIND_CLI_OPTION),
        unknown_annotations: Vec::new(),
    };

    let seen_ids = documentations
        .iter()
        .flat_map(|documentation| matrix.analyze_document(annotations, documentation))
        .collect::<HashSet<_>>();

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
    pub(crate) cli_options: MatrixAnalysis,
    pub(crate) unknown_annotations: Vec<UnknownAnnotation>,
}

impl TraceabilityMatrix {
    pub(crate) fn analyses_by_kind(&self) -> impl Iterator<Item = &MatrixAnalysis> {
        [&self.sections, &self.cli_options].into_iter()
    }

    fn analyze_document<'a>(
        &mut self,
        annotations: &Annotations,
        documentation: &'a Documentation,
    ) -> HashSet<&'a String> {
        let to_url = |url: &str| format!("{}/{url}", documentation.url);

        let mut seen_ids = HashSet::new();
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
                self.sections.add(
                    annotations,
                    &extra_section_tests,
                    Element::section(&matrix_page, section, to_url(&section.link)),
                );
            }

            for option in &page.options {
                seen_ids.insert(&option.id);
                self.cli_options.add(
                    annotations,
                    &[],
                    Element::cli_option(&matrix_page, option, to_url(&option.link)),
                );
            }
        }
        seen_ids
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
    fn new(kind: &'static ElementKind) -> Self {
        MatrixAnalysis {
            kind,
            linked: BTreeSet::new(),
            partially_linked: BTreeSet::new(),
            unlinked: BTreeSet::new(),
        }
    }

    /// Returns `true` if tests were linked (or partially linked).
    fn add(
        &mut self,
        annotations: &Annotations,
        extra_tests: &[LinkTest],
        element: Element,
    ) -> bool {
        let mut tests = extra_tests.to_vec();
        if let Some(files) = annotations.ids.get(&element.id).cloned() {
            tests.extend(files.into_iter().map(LinkTest::File));
        }

        if tests.is_empty() {
            self.unlinked.insert(element);
            false
        } else {
            tests.sort();

            let ignored = filter_targets_from_tests(&tests, |t| &t.targets.ignored.0);
            let executed = filter_targets_from_tests(&tests, |t| &t.targets.executed.0);
            let untested_targets =
                ignored.difference(&executed).map(ToString::to_string).collect::<BTreeSet<_>>();

            if untested_targets.is_empty() { &mut self.linked } else { &mut self.partially_linked }
                .insert(Link { element, tests, untested_targets });

            true
        }
    }

    pub(crate) fn informational_count(&self) -> usize {
        self.linked.iter().filter(|l| l.informational()).count()
    }
}

fn filter_targets_from_tests<F>(tests: &[LinkTest], getter: F) -> BTreeSet<&String>
where
    F: Fn(&AnnotatedFile) -> &BTreeSet<String>,
{
    tests.iter().filter_map(LinkTest::unwrap_file).flat_map(getter).collect()
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
    pub(crate) fn informational(&self) -> bool {
        self.tests.iter().any(|t| matches!(t, LinkTest::Informational))
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(crate) enum LinkTest {
    File(AnnotatedFile),
    NoParagraphsInSection,
    Informational,
}

impl LinkTest {
    fn unwrap_file(&self) -> Option<&AnnotatedFile> {
        match self {
            LinkTest::File(annotated) => Some(annotated),
            _ => None,
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

    fn cli_option(matrix_page: &Page, option: &CliOption, link: String) -> Self {
        Self {
            kind: &ELEMENT_KIND_CLI_OPTION,
            id: option.id.clone(),
            number: None,
            title: Some(format!("{} {}", option.program, option.option)),
            link,
            page: matrix_page.clone(),
        }
    }

    fn section(matrix_page: &Page, section: &Section, link: String) -> Self {
        Self {
            kind: &ELEMENT_KIND_SECTION,
            number: Some(ElementNumber(section.number.clone())),
            title: Some(section.title.clone()),
            id: section.id.clone(),
            link,
            page: matrix_page.clone(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Hash)]
pub(crate) struct ElementKind {
    pub(crate) singular: &'static str,
    pub(crate) plural: &'static str,
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
    use std::collections::BTreeMap;
    use std::path::Path;

    use super::*;
    use crate::annotations::AnnotationSource;
    use crate::documentations::{Document, Paragraph, TraceabilityIds};

    #[test]
    fn test_prepare() -> anyhow::Result<()> {
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
                                paragraphs: vec![Paragraph {}],
                            },
                            Section {
                                id: "fls_03".into(),
                                number: "12.2".into(),
                                title: "Example subsection".into(),
                                link: "example.html#example-subsection".into(),
                                informational: false,
                                paragraphs: vec![Paragraph {}, Paragraph {}],
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
                                paragraphs: vec![Paragraph {}],
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
                            paragraphs: vec![Paragraph {}],
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
