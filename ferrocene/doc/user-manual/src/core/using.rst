.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using the certified libraries
=============================

Use the certified libraries exactly how you'd use the normal Rust standard libraries.

Required compiler flags
-----------------------

The certified libraries require the use of the ``-C panic=abort`` compiler flag.
Refer to :doc:`the safety manual <safety-manual:core/usage>`.

Building an executable
----------------------

Follow the Ferrocene rustc docs on how to :doc:`build an executable </rustc/executable>`.

Using the certified libraries in an existing Rust project
---------------------------------------------------------

To use the certified libraries in an existing project, make sure you're using only the permitted functions according to :doc:`safety-manual:core/subset`.
