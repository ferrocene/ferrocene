.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

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
       PatternWithoutAlternation (TypeAscription | ($$: $$FunctionParameterVariadicPart))

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

:def_p:`fls_yfm0jh62oaxr`
A :term:`function` shall have a :syntax:`FunctionBody` unless it is an
:term:`associated trait function` or :term:`external function`.

:def_p:`fls_ijbt4tgnl95n`
A :term:`function` shall not specify a :syntax:`SelfParameter` unless it is an
:term:`associated function`.

:def_p:`fls_icdzs1mjh0n4`
A :term:`function` shall not specify a :syntax:`FunctionParameterVariadicPart`
unless it is an :term:`external function`.

A :term:`self parameter` or a :term:`receiver` is a :term:`function parameter`
expressed by :term:`keyword` ``self``.

:def_p:`fls_gn1ngtx2tp2s`
A :term:`function` is a :term:`value` of a :term:`function type` that models
a behavior.

:def_p:`fls_nwywh1vjt6rr`
A :term:`function` shall not be subject to both :term:`keyword` ``async`` and
:term:`keyword` ``const``.

:def_p:`fls_bdx9gnnjxru3`
A :term:`function` declares a unique :term:`function item type` for itself.

:def_p:`fls_87jnkimc15gi`
A :term:`function qualifier` is a :term:`construct` that determines the role of
a :term:`function`.

:def_p:`fls_uwuthzfgslif`
A :term:`function parameter` is a :term:`construct` that matches an input
:term:`value` at the site of a :term:`call expression` or a :term:`method call
expression` to a pattern.

:def_p:`fls_lxzinvqveuqh`
A :term:`function parameter` is an :term:`irrefutable pattern`.

:def_p:`fls_vljy4mm0zca2`
A :term:`return type` is the :term:`type` of the result a :term:`function`
returns.

:def_p:`fls_927nfm5mkbsp`
A :term:`function body` is the :term:`block expression` of a :term:`function`.

:def_p:`fls_owdlsaaygtho`
A :term:`function signature` is a unique identification of a :term:`function`
that encompases of its :term:`function qualifier`\ s, :term:`name`,
:term:`generic parameter`\ s, :term:`function parameter`\ s, :term:`return
type`, and :term:`where clause`.

:def_p:`fls_2049qu3ji5x7`
A :term:`constant function` is a :term:`function` subject to :term:`keyword`
``const``.

:def_p:`fls_7mlanuh5mvpn`
The :term:`function body` of a :term:`constant function` shall be a
:term:`constant expression`.

:def_p:`fls_otr3hgp8lj1q`
A :term:`constant function` shall be callable from a :term:`constant context`.

:def_p:`fls_m3jiunibqj81`
An :term:`async function` is a :term:`function` subject to :term:`keyword`
``async``. An :term:`async function` of the form

.. code-block:: text

   async fn async_fn(param: &param_type) -> return_type {
       /* tail expression */
   }

:def_p:`fls_7vogmqyd87ey`
is equivalent to :term:`function`

.. code-block:: text

   fn async_fn<'a>(param: &'a param_type) -> impl Future<Output = return_type> + 'a {
       async move {
           /* tail expression */
       }
   }

:def_p:`fls_7ucwmzqtittv`
An :term:`unsafe function` is a :term:`function` subject to :term:`keyword`
``unsafe``.

:def_p:`fls_5hn8fkf7rcvz`
The invocation of an :term:`unsafe function` shall require :term:`unsafe
context`.

:def_p:`fls_nw49shkqx40b`
A :term:`main function` is a :term:`function` that acts as an entry point into a
program. A :term:`main function` is subject to the following restrictions:

* :def_p:`fls_o4fxok23134r`
  It lacks :term:`function qualifier`\ s ``async`` and ``unsafe``,

* :def_p:`fls_bk755pvc1l53`
  Its :term:`ABI` is Rust,

* :def_p:`fls_5j2vbkt2hitj`
  Its :term:`name` is the word ``main``,

* :def_p:`fls_a3je4wc53bmo`
  It lacks :term:`generic parameter`\ s,

* :def_p:`fls_w8q15zp7kyl0`
  It lacks :term:`function parameter`\ s,

* :def_p:`fls_4psnfphsgdek`
  It lacks a :term:`return type`,

* :def_p:`fls_m7xfrhqif74`
  It lacks a :term:`where clause`,

* :def_p:`fls_qq9fzrw4aykd`
  It has a :term:`function body`.

.. rubric:: Examples

.. code-block:: text

   fn eucledian_distance(left: &Point, right: &Point) -> f64 {
       let x_delta_squared: f64 = (right.x - left.x).powi(2);
       let y_delta_squared: f64 = (right.y - left.y).powi(2);

       (x_delta_squared + y_delta_squared).sqrt()
   }

   fn main() {}

