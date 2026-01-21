use crate::spec::Target;

pub(crate) fn target() -> Target {
    let target = super::thumbv7em_none_eabihf::target();

    target
}
