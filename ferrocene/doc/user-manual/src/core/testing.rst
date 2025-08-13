.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing with the certified core library
=======================================

When using the certified core library, run tests exactly how you'd run tests with the normal Rust core library,
by running ``cargo test``.

You can use uncertified code in your tests, but need to make sure that you're only
using code within the certified subset in production.
