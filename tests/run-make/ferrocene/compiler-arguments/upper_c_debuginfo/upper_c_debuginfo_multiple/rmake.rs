//@ ignore-apple On Mac, signatures mean that the diff'd files are not equal when we expect them to
//               be.

// Multiple debuginfo arguments are passed with different options.
// The last one overrides the previous.

use std::path::Path;

use run_make_support::{rfs, rustc, target};

fn main() {
    let out_dir = Path::new("out");
    rfs::create_dir_all(&out_dir);

    // Compile with a single debuginfo flag
    let debug_0 = out_dir.join("0");
    rustc().target(target()).input("main.rs").debuginfo("0").output(&debug_0).run();
    let debug_2 = out_dir.join("2");
    rustc().target(target()).input("main.rs").debuginfo("2").output(&debug_2).run();

    // Compile with multiple debuginfo flags
    let debug_02 = out_dir.join("02");
    rustc().target(target()).input("main.rs").debuginfo("0").debuginfo("2").output(&debug_02).run();
    let debug_20 = out_dir.join("20");
    rustc().target(target()).input("main.rs").debuginfo("2").debuginfo("0").output(&debug_20).run();

    // Compare build artifacts, they must be equal
    assert_eq!(rfs::read(&debug_2), rfs::read(&debug_02));
    assert_eq!(rfs::read(&debug_0), rfs::read(&debug_20));

    assert_ne!(rfs::read(&debug_02), rfs::read(&debug_20));
}

// ferrocene-annotations: um_rustc_C_debuginfo
