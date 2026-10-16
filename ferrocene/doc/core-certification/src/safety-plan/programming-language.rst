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

Rust is well matched to the needs of the certified Rust libraries.

Heap allocations
----------------

While Rust supports heap allocation, it is not used in the core library, which is enforced by core being ``no_std``.

.. TODO: talk about alloc
