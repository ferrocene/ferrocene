.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_wdicg3sqa98e:

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

:dp:`fls_7zh6ziglo5iy`
An :t:`expression statement` is an :t:`expression` whose result is ignored.

:dp:`fls_kdxe1ukmgl1`
An :t:`item statement` is a :t:`statement` that is expressed as an :t:`item`.

:dp:`fls_fftdnwe22xrb`
An :t:`empty statement` is a :t:`statement` expressed as character 0x3B
(semicolon).

:dp:`fls_or125cqtxg9j`
A :t:`macro statement` is a :t:`statement` expressed as a
:t:`terminated macro invocation`.

.. rubric:: Dynamic Semantics

:dp:`fls_estqu395zxgk`
:t:`Execution` is the process by which a :t:`statement` achieves its runtime
effects.

:dp:`fls_dl763ssb54q1`
The :t:`execution` of an :t:`empty statement` has no effect.

.. _fls_yivm43r5wnp1:

Let Statements
--------------

.. rubric:: Syntax

.. syntax::

   LetStatement ::=
       OuterAttributeOrDoc* $$let$$ PatternWithoutAlternation TypeAscription? LetInitializer? $$;$$

   LetInitializer ::=
       $$=$$ Expression ($$else$$ BlockExpression)?

.. rubric:: Legality Rules

:dp:`fls_ct7pp7jnfr86`
A :t:`let statement` is a :t:`statement` that introduces new :t:`[binding]s`
produced by its :t:`pattern-without-alternation` that are optionally
initialized to a :t:`value`.

:dp:`fls_SR3dIgR5K0Kq`
A :t:`let initializer` is a :t:`construct` that provides the :t:`value` of
the :t:`[binding]s` of the :t:`let statement` using an :t:`expression`, or
alternatively executes a :t:`block expression`.

:dp:`fls_iqar7vvtw22c`
If a :t:`let statement` lacks a :t:`block expression`, then the :t:`pattern` of
the :t:`let statement` shall be an :t:`irrefutable pattern`.

:dp:`fls_1s1UikGU5YQb`
If a :t:`let statement` has a :t:`block expression`, then the :s:`Expression` of
the :s:`LetInitializer` shall not be a :s:`LazyBooleanExpression` or end with
token ``}``.

:dp:`fls_iB25BeFys0j8`
The :t:`expected type` of the :t:`pattern` of the :t:`let statement` is determined as follows:

* :dp:`fls_zObyLdya4DYc`
  If the :t:`let statement` lacks a :t:`type ascription` and a :t:`let initializer, then the :t:`expected type` is the :t:`inferred type`.

* :dp:`fls_r38TXWKQPjxv`
  If the :t:`let statement` lacks a :t:`type ascription`, then the :t:`expected type` is the :t:`type` of the :t:`let initializer`.

* :dp:`fls_6QSdwF4pzjoe`
  Otherwise the :t:`expected type` is the :t:`type` specified by the :t:`type ascription`.

:dp:`fls_1prqh1trybwz`
The :t:`type` of a :t:`binding` introduced by a :t:`let statement` is
determined as follows:

* :dp:`fls_djkm8r2iuu6u`
  If the :t:`let statement` appears with a :t:`type ascription`, then the
  :t:`type` is the :t:`type` specified by the :t:`type ascription`.

* :dp:`fls_ppj9gvhp8wcj`
  If the :t:`let statement` lacks a :t:`type ascription`, then the :t:`type` is
  inferred using :t:`type inference`.

:dp:`fls_1eBQDZdBuDsN`
The :t:`type` of the :t:`block expression` of a :t:`let statement` shall be the
:t:`never type`.

:dp:`fls_m8a7gesa4oim`
The :t:`value` of a :t:`binding` introduced by a :t:`let statement` is
determined as follows:

* :dp:`fls_oaxnre7m9s10`
  If the :t:`let statement` appears with a :t:`let initializer`, then the
  :t:`value` is the :t:`value` of the :t:`expression` of the
  :t:`let initializer`.

* :dp:`fls_t5bjwluyv8za`
  Otherwise the :t:`binding` is uninitialized.

.. rubric:: Dynamic Semantics

:dp:`fls_4j9riqyf4p9`
The :t:`execution` of a :t:`let statement` with a :t:`let initializer` proceeds
as follows:

#. :dp:`fls_t53g5hlabqw1`
   The :t:`expression` of the :t:`let initializer` is evaluated.

#. :dp:`fls_7j4qlwg72ege`
   If the :t:`value` of the :t:`expression` is matched successfully against the
   :t:`pattern` of the :t:`let statement`, then the :t:`value` is assigned to
   each :t:`binding` introduced by the :t:`let statement`.

#. :dp:`fls_ea9bRFZjH8Im`
   Otherwise the :t:`block expression` of the :t:`let initializer` is evaluated.

.. rubric:: Examples

.. code-block:: rust

   let local = 0;
   let local: u32;
   let (a, b) = (0, 0);
   let Some(value) = vector.pop() else {
       panic!();
   };

.. _fls_1pg5ig740tg1:

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

.. code-block:: rust

   let mut values = vec![1, 2, 3];

:dp:`fls_4q90jb39apwr`
The following expression statement ignores the result from ``pop()``.

.. code-block:: rust

   values.pop();

:dp:`fls_xqtztcu8ibwq`
The following expression statement does not require a semicolon.

.. code-block:: rust

   if values.is_empty() {
       values.push(42);
   }
   else {
       values.remove(0);
   }

:dp:`fls_2p9xnt519nbw`
The following expression statement is not an index expression.

.. code-block:: rust

   [42];
