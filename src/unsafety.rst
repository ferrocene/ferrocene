.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_jep7p27kaqlp:

Unsafety
========

.. rubric:: Legality Rules

:dp:`fls_8kqo952gjhaf`
:t:`Unsafety` is the presence of :t:`[unsafe operation]s` in program text.

:dp:`fls_ovn9czwnwxue`
An :t:`unsafe operation` is an operation that can potentially violate the
memory-safety guarantees of Rust. :t:`[Unsafe operation]s` are referred to as
:t:`unsafe Rust`.

:dp:`fls_pfhmcafsjyf7`
The :t:`[unsafe operation]s` are:

* :dp:`fls_jd1inwz7ulyw`
  Dereferencing a :t:`value` of a :t:`raw pointer type`.

* :dp:`fls_3ra8s1v1vbek`
  Reading or writing an :t:`external static`.

* :dp:`fls_6ipl0xo5qjyl`
  Reading or writing a :t:`mutable static`.

* :dp:`fls_ucghxcnpaq2t`
  Accessing a :t:`field` of a :t:`union`, other than to assign to it.

* :dp:`fls_ljocmnaz2m49`
  Calling an :t:`unsafe function`.

:dp:`fls_jb6krd90tjmc`
An :t:`unsafe context` is either an :t:`unsafe block` or an :t:`unsafe
function`.

:dp:`fls_ybnpe7ppq1vh`
An :t:`unsafe operation` shall be used only within an :t:`unsafe context`.

