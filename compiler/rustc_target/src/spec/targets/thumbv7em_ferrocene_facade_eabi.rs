use crate::spec::{Env, LinkSelfContainedDefault, Os, Target, TargetMetadata, crt_objects, cvs};

pub(crate) fn target() -> Target {
    let mut target = super::thumbv7em_none_eabi::target();

    target.metadata = TargetMetadata::default();

    // libstd port
    target.families = cvs!["unix"];
    target.os = Os::Linux;
    target.env = Env::Musl;
    target.has_thread_local = true;

    // crt and libc are self-contained
    target.pre_link_objects_self_contained = crt_objects::all("crt1.o");
    target.link_self_contained = LinkSelfContainedDefault::True;

    target
}
