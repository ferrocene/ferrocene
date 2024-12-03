.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_qcb1n9c0e5hz:

Functions
=========

.. rubric:: Syntax

.. syntax::

   FunctionDeclaration ::=
       FunctionQualifierList $$fn$$ Name GenericParameterList? $$($$ FunctionParameterList? $$)$$ ReturnType? WhereClause? (FunctionBody | ;)

   FunctionQualifierList ::=
       $$const$$? $$async$$? ItemSafety? AbiSpecification?

   FunctionParameterList ::=
       (FunctionParameter ($$,$$ FunctionParameter)* $$,$$?)
     | (SelfParameter ($$,$$ FunctionParameter)* $$,$$?)

   FunctionParameter ::=
       OuterAttributeOrDoc* (FunctionParameterPattern | FunctionParameterVariadicPart | TypeSpecification)

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

:dp:`fls_gn1ngtx2tp2s`
A :t:`function` is a :t:`value` of a :t:`function type` that models a behavior.

:dp:`fls_bdx9gnnjxru3`
A :t:`function` declares a unique :t:`function item type` for itself.

:dp:`fls_87jnkimc15gi`
A :t:`function qualifier` is a :t:`construct` that determines the role of
a :t:`function`.

:dp:`fls_nwywh1vjt6rr`
A :t:`function` shall not be subject to both :t:`keyword` ``async`` and
:t:`keyword` ``const``.

:dp:`fls_uwuthzfgslif`
A :t:`function parameter` is a :t:`construct` that yields a set of
:t:`[binding]s` that bind matched input :t:`[value]s` to :t:`[name]s` at the
site of a :t:`call expression` or a :t:`method call expression`.

:dp:`fls_ymeo93t4mz4`
A :t:`self parameter` is a :t:`function parameter` expressed by :t:`keyword`
``self``.

:dp:`fls_ijbt4tgnl95n`
A :t:`function` shall not specify a :t:`self parameter` unless it is an
:t:`associated function`.

:dp:`fls_AAYJDCNMJgTq`
The :t:`type` of a :t:`function parameter` is determined as follows:

* :dp:`fls_PGtp39f6gJwU`
  If the :t:`function parameter` is a :t:`self parameter` without a :s:`TypeSpecification`:

  * :dp:`fls_yZ2yIXxmy2ri`
    And the :t:`self parameter` has token ``&`` and :t:`keyword` ``mut``, then the :t:`type` is ``&mut Self``.

  * :dp:`fls_35aSvBxBnIzm`
    And the :t:`self parameter` has token ``&`` and lacks :t:`keyword` ``mut``, then the :t:`type` is ``&Self``.

  * :dp:`fls_Ogziu8S01qPQ`
    And the :t:`self parameter` lacks token ``&`` and :t:`keyword` ``mut``, then the :t:`type` is ``Self``.

* :dp:`fls_xCSsxYUZUFed`
  Otherwise the :t:`type` is the specified :t:`type`.

:dp:`fls_lxzinvqveuqh`
The :t:`pattern` of a :t:`function parameter` shall be an :t:`irrefutable
pattern`.

:dp:`fls_kcAbTPZXQ5Y8`
The :t:`expected type` of the :t:`pattern` of a :t:`function parameter` is the :t:`type` of the :t:`function parameter`.

:dp:`fls_PGDKWK7nPvgw`
The :t:`[binding]s` of all :t:`[pattern]s` of all :t:`[function parameter]s` of a :t:`function` shall not shadow another.

:dp:`fls_icdzs1mjh0n4`
A :t:`function` shall not specify a :s:`FunctionParameterVariadicPart` unless
it is an :t:`external function`.

:dp:`fls_vljy4mm0zca2`
A :t:`return type` is the :t:`type` of the result a :t:`function`, :t:`closure type` or :t:`function pointer type` returns.

:dp:`fls_EqJb3Jl3vK8K`
The :t:`return type` of a :t:`function` is determined as follows:

* :dp:`fls_C7dvzcXcpQCy`
  If the :s:`FunctionDeclaration` specifies a :s:`ReturnType`, then the :t:`return type` is the specified :s:`ReturnType`.

* :dp:`fls_J8X8ahnJLrMo`
  Otherwise the :t:`return type` is the :t:`unit type`.

:dp:`fls_927nfm5mkbsp`
A :t:`function body` is the :t:`block expression` of a :t:`function`.

:dp:`fls_yfm0jh62oaxr`
A :t:`function` shall have a :t:`function body` unless it is an
:t:`associated trait function` or an :t:`external function`.

:dp:`fls_bHwy8FLzEUi3`
A :t:`function body` denotes a :t:`control flow boundary`.

:dp:`fls_5Q861wb08DU3`
A :t:`function body` of an :t:`async function` denotes an
:t:`async control flow boundary`.

:dp:`fls_owdlsaaygtho`
A :t:`function signature` is a unique identification of a :t:`function`
that encompasses of its :t:`[function qualifier]s`, :t:`name`,
:t:`[generic parameter]s`, :t:`[function parameter]s`, :t:`return type`, and
:t:`where clause`.

:dp:`fls_2049qu3ji5x7`
A :t:`constant function` is a :t:`function` subject to :t:`keyword` ``const``.

:dp:`fls_7mlanuh5mvpn`
The :t:`function body` of a :t:`constant function` shall be a
:t:`constant expression`.

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
An :t:`unsafe function` is a :t:`function` subject to an :s:`ItemSafety` with :t:`keyword` ``unsafe``.

:dp:`fls_8ltVLtAfvy0m`
A :t:`function` shall only be subject to an :s:`ItemSafety` with :t:`keyword` ``safe`` if it is an :t:`external function` in an :t:`unsafe external block`.

:dp:`fls_5hn8fkf7rcvz`
The invocation of an :t:`unsafe function` shall require :t:`unsafe context`.

.. rubric:: Examples

.. code-block:: rust

   fn eucledian_distance(left: &Point, right: &Point) -> f64 {
       let x_delta_squared: f64 = (right.x - left.x).powi(2);
       let y_delta_squared: f64 = (right.y - left.y).powi(2);

       (x_delta_squared + y_delta_squared).sqrt()
   }

   fn main() {}
