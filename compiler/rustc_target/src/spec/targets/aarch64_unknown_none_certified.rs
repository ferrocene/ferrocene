use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();

    target.metadata = Default::default();

    target
}
