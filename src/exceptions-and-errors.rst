.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Exceptions and Errors
=====================

.. rubric:: Legality Rules

:def_p:`fls_vsk4vhnuiyyz`
The Rust programming language lacks exceptions and exception handlers. Instead,
the language uses the following tiered error handling scheme:

* :def_p:`fls_ebangxc36t74`
  A possibly absent :term:`value` is represented usually using :term:`enum`
  :codeterm:`core::option::Option`.

* :def_p:`fls_ckeitwiv326r`
  The result of a possibly erroneous computation is represented usually using
  :term:`enum` :codeterm:`core::result::Result`.

* :def_p:`fls_eg0orgibg98m`
  Erroneous behavior is signaled using :term:`macro` :codeterm:`core::panic`.

:def_p:`fls_ko1x0gp9e7y3`
:term:`Enum` :codeterm:`core::option::Option` indicates whether a :term:`value`
is either present using :codeterm:`core::option::Option::Some` or absent using
:codeterm:`core::option::Option::None`.

:def_p:`fls_gwu4cn4ziabe`
:term:`Enum` :codeterm:`core::result::Result` indicates whether a
computation completed successfully and produced a :term:`value` using
:codeterm:`core::result::Result::Ok` or the computation failed with an error
using :codeterm:`core::result::Result::Err`.

Panic
-----

:def_p:`fls_m3r7wvepljhs`
`Rust <https://rustc-dev-guide.rust-lang.org/panic-implementation.html>`_

.. rubric:: Legality Rules

:def_p:`fls_a554v4n0khye`
A :term:`panic` is an abnormal program state caused by invoking :term:`macro`
:codeterm:`core::panic`.

.. rubric:: Dynamic Semantics

:def_p:`fls_i9njhpte5l0t`
Invoking :term:`macro` :codeterm:`core::panic` has the following runtime
effects:

#. :def_p:`fls_n6q7bksyn1m`
   Control flow halts the execution of the current thread.

#. :def_p:`fls_xmtt04lw517w`
   Control flow invokes the :term:`function` subject to :term:`attribute`
   :codeterm:`panic_handler`. The :term:`function` may choose to loop infinitely
   or :term:`abort` the program.

.. rubric:: Undefined Behavior

:def_p:`fls_krhb07mvojph`
It is undefined behavior when a :term:`panic` crosses a :term:`foreign function
interface` boundary.

.. rubric:: Examples

.. code-block:: text

   panic!("This was a terrible mistake!");

Abort
-----

.. rubric:: Legality Rules

:def_p:`fls_9a1izu3omkbn`
:term:`Abort` is the immediate termination of a program.

.. rubric:: Dynamic Semantics

:def_p:`fls_iq6olct3rw4u`
:term:`Abort` has the following runtime effects:

#. :def_p:`fls_wd2q6ft9yzrg`
   Control flow halts the execution of all threads.

#. :def_p:`fls_7bnrbjb0pq5n`
   The program terminates.

