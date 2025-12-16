use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabihf::target();

    target.ferrocene_subset();

    target
}
