use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabi::target();

    target.ferrocene_certified_runtime();

    target
}
