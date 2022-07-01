.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Implementations
===============

.. rubric:: Syntax

.. syntax::

   Implementation ::=
       InherentImplementation
     | TraitImplementation

   InherentImplementation ::=
       $$impl$$ GenericParameterList? ImplementingType WhereClause? $${$$
         InnerAttributeOrDoc*
         AssociatedItem*
       $$}$$

   TraitImplementation ::=
       $$unsafe$$? $$impl$$ GenericParameterList? $$!$$? ImplementedTrait $$for$$ ImplementingType WhereClause? $${$$
         InnerAttributeOrDoc*
         AssociatedItem*
       $$}$$

   ImplementingType ::=
       TypeSpecification

   ImplementedTrait ::=
       TypePath

.. rubric:: Legality Rules

:def_p:`fls_ivxpoxggy7s6`
An :term:`implementation` is an :term:`item` that supplements an
:term:`implementing type` by extending its functionality.

:def_p:`fls_yopmjbnw8tbl`
An :term:`implementing type` is the :term:`type` that the :term:`[associated
item]s` of an :term:`implementation` are associated with.

:def_p:`fls_v0n0bna40dqr`
An :term:`inherent implementation` is an :term:`implementation` that adds direct
functionality.

:def_p:`fls_gufjaib913d6`
An :term:`implementing type` may have multiple :term:`[inherent
implementation]s`. :term:`[Inherent implementation]s` of the same
:term:`implementing type` share a common :term:`scope`.

:def_p:`fls_797etpdk5dyb`
:term:`[Inherent implementation]s` of the same :term:`implementing type` shall
be defined within the same :term:`crate`.

:def_p:`fls_ry3an0mwb63g`
A :term:`trait implementation` is an :term:`implementation` that adds
functionality specified by a :term:`trait`.

:def_p:`fls_8pwr7ibvhmhu`
An :term:`unsafe trait implementation` is a :term:`trait implementation` subject
to :term:`keyword` ``unsafe``.

:def_p:`fls_47x0ep8of8wr`
An :term:`implemented trait` is a :term:`trait` whose functionality has been
implemented by an :term:`implementation type`.

:def_p:`fls_agitlryvyc16`
The :term:`type path` of a :term:`trait implementation` shall resolve to a
:term:`trait`.

:def_p:`fls_mx5xjcejwa6u`
A :term:`trait implementation` shall be an :term:`unsafe trait implementation`
if and only if it :term:`[implement]s` an :term:`unsafe trait`.

:def_p:`fls_z78dg261oob6`
:term:`[Trait implementation]s` are subject to :term:`implementation coherence`
and :term:`implementation conformance`.

.. rubric:: Examples

.. code-block:: text

   trait Shape {
       fn area(self) -> f64;
   }


:def_p:`fls_yuyesijndu9n`
``Circle`` is an implementing type.

.. code-block:: text


   struct Circle {
       radius: f64
   }


:def_p:`fls_o62i75sjzp9y`
The following is an inherent implementation:

.. code-block:: text


   impl Circle {
       fn set_radius(mut self, new_radius: f64) {
           self.radius = new_radius;
       }
   }


:def_p:`fls_a2utf0tmuhy4`
The following is a trait implementation:

.. code-block:: text


   impl Shape for Circle {
       fn area(self) -> f64 {
           self.radius.powi(2) * std::f64::consts::PI
       }
   }


Implementation Coherence
------------------------

.. rubric:: Legality Rules

:def_p:`fls_swdusjwzgksx`
Two :term:`[trait implementation]s` of the same :term:`implemented trait`
overlap when the intersection of the :term:`[implemented type]s` is non-empty.

:def_p:`fls_ir7hp941ky8t`
Given :term:`trait implementation` ``impl<P1, P2, .., PN> Trait<T1, T2, .., TN>
for T0``, the :term:`trait implementation` is considered valid when

* :def_p:`fls_3tbm20k2ixol`
  ``Trait`` is a :term:`local trait`, or

* :def_p:`fls_lscc9ileg3gm`
  All of

  * :def_p:`fls_9klwbsh3vlxu`
    At least one of :term:`[type]s` ``T0, T1, .., TN`` is a :term:`local type`.

* * :def_p:`fls_9gmc1tcscq9v`
    No :term:`type parameter` of ``P1, P2, .., PN`` that is not used in another
    :term:`type` may appear in the :term:`non-[local type]s` of ``T0, T1, ..,
    TN``.

:def_p:`fls_fv1l4yjuut7p`
A :term:`trait implementation` is coherent when it is valid and does not overlap
with another :term:`trait implementation`.

:def_p:`fls_koy70k770ayu`
A :term:`trait implementation` shall be coherent.

Implementation Conformance
--------------------------

.. rubric:: Legality Rules

:def_p:`fls_v31idwjau90d`
An :term:`associated trait constant` is conformant with an :term:`associated
constant` of an :term:`implemented trait` when

* :def_p:`fls_k3wfh5japmyw`
  The :term:`[name]s` of both :term:`[associated constant]s` are the same, and

* :def_p:`fls_11qrqfuc3rmh`
  The :term:`[type]s` of both :term:`[associated constant]s` are the same.

:def_p:`fls_qmhduwunxww0`
An :term:`associated trait function` is conformant with an :term:`associated
function` of an :term:`implemented trait` when the :term:`[function signature]s`
of both :term:`[function]s` are the same.

* :def_p:`fls_2500ivh0cc3y`
  The :term:`signature` of the :term:`associated function` of the
  :term:`implemented trait` is a :term:`subtype` of the :term:`signature` of the
  :term:`associated trait function`, and

* :def_p:`fls_18gimgfy0kw9`
  The :term:`[bound]s` of the :term:`associated function` of the
  :term:`implemented trait` are more general that the :term:`[bound]s` of the
  :term:`associated trait function`.

:def_p:`fls_fi4qmauirlsm`
An :term:`associated type` of a :term:`trait implementation` is conformant with
an :term:`associated type` of an :term:`implemented trait` when:

* :def_p:`fls_2s8lh3k4rw6u`
  The :term:`[name]s` of both :term:`[type]s` are the same, and

* :def_p:`fls_bb874uu2alt3`
  The :term:`type specification` of the :term:`associated type` of the
  :term:`implemented trait` conforms to the :term:`[bound]s` of the
  :term:`associated type` of the :term:`trait implementation`.

:def_p:`fls_so8em6rphkhv`
A :term:`trait implementation` is conformant with an :term:`implemented trait`
when:

* :def_p:`fls_ldu9bmb9cy10`
  The :term:`trait implementation` has a conformant :term:`associated constant`
  for each :term:`associated constant` of the :term:`implemented trait`,
  unless the :term:`associated constant` of the :term:`implemented trait` has a
  :term:`default value`,

* :def_p:`fls_5cr6un2gzdft`
  The :term:`trait implementation` has a conformant :term:`associated function`
  for each :term:`associated function` of the :term:`implemented trait`, unless
  the :term:`associated function` of the :term:`implemented trait` has a default
  implementation in the :term:`implemented trait`,

* :def_p:`fls_pshfe3ioh0mg`
  The :term:`trait implementation` has a conformant :term:`associated type` for
  each :term:`associated type` of the :term:`implemented trait`.

:def_p:`fls_8yq1g7nzv9px`
A :term:`trait implementation` shall be conformant.

