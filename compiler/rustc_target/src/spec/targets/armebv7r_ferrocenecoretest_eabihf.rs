use crate::spec::{LinkSelfContainedDefault, Target, TargetMetadata, crt_objects, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::armebv7r_none_eabihf::target();

    target.metadata = TargetMetadata::default();

    // libstd port
    target.families = cvs!["unix"];
    target.os = "linux".into();
    target.env = "musl".into();
    target.has_thread_local = true;

    // crt and libc are self-contained
    target.pre_link_objects_self_contained = crt_objects::all("crt1.o");
    target.link_self_contained = LinkSelfContainedDefault::True;

    target
}
