.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Building a library
==================

This chapter describes how to build a library using rustc.

The examples in this chapter assume the following directory structure:

.. code-block::

   .
   └── rust_library
       ├── addition_ops.rs
       ├── multiplication_ops.rs
       └── ops.rs

We are going to build a library consisting of simple arithmetic operations.
Source file ``./rust_library/addition_ops.rs`` contains:

.. code-block:: rust

   pub fn add(left: i32, right: i32) -> i32 {
       left + right
   }

   pub fn sub(left: i32, right: i32) -> i32 {
       left - right
   }

Source file ``./rust_library/multiplication_ops.rs`` contains:

.. code-block:: rust

   pub fn div(left: i32, right: i32) -> i32 {
       left / right
   }

   pub fn mul(left: i32, right: i32) -> i32 {
       left * right
   }

   pub fn rem(left: i32, right: i32) -> i32 {
       left % right
   }

We can bundle the two source files into a convenient single library by
supplying a special source file called a
`crate root module <../../specification/glossary.html#crate-root-module>`_.
Crate root module ``./rust_library/ops.rs`` contains:

.. code-block:: rust

   pub mod addition_ops;
   pub mod multiplication_ops;

The `outline modules <../../specification/glossary.html#outline-module>`_
cause the source files that correspond to the module names to be inlined
within the crate root module.

To create the library, run:

.. code-block::

   $ rustc --edition 2021 --crate-type lib ops.rs

.. include:: ../partials/target-flags-note.rst

This invocation produces library file ``libops.rlib``.

The compiler is capable of producing different kinds of libraries, such as
static, dynamic, and native libraries. To do so, consult the documentation of
compiler argument :cli:option:`--crate-type <rustc --crate-type <types>>` for
further details.
