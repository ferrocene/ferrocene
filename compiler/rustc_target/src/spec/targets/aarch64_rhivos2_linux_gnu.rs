use crate::spec::Target;

pub(crate) fn target() -> Target {
    let mut target = super::aarch64_unknown_linux_gnu::target();

    target.metadata.description = Some("ARM64 Linux (kernel 4.1, glibc 2.17+, RHIVOS)".into());

    target
}
