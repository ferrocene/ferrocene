use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::x86_64_pc_qnx::target();
    target.metadata.description = Some("x86 64-bit QNX OS for Safety SDP 8.0+".into());
    target
}
