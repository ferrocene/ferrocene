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

:dp:`fls_kdxe1ukmgl1`
An :t:`item statement` is a :t:`statement` that is expressed as an :t:`item`.

:dp:`fls_fftdnwe22xrb`
An :t:`empty statement` is a :t:`statement` expressed as character 0x3B
(semicolon).

:dp:`fls_or125cqtxg9j`
A :t:`macro statement` is a :t:`statement` expressed as a :t:`terminated macro
invocation`.

.. rubric:: Dynamic Semantics

:dp:`fls_estqu395zxgk`
:t:`Execution` is the process by which a :t:`statement` achieves its runtime
effects.

:dp:`fls_dl763ssb54q1`
The :t:`execution` of an :t:`empty statement` has no effect.

Let Statements
--------------

.. rubric:: Syntax

.. syntax::

   LetStatement ::=
       OuterAttributeOrDoc* $$let$$ PatternWithoutAlternation TypeAscription? ($$=$$ Expression)? $$;$$

.. rubric:: Legality Rules

:dp:`fls_ct7pp7jnfr86`
A :t:`let statement` is a :t:`statement` that introduces new :t:`[binding]s`
produced by its :t:`pattern-without-alternation`.

:dp:`fls_1prqh1trybwz`
The :t:`type` of a :t:`binding` introduced by a :t:`let statement` is determined
as follows:

* :dp:`fls_djkm8r2iuu6u`
  If the :t:`let statement` appears with a :t:`type ascription`, then the
  :t:`type` is the :t:`type` specified by the :t:`type ascription`.

* :dp:`fls_ppj9gvhp8wcj`
  If the :t:`let statement` lacks a :t:`type ascription`, then the :t:`type` is
  :t:`inferred` using :t:`type inference`.

:dp:`fls_m8a7gesa4oim`
The :t:`value` of a :t:`binding` introduced by a :t:`let statement` is
determined as follows:

* :dp:`fls_oaxnre7m9s10`
  If the :t:`let statement` appears with an :t:`expression`, then the :t:`value`
  is the :t:`value` of the :t:`expression`.

* :dp:`fls_t5bjwluyv8za`
  If the :t:`let statement` lacks an :t:`expression`, then the :t:`binding`
  is uninitialized.

:dp:`fls_iqar7vvtw22c`
The :t:`pattern-without-alternation` of a :t:`let statement` shall be
:t:`irrefutable`.

.. rubric:: Dynamic Semantics

:dp:`fls_4j9riqyf4p9`
The :t:`execution` of a :t:`let statement` proceeds as follows:

#. :dp:`fls_mvvigioc1ozm`
   If the :t:`let statement` appears with an :t:`expression`, then:

   #. :dp:`fls_t53g5hlabqw1`
      The :t:`expression` is evaluated.

   #. :dp:`fls_7j4qlwg72ege`
      The :t:`value` of the :t:`expression` is assigned to each :t:`binding`
      introduced by the :t:`let statement`.

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

:dp:`fls_xmdj8uj7ixoe`
An :t:`expression statement` is an :t:`expression` whose result is ignored.

:dp:`fls_gzzmudc1hl6s`
The :t:`expected type` of an :t:`expression statement` without character 0x3B
(semicolon) is the :t:`unit type`.

.. rubric:: Dynamic Semantics

:dp:`fls_kc99n8qrszxh`
The :t:`execution` of an :t:`expression statement` proceeds as follows:

#. :dp:`fls_r8poocwqaglf`
   The :t:`operand` is evaluated.

#. :dp:`fls_88e6s3erk8tj`
   The :t:`value` of the :t:`operand` is :t:`dropped`.

.. rubric:: Examples

.. code-block:: text

   let mut values = vec![1, 2, 3];

:dp:`fls_4q90jb39apwr`
The following expression statement ignores the result from ``pop()``.

.. code-block:: text

   values.pop();

:dp:`fls_xqtztcu8ibwq`
The following expression statement does not require a semicolon.

.. code-block:: text

   if values.is_empty() {
       values.push(42);
   }
   else {
       values.remove(0);
   }

:dp:`fls_2p9xnt519nbw`
The following expression statement is not an array index expression.

.. code-block:: text

   [42];

