.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Unsafety
========

.. rubric:: Legality Rules

:def_p:`fls_8kqo952gjhaf`
:term:`Unsafety` is the presence of :term:`[unsafe operation]s` in program text.

:def_p:`fls_ovn9czwnwxue`
An :term:`unsafe operation` is an operation that can potentially violate the
memory-safety guarantees of Rust. :term:`[Unsafe operation]s` are referred to as
:term:`unsafe Rust`.

:def_p:`fls_pfhmcafsjyf7`
The :term:`[unsafe operation]s` are:

* :def_p:`fls_jd1inwz7ulyw`
  Dereferencing a :term:`value` of a :term:`raw pointer type`.

* :def_p:`fls_3ra8s1v1vbek`
  Reading or writing an :term:`external static`.

* :def_p:`fls_6ipl0xo5qjyl`
  Reading or writing a :term:`mutable static`.

* :def_p:`fls_ucghxcnpaq2t`
  Accessing a :term:`field` of a :term:`union`, other than to assign to it.

* :def_p:`fls_ljocmnaz2m49`
  Calling an :term:`unsafe function`.

:def_p:`fls_jb6krd90tjmc`
An :term:`unsafe context` is either an :term:`unsafe block` or an :term:`unsafe
function`.

:def_p:`fls_ybnpe7ppq1vh`
An :term:`unsafe operation` shall be used only within an :term:`unsafe context`.

