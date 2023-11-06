<<<PULL-UPSTREAM>>> file deleted by Rust, fix the conflict and remove this line
// aux-build:lint-plugin-test.rs
// ignore-stage1
// compile-flags: -D test-lint

#![feature(plugin)]
#![plugin(lint_plugin_test)]
//~^ WARN use of deprecated attribute `plugin`

fn lintme() { } //~ ERROR item is named 'lintme'

pub fn main() {
    lintme();
}

// ferrocene-annotations: um_rustc_D
