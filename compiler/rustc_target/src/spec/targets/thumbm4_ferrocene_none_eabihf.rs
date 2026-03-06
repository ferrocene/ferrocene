use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbm4_none_eabihf::target();

    target.ferrocene_certified_runtime();

    target
}
