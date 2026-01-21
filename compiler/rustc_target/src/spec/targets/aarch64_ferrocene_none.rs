use crate::spec::Target;

pub(crate) fn target() -> Target {
    let target = super::aarch64_unknown_none::target();

    target
}
