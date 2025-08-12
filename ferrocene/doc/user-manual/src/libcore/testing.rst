.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing libcore
===============

Test the certified subset of Libcore exactly how you'd test standard Rust Libcore,
by running ``cargo test``.

You can use uncertified code in your tests, but need to make sure that you're only
using code within the certified subset in production.
