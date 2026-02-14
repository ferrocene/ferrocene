use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64v8r_unknown_none_softfloat::target();
    target.options.cpu = "cortex-r82".into();
    target
}
