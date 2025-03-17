//@ ignore-apple On Mac, signatures cause the binaries to differ
// ferrocene-annotations: um_rustc_C_metadata

// From rustc documentation:
//
// This option allows you to control the metadata used for symbol mangling. This
// takes a space-separated list of values. Mangled symbols will incorporate a
// hash of the metadata. This may be used, for example, to differentiate symbols
// between two different versions of the same crate being linked.
//
// What we test:
//
// Ensure files with same metadata are equal and files with different metadata
// are different.
//
// Other non-documented rules, which are clear by looking at the code in charge
// of generating a stable crate id:
//
// Metadata is a list of values. The order of such values does not matter.
// So ["one, two"] and ["two", "one"] are equivalent
//
// Metadata values can be added with a single -Cmetadata option, which takes
// a string of space-separated values.
//
// Metadata values can also be added by multiple -Cmetadata options.
//
// Every distinct -C metadata value is only incorporated once
//
// So these are equivalent:
//
// -Cmetadata=one -Cmetadata=two
// -Cmetadata="one two"
// -Cmetadata="two one"
// -Cmetadata="one two one"
//
// Only space is a valid delimiter. Unlike other codegen options, comma is
// treated as any other character.
//
// A single metadata "ab" has a different meaning than two separated "a" "b"

use run_make_support::rfs::read;
use run_make_support::{run_in_tmpdir, rustc, target};

fn main() {
    let one = compile_and_read(&["-Cmetadata=one"]);
    let one_comma_two = compile_and_read(&["-Cmetadata=one,two"]);
    let one_separator_two = compile_and_read(&["-Cmetadata=one", "-Cmetadata=two"]);
    let one_space_two = compile_and_read(&["-Cmetadata=one two"]);
    let one_space_two_separator_one = compile_and_read(&["-Cmetadata=one two", "-Cmetadata=one"]);
    let onetwo = compile_and_read(&["-Cmetadata=onetwo"]);
    let two_separator_one = compile_and_read(&["-Cmetadata=two", "-Cmetadata=one"]);
    let two_space_one = compile_and_read(&["-Cmetadata=two one"]);

    assert!(one != one_space_two);
    assert!(one_space_two != one_comma_two);
    assert!(one_space_two != onetwo);
    assert!(one_space_two == one_separator_two);
    assert!(one_space_two == one_space_two_separator_one);
    assert!(one_space_two == two_separator_one);
    assert!(one_space_two == two_space_one);
}

fn compile_and_read(extra_flags: &[&str]) -> Vec<u8> {
    let mut content = None;
    run_in_tmpdir(|| {
        rustc().target(target()).input("main.rs").crate_type("bin").args(extra_flags).run();
        content = Some(read("main"));
    });
    content.unwrap()
}
