use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetMetadata, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabihf::target();

    target.metadata = TargetMetadata::default();

    // libstd port
    target.families = cvs!["unix"];
    target.os = "linux".into();
    target.env = "musl".into();
    target.has_thread_local = true;

    let path = "/tmp/ferrocene/thumbv7em-ferrocenecoretest-eabihf";
    target.add_pre_link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), &["-L", path]);

    target
}
