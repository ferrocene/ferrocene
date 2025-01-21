use crate::spec::{Cc, LinkerFlavor, Lld, Target, TargetMetadata, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabi::target();

    target.metadata = TargetMetadata::default();

    // libstd port
    target.families = cvs!["unix"];
    target.os = "linux".into();
    target.env = "musl".into();
    target.has_thread_local = true;

    let crt_path = "/tmp/ferrocene/thumbv7em-ferrocenecoretest-eabi/crt1.o";
    let library_path = crt_path.rsplit_once('/').unwrap().0;
    target.add_pre_link_args(LinkerFlavor::Gnu(Cc::No, Lld::No), &[crt_path, "-L", library_path]);

    target
}
