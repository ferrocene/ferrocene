.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Using libcore
=============

Use the certified subset of Libcore exactly how you'd use standard Rust Libcore.

Required compiler flags
-----------------------

The certified libcore subset requires use of the ``-C panic=abort`` compiler flag.

Building an executable
----------------------

Follow the Ferrocene rustc docs on how to :doc:`build an executable </rustc/executable>`.

Using libcore in an existing Rust project
-----------------------------------------

To use the certified subset of libcore in an existing project, make sure you're using only the permitted functions listed
in the Libcore Certified subset spreadsheet in your source code.
