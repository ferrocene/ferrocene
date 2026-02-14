use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64v8r_unknown_none::target();
    target.options.cpu = "cortex-r82".into();
    target
}
