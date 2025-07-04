use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabihf::target();

    target.metadata.description =
        target.metadata.description.map(|v| format!("{v} (certified)").into());
    target.metadata.host_tools = Some(false);
    target.metadata.tier = None;
    target.metadata.std = Some(false);

    target
}
