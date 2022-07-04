.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Exceptions and Errors
=====================

.. rubric:: Legality Rules

:dp:`fls_vsk4vhnuiyyz`
The Rust programming language lacks exceptions and exception handlers. Instead,
the language uses the following tiered error handling scheme:

* :dp:`fls_ebangxc36t74`
  A possibly absent :t:`value` is represented usually using :t:`enum`
  :c:`core::option::Option`.

* :dp:`fls_ckeitwiv326r`
  The result of a possibly erroneous computation is represented usually using
  :t:`enum` :c:`core::result::Result`.

* :dp:`fls_eg0orgibg98m`
  Erroneous behavior is signaled using :t:`macro` :c:`core::panic`.

:dp:`fls_ko1x0gp9e7y3`
:t:`Enum` :c:`core::option::Option` indicates whether a :t:`value` is
either present using :c:`core::option::Option::Some` or absent using
:c:`core::option::Option::None`.

:dp:`fls_gwu4cn4ziabe`
:t:`Enum` :c:`core::result::Result` indicates whether a computation completed
successfully and produced a :t:`value` using :c:`core::result::Result::Ok` or
the computation failed with an error using :c:`core::result::Result::Err`.

Panic
-----

:dp:`fls_m3r7wvepljhs`
`Rust <https://rustc-dev-guide.rust-lang.org/panic-implementation.html>`_

.. rubric:: Legality Rules

:dp:`fls_a554v4n0khye`
A :t:`panic` is an abnormal program state caused by invoking :t:`macro`
:c:`core::panic`.

.. rubric:: Dynamic Semantics

:dp:`fls_i9njhpte5l0t`
Invoking :t:`macro` :c:`core::panic` has the following runtime effects:

#. :dp:`fls_n6q7bksyn1m`
   Control flow halts the execution of the current thread.

#. :dp:`fls_xmtt04lw517w`
   Control flow invokes the :t:`function` subject to :t:`attribute`
   :c:`panic_handler`. The :t:`function` may choose to loop infinitely or
   :t:`abort` the program.

.. rubric:: Undefined Behavior

:dp:`fls_krhb07mvojph`
It is undefined behavior when a :t:`panic` crosses a :t:`foreign function
interface` boundary.

.. rubric:: Examples

.. code-block:: text

   panic!("This was a terrible mistake!");

Abort
-----

.. rubric:: Legality Rules

:dp:`fls_9a1izu3omkbn`
:t:`Abort` is the immediate termination of a program.

.. rubric:: Dynamic Semantics

:dp:`fls_iq6olct3rw4u`
:t:`Abort` has the following runtime effects:

#. :dp:`fls_wd2q6ft9yzrg`
   Control flow halts the execution of all threads.

#. :dp:`fls_7bnrbjb0pq5n`
   The program terminates.

