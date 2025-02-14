use crate::spec::{Target, TargetMetadata};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabi::target();

    target.metadata = TargetMetadata::default();
    target.metadata.description = Some("Armv7E-M bare-metal (soft-float) for Cortex-M4 CPUs".into());

    target
}
