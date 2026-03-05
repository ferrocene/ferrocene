use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();

    target.cpu = "cortex-a53".into();

    target
}
