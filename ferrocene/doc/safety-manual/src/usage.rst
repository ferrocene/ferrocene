.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

Build tools listed below can be used individually or through a Ferrocene
project. File formats are documented in :doc:`User Manual - File Formats
<user-manual:build/file-formats>`.

Cleaning the Build Space
------------------------

In order to prevent the use of out-of-date compilation results, users shall
ensure that the build space (object and library directories) is empty before
compiling.

If a compilation was performed using `rustc` directly, then the user shall
delete any previous compilation artifacts, as follows:

* `exec` - Executables, where `exec` is the name of an executable.

* `libname.a` - Native static libraries, where `name` is the name of a library.

* `libname.rlib` - Rust static libraries, where `name` is the name of a library.

* `libname.so` - Native dynamic libraries, where `name` is the name of a
  library.


Warnings and Errors
-------------------

The user shall pass command line option :cli:option:`-D warnings <rustc -D
<lints>>` to the Ferrocene rustc in order to treat warnings as errors. The
user shall **not** employ command line options :cli:option:`-A <rustc -A
<lints>>` and :cli:option:`-W <rustc -W <lints>>` as those options lower the
severity of errors.


Building a Library
------------------

Refer to the User Manual - :doc:`user-manual:build/library`.

Building an Executable
----------------------

Refer to the User Manual - :doc:`user-manual:build/executable`.

Building Mixed-language Programs
--------------------------------

Refer to the User Manual - :doc:`user-manual:build/mixed-language`.

Performing System Calls
-----------------------

Refer to the User Manual - :doc:`user-manual:build/system-calls`.

