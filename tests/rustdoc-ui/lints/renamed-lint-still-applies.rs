<<<<<<< HEAD
// @compile-args: --crate-type lib
||||||| 55e86c99680
// compile-args: --crate-type lib
=======
//@ check-pass
// compile-args: --crate-type lib
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
#![deny(broken_intra_doc_links)]
//~^ WARNING renamed to `rustdoc::broken_intra_doc_links`

#![deny(rustdoc::non_autolinks)]
//~^ WARNING renamed to `rustdoc::bare_urls`
