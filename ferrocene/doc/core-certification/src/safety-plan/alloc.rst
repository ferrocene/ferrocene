.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Alloc library
=============

allocator traits and types
++++++++++++++++++++++++++

The ``alloc`` library does not include a memory allocator implementation. Instead its data structures are generic over the ``core::alloc::Allocator`` trait. If a user does not explicitly specify the allocator a data structure should use, the ``Global`` allocator is used.

Any allocator that implements the ``core::alloc::GlobalAllocator`` trait can be configured as the global allocator using the ``#[global_allocator]`` attribute. In a ``no_std`` crate the global allocator has to be supplied by the user. In a ``yes_std`` crate there is a default global allocator implementation supplied by the ``std`` library. Since ``std`` is currently out of scope of the certification, every user has to configure their own allocator.

Note that both the ``Allocator`` and the ``GlobalAllocator``  traits are currently unstable and cannot be directly implemented by users. The ``Allocator`` trait is expected to be stabilised in Rust version ``1.100.0`` (the next upcoming stable release). The ``GlobalAllocator`` trait has no clear date for stabilisation yet. Instead users can currently implement the ``core::alloc::GlobalAlloc`` trait (note the missing "-or"), which has a blanket implementation ``impl<A> GlobalAlloc for A where A: GlobalAllocator + ?Size``. In effect this means that currently user can implement custom own global allocators, but not custom per-datastructure allocators.

Below you can see a diagram of all allocator traits and types and how they interact (you can zoom):

.. mermaid:: ./alloc-traits-types.mmd
    :zoom:

compiler-builtins
+++++++++++++++++

The ``alloc`` library depends on the ``compiler-builtins`` library.

``compiler-builtins`` provides external symbols that the compiler expects to be available when building Rust projects, typically software routines for basic operations that do not have hardware support.
It is largely a port of LLVM's [`compiler-rt`].

``compiler-builtins`` is considered an implementation detail of ``alloc`` and not certified as a stand-alone library.
