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

The behavior of the core library is different when using a certified target. In
particular, ``panic!`` macros only support static strings, not arbitrary
``format_args!`` macros. This does not affect normal operation of your program,
but does mean that panic messages may be less informative when compiled for a
certified target.

Building an executable
----------------------

Follow the Ferrocene rustc docs on how to :doc:`build an executable </rustc/executable>`.

Using the certified core library in an existing Rust project
------------------------------------------------------------

To use the certified core library in an existing project, make sure you're using only the permitted functions according to :doc:`safety-manual:core/subset`.
