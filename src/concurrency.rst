.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Concurrency
===========

:def_p:`fls_opt7v0mopxc8`
The Rust programming language provides features for concurrent programming
without :term:`data race`\ s, whose rules are presented in this chapter.

.. rubric:: Legality Rules

:def_p:`fls_tx4b8r6i93n4`
A :term:`data race` is a scenario where two or more threads access a shared
memory location concurrently without any synchronization, where one of the
accesses is a modification.

.. rubric:: Undefined Behavior

:def_p:`fls_isypweqewe78`
It is undefined behavior if two or more threads engage in a :term:`data race`.

Send and Sync
-------------

.. rubric:: Legality Rules

:def_p:`fls_n5l17mlglq11`
The Rust programming language provides synchronization facilities for
:term:`type`\ s through the :codeterm:`core::marker::Send` :term:`trait` and the
:codeterm:`core::marker::Sync` :term:`trait`.

:def_p:`fls_2jujsujpjp3w`
A :term:`send type` is a :term:`type` that implements the
:codeterm:`core::marker::Send` :term:`trait`.

:def_p:`fls_cax6fe4em23k`
An :term:`abstract data type` automatically implements the
:codeterm:`core::marker::Send` :term:`trait` if the :term:`type`\ s of all its
:term:`field`\ s are :term:`send type`\ s.

:def_p:`fls_4ypqdehn7b0v`
A :term:`send type` shall have :term:`value`\ s that are safe to transfer across
thread boundaries.

:def_p:`fls_dekskhk4g895`
A :term:`sync type` is a :term:`type` that implements the
:codeterm:`core::marker::Sync` :term:`trait`.

:def_p:`fls_y0iqr5ibnbfe`
An :term:`abstract data type` automatically implements the
:codeterm:`core::marker::Sync` :term:`trait` if the :term:`type`\ s of all its
:term:`field`\ s are :term:`sync type`\ s.

:def_p:`fls_zgemofbs5q2x`
A :term:`sync type` shall have :term:`reference`\ s that are safe to transfer
across thread boundaries.

Atomics
-------

.. rubric:: Legality Rules

:def_p:`fls_3pjla9s93mhd`
An :term:`atomic type` is a :term:`type` defined in :term:`module`
:codeterm:`core::sync::atomic`. :term:`Atomic type`\ s provide primitive
shared-memory communication between threads.

:def_p:`fls_wn4ynaio8u47`
:term:`Atomic type`\ s are related to :term:`type`\ s as follows:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Atomic Type
   * - :def_p:`fls_jx0784jzxy00`
       :codeterm:`bool`
     - :def_p:`fls_fwz6i6t185mu`
       :codeterm:`core::sync::atomic::AtomicBool`
   * - :def_p:`fls_vzuwnpx7mt08`
       :codeterm:`i8`
     - :def_p:`fls_agklqlvb9ab2`
       :codeterm:`core::sync::atomic::AtomicI8`
   * - :def_p:`fls_cpcd0vexfbhj`
       :codeterm:`i16`
     - :def_p:`fls_pj95qwj2ebxa`
       :codeterm:`core::sync::atomic::AtomicI16`
   * - :def_p:`fls_jt7rfq9atbiv`
       :codeterm:`i32`
     - :def_p:`fls_1jpjggqq1a7o`
       :codeterm:`core::sync::atomic::AtomicI32`
   * - :def_p:`fls_2hqmfwswc6k`
       :codeterm:`i64`
     - :def_p:`fls_suasjh3qmyrw`
       :codeterm:`core::sync::atomic::AtomicI64`
   * - :def_p:`fls_5ab2sw3gwmt3`
       :codeterm:`isize`
     - :def_p:`fls_ahfj32bad35r`
       :codeterm:`core::sync::atomic::AtomicIsize`
   * - :def_p:`fls_w2mw833g28eb`
       ``*mut T``
     - :def_p:`fls_y8k3foxazeny`
       :codeterm:`core::sync::atomic::AtomicPtr`
   * - :def_p:`fls_mjq1l1y0vmz4`
       :codeterm:`u8`
     - :def_p:`fls_n5p38asgq6s4`
       :codeterm:`core::sync::atomic::AtomicU8`
   * - :def_p:`fls_906978wtss6n`
       :codeterm:`u16`
     - :def_p:`fls_d9rhlghjuwxj`
       :codeterm:`core::sync::atomic::AtomicU16`
   * - :def_p:`fls_4urmnh4mfehl`
       :codeterm:`u32`
     - :def_p:`fls_gvyne0ppdcpg`
       :codeterm:`core::sync::atomic::AtomicU32`
   * - :def_p:`fls_2qkrcd5eovpe`
       :codeterm:`u64`
     - :def_p:`fls_2qhkexuo326g`
       :codeterm:`core::sync::atomic::AtomicU64`
   * - :def_p:`fls_cry1e78gp19q`
       :codeterm:`usize`
     - :def_p:`fls_ojqhbb32l6gh`
       :codeterm:`core::sync::atomic::AtomicUsize`

Asynchronous Computation
------------------------

.. rubric:: Legality Rules

:def_p:`fls_g40xp4andj5g`
The Rust programming language provides asynchronous computation through
:term:`module` :codeterm:`core::task` and the :codeterm:`core::future::Future`
:term:`trait`.

:def_p:`fls_fte085hi1yqj`
A :term:`future` represents a :term:`value` of a :term:`type` that implements
the :codeterm:`core::future::Future` :term:`trait` which may not have finished
computing yet.

:def_p:`fls_7muubin2wn1v`
The computed :term:`value` of a :term:`future` is obtained by using an
:term:`await expression` or by invoking :codeterm:`core::future::Future::poll`.

:def_p:`fls_ftzey2156ha`
:codeterm:`core::future::Future::poll` shall not be invoked on a :term:`future`
that has already returned :codeterm:`core::task::Poll::Ready`.

