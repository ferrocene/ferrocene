.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Programming language
====================

Compiler
--------

The certified libraries are being built with the safety qualified Ferrocene compiler. The compiler is qualified for the same safety standards as the certified libraries, see :doc:`evaluation-plan:index`.

The Ferrocene compiler uses Rust as defined by the Ferrocene language specification.

The Ferrocene compiler performs thorough control flow and data flow analysis. The data flow analysis is usually referred to as "Borrow Checker" and is one of the central features of Rust. One example for the outstanding control flow analysis is that rustc detects when some variants of an enum are not handled and throws a hard error.

Rust
----

Characteristics of Rust:

- strong typing and assertions
- memory safety
- well suited to structured and defensive programming
- support for modular design
- all items within a module are private by default and must be made public explicitly
- no support for goto jumps, except in inline assembly
- complex return types, therefore the use of out parameters is not common
- explicit type conversions (``.into`` or ``as``), except for convenience (e.g. `& &mut T` to `&T`) or method dispatch
- all fields and methods of a data structure are private by default and must be made public explicitly
- functions in Rust can only be called through one interface (i.e. no overloading)
- Usage of raw pointers is discouraged and limited in safe Rust
- Functions with many parameters are discouraged (`clippy::too_many_arguments <https://clippy.rust-lang.org/main/index.html?search=too_many#too_many_arguments>`_)

Rust is well matched to the needs of the certified Rust libraries.

Heap allocations
----------------

The ``core`` library does not use heap allocations which is enforced by ``core`` being ``no_std`` and not specifying ``extern alloc``.

The ``alloc`` library defines interfaces, data structures and functions that use heap allocations. But no allocator implementation is supplied, but must be supplied by the user.

The ``std`` library supplies allocator implementations, but ``std`` is currently out of scope of the certification.

Recursion limit
---------------

Compile-time
~~~~~~~~~~~~

By default, the Rust compiler has a recursion limit of 128 for compile-time operations such as macro expansion and automatic dereferencing, to limit the maximum source code complexity. This can be configured with the `recursion_limit attribute <https://doc.rust-lang.org/reference/attributes/limits.html#the-recursion_limit-attribute>`_.

Runtime
~~~~~~~

Rust has no runtime recursion limit, but it is the user responsiblity to enforce any limits.

To support users there are a few lints that catch common problems related to recursion, e.g.:

- `unconditional_recursion <https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#unconditional-recursion>`_
- `clippy::main_recursion <https://clippy.rust-lang.org/main/index.html#main_recursion>`_
- `clippy::only_used_in_recursion <https://clippy.rust-lang.org/main/index.html#only_used_in_recursion>`_

Note that the `recursion_depth_exceeding_limit <https://doc.rust-lang.org/rustc/lints/listing/warn-by-default.html#recursion-depth-exceeding-limit>`_ lint is not related to runtime recursion, but compile time recursion.
