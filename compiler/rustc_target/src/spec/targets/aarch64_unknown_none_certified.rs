use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_none::target();

    target.metadata.host_tools = Some(false);
    target.metadata.tier = None;
    target.metadata.std = Some(false);

    target
}
