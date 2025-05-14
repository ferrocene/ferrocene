//@ compile-flags: -Ctarget-feature=banana --crate-type=rlib
//@ build-pass

//~? WARN unknown feature specified for `-Ctarget-feature`: `banana`

// ferrocene-annotations: um_rustc_C_target_feature
