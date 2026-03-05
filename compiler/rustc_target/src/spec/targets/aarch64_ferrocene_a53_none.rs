use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_a53_none::target();

    target.ferrocene_certified_runtime();

    target
}
