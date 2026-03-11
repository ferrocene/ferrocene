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
static VARIANTS: &[(&str, TestVariantBase)] = &[
    // FIXME: all of these point to edition 2015 instead of 2021!!!

    // The snippet between INTERNAL_PROCEDURES_{START,END}_TEST_VARIANTS is included in our
    // documentation, as the list of test variants currently supported.

    // INTERNAL_PROCEDURES_START_TEST_VARIANTS
    ("2021", TestVariantBase::new().edition("2015")),
    ("2021-cortex-a53", TestVariantBase::new().edition("2015").qemu_cpu("cortex-a53")),
    ("2021-neoverse-v1", TestVariantBase::new().edition("2015").qemu_cpu("neoverse-v1")),
    ("2021-cortex-m4", TestVariantBase::new().edition("2015").qemu_cpu("cortex-m4")),
    // INTERNAL_PROCEDURES_END_TEST_VARIANTS
];
static DEFAULT_VARIANTS_BY_TARGET: &[(&str, &str)] = &[
    ("aarch64-unknown-ferrocene.facade", "2021-cortex-a53"),
    ("thumbv7em-ferrocene.facade-eabi", "2021-cortex-m4"),
    ("thumbv7em-ferrocene.facade-eabihf", "2021-cortex-m4"),
];
static DEFAULT_VARIANT_FALLBACK: &str = "2021";

#[derive(Clone)]
pub(crate) struct MaskedValue<T, B> {
    inner: T,
    mask: B,
}

impl<T> From<MaskedValue<T, ()>> for MaskedValue<T, RefCell<bool>> {
    fn from(value: MaskedValue<T, ()>) -> Self {
        Self { inner: value.inner, mask: RefCell::new(true) }
    }
}

impl<T> MaskedValue<T, RefCell<bool>> {
    pub(crate) fn get(&self) -> Option<MaskedValueRef<'_, T, RefCell<bool>>> {
        self.mask.borrow().then(|| MaskedValueRef { inner: &self.inner, mask: &self.mask })
    }
}

pub(crate) struct MaskedValueRef<'a, T, B> {
    inner: &'a T,
    mask: &'a B,
}

impl<'a, T> MaskedValueRef<'a, T, RefCell<bool>> {
    pub(crate) fn mark_unused(&self) {
        *self.mask.borrow_mut() = false;
    }
}

impl<'a, T, B> core::ops::Deref for MaskedValueRef<'a, T, B> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: core::fmt::Display, B> core::fmt::Display for MaskedValueRef<'a, T, B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Clone)]
pub(crate) struct TestVariantInner<B> {
    edition: Option<MaskedValue<&'static str, B>>,
    qemu_cpu: Option<MaskedValue<&'static str, B>>,
}

type TestVariantBase = TestVariantInner<()>;

impl TestVariantBase {
    const fn new() -> Self {
        Self { edition: None, qemu_cpu: None }
    }

    const fn edition(mut self, edition: &'static str) -> Self {
        self.edition = Some(MaskedValue { inner: edition, mask: () });
        self
    }

    const fn qemu_cpu(mut self, qemu_cpu: &'static str) -> Self {
        self.qemu_cpu = Some(MaskedValue { inner: qemu_cpu, mask: () });
        self
    }
}

pub(crate) type TestVariant = TestVariantInner<RefCell<bool>>;

impl TestVariant {
    pub(crate) fn edition(&self) -> Option<MaskedValueRef<'_, &'static str, RefCell<bool>>> {
        self.edition.as_ref()?.get()
    }

    pub(crate) fn qemu_cpu(&self) -> Option<MaskedValueRef<'_, &'static str, RefCell<bool>>> {
        self.qemu_cpu.as_ref()?.get()
    }
}

impl From<TestVariantBase> for TestVariant {
    fn from(base: TestVariantBase) -> Self {
        Self {
            edition: base.edition.clone().map(|edition| edition.into()),
            qemu_cpu: base.qemu_cpu.clone().map(|qemu_cpu| qemu_cpu.into()),
        }
    }
}

impl TestVariant {
    pub(crate) fn current(builder: &Builder<'_>, target: TargetSelection) -> Self {
        let name = match &builder.config.cmd {
            Subcommand::Test { test_variant: Some(name), .. } => name,
            _ => find_in_slice(DEFAULT_VARIANTS_BY_TARGET, &target.triple)
                .map(|s| *s)
                .unwrap_or(DEFAULT_VARIANT_FALLBACK),
        };

        let base = find_in_slice(VARIANTS, name).unwrap_or_else(|| {
            panic!(
                "unknown test variant: {name}\nexpected one of {:?}",
                VARIANTS.iter().map(|(k, _)| k).collect::<Vec<_>>()
            )
        });

        base.clone().into()
    }

    #[cfg(feature = "build-metrics")]
    pub(crate) fn id(&self) -> String {
        let mut fragments = vec![];

        if let Some(edition) = self.edition() {
            fragments.push(format!("e{edition}"));
        }
        if let Some(qemu_cpu) = self.qemu_cpu() {
            fragments.push(format!("q{qemu_cpu}"));
        }

        match fragments.into_iter().reduce(|acc, val| format!("{acc}-{val}")) {
            Some(id) => id,
            None => "empty".to_owned(),
        }
    }

    #[cfg(feature = "build-metrics")]
    pub(crate) fn for_metrics(&self) -> FerroceneVariantMetadata {
        let mut fields = BTreeMap::new();

        if let Some(edition) = self.edition() {
            fields.insert("Edition".into(), edition.to_string());
        }
        if let Some(qemu_cpu) = self.qemu_cpu() {
            fields.insert("Emulated CPU".into(), qemu_cpu.to_string());
        }

        FerroceneVariantMetadata { id: self.id(), human_readable_fields: fields }
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
