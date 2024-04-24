<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: The Ferrocene Developers -->

# traceability-matrix

The traceability matrix is part of our qualification material. You can find it at https://public-docs.ferrocene.dev/main/qualification/traceability-matrix.html.

In the overview it states that it is:

> A matrix that ties the specifics of the Ferrocene Language Specification to the test suite. This was written to ensure that all documented requirements are covered.

While it says "written" it is (fortunately) generated semi-automatically.

How does it work?

On the one hand, we have the Ferrocene Language Specification (FLS). Simply speaking, the FLS is a list of statements, which are grouped in sections and subsections. Each section, subsection and statement has a randombly-generated unique id, examples being `fls_wt81sbsecmu0` and `fls_3xvm61x0t251`.

On the other hand we have tests. We need to find at least one test for each section from the FLS. This test is then annotated with the id of the statement. This annotating is happening through a comment which is getting added to the end of the file. It is possible to annotate one test with multiple ids.

Currently we annotate tests in `tests/ui` (for the FLS) and `tests/run-make` (for the command-line interface).

For example `tests/ui/borrowck/borrowck-break-uninit-2.rs` got annotated with `fls_3xvm61x0t251` ([file at c80de1f](https://github.com/ferrocene/ferrocene/blob/c80de1fa7eecdfbe4579b13e2aaa93fb0586f9c6/tests/ui/borrowck/borrowck-break-uninit-2.rs)):

```rust
fn foo() -> isize {
    let x: isize;

    while 1 != 2  {
        break;
        x = 0;
    }

    println!("{}", x); //~ ERROR E0381

    return 17;
}

fn main() { println!("{}", foo()); }

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
```

It is also possible to annotate bulks of tests. This happens through `ferrocene-annotations` files, which basically contain the same comments. For example `tests/ui/borrowck/ferrocene-annotations` contains three ids:

```
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_411up5z0b6n6
// Lexical Elements
//
// ferrocene-annotations: fls_fgnllgz5k3e6
// Lexical Elements, Separators, and Punctuation
```

This annotates all the tests in `tests/ui/borrowck/` with those three ids.

When using bulk annotation, individual tests can still be annotated with additional ids, as is the case for `tests/ui/borrowck/borrowck-break-uninit-2.rs`. It is annotated with four ids. The three from `tests/ui/borrowck/ferrocene-annotations` and one from the file itself.

All of this was manual so far, so where does the automation come in?

The automation is in the `traceability-matrix` tool (`ferrocene/tools/traceability-matrix`) and `bootstrap`.

The `./x doc ferrocene/tools/traceability-matrix` (or `./x run` to be precise) implementation utilizes `compiletest` to go through all the tests in `tests/ui` and `tests/run-make` and collect the annotations from there. This required patching compiletest. It generates two json files in `build/x86_64-unknown-linux-gnu/ferrocene/test-annotations/` (one for each test suite).

One of them is `tests-ui.json`:

```json
{
    "bulk_annotations_file_name": "ferrocene-annotations",
    "tests": [
        // <...>
        {
            "file": "/home/urhengulas/Documents/github.com/ferrocene/ferrocene/tests/ui/borrowck/borrowck-break-uninit-2.rs",
            "annotations": [
                {
                    "id": "fls_3xvm61x0t251",
                    "file": "/home/urhengulas/Documents/github.com/ferrocene/ferrocene/tests/ui/borrowck/borrowck-break-uninit-2.rs"
                },
                {
                    "id": "fls_a14slch83hzn",
                    "file": "/home/urhengulas/Documents/github.com/ferrocene/ferrocene/tests/ui/borrowck/ferrocene-annotations"
                },
                {
                    "id": "fls_411up5z0b6n6",
                    "file": "/home/urhengulas/Documents/github.com/ferrocene/ferrocene/tests/ui/borrowck/ferrocene-annotations"
                },
                {
                    "id": "fls_fgnllgz5k3e6",
                    "file": "/home/urhengulas/Documents/github.com/ferrocene/ferrocene/tests/ui/borrowck/ferrocene-annotations"
                }
            ]
        },
        // <...>
    ]
}
```

The top-level object only contains two fields. The first is `bulk_annotations_file_name` which is the name of the file to annotate bulks of tests. The second is `tests` which is the interesting part, a list of test objects. There is one test object for each test file. A test object contains `file`, which is the path of the file, and `annotations`, which is a list of one or more annotation objects. A annotation object contains the `id` from the FLS and the `file` path the annotation is coming from. This is either the test file itself, or a bulk annotation file.

The `traceability-matrix` tool then picks up the json files and generates the HTML report from it. Tada 🎉
