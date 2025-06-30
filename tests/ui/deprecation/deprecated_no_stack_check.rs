<<<<<<< HEAD
//@ normalize-stderr: "\(you are using [0-9]\.[0-9]+\.[0-9]+(.+)\)" -> "(you are using $$RUSTC_VERSION)"

=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
#![deny(warnings)]
#![feature(no_stack_check)]
//~^ ERROR: feature has been removed [E0557]
fn main() {

}
