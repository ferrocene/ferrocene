use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_m4_none_eabihf::target();

    target.ferrocene_certified_runtime();

    target
}
