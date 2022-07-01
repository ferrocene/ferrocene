.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Generics
========

Generic Parameters
------------------

.. rubric:: Syntax

.. syntax::


   GenericParameterList ::=
       $$< $$(GenericParameter ($$,$$ GenericParameter)* $$,$$?)? $$>$$

   GenericParameter ::=
       OuterAttributeOrDoc* (
           LifetimeParameter
         | ConstantParameter
         | TypeParameter
       )

   LifetimeParameter ::=
       Lifetime ($$:$$ LifetimeIndicationList)?

   ConstantParameter ::=
      $$const$$ Name TypeAscription

   TypeParameter ::=
      Name ($$:$$ TypeBounds?)? ($$=$$ TypeSpecification)?

.. rubric:: Legality Rules

:def_p:`fls_sye3d17l9bf5`
A :term:`generic parameter` is a placeholder for a :term:`constant`, a
:term:`lifetime`, or a :term:`type` whose :term:`value` is supplied statically
by a :term:`generic argument`.

:def_p:`fls_dalqke3rznrb`
All :syntax:`LifetimeParameter`\ s in a :syntax:`GenericParameterList` shall
precede all :syntax:`ConstantParameter`\ s and :syntax:`TypeParameter`\ s.

:def_p:`fls_gw8gutq2215z`
A :syntax:`LifetimeParameterList` shall be terminated by character 0x2C (comma)
when followed by a :syntax:`ConstantOrTypeParameterList`.

:def_p:`fls_pi6eukz7kc99`
A :term:`generic enum` is an :term:`enum` with :term:`generic parameter`\ s.

:def_p:`fls_ixmgqupxvf73`
A :term:`generic function` is a :term:`function` with :term:`generic parameter`\
s.

:def_p:`fls_z311nxou9yi3`
A :term:`generic implementation` is an :term:`implementation` with
:term:`generic parameter`\ s.

:def_p:`fls_wmcp0n36jlbr`
A :term:`generic struct` is a :term:`struct` with :term:`generic parameter`\ s.

:def_p:`fls_h42kg56vsefx`
A :term:`generic trait` is a :term:`trait` with :term:`generic parameter`\ s.

:def_p:`fls_372h3oevejih`
A :term:`generic type alias` is a :term:`type alias` with :term:`generic
parameter`\ s.

:def_p:`fls_u8mqct93yimd`
A :term:`generic union` is a :term:`union` with :term:`generic parameter`\ s.

:def_p:`fls_vpcqgec83ybt`
A :term:`constant parameter` is a :term:`generic parameter` for a
:term:`constant`.

:def_p:`fls_s0nrjwqg2wox`
A :term:`lifetime parameter` is a :term:`generic parameter` for a
:term:`lifetime`.

:def_p:`fls_95eooah0vcqx`
A :term:`type parameter` is a :term:`generic parameter` for a :term:`type`.

:def_p:`fls_x4s7p2v981r6`
A :term:`generic enum` shall use all of its :term:`type parameter`\ s and
:term:`lifetime parameter`\ s at least once in at least one of its :term:`enum
variant`\ s.

:def_p:`fls_jzfk9fspzqja`
A :term:`generic struct` shall use all of its :term:`type parameter`\ s
and :term:`lifetime parameter`\ s at least once in at least one of its
:term:`field`\ s.

:def_p:`fls_6j616ydf2mnh`
A :term:`generic union` shall use all of its :term:`type parameter`\ s
and :term:`lifetime parameter`\ s at least once in at least one of its
:term:`field`\ s.

:def_p:`fls_hyi2jnp38v1n`
A :term:`generic parameter` is said to :term:`constrain` an
:term:`implementation` if the :term:`generic parameter` appears at least once in
one of the following:

* :def_p:`fls_62b59qvom3nm`
  The :term:`implemented trait`, or

* :def_p:`fls_oq76uff9gp0k`
  The :term:`implementing type`, or

* :def_p:`fls_sseo6u6pbcki`
  As a :term:`binding argument` in the :term:`bound`\ s of a :term:`type` that
  contains another :term:`generic parameter` that :term:`constrain`\ s the
  :term:`implementation`.

:def_p:`fls_ua3w16qo9o4`
It is a static error if a :term:`type parameter` or :term:`constant
parameter` of an :term:`implementation` does not :term:`constrain` the
:term:`implementation`.

:def_p:`fls_w9ol06mldwb`
It is a static error if a :term:`lifetime parameter` of an
:term:`implementation` is used in an :term:`associated type` without
:term:`constrain`\ ing the :term:`implementation`.

:def_p:`fls_g2pfrqhmeys8`
The :term:`type` of a :term:`constant parameter` shall be a :term:`scalar type`.

:def_p:`fls_56jq9k9l31rt`
A :term:`constant parameter` shall be used in the following contexts:

* :def_p:`fls_sh669lnc5o1b`
  As a :term:`constant argument` in the signature and fields of an item.

* :def_p:`fls_h6kx8dxh5u96`
  In the :term:`initialization expression` of an :term:`associated constant`.

* :def_p:`fls_5r7ontjlmgwj`
  As a :term:`constant argument` of an :term:`associated type`'s
  :syntax:`InitializationType`.

* :def_p:`fls_prbwj1pmng6k`
  As a :term:`constant argument` of any :term:`type` used within a
  :term:`function body`.

* :def_p:`fls_byqjs5fvy2bj`
  As a :term:`value` of any :term:`expression` within a :term:`function body`.

:def_p:`fls_hidfwkwr2r73`
A :term:`type parameter` has an implicit :codeterm:`core::marker::Sized` bound,
unless a ``?core::marker::Sized`` bound is present.

:def_p:`fls_m0bzw4jap6sg`
A :term:`generic parameter` with a :term:`bound` of the form

.. code-block:: text

   	<X: Bound>

:def_p:`fls_vo7mgm34hwg2`
is equivalent to the :term:`generic parameter` without the bound and a
:term:`where clause` of the following form:

.. code-block:: text

   	where X: Bound

.. rubric:: Examples

.. code-block:: text

   struct Array<T, const N: usize>([T; N])

   fn generic_function<'a, T>() {}

   struct Reference<'a, T: 'a> {
       the_reference: &'a T
   }

Where Clauses
-------------

.. rubric:: Syntax

.. syntax::

   WhereClause ::=
       $$where$$ WhereClausePredicateList

   WhereClausePredicateList ::=
       WhereClausePredicate (, WhereClausePredicate)* $$,$$?

   WhereClausePredicate ::=
       LifetimePredicate
     | TypeBoundPredicate

   LifetimePredicate ::=
      LifetimeIndication $$:$$ LifetimeIndicationList?

   TypeBoundPredicate ::=
      ForGenericParameterList? TypeSpecification $$:$$ TypeBoundList?

.. rubric:: Legality Rules

:def_p:`fls_3nqb7p5ifvio`
A :term:`where clause` is a :term:`construct` that specifies when a
:term:`construct` with generic arguments supplied is valid.

:def_p:`fls_ytk74dyxuy6d`
A :term:`construct` is valid when all of its where clause predicates hold true
for the supplied generic arguments.

:def_p:`fls_fhy4rsmmbvyy`
A :term:`where clause predicate` is a :term:`construct` that specifies lifetime
bounds on :term:`lifetime parameter`\ s and trait :term:`bound`\ s and lifetimes
bounds on types.

:def_p:`fls_1xgw1dq60quz`
A :term:`trivial predicate` is a :term:`where clause predicate` that does not
use the :term:`generic parameter`\ s or :term:`higher-ranked lifetime`\ s of the
related :term:`construct`.

:def_p:`fls_47s8i7pzb9gg`
It is a static error to create a :term:`trivial predicate` that does not hold.

.. rubric:: Examples

.. code-block:: text

   struct Clause<T> where T: Iterator {
       field: T
   }

Generic Arguments
-----------------

.. rubric:: Syntax

.. syntax::

   GenericArgumentList ::=
       $$<$$ ( GenericArgument ($$,$$ GenericArgument)* $$,$$? )? $$>$$

   GenericArgument ::=
       BindingArgument
     | ConstantArgument
     | LifetimeArgument
     | TypeArgument

   ConstantArgument ::=
       BlockExpression
     | $$-$$? LiteralExpression
     | SimplePathSegment
   BindingArgument ::=
       Identifier $$=$$ TypeSpecification

   LifetimeArgument ::=
       LifetimeIndication

   TypeArgument ::=
       TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_3x6qd8vt5uus`
A :term:`generic argument` supplies a static input for an :term:`associated
trait type` or a  :term:`generic parameter`.

:def_p:`fls_ky39fb2vcom6`
A :syntax:`BindingArgument` shall follow :syntax:`ConstantArgument`\
s, :syntax:`LifetimeArgument`\ s, and :syntax:`TypeArgument`\ s in a
:syntax:`GenericArgumentList`.

:def_p:`fls_9n1ejjili06h`
A :syntax:`LifetimeArgument` shall precede :syntax:`BindingArgument`\
s, :syntax:`ConstantArgument`\ s, and :syntax:`TypeArgument`\ s in a
:syntax:`GenericArgumentList`.

:def_p:`fls_i3z9ueoe99zd`
A :term:`constant argument` is a :term:`generic argument` that supplies the
:term:`value` of a :term:`constant parameter`.

:def_p:`fls_d4vdvpihoeb1`
A :term:`type argument` is a :term:`generic argument` that supplies the
:term:`type` of a :term:`type parameter`.

:def_p:`fls_10k9gdxlpuls`
A :term:`lifetime argument` is a :term:`generic argument` that supplies the
:term:`lifetime` of a :term:`lifetime parameter`.

:def_p:`fls_9pda3ja0ihks`
A :term:`binding argument` is a :term:`generic argument` that supplies the
:term:`type` of an :term:`associated trait type`.

:def_p:`fls_al4dhmqodvwc`
A :term:`constant argument` may only appear as a single segment :term:`path
expression`, optionally inside a :term:`block expression`, inside of a
:term:`type` or :term:`array repeat expression`.

:def_p:`fls_ukarc98ceesz`
:term:`Generic argument`\ s are subject to :term:`generic conformance`.

.. rubric:: Examples

.. code-block:: text

   trait Trait {
       type Assoc;
   }


:def_p:`fls_l88o2snx9qbt`
The following is a generic function with a binding argument.

.. code-block:: text


   fn func<'lifetime, T, const C: usize>() where T: Trait<Assoc = usize> {}


:def_p:`fls_thpj9io9tyuy`
The following are generic arguments for ``func``.

.. syntax::


   func::<'static, u32, 0>();

Generic Conformance
-------------------

.. rubric:: Legality Rules

:def_p:`fls_gb3mpt5rxjoa`
A :term:`constant argument` is conformant with a :term:`constant parameter`
when the :term:`type`\ s of the :term:`constant argument` and :term:`constant
parameter` are :term:`unifiable`.

:def_p:`fls_kdeltu9dsd0d`
A :term:`lifetime argument` is conformant with a :term:`lifetime parameter` when
it outlives the lifetimes specified by the :term:`lifetime parameter`\ **.**

:def_p:`fls_ws1h57fk1mkh`
A :term:`type argument` is conformant with a :term:`type parameter` when the
:term:`type` of the :term:`type argument` fulfills the required :term:`trait
bound`\ s of the :term:`type parameter`.

:def_p:`fls_ltch5eivxgaa`
A :term:`binding argument` is conformant with an :term:`associated type` when
the supplied :term:`type` of the :term:`binding argument` fulfills the required
:term:`trait bound`\ s of the :term:`associated type`.

:def_p:`fls_w0ozotuwtr9`
:term:`Generic argument`\ s are conformant with :term:`generic parameter`\
s when

* :def_p:`fls_91bylteu35bi`
  The :term:`generic argument`\ s consist only of conformant :term:`constant
  argument`\ s, conformant :term:`lifetime argument`\ s, conformant :term:`type
  argument`\ s, and conformant :term:`binding argument`\ s, and

* :def_p:`fls_j6xtrxc6aik`
  Any remaining :term:`generic parameter`\ s without corresponding conformant
  :term:`generic argument`\ s are :term:`lifetime parameter`\ s with either
  :term:`inferred lifetime argument`\ s or :term:`elided lifetime`\ s, and

* :def_p:`fls_us7d30cbwgpz`
  All :term:`lifetime argument`\ s come first, followed by :term:`constant
  argument`\ s and :term:`type argument`\ s in the order as defined by the
  :term:`generic parameter`\ s, followed by :term:`binding argument`\ s, and

* :def_p:`fls_dp3hpvf0fmr8`
  All :term:`lifetime argument`\ s, :term:`constant argument`\ s and :term:`type
  argument`\ s have a corresponding :term:`generic parameter`.

:def_p:`fls_mg45zcguxxg5`
:term:`Generic argument`\ s shall be conformant.

