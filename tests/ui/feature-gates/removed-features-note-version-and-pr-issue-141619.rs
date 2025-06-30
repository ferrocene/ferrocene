<<<<<<< HEAD
//@ normalize-stderr: "\(you are using [0-9]\.[0-9]+\.[0-9]+(.+)\)" -> "(you are using $$RUSTC_VERSION)"

=======
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
#![feature(external_doc)] //~ ERROR feature has been removed
#![doc(include("README.md"))] //~ ERROR unknown `doc` attribute `include`

fn main(){}
