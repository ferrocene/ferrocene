// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

//! Test variants are a mechanism to run test suites for a target multiple times, each time varying
//! some of the parameters passed to the suite.
//!
//! Every time a test suite is called, `TestVariant::current` should be called to determine the
//! current test variant (based on the CLI argument --test-variant, or if missing, the default).
//! Then, the code should iterate through all the conditions and apply them:
//!
//! ```rust
//! let variant = TestVariant::current(builder, target);
//! for condition in variant.conditions() {
//!     match condition.get() {
//!         // Replace those with the actual conditions:
//!         VariantCondition::Foo(content) => do_something_with(content),
//!         VariantCondition::Bar(_) => condition.mark_unused(),
//!     }
//! }
//! ```
//!
//! Then later, `variant.for_metrics()` should be passed to the test suite representation in the
//! build metrics, so that the variant gets propagated to the traceability matrix.
//!
//! **Note:** it is imperative that all conditions are applied to each test suite. If a condition is
//! not applicable you must call `condition.mark_unused()` to mark it as such.

use std::cell::RefCell;
#[cfg(feature = "build-metrics")]
use std::collections::BTreeMap;

#[cfg(feature = "build-metrics")]
use build_helper::metrics::FerroceneVariantMetadata;

use crate::Subcommand;
use crate::builder::Builder;
use crate::core::config::TargetSelection;

// The variants are so few that setting up an OnceLock to define a global hashmap is too
// much complexity. If we are able to test so many variants that this loop becomes a
// bottleneck I guess congrats on Ferrocene's success :D
#[rustfmt::skip]
static VARIANTS: &[(&str, &[VariantCondition])] = &[
    // FIXME: all of these point to edition 2015 instead of 2021!!!

    // The snippet between INTERNAL_PROCEDURES_{START,END}_TEST_VARIANTS is included in our
    // documentation, as the list of test variants currently supported.

    // INTERNAL_PROCEDURES_START_TEST_VARIANTS
    ("2021", &[
        VariantCondition::Edition("2021"),
    ]),
    ("2021-cortex-a53", &[
        VariantCondition::Edition("2021"),
        VariantCondition::QemuCpu("cortex-a53"),
    ]),
    ("2021-cortex-m4", &[
        VariantCondition::Edition("2021"),
        VariantCondition::QemuCpu("cortex-m4"),
    ]),
    // INTERNAL_PROCEDURES_END_TEST_VARIANTS
];
static DEFAULT_VARIANTS_BY_TARGET: &[(&str, &str)] = &[
    ("aarch64-unknown-ferrocene.facade", "2021-cortex-a53"),
    ("thumbv7em-ferrocene.facade-eabi", "2021-cortex-m4"),
    ("thumbv7em-ferrocene.facade-eabihf", "2021-cortex-m4"),
];
static DEFAULT_VARIANT_FALLBACK: &str = "2021";

pub(crate) enum VariantCondition {
    Edition(&'static str),
    QemuCpu(&'static str),
}

#[derive(Clone)]
pub(crate) struct TestVariant {
    base: &'static [VariantCondition],
    masks: RefCell<Vec<bool>>,
}

impl TestVariant {
    pub(crate) fn current(builder: &Builder<'_>, target: TargetSelection) -> Self {
        let name = match &builder.config.cmd {
            Subcommand::Test { test_variant: Some(name), .. } => name,
            _ => find_in_slice(DEFAULT_VARIANTS_BY_TARGET, &target.triple)
                .map(|s| *s)
                .unwrap_or(DEFAULT_VARIANT_FALLBACK),
        };

        let base = find_in_slice(VARIANTS, name).expect(&format!("unknown test variant: {name}"));
        TestVariant { masks: RefCell::new(vec![true; base.len()]), base }
    }

    pub(crate) fn condititions(&self) -> impl Iterator<Item = VariantConditionAccessor<'_>> {
        self.base
            .iter()
            .enumerate()
            .filter(|(idx, _)| self.masks.borrow()[*idx])
            .map(|(index, condition)| VariantConditionAccessor { parent: self, condition, index })
    }

    #[cfg(feature = "build-metrics")]
    pub(crate) fn id(&self) -> String {
        let mut id = String::new();
        for condition in self.condititions() {
            if !id.is_empty() {
                id.push('-');
            }
            match condition.get() {
                VariantCondition::Edition(edition) => id.push_str(&format!("e{edition}")),
                VariantCondition::QemuCpu(cpu) => id.push_str(&format!("q{cpu}")),
            }
        }
        if id.is_empty() { "empty".into() } else { id }
    }

    #[cfg(feature = "build-metrics")]
    pub(crate) fn for_metrics(&self) -> FerroceneVariantMetadata {
        let mut fields = BTreeMap::new();
        for condition in self.condititions() {
            match condition.get() {
                VariantCondition::Edition(edition) => {
                    fields.insert("Edition".into(), edition.to_string());
                }
                VariantCondition::QemuCpu(cpu) => {
                    fields.insert("Emulated CPU".into(), cpu.to_string());
                }
            }
        }
        FerroceneVariantMetadata { id: self.id(), human_readable_fields: fields }
    }
}

pub(crate) struct VariantConditionAccessor<'a> {
    parent: &'a TestVariant,
    condition: &'a VariantCondition,
    index: usize,
}

impl VariantConditionAccessor<'_> {
    pub(crate) fn get(&self) -> &VariantCondition {
        self.condition
    }

    pub(crate) fn mark_unused(&self) {
        self.parent.masks.borrow_mut()[self.index] = false;
    }
}

fn find_in_slice<'a, T>(slice: &'a [(&str, T)], expected: &str) -> Option<&'a T> {
    for (key, value) in slice {
        if *key == expected {
            return Some(value);
        }
    }
    None
}
