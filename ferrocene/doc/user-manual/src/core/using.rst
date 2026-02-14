.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using the core library
======================

Use the certified core library exactly how you'd use the normal core library.

Required compiler flags
-----------------------

The certified core library requires use of the ``-C panic=abort`` compiler flag.
It also requires compiling using a target with a certified core runtime.
Refer to :doc:`Certified core library <safety-manual:core/usage>`.

Building an executable
----------------------

Follow the Ferrocene rustc docs on how to :doc:`build an executable </rustc/executable>`.

Using the certified core library in an existing Rust project
------------------------------------------------------------

To use the certified core library in an existing project, make sure you're using only the permitted functions according to :doc:`safety-manual:core/subset`.
