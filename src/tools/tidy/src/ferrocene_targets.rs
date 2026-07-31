use core::fmt;
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::{fs, str};

use serde::Deserialize;

use crate::diagnostics::{CheckId, RunningCheck, TidyCtx};

const USER_MANUALS: &[&str] = &["user-manual", "qnx7-manual", "qnx8-manual", "rhivos-manual"];
const SAFETY_MANUALS: &[&str] = &["safety-manual", "qnx7-manual", "qnx8-manual", "rhivos-manual"];
const QUALIFICATION_REPORTS: &[&str] =
    &["qualification-report", "qnx7-manual", "qnx8-manual", "rhivos-manual"];

pub fn check(root_path: &Path, tidy_ctx: TidyCtx) {
    let ferrocene_dir = root_path.join("ferrocene");
    let mut check = tidy_ctx.start_check(CheckId::new("ferrocene_targets").path(&ferrocene_dir));
    let targets_toml = TargetsToml::parse(&ferrocene_dir, &mut check);

    check_tests_results(&ferrocene_dir, &targets_toml, &mut check);
    check_target_pages_in_user_manuals(&ferrocene_dir, &targets_toml, &mut check);
    check_qualification_scopes(&ferrocene_dir, &targets_toml, &mut check);
    check_target_overviews(&ferrocene_dir, &targets_toml, &mut check);
    check_packages_toml(&ferrocene_dir, &targets_toml, &mut check);
    check_target_names_toml(ferrocene_dir, &targets_toml, check);
}

struct TargetsToml {
    hosts: BTreeSet<String>,
    targets: BTreeMap<String, Properties>,
}

impl TargetsToml {
    fn parse(ferrocene_dir: &PathBuf, check: &mut RunningCheck) -> Self {
        #[derive(Deserialize)]
        struct Raw {
            qualified: Qualified,
        }

        #[derive(Deserialize)]
        struct Qualified {
            targets: Vec<Target>,
            hosts: BTreeSet<String>,
        }

        #[derive(Deserialize)]
        struct Target {
            tuple: String,
            is_bare_metal: bool,
            host: String,
        }

        let toml_path = ferrocene_dir.join("targets.toml");
        let toml_contents = fs::read_to_string(&toml_path).unwrap();
        let Raw { qualified: Qualified { targets: raw_targets, hosts } } =
            toml::from_str(&toml_contents).unwrap();

        let mut targets = BTreeMap::new();

        for Target { tuple, is_bare_metal, host } in raw_targets {
            if !hosts.contains(&host) {
                check.error(format!("target '{tuple}' specifies host '{host}' but its missing from `hosts` list, in `ferrocene/targets.toml`"));
            }

            let std = if is_bare_metal { Std::BareMetal } else { Std::Full };
            if targets.insert(tuple.clone(), Properties { std, host }).is_some() {
                check.error(format!("duplicate target '{tuple}' in `ferrocene/targets.toml`"));
            }
        }

        Self { hosts, targets }
    }
}

#[derive(Clone, Debug, PartialEq)]
struct Properties {
    std: Std,
    host: String,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Std {
    BareMetal,
    Full,
}

// ferrocene/doc/*/src/rustc/$T.rst
fn check_tests_results(
    ferrocene_dir: &PathBuf,
    targets_toml: &TargetsToml,
    check: &mut RunningCheck,
) {
    fn parse(
        ferrocene_dir: &Path,
        check: &mut RunningCheck,
    ) -> BTreeMap<String, (Properties, PathBuf)> {
        const OPT_TARGET: &str = ":target:";
        const OPT_TESTED_TARGET: &str = ":tested_target_with_std:";
        const OPT_HOST: &str = ":host:";

        let doc_dir = ferrocene_dir.join("doc");
        let mut found = BTreeMap::new();
        for report in QUALIFICATION_REPORTS {
            let rustc_dir = doc_dir.join(report).join("src/rustc");
            for entry in fs::read_dir(&rustc_dir).unwrap() {
                let path = entry.unwrap().path();
                if path.extension().unwrap_or_default() != "rst" {
                    continue;
                }
                let stem = path.file_stem().unwrap();
                if stem == "index" {
                    continue;
                }
                let filename = stem.to_string_lossy().into_owned();

                let contents = fs::read_to_string(&path).unwrap();
                let mut lines = contents.lines();

                let mut target = None;
                let mut tested_target = None;
                let mut host = None;
                while let Some(line) = lines.next() {
                    if let Some(tuple) = strip_prefixes(line, &[OPT_TARGET]) {
                        if target.is_some() {
                            check.error(format!(
                                "duplicate {OPT_TARGET} option in {}",
                                path.display()
                            ));
                            continue;
                        }
                        if filename != tuple {
                            check.error(format!(
                                "{OPT_TARGET} does not match filename in {}",
                                path.display()
                            ));
                            continue;
                        }

                        target = Some(tuple);
                    }

                    if let Some(tuple) = strip_prefixes(line, &[OPT_TESTED_TARGET]) {
                        if tested_target.is_some() {
                            check.error(format!(
                                "duplicate {OPT_TESTED_TARGET} option in {}",
                                path.display()
                            ));
                        }
                        tested_target = Some(tuple);
                    }

                    if let Some(tuple) = strip_prefixes(line, &[OPT_HOST]) {
                        if host.is_some() {
                            check.error(format!(
                                "duplicate {OPT_HOST} option in {}",
                                path.display()
                            ));
                        }
                        host = Some(tuple);
                    }
                }
                let Some(target) = target else {
                    check.error(format!("{OPT_TARGET} not found in {}", path.display()));
                    continue;
                };

                let Some(host) = host else {
                    check.error(format!("{OPT_HOST} not found in {}", path.display()));
                    continue;
                };

                let is_bare_metal = tested_target.is_some() && Some(target) != tested_target;
                let std = if is_bare_metal { Std::BareMetal } else { Std::Full };
                let host = host.to_string();
                assert!(
                    found.insert(filename, (Properties { std, host }, path)).is_none(),
                    "target `{target}` is in multiple manuals but this is not currently supported; update this tidy check",
                );
            }
        }
        found
    }

    let in_docs = parse(ferrocene_dir, check);
    // both qualified targets and qualified hosts that are not
    // qualified targets need test results
    let in_toml = targets_toml
        .targets
        .clone()
        .into_iter()
        .chain(
            targets_toml
                .hosts
                .iter()
                .cloned()
                .filter(|host| !targets_toml.targets.contains_key(host))
                .map(|host| (host.clone(), Properties { std: Std::Full, host })),
        )
        .collect();
    compare_both_ways(&in_toml, &in_docs, "test results (ferrocene/doc/*/src/rustc)", check)
}

// ferrocene/doc/*/src/targets/$T.rst
fn check_target_pages_in_user_manuals(
    ferrocene_dir: &PathBuf,
    targets_toml: &TargetsToml,
    check: &mut RunningCheck,
) {
    fn parse(ferrocene_dir: &Path) -> BTreeSet<String> {
        let doc_dir = ferrocene_dir.join("doc");
        let mut found = BTreeMap::new();
        for manual in USER_MANUALS {
            let rustc_dir = doc_dir.join(manual).join("src/targets");
            extend_with_targets_in_dir(&rustc_dir, &mut found)
        }
        found.into_keys().collect()
    }

    let in_docs = parse(ferrocene_dir);
    let in_toml =
        targets_toml.targets.keys().cloned().chain(targets_toml.hosts.iter().cloned()).collect();
    // one way comparison because the manuals include pages for Quality Managed and other targets
    compare_one_way(&in_toml, &in_docs, "user manual (ferrocene/doc/*/src/targets)", check)
}

// ferrocene/doc/*/src/scope.rst
fn check_qualification_scopes(
    ferrocene_dir: &PathBuf,
    targets_toml: &TargetsToml,
    check: &mut RunningCheck,
) {
    fn parse(
        ferrocene_dir: &Path,
        check: &mut RunningCheck,
    ) -> BTreeMap<String, (Properties, PathBuf)> {
        let doc_dir = ferrocene_dir.join("doc");
        let mut targets = BTreeMap::new();
        for manual in SAFETY_MANUALS {
            let path = doc_dir.join(manual).join("src/scope.rst");
            let contents = fs::read_to_string(&path).unwrap();

            let mut lines = contents.lines();

            // look for this pattern:
            //     * - :target:`x86_64-unknown-linux-gnu`
            //       - :target:`aarch64-unknown-none`
            //       - ``core``
            //       - [``alloc``|``alloc``, ``std``, ``test``]
            //       - [|``proc_macro``]
            //
            // the first tuple is the host and the second is the target
            while let Some(line) = lines.next() {
                // * - :target:`x86_64-unknown-linux-gnu`
                let Some(rest) = strip_prefixes(line, &["*", "-", ":target:", "`"]) else {
                    continue;
                };
                let Some(host) = rest.strip_suffix('`') else { continue };

                //   - :target:`aarch64-unknown-none`
                let Some(line) = lines.next() else { continue };
                let Some(rest) = strip_prefixes(line, &["-", ":target:", "`"]) else { continue };
                let Some(target) = rest.strip_suffix('`') else { continue };

                // - ``core``
                let Some(line) = lines.next() else { continue };
                let Some(certified) = strip_prefixes(line, &["-"]) else { continue };

                const CERTIFIED: &str = "``core``";
                if certified != CERTIFIED {
                    check.error(format!("{target} is missing certified {CERTIFIED} library"));
                }

                // - [``alloc``|``alloc``, ``std``, ``test``]
                let Some(line) = lines.next() else { continue };
                let Some(uncertified) = strip_prefixes(line, &["-"]) else { continue };

                // - [|``proc_macro``]
                let Some(line) = lines.next() else { continue };
                let Some(qualified) = strip_prefixes(line, &["-"]) else { continue };

                const BARE_METAL_UNCERTIFIED: &str = "``alloc``";
                const BARE_METAL_QUALIFIED: &str = "";
                const FULL_UNCERTIFIED: &str = "``alloc``, ``std``, ``test``";
                const FULL_QUALIFIED: &str = "``proc_macro``";

                let std = if uncertified == BARE_METAL_UNCERTIFIED
                    && qualified == BARE_METAL_QUALIFIED
                {
                    Std::BareMetal
                } else if uncertified == FULL_UNCERTIFIED && qualified == FULL_QUALIFIED {
                    Std::Full
                } else {
                    check.error(format!(
                    "unexpected Uncertified libraries and Qualified libraries values: {uncertified}; {qualified}"
                ));
                    continue;
                };

                let host = host.to_string();
                targets.insert(target.to_string(), (Properties { host, std }, path.clone()));
            }
        }
        targets
    }

    let in_docs = parse(ferrocene_dir, check);
    compare_both_ways(
        &targets_toml.targets,
        &in_docs,
        "qualification scope (ferrocene/doc/*/src/scope.rst)",
        check,
    );
}

// ferrocene/doc/*/src/targets/index.rst
fn check_target_overviews(
    ferrocene_dir: &PathBuf,
    targets_toml: &TargetsToml,
    check: &mut RunningCheck,
) {
    fn parse(ferrocene_dir: &Path, check: &mut RunningCheck) -> BTreeMap<String, (Std, PathBuf)> {
        let doc_dir = ferrocene_dir.join("doc");
        let mut all = BTreeMap::new();
        for manual in USER_MANUALS {
            const ANCHOR1: &str = "_qualified-targets:";
            const ANCHOR2: &str = "list-table::";

            let path = doc_dir.join(manual).join("src/targets/index.rst");
            let contents = fs::read_to_string(&path).unwrap();

            let mut lines = contents.lines();

            advance_until_anchor(&path, &mut lines, ANCHOR1);
            advance_until_anchor(&path, &mut lines, ANCHOR2);

            // look for this pattern:
            //     * - :ref:`x86_64-unknown-linux-gnu`
            //       - (..)
            //       - [Cross-compilation|Host platform]
            //       - [Full|Bare-metal]
            while let Some(line) = lines.next() {
                if line.trim().starts_with("--") {
                    // stop at start of next section
                    break;
                }

                // * - :ref:`x86_64-unknown-linux-gnu`
                let Some(rest) = strip_prefixes(line, &["*", "-", ":ref:", "`"]) else { continue };
                let Some(tuple) = rest.strip_suffix('`') else { continue };

                // - (..)
                _ = lines.next();

                // - [Cross-compilation|Host platform]
                let Some(line) = lines.next() else { continue };
                let Some(kind) = strip_prefixes(line, &["-"]) else { continue };

                const KIND_TARGET: &str = "Cross-compilation";
                const KIND_HOST: &str = "Host platform";

                if kind != KIND_TARGET && kind != KIND_HOST {
                    check.error(format!(
                        "unexpected 'Kind' column value '{kind}' in {}",
                        path.display()
                    ));
                    continue;
                }

                // - [Full|Bare-metal]
                let Some(line) = lines.next() else { continue };
                let Some(std) = strip_prefixes(line, &["-"]) else { continue };

                const STD_FULL: &str = "Full";
                const STD_BARE_METAL: &str = "Bare-metal";

                let std = if std == STD_FULL {
                    Std::Full
                } else if std == STD_BARE_METAL {
                    Std::BareMetal
                } else {
                    check.error(format!(
                        "unexpected 'Standard library' column value '{std}' in {}",
                        path.display()
                    ));
                    continue;
                };

                all.insert(tuple.to_string(), (std, path.clone()));
            }
        }

        all
    }

    let in_docs = parse(ferrocene_dir, check);
    let in_toml = targets_toml
        .targets
        .iter()
        .map(|(target, props)| (target.clone(), props.std))
        .chain(
            targets_toml
                .hosts
                .iter()
                .cloned()
                .filter(|host| !targets_toml.targets.contains_key(host))
                .map(|host| (host, Std::Full)),
        )
        .collect();
    compare_both_ways(
        &in_toml,
        &in_docs,
        "compilation targets overview (ferrocene/doc/*/src/targets/index.rst)",
        check,
    );
}

fn check_packages_toml(
    ferrocene_dir: &PathBuf,
    targets_toml: &TargetsToml,
    check: &mut RunningCheck,
) {
    // ferrocene/packages.toml
    fn in_docs(ferrocene_dir: &Path) -> HostsAndTargets {
        #[derive(Deserialize)]
        struct PackagesToml {
            groups: Groups,
        }

        #[derive(Deserialize)]
        struct Groups {
            hosts: Group,
            #[serde(rename = "cross-compilation")]
            cross_compilation: Group,
            qnx: Group,
        }

        #[derive(Deserialize)]
        struct Group {
            targets: BTreeSet<String>,
        }

        let path = ferrocene_dir.join("packages.toml");
        let contents = fs::read_to_string(&path).unwrap();
        let PackagesToml { groups: Groups { hosts, cross_compilation, qnx } } =
            toml::from_str(&contents).unwrap();

        let mut targets = BTreeSet::new();
        for tuple in cross_compilation.targets.into_iter().chain(qnx.targets) {
            assert!(
                targets.insert(tuple.clone()),
                "found the same target `{tuple}` in different groups at {}",
                path.display()
            );
        }

        HostsAndTargets { hosts: hosts.targets, targets }
    }

    let in_docs = in_docs(ferrocene_dir);
    // one way comparisons because Quality Managed and Supported targets are
    // also listed in packages.toml
    {
        let in_docs = in_docs.hosts;
        let in_toml = &targets_toml.hosts;
        compare_one_way(
            in_toml,
            &in_docs,
            "[groups.hosts] package metadata (ferrocene/packages.toml)",
            check,
        );
    }

    // being in `groups.hosts` implies that rust-std is present for that tuple so
    // a tuple in `groups.hosts` may not appear in `groups.{cross-compilation,qnx}`
    // hence we use `(target - hosts)` as the reference
    {
        let in_docs = in_docs.targets;
        let in_toml = targets_toml
            .targets
            .keys()
            .cloned()
            .filter(|target| !targets_toml.hosts.contains(target))
            .collect();
        compare_one_way(
            &in_toml,
            &in_docs,
            "[groups.*] package metadata (ferrocene/packages.toml)",
            check,
        );
    }
}

// ferrocene/doc/target-names.toml
fn check_target_names_toml(
    ferrocene_dir: PathBuf,
    targets_toml: &TargetsToml,
    mut check: RunningCheck,
) {
    fn parse(ferrocene_dir: &Path) -> BTreeSet<String> {
        let path = ferrocene_dir.join("doc/target-names.toml");
        let contents = fs::read_to_string(&path).unwrap();
        let table: toml::Table = toml::from_str(&contents).unwrap();
        table.keys().cloned().collect()
    }

    let in_docs = parse(&ferrocene_dir);
    let in_toml =
        targets_toml.targets.keys().cloned().chain(targets_toml.hosts.iter().cloned()).collect();
    // one way comparison because Quality Managed and Supported targets are also listed in
    // target-names.toml
    compare_one_way(
        &in_toml,
        &in_docs,
        "target names (ferrocene/doc/target-names.toml)",
        &mut check,
    );
}

fn compare_both_ways<T>(
    in_toml: &BTreeMap<String, T>,
    in_docs: &BTreeMap<String, (T, PathBuf)>,
    location: &str,
    check: &mut RunningCheck,
) where
    T: PartialEq + fmt::Debug,
{
    for target in in_toml.keys() {
        if !in_docs.contains_key(target) {
            check.error(format!(
                "target `{target}` was specified in ferrocene/targets.toml but it's not present in {location}"
            ))
        }
    }

    for (target, (docs_property, location)) in in_docs {
        if let Some(toml_property) = in_toml.get(target) {
            if docs_property != toml_property {
                check.error(format!(
                    "target `{target}` was specified as '{toml_property:?}' in ferrocene/targets.toml but doc in {} states '{docs_property:?}'",
                    location.display()
                ));
            }
        } else {
            check.error(format!(
                "target `{target}` was found in {} but it's not specified in ferrocene/targets.toml",
                location.display()
            ))
        }
    }
}

fn compare_one_way(
    in_toml: &BTreeSet<String>,
    in_docs: &BTreeSet<String>,
    location: &str,
    check: &mut RunningCheck,
) {
    for tuple in in_toml {
        if !in_docs.contains(tuple) {
            check.error(format!(
                "target `{tuple}` was specified in ferrocene/targets.toml but it's not present in {location}"
            ))
        }
    }
}

fn advance_until_anchor(path: &PathBuf, lines: &mut str::Lines<'_>, anchor: &str) {
    let mut found_anchor = false;
    while let Some(line) = lines.next() {
        let Some(markup) = strip_prefixes(line, &[".."]) else { continue };

        if markup == anchor {
            found_anchor = true;
            break;
        }
    }
    assert!(found_anchor, "did not found anchor '{anchor}' in {}", path.display());
}

fn strip_prefixes<'a>(mut input: &'a str, prefixes: &[&str]) -> Option<&'a str> {
    for prefix in prefixes {
        let rest = input.trim().strip_prefix(prefix)?;
        input = rest.trim();
    }
    Some(input.trim())
}

fn extend_with_targets_in_dir(dir: &Path, map: &mut BTreeMap<String, PathBuf>) {
    for entry in fs::read_dir(&dir).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().unwrap_or_default() != "rst" {
            continue;
        }
        let stem = path.file_stem().unwrap();
        if stem == "index" {
            continue;
        }
        let target = stem.to_string_lossy().into_owned();
        assert!(
            map.insert(target.clone(), path).is_none(),
            "target `{target}` is in multiple manuals but this is not currently supported; update this tidy check",
        );
    }
}

struct HostsAndTargets {
    hosts: BTreeSet<String>,
    targets: BTreeSet<String>,
}
