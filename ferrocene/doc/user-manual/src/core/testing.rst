.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing with the certified libraries
====================================

When using the certified libraries, run tests exactly how you'd run tests with the normal Rust standard libraries,
by running ``cargo test``.

You can use uncertified code in your tests, but need to make sure that you're only
using code within the certified subset in production.
