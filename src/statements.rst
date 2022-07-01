.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Statements
==========

.. rubric:: Syntax

.. syntax::

   Statement ::=
       ExpressionStatement
     | Item
     | LetStatement
     | TerminatedMacroInvocation
     | $$;$$


.. rubric:: Legality Rules

:def_p:`fls_kdxe1ukmgl1`
An :term:`item statement` is a :term:`statement` that is expressed as an
:term:`item`.

:def_p:`fls_fftdnwe22xrb`
An :term:`empty statement` is a :term:`statement` expressed as character 0x3B
(semicolon).

:def_p:`fls_or125cqtxg9j`
A :term:`macro statement` is a :term:`statement` expressed as a
:term:`terminated macro invocation`.

.. rubric:: Dynamic Semantics

:def_p:`fls_estqu395zxgk`
:term:`Execution` is the process by which a :term:`statement` achieves its
runtime effects.

:def_p:`fls_dl763ssb54q1`
The :term:`execution` of an :term:`empty statement` has no effect.

Let Statements
--------------

.. rubric:: Syntax

.. syntax::


   LetStatement ::=
       OuterAttributeOrDoc* $$let$$ PatternWithoutAlternation TypeAscription? ($$=$$ Expression)? $$;$$


.. rubric:: Legality Rules

:def_p:`fls_ct7pp7jnfr86`
A :term:`let statement` is a :term:`statement` that introduces new
:term:`[binding]s` produced by its :term:`pattern-without-alternation`.

:def_p:`fls_1prqh1trybwz`
The :term:`type` of a :term:`binding` introduced by a :term:`let statement` is
determined as follows:

* :def_p:`fls_djkm8r2iuu6u`
  If the :term:`let statement` appears with a :term:`type ascription`, then the
  :term:`type` is the :term:`type` specified by the :term:`type ascription`.

* :def_p:`fls_ppj9gvhp8wcj`
  If the :term:`let statement` lacks a :term:`type ascription`, then the
  :term:`type` is :term:`inferred` using :term:`type inference`.

:def_p:`fls_m8a7gesa4oim`
The :term:`value` of a :term:`binding` introduced by a :term:`let statement` is
determined as follows:

* :def_p:`fls_oaxnre7m9s10`
  If the :term:`let statement` appears with an :term:`expression`, then the
  :term:`value` is the :term:`value` of the :term:`expression`.

* :def_p:`fls_t5bjwluyv8za`
  If the :term:`let statement` lacks an :term:`expression`, then the
  :term:`binding` is uninitialized.

:def_p:`fls_iqar7vvtw22c`
The :term:`pattern-without-alternation` of a :term:`let statement` shall be
:term:`irrefutable`.

.. rubric:: Dynamic Semantics

:def_p:`fls_4j9riqyf4p9`
The :term:`execution` of a :term:`let statement` proceeds as follows:

#. :def_p:`fls_mvvigioc1ozm`
   If the :term:`let statement` appears with an :term:`expression`, then:

   #. :def_p:`fls_t53g5hlabqw1`
      The :term:`expression` is evaluated.

   #. :def_p:`fls_7j4qlwg72ege`
      The :term:`value` of the :term:`expression` is assigned to each
      :term:`binding` introduced by the :term:`let statement`.

.. rubric:: Examples

.. code-block:: text

   let local = 0;
   let local: u32;
   let (a, b) = (0, 0);

Expression Statements
---------------------

.. rubric:: Syntax

.. syntax::

   ExpressionStatement ::=
       ExpressionWithBlock $$;$$?
     | ExpressionWithoutBlock $$;$$


.. rubric:: Legality Rules

:def_p:`fls_xmdj8uj7ixoe`
An :term:`expression statement` is an :term:`expression` whose result is
ignored.

:def_p:`fls_gzzmudc1hl6s`
The :term:`expected type` of an :term:`expression statement` without character
0x3B (semicolon) is the :term:`unit type`.

.. rubric:: Dynamic Semantics

:def_p:`fls_kc99n8qrszxh`
The :term:`execution` of an :term:`expression statement` proceeds as follows:

#. :def_p:`fls_r8poocwqaglf`
   The :term:`operand` is evaluated.

#. :def_p:`fls_88e6s3erk8tj`
   The :term:`value` of the :term:`operand` is :term:`dropped`.

.. rubric:: Examples

.. code-block:: text

   let mut values = vec![1, 2, 3];


:def_p:`fls_4q90jb39apwr`
The following expression statement ignores the result from ``pop()``.

.. code-block:: text


   values.pop();


:def_p:`fls_xqtztcu8ibwq`
The following expression statement does not require a semicolon.

.. code-block:: text


   if values.is_empty() {
       values.push(42);
   }
   else {
       values.remove(0);
   }


:def_p:`fls_2p9xnt519nbw`
The following expression statement is not an array index expression.

.. code-block:: text


   [42];

