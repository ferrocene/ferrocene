.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

HazOp Analysis Parameters
=========================

HazOp Guide Words
-----------------

Guide words must be in accordance with the software tool use domains. For the
need of Ferrocene qualification, we have selected the following guide words:

* **INVALID**: One of the parameter violates its semantics.
* **CHANGED**: A change occurs during processing.
* **MORE INFO**: An input or an output contains more information than expected.
* **LESS INFO**: An input or an output contains less information than expected.


HazOp Selected Nodes
--------------------

Ferrocene is composed of the elements presented in the :doc:`Toolchain
Workflow diagram <purpose>`.

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

   "gcc" is the name of the linker used, and in terms of HazOp nodes, it is
   equivalent to "Linker".
