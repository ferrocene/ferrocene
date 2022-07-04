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

:dp:`fls_ivxpoxggy7s6`
An :t:`implementation` is an :t:`item` that supplements an :t:`implementing
type` by extending its functionality.

:dp:`fls_yopmjbnw8tbl`
An :t:`implementing type` is the :t:`type` that the :t:`[associated item]s` of
an :t:`implementation` are associated with.

:dp:`fls_v0n0bna40dqr`
An :t:`inherent implementation` is an :t:`implementation` that adds direct
functionality.

:dp:`fls_gufjaib913d6`
An :t:`implementing type` may have multiple :t:`[inherent implementation]s`.
:t:`[Inherent implementation]s` of the same :t:`implementing type` share a
common :t:`scope`.

:dp:`fls_797etpdk5dyb`
:t:`[Inherent implementation]s` of the same :t:`implementing type` shall be
defined within the same :t:`crate`.

:dp:`fls_ry3an0mwb63g`
A :t:`trait implementation` is an :t:`implementation` that adds functionality
specified by a :t:`trait`.

:dp:`fls_8pwr7ibvhmhu`
An :t:`unsafe trait implementation` is a :t:`trait implementation` subject to
:t:`keyword` ``unsafe``.

:dp:`fls_47x0ep8of8wr`
An :t:`implemented trait` is a :t:`trait` whose functionality has been
implemented by an :t:`implementation type`.

:dp:`fls_agitlryvyc16`
The :t:`type path` of a :t:`trait implementation` shall resolve to a :t:`trait`.

:dp:`fls_mx5xjcejwa6u`
A :t:`trait implementation` shall be an :t:`unsafe trait implementation` if and
only if it :t:`[implement]s` an :t:`unsafe trait`.

:dp:`fls_z78dg261oob6`
:t:`[Trait implementation]s` are subject to :t:`implementation coherence` and
:t:`implementation conformance`.

.. rubric:: Examples

.. code-block:: text

   trait Shape {
       fn area(self) -> f64;
   }

:dp:`fls_yuyesijndu9n`
``Circle`` is an implementing type.

.. code-block:: text

   struct Circle {
       radius: f64
   }

:dp:`fls_o62i75sjzp9y`
The following is an inherent implementation:

.. code-block:: text

   impl Circle {
       fn set_radius(mut self, new_radius: f64) {
           self.radius = new_radius;
       }
   }

:dp:`fls_a2utf0tmuhy4`
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

:dp:`fls_swdusjwzgksx`
Two :t:`[trait implementation]s` of the same :t:`implemented trait` overlap when
the intersection of the :t:`[implemented type]s` is non-empty.

:dp:`fls_ir7hp941ky8t`
Given :t:`trait implementation` ``impl<P1, P2, .., PN> Trait<T1, T2, .., TN> for
T0``, the :t:`trait implementation` is considered valid when

* :dp:`fls_3tbm20k2ixol`
  ``Trait`` is a :t:`local trait`, or

* :dp:`fls_lscc9ileg3gm`
  All of

  * :dp:`fls_9klwbsh3vlxu`
    At least one of :t:`[type]s` ``T0, T1, .., TN`` is a :t:`local type`.

* * :dp:`fls_9gmc1tcscq9v`
    No :t:`type parameter` of ``P1, P2, .., PN`` that is not used in another
    :t:`type` may appear in the :t:`non-[local type]s` of ``T0, T1, .., TN``.

:dp:`fls_fv1l4yjuut7p`
A :t:`trait implementation` is coherent when it is valid and does not overlap
with another :t:`trait implementation`.

:dp:`fls_koy70k770ayu`
A :t:`trait implementation` shall be coherent.

Implementation Conformance
--------------------------

.. rubric:: Legality Rules

:dp:`fls_v31idwjau90d`
An :t:`associated trait constant` is conformant with an :t:`associated constant`
of an :t:`implemented trait` when

* :dp:`fls_k3wfh5japmyw`
  The :t:`[name]s` of both :t:`[associated constant]s` are the same, and

* :dp:`fls_11qrqfuc3rmh`
  The :t:`[type]s` of both :t:`[associated constant]s` are the same.

:dp:`fls_qmhduwunxww0`
An :t:`associated trait function` is conformant with an :t:`associated function`
of an :t:`implemented trait` when the :t:`[function signature]s` of both
:t:`[function]s` are the same.

* :dp:`fls_2500ivh0cc3y`
  The :t:`signature` of the :t:`associated function` of the :t:`implemented
  trait` is a :t:`subtype` of the :t:`signature` of the :t:`associated trait
  function`, and

* :dp:`fls_18gimgfy0kw9`
  The :t:`[bound]s` of the :t:`associated function` of the :t:`implemented
  trait` are more general that the :t:`[bound]s` of the :t:`associated trait
  function`.

:dp:`fls_fi4qmauirlsm`
An :t:`associated type` of a :t:`trait implementation` is conformant with an
:t:`associated type` of an :t:`implemented trait` when:

* :dp:`fls_2s8lh3k4rw6u`
  The :t:`[name]s` of both :t:`[type]s` are the same, and

* :dp:`fls_bb874uu2alt3`
  The :t:`type specification` of the :t:`associated type` of the :t:`implemented
  trait` conforms to the :t:`[bound]s` of the :t:`associated type` of the
  :t:`trait implementation`.

:dp:`fls_so8em6rphkhv`
A :t:`trait implementation` is conformant with an :t:`implemented trait` when:

* :dp:`fls_ldu9bmb9cy10`
  The :t:`trait implementation` has a conformant :t:`associated constant`
  for each :t:`associated constant` of the :t:`implemented trait`, unless the
  :t:`associated constant` of the :t:`implemented trait` has a :t:`default
  value`,

* :dp:`fls_5cr6un2gzdft`
  The :t:`trait implementation` has a conformant :t:`associated function`
  for each :t:`associated function` of the :t:`implemented trait`, unless
  the :t:`associated function` of the :t:`implemented trait` has a default
  implementation in the :t:`implemented trait`,

* :dp:`fls_pshfe3ioh0mg`
  The :t:`trait implementation` has a conformant :t:`associated type` for each
  :t:`associated type` of the :t:`implemented trait`.

:dp:`fls_8yq1g7nzv9px`
A :t:`trait implementation` shall be conformant.

