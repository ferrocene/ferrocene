// Tests that empty source_maps don't ICE (#23301)

//@ compile-flags: --cfg ""

pub fn main() {
}

<<<<<<< HEAD
// ferrocene-annotations: um_rustc_cfg
=======
//~? ERROR invalid `--cfg` argument: `""` (expected `key` or `key="value"`)
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
