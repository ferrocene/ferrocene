.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Values and Objects
==================

.. rubric:: Legality Rules

:dp:`fls_buyaqara7am4`
A :t:`value` is either a :t:`literal` or the result of a computation, that may
be stored in a memory location, and interpreted based on some :t:`type`.

:dp:`fls_rixdyyc525xp`
Two :t:`[value]s` :t:`overlap` when

* :dp:`fls_m6ctqq70vcxr`
  Both :t:`[value]s` are the same, or

* :dp:`fls_s231d18x5eay`
  One :t:`value` is of an :t:`abstract data type` and the other denotes a
  :t:`field` of the same :t:`value`, or

* :dp:`fls_dfr4yqo93fsn`
  One :t:`value` denotes an :t:`array` and the other denotes an element of the
  same :t:`array`, or

* :dp:`fls_eoak5mdl6ma`
  Both :t:`[value]s` are elements of the same :t:`array`.

:dp:`fls_jmwhiz1qrtmy`
An :t:`object` relates a :t:`value` to a :t:`name`, and dictates how the
:t:`value` is initialized and modified.

:dp:`fls_prxicw2q70lj`
An :t:`object` is :t:`valid` when it has been :t:`initialized` by all reachable
control flow paths.

.. rubric:: Undefined Behavior

:dp:`fls_6lg0oaaopc26`
It is undefined behavior to create a :t:`value` from uninitialized memory.

Constants
---------

.. rubric:: Syntax

.. syntax::

   ConstantDeclaration ::=
       $$const$$ (Name | $$_$$) TypeAscription ConstantInitializer? $$;$$

   ConstantInitializer ::=
       $$=$$ Expression

.. rubric:: Legality Rules

:dp:`fls_5o5iu4j8in4l`
A :t:`constant` is an :t:`immutable` :t:`object` that is not associated with a
specific memory location. The address of a :t:`constant` may differ from other
:t:`[object]s` derived from the same :t:`constant`.

:dp:`fls_3mhj0kkupwuz`
An :t:`unnamed constant` is a :t:`constant` declared with character 0x5F (low
line).

:dp:`fls_ka4y2yd100dx`
The :t:`type specification` of a :t:`constant` shall have a :t:`static
lifetime`.

:dp:`fls_vt9tlkd676ql`
The :t:`type` of a :t:`constant` shall implement the :std:`core::marker::Sized`
:t:`trait`.

:dp:`fls_ndmfqxjpvsqy`
A :t:`constant initializer` is a :t:`construct` that provides the :t:`value` of
its related :t:`constant`.

:dp:`fls_6rxwbbhf5tc5`
A :t:`constant` shall have a :t:`constant initializer`, unless it is an
:t:`associated trait constant`.

:dp:`fls_vnc3ttnid1qr`
The :t:`expression` of a :t:`constant initializer` shall be a :t:`constant
expression`.

:dp:`fls_deuo1pn8cjd6`
The value of a :t:`constant` is determined by evaluating its :t:`constant
initializer`.

:dp:`fls_5x0jv4cgbolx`
A use of a :t:`constant` is a :t:`value expression` and creates a copy of the
constant's value.

.. rubric:: Dynamic Semantics

:dp:`fls_xezt9hl069h4`
The :t:`elaboration` of a :t:`constant` evaluates its :t:`constant initializer`.

:dp:`fls_ndobth7s92if`
A :t:`path` that refers to a :t:`constant` is replaced with the :t:`value` of
the :t:`constant`.

.. rubric:: Examples

.. code-block:: text

   const ZERO: u32 = 0;

Statics
-------

.. rubric:: Syntax

.. syntax::

   StaticDeclaration ::=
       $$static$$ $$mut$$? Name TypeAscription StaticInitializer? $$;$$

   StaticInitializer ::=
   $$=$$ Expression

.. rubric:: Legality Rules

:dp:`fls_ibrmiwfypldh`
A :t:`static` is an :t:`object` that is associated with a specific memory
location.

:dp:`fls_mt94jvoot9dx`
A :t:`static` defined within a :t:`generic` is declared once and shared between
all :t:`[instantiation]s`.

:dp:`fls_k0r2c6uq29tu`
The :t:`type specification` of a :t:`static` shall have a :t:`static lifetime`.

:dp:`fls_b6ods85htuyn`
The :t:`type` of a :t:`static` shall implement the :std:`core::marker::Sized`
:t:`trait`.

:dp:`fls_doi4z6u55bi7`
A :t:`mutable static` is a :t:`static` whose :t:`value` can be modified.

:dp:`fls_74hp208pto22`
Access to a :t:`mutable static` shall require :t:`unsafe context`.

:dp:`fls_jfde2vg6mtww`
An :t:`immutable static` is a :t:`static` whose :t:`value` cannot be modified.

:dp:`fls_k4tyqb1j6zjo`
The type of an :t:`immutable static` shall implement the
:std:`core::marker::Sync` :t:`trait`.

:dp:`fls_t17h5h6a6v4c`
A :t:`static initializer` is a :t:`construct` that provides the :t:`value` of
its related :t:`static`.

:dp:`fls_yq0hpy4jx2qb`
A :t:`static` shall have a :t:`static initializer`, unless it is an :t:`external
static`.

:dp:`fls_vgidvfwzm4ks`
The :t:`expression` of a :t:`static initializer` shall be a :t:`constant
expression`.

:dp:`fls_8dcldbvu7lav`
A use of a :t:`static` is a :t:`place expression` referring to the unique
location of the :t:`static`.

.. rubric:: Dynamic Semantics

:dp:`fls_w0nb0mphho7b`
The :t:`elaboration` of a :t:`static` evaluates its :t:`static initializer`.

:dp:`fls_eeocxst9vafn`
All :t:`[path]s` that refer to a :t:`static` refer to the same memory location.

:dp:`fls_47khd5ljsxeq`
A :t:`static` is not :t:`dropped` during :t:`destruction`.

:dp:`fls_dowxbphqvk3n`
A :t:`mutable static` whose :t:`type` is not :t:`interiorly mutable` may reside
in read-only memory.

.. rubric:: Undefined Behavior

:dp:`fls_b5wsmii7vz3v`
It is undefined behavior to mutate an :t:`immutable static` that is not
:t:`interiorly mutable`.

.. rubric:: Examples

.. code-block:: text

   static mut GLOBAL: u32 = 0;

Temporaries
-----------

.. rubric:: Legality Rules

:dp:`fls_awpw61yofckz`
A :t:`temporary` is an anonymous :t:`object` that holds the result of some
intermediate computation.

Variables
---------

.. rubric:: Legality Rules

:dp:`fls_hl5tnd9yy252`
A :t:`variable` is an :t:`object` that is a component of a stack frame.

:dp:`fls_vgi0gh5zmoiu`
The following :t:`[construct]s` are :t:`[variable]s`:

* :dp:`fls_3p0sb9ppmg3w`
  An anonymous :t:`temporary`.

* :dp:`fls_81dlbula47nu`
  A named :t:`binding`.

* :dp:`fls_adqfhc5k051x`
  A named :t:`function parameter`.

:dp:`fls_xfiuc52r672i`
A :t:`local variable` is a :t:`variable` that refers to a :t:`value` allocated
directly on the stack.

:dp:`fls_r9km9f969bu8`
A :t:`local variable` shall be used only after it has been initialized through
all reachable control flow paths.

.. rubric:: Dynamic Semantics

:dp:`fls_g8etd5lsgn9j`
A :t:`local variable` is not initialized when allocated.

