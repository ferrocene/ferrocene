.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Building and using procedural macros
====================================

Procedural macros are specified in the :doc:`chapter "Macros" of the FLS <specification:macros>`.

See the `"Procedual macros" chapter of Rust reference <../../reference/procedural-macros.html>`_ as informational background on procedural macros.

If another crate called ``my_bin`` uses a procedural macro from a ``proc-macro`` crate called ``my_proc_macro``, the procedural macro is only executed during the compilation of the other crate, not during the runtime of the other crate. The object file generated from ``my_proc_macro`` will not be part of the binary produced from compiling ``my_bin``. Instead the procedural macro generates new tokens in the AST that are then compiled by rustc.

Because procedural macros are only executed during compilation and not during runtime, they have to be qualified as a tool and not certified as a library.

It is the end-user responsibility to qualify procedural macros if they are used in their code.

``proc-macro`` crates by default are linked to ``proc_macro`` and ``std``. They can be marked as ``no_std`` in order to not be linked to ``std`` but only to ``proc_macro`` and ``core``.

The ``proc_macro`` compile-time library
---------------------------------------

The ``proc_macro`` library defines the interface between the compiler and a procedural macro. Most notable is the ``proc_macro::TokenStream`` type. The ``proc_macro`` library is available in ``proc-macro`` crates only. If a crate of another crate type tries to use the ``proc_macro`` library, the compiler will error.

Because of this and because its code is not included in the final binary, the ``proc_macro`` library is qualified as part of the compiler qualification and not certified as a standalone library.

Building a ``proc-macro`` crate
-------------------------------

This section describes how to compile a ``proc-macro`` crate and use it from another crate.

The examples in this section assume the following directory structure:

.. code-block:: console

   $ tree .
   .
   ├── my_bin.rs
   └── my_lib.rs
   └── my_proc_macro.rs
   $
   $ cat my_proc_macro.rs
   #[proc_macro]
   pub fn my_fn_like_macro(_item: proc_macro::TokenStream) -> proc_macro::TokenStream {
      "fn add(left: i32, right: i32) -> i32 { left + right }".parse().unwrap()
   }
   $
   $ cat my_lib.rs
   my_proc_macro::my_fn_like_macro!();
   pub fn run() { assert_eq!(add(1, 2), 3); }
   $
   $ cat my_bin.rs
   my_proc_macro::my_fn_like_macro!();
   pub fn main() { assert_eq!(add(1, 2), 3); }

Building a simple ``proc-macro`` crate
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To build a ``proc-macro`` crate which only depends on ``proc_macro`` and ``std`` (or ``core``), execute the following:

.. code-block:: console

   $ rustc \
      --edition=2021 \
      --crate-name my_proc_macro \
      --crate-type proc-macro \
      --extern proc_macro \
      my_proc_macro.rs
   $ # Produces libmy_proc_macro.so

Building a ``proc-macro`` crate with dependencies
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

To build a ``proc-macro`` crate which depends on other libraries additionally to ``proc_macro`` and ``std``, first build the other libraries following :doc:`library`. Afterwards execute the following:

.. code-block:: console

   $ rustc \
      --edition=2021 \
      --crate-name my_proc_macro \
      --crate-type proc-macro \
      --extern other_lib=libother_lib.rlib \
      --extern proc_macro \
      my_proc_macro.rs
   $ # Produces libmy_proc_macro.so

Building a library/executable with a ``proc-macro`` crate as dependency
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

After you have compiled a ``proc-macro`` crate you can use it from another library or executable like this:

.. code-block:: console

   $ # executable
   $ rustc \
      --edition=2021 \
      --crate-name my_bin \
      --crate-type bin \
      --extern my_proc_macro=libmy_proc_macro.so \
      my_bin.rs
   $ # library
   $ rustc \
      --edition=2021 \
      --crate-name my_lib \
      --crate-type lib \
      --extern my_proc_macro=libmy_proc_macro.so \
      my_lib.rs
