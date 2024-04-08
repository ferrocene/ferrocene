.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

File formats
============

This chapter describes the file formats recognized by rustc.

A `source file <../../specification/glossary.html#source-file>`_ that contains
Rust code must have file extension ``rs``, for example:

.. code-block:: rust

   my_program.rs

Libraries
---------

A library is a reusable collection of non-volatile resources, such as
functionality and data, used by another library or executable.

Compiling a source file with name ``name`` using rustc
produces a library with name ``libname``, where the file extension is either
``rlib`` for a Rust static library or ``so`` for a native dynamic library.

The ``rlib`` file format is specific to rustc, and may change over time. It is
similar to a ``tar`` file, and contains roughly:

* Object code, produced by code generation,
* LLVM bitcode,
* rustc metadata,
* A symbol table.

The ``so`` file format is target-dependent. Consult the documentation of your
specific target.

Executables
-----------

An executable (or a binary) causes a computer to perform indicated tasks
according to encoded instructions.

Compiling a source file with name ``name`` using rustc
produces an executable with name ``name``, without a file extension.

The file format of an executable is target-dependent. Consult the documentation
of your specific target.
