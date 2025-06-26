Using libcore
=============

Use the certified subset of Libcore exactly how you'd use standard Rust Libcore.

Required compiler flags
-----------------------

The certified libcore subset requires use of the :cli:option:`-C panic=abort` compiler flag.

Building an executable
----------------------

Follow the Ferrocene rustc docs on how to :doc:`build an executable </rustc/executable>`.

Using libcore in an existing Rust project
-----------------------------------------

To use the certified subset of libcore in an existing project, make sure you're using only the permitted functions listed in TODO in your source code.
