.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Associated Items
================

.. rubric:: Syntax

.. syntax::

   AssociatedItem ::=
       OuterAttributeOrDoc* (AssociatedItemWithVisibility | TerminatedMacroInvocation)

   AssociatedItemWithVisibility ::=
       VisibilityModifier? (
           ConstantDeclaration
         | FunctionDeclaration
         | TypeDeclaration
       )

.. rubric:: Legality Rules

:def_p:`fls_ckzd25qd213t`
An :term:`associated item` is an :term:`item` that appears within an
:term:`implementation` or a :term:`trait`.

:def_p:`fls_5y6ae0xqux57`
An :term:`associated constant` is a :term:`constant` that appears as an
:term:`associated item`.

:def_p:`fls_lj7492aq7fzo`
An :term:`associated function` is a :term:`function` that appears as an
:term:`associated item`.

:def_p:`fls_8cz4rdrklaj4`
An :term:`associated type` is a :term:`type alias` that appears as an
:term:`associated item`.

:def_p:`fls_w8nu8suy7t5`
An :term:`associated type` shall not be used in the :term:`path-in-expression`
of a :term:`struct expression`\ ``.``

:def_p:`fls_wasocqdnuzd1`
An :term:`associated type` with a :syntax:`TypeBoundList` shall appear only as
an :term:`associated trait type`.

:def_p:`fls_l3iwn56n1uz8`
An :term:`associated implementation constant` is an :term:`associated constant`
that appears within an :term:`implementation`.

:def_p:`fls_4ftfefcotb4g`
An :term:`associated implementation constant` shall have a :term:`constant
initializer`.

:def_p:`fls_qb5qpfe0uwk`
An :term:`associated implementation function` is an :term:`associated function`
that appears within an :term:`implementation`.

:def_p:`fls_1zlkeb6fz10j`
An :term:`associated implementation function` shall have a :term:`function
body`.

:def_p:`fls_tw8u0cc5867l`
An :term:`associated implementation type` is an :term:`associated type` that
appears within an :term:`implementation`.

:def_p:`fls_bx7931x4155h`
An :term:`associated implementation type` shall have an :term:`initialization
type`.

:def_p:`fls_x564isbhobym`
An :term:`associated trait constant` is an :term:`associated constant` that
appears within a :term:`trait`.

:def_p:`fls_b6nns7oqvdpm`
An :term:`associated trait function` is an :term:`associated function` that
appears within a :term:`trait`.

:def_p:`fls_yyhebj4qyk34`
An :term:`associated trait type` is an :term:`associated type` that appears
within a :term:`trait`.

:def_p:`fls_kl9p3ycl5mzf`
An :term:`associated trait type` shall not have an :term:`initialization type`.

:def_p:`fls_a5prbmuruma4`
An :term:`associated trait type` has an implicit :codeterm:`core::marker::Sized`
:term:`bound`.

:def_p:`fls_vp2ov6ykueue`
An :term:`associated trait type` of the form

.. code-block:: text

   	trait T {
   	    type X: Bound;
   	}

:def_p:`fls_5uf74nvdm64o`
is equivalent to a :term:`where clause` of the following form:

.. code-block:: text

   	trait T where Self::X: Bound {
   	    type X;
   	}

:def_p:`fls_oy92gzxgc273`
A :term:`method` is an :term:`associated function` with a :term:`receiver`.

.. rubric:: Examples

.. code-block:: text

   trait Greeter {
       const MAX_GREETINGS: i32;

       fn greet(self, message: &str);
   }

   struct Implementor {
       delivered_greetings: i32
   }

   impl Greeter for Implementor {
       const MAX_GREETINGS: i32 = 42;

       fn greet(mut self, message: &str) {
           if self.delivered_greetings < Self::MAX_GREETINGS {
               self.delivered_greetings += 1;
               println!("{}", message);
           }
       }
   }

