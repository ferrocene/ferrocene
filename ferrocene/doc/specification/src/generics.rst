.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_y2k5paj8m8ug:

Generics
========

.. _fls_vhpwge5123cm:

Generic Parameters
------------------

.. rubric:: Syntax

.. syntax::

   GenericParameterList ::=
       $$<$$ (GenericParameter ($$,$$ GenericParameter)* $$,$$?)? $$>$$

   GenericParameter ::=
       OuterAttributeOrDoc* (
           ConstantParameter
         | LifetimeParameter
         | TypeParameter
       )

   ConstantParameter ::=
      $$const$$ Name TypeAscription ($$=$$ ConstantParameterInitializer)?

   ConstantParameterInitializer ::=
       BlockExpression
     | Identifier
     | $$-$$? LiteralExpression

   LifetimeParameter ::=
       Lifetime ($$:$$ LifetimeIndicationList)?

   TypeParameter ::=
       Name ($$:$$ TypeBoundList?)? ($$=$$ TypeParameterInitializer)?

   TypeParameterInitializer ::=
       TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_sye3d17l9bf5`
A :t:`generic parameter` is a placeholder for a :t:`constant`, a :t:`lifetime`,
or a :t:`type`, whose :t:`constant`, :t:`lifetime`, or :t:`type` is supplied
statically by a :t:`generic argument`.

:dp:`fls_dalqke3rznrb`
All :s:`[LifetimeParameter]s` in a :s:`GenericParameterList` shall precede all
:s:`[ConstantParameter]s` and :s:`[TypeParameter]s`.

:dp:`fls_pi6eukz7kc99`
A :t:`generic enum` is an :t:`enum` with :t:`[generic parameter]s`.

:dp:`fls_ixmgqupxvf73`
A :t:`generic function` is a :t:`function` with :t:`[generic parameter]s`.

:dp:`fls_z311nxou9yi3`
A :t:`generic implementation` is an :t:`implementation` with
:t:`[generic parameter]s`.

:dp:`fls_wmcp0n36jlbr`
A :t:`generic struct` is a :t:`struct` with :t:`[generic parameter]s`.

:dp:`fls_h42kg56vsefx`
A :t:`generic trait` is a :t:`trait` with :t:`[generic parameter]s`.

:dp:`fls_372h3oevejih`
A :t:`generic type alias` is a :t:`type alias` with :t:`[generic parameter]s`.

:dp:`fls_u8mqct93yimd`
A :t:`generic union` is a :t:`union` with :t:`[generic parameter]s`.

:dp:`fls_vpcqgec83ybt`
A :t:`constant parameter` is a :t:`generic parameter` for a :t:`constant`.

:dp:`fls_3SjMBlc0b7qo`
A :t:`constant parameter initializer` is a :t:`construct` that provides the
default :t:`value` of its related :t:`constant parameter`.

:dp:`fls_p4yb8EAXlRU0`
A :t:`constant parameter initializer` shall be a :t:`constant expression`.

:dp:`fls_4a2qshaf5se7`
It is a static error to use a :t:`generic parameter` in the
:t:`discriminant initializer` of an :t:`enum variant`.

:dp:`fls_s0nrjwqg2wox`
A :t:`lifetime parameter` is a :t:`generic parameter` for a :t:`lifetime`.

:dp:`fls_2grtygcj8o3`
A :t:`lifetime parameter` shall not be used within a :t:`constant context`,
except for the ``'static`` :t:`lifetime`.

:dp:`fls_95eooah0vcqx`
A :t:`type parameter` is a :t:`generic parameter` for a :t:`type`.

:dp:`fls_ahCqtkh0m5sR`
A :t:`type parameter initializer` is a :t:`construct` that provides the
default :t:`value` of its related :t:`type parameter`.

:dp:`fls_3qZRBp9j26w3`
The :t:`type` of the :t:`type parameter initializer` of a :t:`type parameter`
shall satisfy the :t:`[trait bound]s` of the :t:`type parameter`.

:dp:`fls_x4s7p2v981r6`
A :t:`generic enum` shall use all of its :t:`[type parameter]s` and
:t:`[lifetime parameter]s` at least once in at least one of its
:t:`[enum variant]s`.

:dp:`fls_jzfk9fspzqja`
A :t:`generic struct` shall use all of its :t:`[type parameter]s` and
:t:`[lifetime parameter]s` at least once in at least one of its :t:`[field]s`.

:dp:`fls_6j616ydf2mnh`
A :t:`generic union` shall use all of its :t:`[type parameter]s` and
:t:`[lifetime parameter]s` at least once in at least one of its :t:`[field]s`.

:dp:`fls_hyi2jnp38v1n`
A :t:`generic parameter` is said to constrain an :t:`implementation` if the
:t:`generic parameter` appears at least once in one of the following:

* :dp:`fls_sseo6u6pbcki`
  As a :t:`binding argument` in the :t:`[trait bound]s` of a :t:`type` that
  contains another :t:`generic parameter` that constrains the
  :t:`implementation`, or

* :dp:`fls_62b59qvom3nm`
  The :t:`implemented trait`, or

* :dp:`fls_oq76uff9gp0k`
  The :t:`implementing type`.

:dp:`fls_ua3w16qo9o4`
It is a static error if a :t:`constant parameter` or a :t:`type parameter` of
an :t:`implementation` does not constrain the :t:`implementation`.

:dp:`fls_w9ol06mldwb`
It is a static error if a :t:`lifetime parameter` of an :t:`implementation`
is used in an :t:`associated type` without constraining the
:t:`implementation`.

:dp:`fls_g2pfrqhmeys8`
The :t:`type` of a :t:`constant parameter` shall be a :t:`scalar type`.

:dp:`fls_56jq9k9l31rt`
A :t:`constant parameter` shall be used in the following contexts:

* :dp:`fls_sh669lnc5o1b`
  As a :t:`constant argument` in the signature and :t:`[field]s` of an
  :t:`item`.

* :dp:`fls_h6kx8dxh5u96`
  In the :t:`constant initializer` of an :t:`associated constant`.

* :dp:`fls_5r7ontjlmgwj`
  As a :t:`constant argument` of an :t:`[associated type]'s`
  :s:`InitializationType`.

* :dp:`fls_prbwj1pmng6k`
  As a :t:`constant argument` of any :t:`type` used within a :t:`function body`.

* :dp:`fls_byqjs5fvy2bj`
  As a :t:`value` of any :t:`expression` within a :t:`function body`.

:dp:`fls_hidfwkwr2r73`
A :t:`type parameter` has an implicit :std:`core::marker::Sized` :t:`bound`,
unless an :t:`opt-out trait bound` for the :std:`core::marker::Sized` :t:`trait`
is present.

:dp:`fls_yaYedRNaxhTe`
A :t:`type parameter` of an :t:`abstract data type` has implicit
:t:`[lifetime bound]s` depending on its usage in the :t:`[field]s` of the
:t:`abstract data type` as follows:

* :dp:`fls_axMVssU7an8p`
  If the :t:`type parameter` is used in a :t:`reference type`, then the
  :t:`type parameter` has an implicit :t:`lifetime bound` for the
  :t:`lifetime` of the :t:`reference type`,

* :dp:`fls_iAnCoxISuxxb`
  If the :t:`type parameter` is used as a :t:`generic argument` of an
  :t:`abstract data type`, the :t:`type parameter` inherits all the
  :t:`[lifetime bound]s` from the corresponding :t:`generic parameter` of
  the :t:`generic argument`.

:dp:`fls_wUwJEGzjCUAT`
A :t:`type parameter` of a :t:`function` has implicit :t:`[lifetime bound]s`
depending on its usages in the :t:`[function parameter]s` and :t:`return type`
as follows:

* :dp:`fls_BN5nUuvq9AwY`
  If the :t:`type parameter` is used in a :t:`reference type`, then the
  :t:`type parameter` has an implicit :t:`lifetime bound` for the :t:`lifetime`
  of the :t:`reference type`,

* :dp:`fls_ouOm2TQKNsP3`
  If the :t:`type parameter` is used as a :t:`generic argument` of an
  :t:`abstract data type`, the :t:`type parameter` inherits all the
  :t:`[lifetime bound]s` from the corresponding :t:`generic parameter` of
  the :t:`generic argument`.

:dp:`fls_m0bzw4jap6sg`
A :t:`generic parameter` with a :t:`bound` of the form

.. code-block:: rust

   	<X: Bound>

:dp:`fls_vo7mgm34hwg2`
is equivalent to the :t:`generic parameter` without the bound and a
:t:`where clause` of the following form:

.. code-block:: rust

   	where X: Bound

.. rubric:: Examples

.. code-block:: rust

   struct Array<T, const N: usize>([T; N])

   fn generic_function<'a, T>() {}

   struct Reference<'a, T: 'a> {
       the_reference: &'a T
   }

.. _fls_7nv8ualeaqe3:

Where Clauses
-------------

.. rubric:: Syntax

.. syntax::

   WhereClause ::=
       $$where$$ WhereClausePredicateList

   WhereClausePredicateList ::=
       WhereClausePredicate (, WhereClausePredicate)* $$,$$?

   WhereClausePredicate ::=
       LifetimeBoundPredicate
     | TypeBoundPredicate

   LifetimeBoundPredicate ::=
      LifetimeIndication $$:$$ LifetimeIndicationList?

   TypeBoundPredicate ::=
      ForGenericParameterList? TypeSpecification $$:$$ TypeBoundList?

.. rubric:: Legality Rules

:dp:`fls_3nqb7p5ifvio`
A :t:`where clause` is a :t:`construct` that specifies :t:`[bound]s` on
:t:`[lifetime parameter]s` and :t:`[type]s` that have
to hold for the :t:`construct` subject to the :t:`where clause` to be valid.

:dp:`fls_fhy4rsmmbvyy`
A :t:`where clause predicate` is either a :t:`lifetime bound predicate` or a
:t:`type bound predicate`.

:dp:`fls_V4PKFqtCsAv6`
A :t:`lifetime bound predicate` is a :t:`construct` that specifies
:t:`[lifetime bound]s` on a :t:`lifetime parameter`.

:dp:`fls_cslGPmVjujHD`
A :t:`type bound predicate` is a :t:`construct` that specifies
:t:`[lifetime bound]s` and :t:`[trait bound]s` on a :t:`type`.

:dp:`fls_ytk74dyxuy6d`
A :t:`construct` is valid when all of its :t:`[where clause predicate]s` hold
true for the supplied :t:`[generic argument]s`.

:dp:`fls_1xgw1dq60quz`
A :t:`trivial predicate` is a :t:`where clause predicate` that does not use
the :t:`[generic parameter]s` or :t:`[higher-ranked trait bound]s` of the related
:t:`construct`.

:dp:`fls_47s8i7pzb9gg`
It is a static error to create a :t:`trivial predicate` that does not hold.

.. rubric:: Examples

.. code-block:: rust

   struct Clause<T> where T: Iterator {
       field: T
   }

.. _fls_utuu8mdbuyxm:

Generic Arguments
-----------------

.. rubric:: Syntax

.. syntax::

   GenericArgumentList ::=
       $$<$$ ( GenericArgument ($$,$$ GenericArgument)* $$,$$? )? $$>$$

   GenericArgument ::=
       BindingArgument
     | BindingBoundArgument
     | ConstantArgument
     | LifetimeArgument
     | TypeArgument

   BindingArgument ::=
       Identifier $$=$$ TypeSpecification

   BindingBoundArgument ::=
       Identifier $$:$$ TypeBoundList

   ConstantArgument ::=
       BlockExpression
     | $$-$$? LiteralExpression
     | Identifier

   LifetimeArgument ::=
       LifetimeIndication

   TypeArgument ::=
       TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_3x6qd8vt5uus`
A :t:`generic argument` supplies a static input for an
:t:`associated trait type` or a :t:`generic parameter`.

:dp:`fls_ky39fb2vcom6`
A :s:`BindingArgument` shall follow :s:`[ConstantArgument]s`,
:s:`[LifetimeArgument]s`, and :s:`[TypeArgument]s` in a
:s:`GenericArgumentList`.

:dp:`fls_9n1ejjili06h`
A :s:`LifetimeArgument` shall precede :s:`[BindingArgument]s`,
:s:`[ConstantArgument]s`, and :s:`[TypeArgument]s` in a
:s:`GenericArgumentList`.

:dp:`fls_9pda3ja0ihks`
A :t:`binding argument` is a :t:`generic argument` that supplies the :t:`type`
of an :t:`associated trait type`.

:dp:`fls_mcUMWsYcxzmZ`
A :t:`binding bound argument` is a :t:`generic argument` that further imposes
:t:`[bound]s` on an :t:`associated trait type`.

:dp:`fls_dxMfAI4EZVS5`
A :t:`binding bound argument` shall only be used within the confines of a
:t:`type bound predicate`'s :t:`[bound]s`, :t:`[impl trait type]`'s
:t:`[bound]s`, :t:`associated type`'s :t:`[bound]s` or :t:`trait`'s
:t:`[supertrait]s`.

:dp:`fls_i3z9ueoe99zd`
A :t:`constant argument` is a :t:`generic argument` that supplies the
:t:`value` of a :t:`constant parameter`.

:dp:`fls_al4dhmqodvwc`
A :t:`constant argument` may only appear as a single segment
:t:`path expression`, optionally encapsulated in a :t:`block expression`, within
an :t:`array repetition constructor` or a :t:`type`.

:dp:`fls_10k9gdxlpuls`
A :t:`lifetime argument` is a :t:`generic argument` that supplies the
:t:`lifetime` of a :t:`lifetime parameter`.

:dp:`fls_d4vdvpihoeb1`
A :t:`type argument` is a :t:`generic argument` that supplies the :t:`type` of
a :t:`type parameter`.

:dp:`fls_ukarc98ceesz`
:t:`[Generic argument]s` are subject to :t:`generic conformance`.

.. rubric:: Examples

.. code-block:: rust

   trait Trait {
       type Assoc;
   }

:dp:`fls_l88o2snx9qbt`
The following is a generic function with a binding argument.

.. code-block:: rust

   fn func<'lifetime, T, const C: usize>() where T: Trait<Assoc = usize> {}

:dp:`fls_thpj9io9tyuy`
The following are generic arguments for ``func``.

.. syntax::

   func::<'static, u32, 0>();

.. _fls_i7g2n7hfg3ch:

Generic Conformance
-------------------

.. rubric:: Legality Rules

:dp:`fls_CBWyxBJeYeb2`
:t:`Generic conformance` measures the compatibility between a set of
:t:`[generic parameter]s` and a set of :t:`[generic argument]s`.

:dp:`fls_ltch5eivxgaa`
A :t:`binding argument` is conformant with an :t:`associated type` when the
supplied :t:`type` of the :t:`binding argument` fulfills the required
:t:`[trait bound]s` of the :t:`associated type`.

:dp:`fls_gb3mpt5rxjoa`
A :t:`constant argument` is conformant with a :t:`constant parameter` when
the :t:`[type]s` of the :t:`constant argument` and the :t:`constant parameter`
are :t:`unifiable`.

:dp:`fls_kdeltu9dsd0d`
A :t:`lifetime argument` is conformant with a :t:`lifetime parameter` when it
outlives the :t:`lifetime` specified by the :t:`lifetime parameter`.

:dp:`fls_ws1h57fk1mkh`
A :t:`type argument` is conformant with a :t:`type parameter` when the
:t:`type` of the :t:`type argument` fulfills the required :t:`[trait bound]s`
of the :t:`type parameter`.

:dp:`fls_w0ozotuwtr9`
:t:`[Generic argument]s` are conformant with :t:`[generic parameter]s` when

* :dp:`fls_91bylteu35bi`
  The :t:`[generic argument]s` consist only of conformant
  :t:`[binding argument]s`, conformant :t:`[constant argument]s`, conformant
  :t:`[lifetime argument]s`, and conformant :t:`[type argument]s`, and

* :dp:`fls_j6xtrxc6aik`
  Any remaining :t:`[generic parameter]s` without corresponding conformant
  :t:`[generic argument]s` are :t:`[constant parameter]s` with
  :t:`[constant parameter initializer]s`, :t:`[lifetime parameter]s` with
  either inferred :t:`[lifetime argument]s` or :t:`[elided lifetime]s`,
  :t:`[type parameter]s` with :t:`[type parameter initializer]s` or inferred
  :t:`[type argument]s`, and

* :dp:`fls_us7d30cbwgpz`
  All :t:`[lifetime argument]s` come first, followed by
  :t:`[constant argument]s` and :t:`[type argument]s` in the order defined by
  the :t:`[generic parameter]s`, followed by :t:`[binding argument]s`, and

* :dp:`fls_dp3hpvf0fmr8`
  All :t:`[constant argument]s`, :t:`[lifetime argument]s`, and
  :t:`[type argument]s` have a corresponding :t:`generic parameter`.

:dp:`fls_mg45zcguxxg5`
:t:`[Generic argument]s` shall be conformant.

:dp:`fls_mDgq5XjzKAl3`
The :t:`value` of a :t:`constant parameter` is determined as follows:

* :dp:`fls_YufUgB25ovh3`
  If the :t:`constant parameter` has a conformant :t:`constant argument`, then
  the :t:`value` is that of the :t:`constant argument`.

* :dp:`fls_OhVxhJ23x7W2`
  Otherwise, if the :t:`constant parameter` has a
  :t:`constant parameter initializer`, then the :t:`value` is that of the
  :t:`constant parameter initializer`.

* :dp:`fls_Kyar0jH9BqeW`
  Otherwise this is a static error.
