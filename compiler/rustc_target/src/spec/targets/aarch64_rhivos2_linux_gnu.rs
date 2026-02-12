use crate::spec::{Target, TargetMetadata};

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_linux_gnu::target();

    target.metadata = TargetMetadata::default();

    target
}
