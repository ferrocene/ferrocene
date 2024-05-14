.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

HazOp Selected Nodes
====================

Ferrocene is composed of the elements presented in the :doc:`Toolchain
Workflow diagram <evaluation-plan:purpose>`.

Based on these elements, we define the following nodes corresponding to each
step of the toolchain:

* Rust Driver
* Rust Front-End
* LLVM
* Linker

.. note::

   "rustc" is the name of the tool, and in terms of HazOp nodes, it is
   equivalent to "Rust Driver".

.. note::

   "lld" is the name of the linker used, and in terms of HazOp nodes, it is
   equivalent to "Linker".
