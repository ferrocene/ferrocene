.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Building an executable
======================

This chapter describes how to build an executable using rustc.

The examples in this chapter assume the following directory structure:

.. code-block::

   .
   ├── executable
   │   ├── hw.rs
   │   └── use_ops.rs
   └── rust_library
       └── libops.rlib

Directory ``rust_library`` and its byproducts are described
`here <library.html>`_.

Building a simple executable
----------------------------

We are going to build a simple Hello World executable. Source file
``./executable/hw.rs`` contains:

.. code-block:: rust

   fn main() {
       println!("Hello World!");
   }

To create the executable, run:

.. code-block::

   $ rustc --edition 2021 hw.rs

.. include:: ../partials/target-flags-note.rst

This invocation produces executable ``hw``. Running ``hw`` should yield:

.. code-block::

   Hello World!

Building a linked executable
----------------------------

We are going to build an executable that depends on library ``libops`` we
created in the `Building a library <library.html>`_ step.
Source file ``./executable/use_ops.rs`` contains:

.. code-block:: rust

   use ops::addition_ops::add;
   use ops::multiplication_ops::mul;

   fn main() {
       println!("1 + 2 = {}", add(1, 2));
       println!("3 * 4 = {}", mul(3, 4));
   }

The `use imports <../../specification/glossary.html#use-import>`_ are
not necessary as long as uses of the functionality are fully qualified via
`paths <../../specification/glossary.html#path>`_, such as
``ops::addition_ops::add(1, 2)``.

To create the executable, run:

.. code-block::

   $ rustc --edition 2021 -L ../rust_library --extern ops use_ops.rs

.. include:: ../partials/target-flags-note.rst

This invocation produces executable ``use_ops``. Running ``use_ops`` should
yield:

.. code-block::

   1 + 2 = 3
   3 * 4 = 12

The compiler is capable of linking a binary to different kinds of libraries,
such as static, dynamic, and native libraries. To do so, consult the
documentation of compiler arguments
:cli:option:`--extern <rustc --extern <details>>`,
:cli:option:`-L <rustc -L <details>>`, and
:cli:option:`-l <rustc -l <details>>`.
