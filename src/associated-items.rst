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

:dp:`fls_ckzd25qd213t`
An :t:`associated item` is an :t:`item` that appears within an
:t:`implementation` or a :t:`trait`.

:dp:`fls_5y6ae0xqux57`
An :t:`associated constant` is a :t:`constant` that appears as an :t:`associated
item`.

:dp:`fls_lj7492aq7fzo`
An :t:`associated function` is a :t:`function` that appears as an :t:`associated
item`.

:dp:`fls_8cz4rdrklaj4`
An :t:`associated type` is a :t:`type alias` that appears as an :t:`associated
item`.

:dp:`fls_w8nu8suy7t5`
An :t:`associated type` shall not be used in the :t:`path-in-expression` of a
:t:`struct expression`\ ``.``

:dp:`fls_wasocqdnuzd1`
An :t:`associated type` with a :s:`TypeBoundList` shall appear only as an
:t:`associated trait type`.

:dp:`fls_l3iwn56n1uz8`
An :t:`associated implementation constant` is an :t:`associated constant` that
appears within an :t:`implementation`.

:dp:`fls_4ftfefcotb4g`
An :t:`associated implementation constant` shall have a :t:`constant
initializer`.

:dp:`fls_qb5qpfe0uwk`
An :t:`associated implementation function` is an :t:`associated function` that
appears within an :t:`implementation`.

:dp:`fls_1zlkeb6fz10j`
An :t:`associated implementation function` shall have a :t:`function body`.

:dp:`fls_tw8u0cc5867l`
An :t:`associated implementation type` is an :t:`associated type` that appears
within an :t:`implementation`.

:dp:`fls_bx7931x4155h`
An :t:`associated implementation type` shall have an :t:`initialization type`.

:dp:`fls_x564isbhobym`
An :t:`associated trait constant` is an :t:`associated constant` that appears
within a :t:`trait`.

:dp:`fls_b6nns7oqvdpm`
An :t:`associated trait function` is an :t:`associated function` that appears
within a :t:`trait`.

:dp:`fls_yyhebj4qyk34`
An :t:`associated trait type` is an :t:`associated type` that appears within
a :t:`trait`.

:dp:`fls_kl9p3ycl5mzf`
An :t:`associated trait type` shall not have an :t:`initialization type`.

:dp:`fls_a5prbmuruma4`
An :t:`associated trait type` has an implicit :c:`core::marker::Sized`
:t:`bound`.

:dp:`fls_vp2ov6ykueue`
An :t:`associated trait type` of the form

.. code-block:: text

   	trait T {
   	    type X: Bound;
   	}

:dp:`fls_5uf74nvdm64o`
is equivalent to a :t:`where clause` of the following form:

.. code-block:: text

   	trait T where Self::X: Bound {
   	    type X;
   	}

:dp:`fls_oy92gzxgc273`
A :t:`method` is an :t:`associated function` with a :t:`receiver`.

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

