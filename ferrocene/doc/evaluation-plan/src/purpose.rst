.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Purpose
=======
Ferrocene is a software development environment for Rust programming
language.

.. figure:: figures/toolchain-workflow.svg
   
   Toolchain Workflow

Ferrocene is composed of the following tools:

* :file:`rustc`: a driver of the Ferrocene Rust compiler; it invokes:

  * :file:`Rust Front-End`: the compilation front-end (see definition below),
  * :file:`LLVM`: the compilation back-end (see definition below),
  * :file:`GCC`: used in its capacity as a linker;

* :file:`Rust Front-End`: a front-end that verifies the syntax and semantics of
  the Rust programming language;
* :file:`LLVM`: a back-end that converts LLVM IR into object files;
* :file:`GCC`: linker only, merges object files into an executable or a library;

These tools are designed and tested to be interoperable and form a consistent
and coherent toolchain.
