
use crate::spec::{cvs, Cc, LinkerFlavor, Lld, Target, TargetOptions};

pub fn target() -> Target {
    let mut target = super::armv8r_none_eabihf::target();
    target.os = "linux".into();
    target.env = "gnu".into();
    target.families = cvs!["unix"];
    target.linker_flavor = LinkerFlavor::Gnu(Cc::Yes, Lld::Yes);
    target.linker = Some("arm-none-linux-gnueabihf-gcc".into());
    target.pre_link_args = TargetOptions::link_args(
        LinkerFlavor::Gnu(Cc::Yes, Lld::No),
        &["-Wl"],
    );
    target.link_self_contained = crate::spec::LinkSelfContainedDefault::with_linker();
    target
}
