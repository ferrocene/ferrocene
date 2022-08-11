.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_qcb1n9c0e5hz:

Functions
=========

.. rubric:: Syntax

.. syntax::

   FunctionDeclaration ::=
       FunctionQualifierList $$fn$$ Name GenericParameterList? $$($$ FunctionParameterList? $$)$$ ReturnType? WhereClause? (FunctionBody | ;)

   FunctionQualifierList ::=
       $$const$$? $$async$$? $$unsafe$$? AbiSpecification?

   FunctionParameterList ::=
       (FunctionParameter ($$,$$ FunctionParameter)* $$,$$?)
     | (SelfParameter ($$,$$ FunctionParameter)* $$,$$?)

   FunctionParameter ::=
       OuterAttributeOrDoc* (FunctionParameterPattern | TypeSpecification | $$...$$)

   FunctionParameterPattern ::=
       PatternWithoutAlternation (TypeAscription | ($$:$$ FunctionParameterVariadicPart))

   FunctionParameterVariadicPart ::=
       $$...$$

   ReturnType ::=
       $$->$$ TypeSpecification



   FunctionBody ::=
       BlockExpression

   SelfParameter ::=
       OuterAttributeOrDoc* (ShorthandSelf | TypedSelf)

   ShorthandSelf ::=
       ($$&$$ LifetimeIndication?)? $$mut$$? $$self$$

   TypedSelf ::=
       $$mut$$? $$self$$ TypeAscription

.. rubric:: Legality Rules

:dp:`fls_yfm0jh62oaxr`
A :t:`function` shall have a :s:`FunctionBody` unless it is an :t:`associated
trait function` or :t:`external function`.

:dp:`fls_ijbt4tgnl95n`
A :t:`function` shall not specify a :s:`SelfParameter` unless it is an
:t:`associated function`.

:dp:`fls_icdzs1mjh0n4`
A :t:`function` shall not specify a :s:`FunctionParameterVariadicPart` unless it
is an :t:`external function`.

:dp:`fls_ymeo93t4mz4`
A :t:`self parameter` or a :t:`receiver` is a :t:`function parameter` expressed
by :t:`keyword` ``self``.

:dp:`fls_gn1ngtx2tp2s`
A :t:`function` is a :t:`value` of a :t:`function type` that models a behavior.

:dp:`fls_nwywh1vjt6rr`
A :t:`function` shall not be subject to both :t:`keyword` ``async`` and
:t:`keyword` ``const``.

:dp:`fls_bdx9gnnjxru3`
A :t:`function` declares a unique :t:`function item type` for itself.

:dp:`fls_87jnkimc15gi`
A :t:`function qualifier` is a :t:`construct` that determines the role of
a :t:`function`.

:dp:`fls_uwuthzfgslif`
A :t:`function parameter` is a :t:`construct` that matches an input :t:`value`
at the site of a :t:`call expression` or a :t:`method call expression` to
a pattern.

:dp:`fls_lxzinvqveuqh`
A :t:`function parameter` is an :t:`irrefutable pattern`.

:dp:`fls_vljy4mm0zca2`
A :t:`return type` is the :t:`type` of the result a :t:`function` returns.

:dp:`fls_927nfm5mkbsp`
A :t:`function body` is the :t:`block expression` of a :t:`function`.

:dp:`fls_bHwy8FLzEUi3`
A :t:`function body` denotes a :t:`control flow boundary`.

:dp:`fls_owdlsaaygtho`
A :t:`function signature` is a unique identification of a :t:`function`
that encompasses of its :t:`[function qualifier]s`, :t:`name`, :t:`[generic
parameter]s`, :t:`[function parameter]s`, :t:`return type`, and :t:`where
clause`.

:dp:`fls_2049qu3ji5x7`
A :t:`constant function` is a :t:`function` subject to :t:`keyword` ``const``.

:dp:`fls_7mlanuh5mvpn`
The :t:`function body` of a :t:`constant function` shall be a :t:`constant
expression`.

:dp:`fls_otr3hgp8lj1q`
A :t:`constant function` shall be callable from a :t:`constant context`.

:dp:`fls_m3jiunibqj81`
An :t:`async function` is a :t:`function` subject to :t:`keyword` ``async``. An
:t:`async function` of the form

.. code-block:: rust

   async fn async_fn(param: &param_type) -> return_type {
       /* tail expression */
   }

:dp:`fls_7vogmqyd87ey`
is equivalent to :t:`function`

.. code-block:: rust

   fn async_fn<'a>(param: &'a param_type) -> impl Future<Output = return_type> + 'a {
       async move {
           /* tail expression */
       }
   }

:dp:`fls_7ucwmzqtittv`
An :t:`unsafe function` is a :t:`function` subject to :t:`keyword` ``unsafe``.

:dp:`fls_5hn8fkf7rcvz`
The invocation of an :t:`unsafe function` shall require :t:`unsafe context`.

:dp:`fls_nw49shkqx40b`
A :t:`main function` is a :t:`function` that acts as an entry point into a
program. A :t:`main function` is subject to the following restrictions:

* :dp:`fls_o4fxok23134r`
  It lacks :t:`[function qualifier]s` ``async`` and ``unsafe``,

* :dp:`fls_bk755pvc1l53`
  Its :t:`ABI` is Rust,

* :dp:`fls_5j2vbkt2hitj`
  Its :t:`name` is the word ``main``,

* :dp:`fls_a3je4wc53bmo`
  It lacks :t:`[generic parameter]s`,

* :dp:`fls_w8q15zp7kyl0`
  It lacks :t:`[function parameter]s`,

* :dp:`fls_4psnfphsgdek`
  It lacks a :t:`return type`,

* :dp:`fls_m7xfrhqif74`
  It lacks a :t:`where clause`,

* :dp:`fls_qq9fzrw4aykd`
  It has a :t:`function body`.

.. rubric:: Examples

.. code-block:: rust

   fn eucledian_distance(left: &Point, right: &Point) -> f64 {
       let x_delta_squared: f64 = (right.x - left.x).powi(2);
       let y_delta_squared: f64 = (right.y - left.y).powi(2);

       (x_delta_squared + y_delta_squared).sqrt()
   }

   fn main() {}

