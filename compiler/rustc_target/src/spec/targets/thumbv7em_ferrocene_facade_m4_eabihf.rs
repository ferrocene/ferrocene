use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_ferrocene_facade_eabihf::target();

    target.cpu = "cortex-m4".into();

    target
}
