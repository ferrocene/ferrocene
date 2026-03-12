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
use crate::utils::exec::BootstrapCommand;

#[derive(Clone, Copy, Debug, clap::ValueEnum)]
pub enum TestVariantName {
    #[clap(name = "2021")]
    Ed2021,
    #[clap(name = "2021-cortex-a53")]
    Ed2021CortexA53,
    #[clap(name = "2021-neoverse-v1")]
    Ed2021NeoverseV1,
    #[clap(name = "2021-cortex-m4")]
    Ed2021CortexM4,
}

impl TestVariantName {
    const fn base(&self) -> TestVariantBase {
        // FIXME: all of these point to edition 2015 instead of 2021!!!

        // The snippet between INTERNAL_PROCEDURES_{START,END}_TEST_VARIANTS is included in our
        // documentation, as the list of test variants currently supported.

        match self {
            // INTERNAL_PROCEDURES_START_TEST_VARIANTS
            Self::Ed2021 => TestVariantBase::new().edition(Edition("2015")),
            Self::Ed2021CortexA53 => {
                TestVariantBase::new().edition(Edition("2015")).qemu_cpu(QemuCpu("cortex-a53"))
            }
            Self::Ed2021NeoverseV1 => {
                TestVariantBase::new().edition(Edition("2015")).qemu_cpu(QemuCpu("neoverse-v1"))
            }
            Self::Ed2021CortexM4 => {
                TestVariantBase::new().edition(Edition("2015")).qemu_cpu(QemuCpu("cortex-m4"))
            } // INTERNAL_PROCEDURES_END_TEST_VARIANTS
        }
    }

    fn default_by_target(target: &str) -> Self {
        match target {
            "aarch64-unknown-ferrocene.facade" => Self::Ed2021CortexA53,
            "thumbv7em-ferrocene.facade-eabi" | "thumbv7em-ferrocene.facade-eabihf" => {
                Self::Ed2021CortexM4
            }
            _ => Self::Ed2021,
        }
    }
}

macro_rules! define_conditions {
    ($($name:ident : $ty:ty,)*) => {
        struct TestVariantBase {
            $($name: Option<$ty>,)*
        }

        impl TestVariantBase {
            const fn new() -> Self {
                Self {
                    $($name: None,)*
                }
            }

            $(
                const fn $name(mut self, value: $ty) -> Self {
                    self.$name = Some(value);
                    self
                }
            )*
        }

        pub(crate) struct TestVariant {
            $($name: Option<MaskedValue<$ty>>,)*
        }

        impl TestVariant {
            fn new(base: TestVariantBase) -> Self {
                Self {
                    $($name: base.$name.map(MaskedValue::new),)*
                }
            }

            $(
                pub(crate) fn $name(&self) -> Option<MaskedValueRef<'_, $ty>> {
                    self.$name.as_ref()?.get()
                }
            )*

            #[cfg(feature = "build-metrics")]
            pub(crate) fn id(&self) -> String {
                let mut fragments = vec![];

                $(
                    if let Some(value) = self.$name() {
                        fragments.push(format!("{}{}", <$ty as TestCondition>::PREFIX, TestCondition::display(&*value)));
                    }
                )*

                match fragments.into_iter().reduce(|acc, val| format!("{acc}-{val}")) {
                    Some(id) => id,
                    None => "empty".to_owned(),
                }
            }

            #[cfg(feature = "build-metrics")]
            pub(crate) fn for_metrics(&self) -> FerroceneVariantMetadata {
                let mut fields = BTreeMap::new();

                $(
                    if let Some(value) = self.$name() {
                        fields.insert(<$ty as TestCondition>::READABLE_NAME.into(), TestCondition::display(&*value).to_string());
                    }
                )*

                FerroceneVariantMetadata { id: self.id(), human_readable_fields: fields }
            }
        }

    };
}

define_conditions! {
    edition: Edition,
    qemu_cpu: QemuCpu,
}

pub(crate) trait TestCondition {
    const PREFIX: &'static str;
    const READABLE_NAME: &'static str;

    fn apply(&self, cmd: &mut BootstrapCommand);

    fn display(&self) -> impl core::fmt::Display;
}

pub(crate) struct Edition(&'static str);

impl TestCondition for Edition {
    const PREFIX: &'static str = "e";
    const READABLE_NAME: &'static str = "Edition";

    fn apply(&self, cmd: &mut BootstrapCommand) {
        cmd.arg(&format!("--edition={}", self.0));
    }

    fn display(&self) -> impl core::fmt::Display {
        self.0
    }
}

pub(crate) struct QemuCpu(&'static str);

impl TestCondition for QemuCpu {
    const PREFIX: &'static str = "q";
    const READABLE_NAME: &'static str = "Emulated CPU";

    fn apply(&self, cmd: &mut BootstrapCommand) {
        cmd.env("QEMU_CPU", self.0);
    }

    fn display(&self) -> impl core::fmt::Display {
        self.0
    }
}

#[derive(Clone)]
pub(crate) struct MaskedValue<T> {
    inner: T,
    mask: RefCell<bool>,
}

impl<T> MaskedValue<T> {
    const fn new(inner: T) -> Self {
        Self { inner, mask: RefCell::new(true) }
    }
    fn get(&self) -> Option<MaskedValueRef<'_, T>> {
        self.mask.borrow().then(|| MaskedValueRef { inner: &self.inner, mask: &self.mask })
    }
}

pub(crate) struct MaskedValueRef<'a, T> {
    inner: &'a T,
    mask: &'a RefCell<bool>,
}

impl<'a, T> MaskedValueRef<'a, T> {
    pub(crate) fn mark_unused(&self) {
        *self.mask.borrow_mut() = false;
    }
}

impl<'a, T> core::ops::Deref for MaskedValueRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: core::fmt::Display> core::fmt::Display for MaskedValueRef<'a, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl TestVariant {
    pub(crate) fn current(builder: &Builder<'_>, target: TargetSelection) -> Self {
        let name = match &builder.config.cmd {
            Subcommand::Test { test_variant: Some(name), .. } => *name,
            _ => TestVariantName::default_by_target(&target.triple),
        };

        TestVariant::new(name.base())
    }
}
