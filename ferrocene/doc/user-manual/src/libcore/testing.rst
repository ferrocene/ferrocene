Testing libcore
===============

Test the certified subset of Libcore exactly how you'd use test Rust Libcore,
by running ``cargo test``.

You can use uncertified code in your tests, but need to make sure that you're only
using code within the certified subset in production.
