// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

use crate::annotations::{AnnotatedFile, Annotations};
use crate::documentations::Documentaton;
use anyhow::Error;
use std::collections::HashSet;
use std::fmt::Debug;
use std::ops::Deref;

pub(crate) const ITEM_KIND_SECTION: ItemKind = ItemKind {
    singular: "section",
    plural: "sections",
    hide_in_annotation_mode: false,
    include_title_when_copying: true,
};

pub(crate) const ITEM_KIND_PARAGRAPH: ItemKind = ItemKind {
    singular: "paragraph",
    plural: "paragraphs",
    hide_in_annotation_mode: true,
    include_title_when_copying: false,
};

pub(crate) const ITEM_KIND_CLI_OPTION: ItemKind = ItemKind {
    singular: "command line option",
    plural: "command line options",
    hide_in_annotation_mode: false,
    include_title_when_copying: false,
};

pub(crate) fn prepare(
    documentations: &[Documentaton],
    annotations: &Annotations,
) -> Result<TraceabilityMatrix, Error> {
    let mut matrix = TraceabilityMatrix {
        paragraphs: MatrixAnalysis::new(&ITEM_KIND_PARAGRAPH),
        sections: MatrixAnalysis::new(&ITEM_KIND_SECTION),
        options: MatrixAnalysis::new(&ITEM_KIND_CLI_OPTION),
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
                    Item {
                        kind: &ITEM_KIND_SECTION,
                        number: Some(ItemNumber(section.number.clone())),
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
                        section_number: ItemNumber(section.number.clone()),
                    });
                }

                for paragraph in &section.paragraphs {
                    seen_ids.insert(&paragraph.id);
                    matrix.paragraphs.add(
                        annotations,
                        &extra_paragraph_tests,
                        Item {
                            kind: &ITEM_KIND_PARAGRAPH,
                            id: paragraph.id.clone(),
                            number: Some(ItemNumber(paragraph.number.clone())),
                            title: None,
                            link: to_url(&paragraph.link),
                            page: matrix_page.clone(),
                        },
                    );
                }
            }

            for option in &page.options {
                seen_ids.insert(&option.id);
                matrix.options.add(
                    annotations,
                    &[],
                    Item {
                        kind: &ITEM_KIND_CLI_OPTION,
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
    for (annotation, files) in &annotations.paragraphs {
        if seen_ids.contains(annotation) {
            continue;
        }
        for file in files {
            matrix
                .unknown_annotations
                .push(UnknownAnnotation { annotation: annotation.clone(), file: file.clone() });
        }
    }

    // Ensure the results do not vary based on the HashMap's random ordering.
    matrix.paragraphs.sort_unstable();
    matrix.sections.sort_unstable();
    matrix.unknown_annotations.sort_unstable();

    Ok(matrix)
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct TraceabilityMatrix {
    pub(crate) paragraphs: MatrixAnalysis,
    pub(crate) sections: MatrixAnalysis,
    pub(crate) options: MatrixAnalysis,
    pub(crate) unknown_annotations: Vec<UnknownAnnotation>,
}

impl TraceabilityMatrix {
    pub(crate) fn analyses_by_kind(&self) -> impl Iterator<Item = &MatrixAnalysis> {
        [&self.sections, &self.paragraphs, &self.options].into_iter()
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct UnknownAnnotation {
    pub(crate) annotation: String,
    pub(crate) file: AnnotatedFile,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct MatrixAnalysis {
    pub(crate) kind: &'static ItemKind,
    pub(crate) linked: Vec<Link>,
    pub(crate) unlinked: Vec<Item>,
}

impl MatrixAnalysis {
    pub(crate) fn new(kind: &'static ItemKind) -> Self {
        MatrixAnalysis { kind, linked: Vec::new(), unlinked: Vec::new() }
    }

    fn sort_unstable(&mut self) {
        self.linked.sort_unstable();
        self.unlinked.sort_unstable();
    }

    fn add(&mut self, annotations: &Annotations, extra_tests: &[LinkTest], item: Item) -> bool {
        let mut tests = extra_tests.to_vec();
        if let Some(files) = annotations.paragraphs.get(&item.id) {
            tests.extend(files.iter().map(|file| LinkTest::File(file.clone())));
        }

        if tests.is_empty() {
            self.unlinked.push(item);
            false
        } else {
            tests.sort();
            self.linked.push(Link { item, tests });
            true
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub(crate) struct Link {
    pub(crate) item: Item,
    pub(crate) tests: Vec<LinkTest>,
}

impl Deref for Link {
    type Target = Item;

    fn deref(&self) -> &Self::Target {
        &self.item
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
    InheritFromSection { section_id: String, section_number: ItemNumber },
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
pub(crate) struct Item {
    pub(crate) kind: &'static ItemKind,
    pub(crate) number: Option<ItemNumber>,
    pub(crate) page: Page,
    pub(crate) title: Option<String>,
    pub(crate) id: String,
    pub(crate) link: String,
}

impl Item {
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
pub(crate) struct ItemKind {
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
pub(crate) struct ItemNumber(String);

impl Ord for ItemNumber {
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
            let (section, paragraph) = input.split_once(':').unwrap_or((input, ""));
            (
                section.split('.').filter(|n| !n.is_empty()).map(parse_number).collect::<Vec<_>>(),
                paragraph
                    .split('.')
                    .filter(|n| !n.is_empty())
                    .map(parse_number)
                    .collect::<Vec<_>>(),
            )
        }

        segments(&self.0).cmp(&segments(&other.0))
    }
}

impl PartialOrd for ItemNumber {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Display for ItemNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl<T: Into<String>> From<T> for ItemNumber {
    fn from(f: T) -> Self {
        Self(f.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::annotations::AnnotationSource;
    use crate::documentations::{CliOption, Document, Paragraph, Section, TraceabilityIds};
    use crate::utils::{hashmap, hashset};
    use std::collections::HashMap;
    use std::path::Path;

    #[test]
    fn test_prepare() -> Result<(), Error> {
        let documentations = [Documentaton {
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
            paragraphs: hashmap! {
                "fls_03".into() => hashset! {
                    test_itself("/example/foobar.rs"),
                },
                "fls_04".into() => hashset! {
                    test_itself("/example/foo.rs"),
                    test_itself("/example/bar.rs"),
                },
                "fls_99".into() => hashset! {
                    test_itself("/example/baz.rs"),
                    test_itself("/example/quux.rs"),
                },
                "fls_rustc_crate_name".into() => hashset! {
                    test_itself("/example/foobar.rs"),
                },
            },
            ignored_tests: HashSet::new(),
            considers_ignored_tests: true,
        };

        assert_eq!(
            TraceabilityMatrix {
                paragraphs: MatrixAnalysis {
                    kind: &ITEM_KIND_PARAGRAPH,
                    linked: vec![
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_PARAGRAPH,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_PARAGRAPH,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_PARAGRAPH,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_PARAGRAPH,
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
                        },
                    ],
                    unlinked: vec![Item {
                        kind: &ITEM_KIND_PARAGRAPH,
                        id: "fls_02".into(),
                        number: Some("12:4".into()),
                        title: None,
                        link: "../fls/example.html#fls_02".into(),
                        page: Page {
                            documentation: "FLS".into(),
                            name: "Example document".into(),
                            link: "../fls/example.html".into()
                        },
                    }],
                },
                sections: MatrixAnalysis {
                    kind: &ITEM_KIND_SECTION,
                    linked: vec![
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_SECTION,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_SECTION,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_SECTION,
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
                        },
                        Link {
                            item: Item {
                                kind: &ITEM_KIND_SECTION,
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
                        },
                    ],
                    unlinked: vec![Item {
                        kind: &ITEM_KIND_SECTION,
                        id: "fls_01".into(),
                        number: Some("12".into()),
                        title: Some("Example section".into()),
                        link: "../fls/example.html#example-section".into(),
                        page: Page {
                            documentation: "FLS".into(),
                            name: "Example document".into(),
                            link: "../fls/example.html".into()
                        },
                    },],
                },
                options: MatrixAnalysis {
                    kind: &ITEM_KIND_CLI_OPTION,
                    linked: vec![Link {
                        item: Item {
                            kind: &ITEM_KIND_CLI_OPTION,
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
                    },],
                    unlinked: vec![]
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
    fn test_item_numbers_ordering() {
        fn numberize(numbers: &[&str]) -> Vec<ItemNumber> {
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
        AnnotatedFile { test: path.as_ref().into(), source: AnnotationSource::TestItself }
    }
}
