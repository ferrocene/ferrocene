.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Building mixed-language programs
================================

This chapter describes how to build a mixed-language program using
Ferrocene's rust compiler.

The examples in this chapter assume the following directory structure:

.. code-block::

   .
   ├── c_library
   │   └── factorial.c
   └── mixed_language
       ├── factorial.rs
       ├── call_factorial_from_c.c
       ├── call_factorial_from_c.h
       └── call_factorial_from_rust.rs

Foreign library, Rust executable
--------------------------------

We start by building a C library for a simple factorial operation. Source file
``./c_library/factorial.c`` contains:

.. code-block:: c

   extern int factorial(int value) {
       if (value <= 1) {
           return 1;
       } else {
           return value * factorial(value - 1);
       }
   }

To create the library, run:

.. code-block::

   $ gcc -c -o libfactorial.o factorial.c
   $ ar crus libfactorial.a libfactorial.o

This invocation produces library ``libfactorial.a``.

Once the library is produced, we can build a Rust executable that depends on
library ``libfactorial``. Source file
``./mixed_language/call_factorial_from_rust.rs`` contains:

.. code-block:: rust

   extern "C" {
       fn factorial(value: i32) -> i32;
   }

   fn main() {
       println!("3! = {}", unsafe { factorial(3) });
   }

The source file first imports ``factorial`` as an
`external C function <../../specification/glossary.html#external-function>`_.
Note that external functions are considered
`unsafe <../../specification/glossary.html#unsafe-operation>`_ because the
compiler cannot guarantee the soundness of values in non-Rust code. The source
file then invokes ``factorial`` as a regular Rust function.

To create the executable, run:

.. code-block::

   $ rustc --edition 2021 -L ../c_library -l factorial call_factorial_from_rust.rs

.. include:: ../partials/criticalup-run.rst

.. include:: ../partials/target-flags-note.rst

This invocation produces executable ``call_factorial_from_rust``. Running
``call_factorial_from_rust`` should yield:

.. code-block::

   3! = 6

The compiler is capable of linking a binary to different kinds of libraries,
such as static, dynamic, and native libraries. Consult the documentation of
compiler arguments :cli:option:`-L <rustc -L <details>>`, and
:cli:option:`-l <rustc -l <details>>`. for further details.

Rust library, foreign executable
--------------------------------

We are first going to build a Rust library for a simple factorial operation.
Source file ``./mixed_language/factorial.rs`` contains:

.. code-block:: rust

   use core::ffi::c_int;

   #[no_mangle]
   pub extern "C" fn factorial(value: c_int) -> c_int {
       match value {
           c_int::MIN ..= 1 => 1,
           _ => value * factorial(value - 1),
       }
   }

The source file exports function ``factorial`` using the C
`ABI <../../specification/glossary.html#term_application_binary_interface>`_,
and ensures symbol compatibility by using
`attribute #[no_mangle] <../../specification/attributes.html#attribute-no-mangle>`_
and type ``std::ffi::c_int`` instead of Rust’s ``i32``.

To create a static library, run:

.. code-block::

   $ rustc --edition 2021 --crate-type staticlib factorial.rs

.. include:: ../partials/criticalup-run.rst

.. include:: ../partials/target-flags-note.rst

This invocation produces library ``libfactorial.a``.

To create a dynamic library, run:

.. code-block::

   $ rustc --edition 2021 --crate-type cdylib factorial.rs

.. include:: ../partials/criticalup-run.rst

.. include:: ../partials/target-flags-note.rst

This invocation produces library ``libfactorial.so``.

We can now build a C executable that depends on library ``libfactorial``.
Source file ``./mixed_language/call_factorial_from_c.h`` contains:

.. code-block:: c

   int factorial(int value);

Source file ``./mixed_language/call_factorial_from_c.c`` contains:

.. code-block:: c

   #include <stdio.h>
   #include "call_factorial_from_c.h"

   int main() {
       printf("3! = %d\n", factorial(3));
       return 0;
   }

To create an executable that links against our static library, run:

.. code-block::

   $ gcc -o call_factorial_from_c -L . call_factorial_from_c.c -l factorial

Or if you want to create an executable that links against out dynamic library,
run:

.. code-block::

   $ gcc -o call_factorial_from_c -L . -l factorial -Wl,-rpath=. call_factorial_from_c.c

These invocations produce executable ``call_factorial_from_c``. Running
``call_factorial_from_c`` should yield:

.. code-block::

   3! = 6
