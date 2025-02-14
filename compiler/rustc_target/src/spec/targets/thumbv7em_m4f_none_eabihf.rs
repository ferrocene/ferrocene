use crate::spec::{Target, TargetMetadata};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabihf::target();

    target.metadata = TargetMetadata::default();
    target.metadata.description = Some("Armv7E-M bare-metal (hard-float) for Cortex-M4F CPUs".into());

    target
}
