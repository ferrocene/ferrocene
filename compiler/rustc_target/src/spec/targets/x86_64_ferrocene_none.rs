use crate::spec::Target;

pub(crate) fn target() -> Target {
    let target = super::x86_64_unknown_none::target();

    target
}
