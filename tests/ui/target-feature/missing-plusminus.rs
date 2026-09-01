//@ compile-flags: -Ctarget-feature=banana --crate-type=rlib
//@ build-pass

<<<<<<< ferrocene/main
//~? WARN unknown feature specified for `-Ctarget-feature`: `banana`

// ferrocene-annotations: um_rustc_C_target_feature
||||||| 65dd30fb9e8
//~? WARN unknown feature specified for `-Ctarget-feature`: `banana`
=======
//~? WARN ignoring feature with missing prefix in `-Ctarget-feature`: `banana`
>>>>>>> rust-lang/rust/HEAD--generated-by-pull-upstream
