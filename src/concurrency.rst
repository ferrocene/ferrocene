.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Concurrency
===========

:dp:`fls_opt7v0mopxc8`
The Rust programming language provides features for concurrent programming
without :t:`[data race]s`, whose rules are presented in this chapter.

.. rubric:: Legality Rules

:dp:`fls_tx4b8r6i93n4`
A :t:`data race` is a scenario where two or more threads access a shared memory
location concurrently without any synchronization, where one of the accesses is
a modification.

.. rubric:: Undefined Behavior

:dp:`fls_isypweqewe78`
It is undefined behavior if two or more threads engage in a :t:`data race`.

Send and Sync
-------------

.. rubric:: Legality Rules

:dp:`fls_n5l17mlglq11`
The Rust programming language provides synchronization facilities for
:t:`[type]s` through the :std:`core::marker::Send` :t:`trait` and the
:std:`core::marker::Sync` :t:`trait`.

:dp:`fls_2jujsujpjp3w`
A :t:`send type` is a :t:`type` that implements the :std:`core::marker::Send`
:t:`trait`.

:dp:`fls_cax6fe4em23k`
An :t:`abstract data type` automatically implements the
:std:`core::marker::Send` :t:`trait` if the :t:`[type]s` of all its
:t:`[field]s` are :t:`[send type]s`.

:dp:`fls_4ypqdehn7b0v`
A :t:`send type` shall have :t:`[value]s` that are safe to transfer across
thread boundaries.

:dp:`fls_dekskhk4g895`
A :t:`sync type` is a :t:`type` that implements the :std:`core::marker::Sync`
:t:`trait`.

:dp:`fls_y0iqr5ibnbfe`
An :t:`abstract data type` automatically implements the
:std:`core::marker::Sync` :t:`trait` if the :t:`[type]s` of all its
:t:`[field]s` are :t:`[sync type]s`.

:dp:`fls_zgemofbs5q2x`
A :t:`sync type` shall have :t:`[reference]s` that are safe to transfer across
thread boundaries.

Atomics
-------

.. rubric:: Legality Rules

:dp:`fls_3pjla9s93mhd`
An :t:`atomic type` is a :t:`type` defined in :t:`module`
:std:`core::sync::atomic`. :t:`[Atomic type]s` provide primitive shared-memory
communication between threads.

:dp:`fls_wn4ynaio8u47`
:t:`[Atomic type]s` are related to :t:`[type]s` as follows:

.. list-table::

   * - :dp:`fls_q7mn6pdd8bix`
     - **Type**
     - **Atomic Type**
   * - :dp:`fls_jx0784jzxy00`
     - :c:`bool`
     - :std:`core::sync::atomic::AtomicBool`
   * - :dp:`fls_vzuwnpx7mt08`
     - :c:`i8`
     - :std:`core::sync::atomic::AtomicI8`
   * - :dp:`fls_cpcd0vexfbhj`
     - :c:`i16`
     - :std:`core::sync::atomic::AtomicI16`
   * - :dp:`fls_jt7rfq9atbiv`
     - :c:`i32`
     - :std:`core::sync::atomic::AtomicI32`
   * - :dp:`fls_2hqmfwswc6k`
     - :c:`i64`
     - :std:`core::sync::atomic::AtomicI64`
   * - :dp:`fls_5ab2sw3gwmt3`
     - :c:`isize`
     - :std:`core::sync::atomic::AtomicIsize`
   * - :dp:`fls_w2mw833g28eb`
     - ``*mut T``
     - :std:`core::sync::atomic::AtomicPtr`
   * - :dp:`fls_mjq1l1y0vmz4`
     - :c:`u8`
     - :std:`core::sync::atomic::AtomicU8`
   * - :dp:`fls_906978wtss6n`
     - :c:`u16`
     - :std:`core::sync::atomic::AtomicU16`
   * - :dp:`fls_4urmnh4mfehl`
     - :c:`u32`
     - :std:`core::sync::atomic::AtomicU32`
   * - :dp:`fls_2qkrcd5eovpe`
     - :c:`u64`
     - :std:`core::sync::atomic::AtomicU64`
   * - :dp:`fls_cry1e78gp19q`
     - :c:`usize`
     - :std:`core::sync::atomic::AtomicUsize`

Asynchronous Computation
------------------------

.. rubric:: Legality Rules

:dp:`fls_g40xp4andj5g`
The Rust programming language provides asynchronous computation through
:t:`module` :std:`core::task` and the :std:`core::future::Future` :t:`trait`.

:dp:`fls_fte085hi1yqj`
A :t:`future` represents a :t:`value` of a :t:`type` that implements the
:std:`core::future::Future` :t:`trait` which may not have finished computing
yet.

:dp:`fls_7muubin2wn1v`
The computed :t:`value` of a :t:`future` is obtained by using an :t:`await
expression` or by invoking :std:`core::future::Future::poll`.

:dp:`fls_ftzey2156ha`
:std:`core::future::Future::poll` shall not be invoked on a :t:`future` that has
already returned :std:`core::task::Poll::Ready`.

