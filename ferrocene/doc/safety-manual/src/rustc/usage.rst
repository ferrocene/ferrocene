.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Usage
=====

Build tools listed below can be used individually or through a Ferrocene
project. File formats are documented in :doc:`User Manual - File Formats
<user-manual:rustc/file-formats>`.

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

* `libname.rmeta` - Rust metadata file, where `name` is the name of a library.

* `libname.so` - Native dynamic libraries, where `name` is the name of a
  library.


Building a Library
------------------

Refer to the User Manual - :doc:`user-manual:rustc/library`.

Building an Executable
----------------------

Refer to the User Manual - :doc:`user-manual:rustc/executable`.

Building Mixed-language Programs
--------------------------------

Refer to the User Manual - :doc:`user-manual:rustc/mixed-language`.

Performing System Calls
-----------------------

Refer to the User Manual - :doc:`user-manual:rustc/system-calls`.

