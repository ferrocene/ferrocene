.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Values and Objects
==================

.. rubric:: Legality Rules

:def_p:`fls_buyaqara7am4`
A :term:`value` is either a :term:`literal` or the result of a computation, that
may be stored in a memory location, and interpreted based on some :term:`type`.

:def_p:`fls_rixdyyc525xp`
Two :term:`value`\ s :term:`overlap` when

* :def_p:`fls_m6ctqq70vcxr`
  Both :term:`value`\ s are the same, or

* :def_p:`fls_s231d18x5eay`
  One :term:`value` is of an :term:`abstract data type` and the other denotes a
  :term:`field` of the same :term:`value`, or

* :def_p:`fls_dfr4yqo93fsn`
  One :term:`value` denotes an :term:`array` and the other denotes an element of
  the same :term:`array`, or

* :def_p:`fls_eoak5mdl6ma`
  Both :term:`value`\ s are elements of the same :term:`array`.

:def_p:`fls_jmwhiz1qrtmy`
An :term:`object` relates a :term:`value` to a :term:`name`, and dictates how
the :term:`value` is initialized and modified.

An :term:`object` is :term:`valid` when it has been :term:`initialized` by all
reachable control flow paths.

.. rubric:: Undefined Behavior

:def_p:`fls_6lg0oaaopc26`
It is undefined behavior to create a :term:`value` from uninitialized memory.

Constants
---------

.. rubric:: Syntax

.. syntax::

   ConstantDeclaration ::=
       $$const$$ (Name | $$_$$) TypeAscription ConstantInitializer? $$;$$

   ConstantInitializer ::=
   $$    =$$ Expression

.. rubric:: Legality Rules

:def_p:`fls_5o5iu4j8in4l`
A :term:`constant` is an :term:`immutable` :term:`object` that is not associated
with a specific memory location. The address of a :term:`constant` may differ
from other :term:`object`\ s derived from the same :term:`constant`.

:def_p:`fls_3mhj0kkupwuz`
An :term:`unnamed constant` is a :term:`constant` declared with character 0x5F
(low line).

:def_p:`fls_ka4y2yd100dx`
The :term:`type specification` of a :term:`constant` shall have a :term:`static
lifetime`.

:def_p:`fls_vt9tlkd676ql`
The :term:`type` of a :term:`constant` shall implement the
:codeterm:`core::marker::Sized` :term:`trait`.

:def_p:`fls_ndmfqxjpvsqy`
A :term:`constant initializer` is a :term:`construct` that provides the
:term:`value` of its related :term:`constant`.

:def_p:`fls_6rxwbbhf5tc5`
A :term:`constant` shall have a :term:`constant initializer`, unless it is an
:term:`associated trait constant`.

:def_p:`fls_vnc3ttnid1qr`
The :term:`expression` of a :term:`constant initializer` shall be a
:term:`constant expression`.

:def_p:`fls_deuo1pn8cjd6`
The value of a :term:`constant` is determined by evaluating its :term:`constant
initializer`.

:def_p:`fls_5x0jv4cgbolx`
A use of a :term:`constant` is a :term:`value expression` and creates a copy of
the constant's value.

.. rubric:: Dynamic Semantics

:def_p:`fls_ndobth7s92if`
A :term:`path` that refers to a :term:`constant` is replaced with the
:term:`value` of the :term:`constant`.

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

:def_p:`fls_ibrmiwfypldh`
A :term:`static` is an :term:`object` that is associated with a specific memory
location.

:def_p:`fls_mt94jvoot9dx`
A :term:`static` defined within a :term:`generic` is declared once and shared
between all :term:`instantiation`\ s.

:def_p:`fls_k0r2c6uq29tu`
The :term:`type specification` of a :term:`static` shall have a :term:`static
lifetime`.

:def_p:`fls_b6ods85htuyn`
The :term:`type` of a :term:`static` shall implement the
:codeterm:`core::marker::Sized` :term:`trait`.

:def_p:`fls_doi4z6u55bi7`
A :term:`mutable static` is a :term:`static` whose :term:`value` can be
modified.

:def_p:`fls_74hp208pto22`
Access to a :term:`mutable static` shall require :term:`unsafe context`.

:def_p:`fls_jfde2vg6mtww`
An :term:`immutable static` is a :term:`static` whose :term:`value` cannot be
modified.

:def_p:`fls_k4tyqb1j6zjo`
The type of an :term:`immutable static` shall implement the
:codeterm:`core::marker::Sync` :term:`trait`.

:def_p:`fls_t17h5h6a6v4c`
A :term:`static initializer` is a :term:`construct` that provides the
:term:`value` of its related :term:`static`.

:def_p:`fls_yq0hpy4jx2qb`
A :term:`static` shall have a :term:`static initializer`, unless it is an
:term:`external static`.

:def_p:`fls_vgidvfwzm4ks`
The :term:`expression` of a :term:`static initializer` shall be a
:term:`constant expression`.

:def_p:`fls_8dcldbvu7lav`
A use of a :term:`static` is a :term:`place expression` referring to the unique
location of the :term:`static`.

.. rubric:: Dynamic Semantics

:def_p:`fls_eeocxst9vafn`
All :term:`path`\ s that refer to a :term:`static` refer to the same memory
location.

:def_p:`fls_47khd5ljsxeq`
A :term:`static` is not :term:`dropped` during :term:`destruction`.

:def_p:`fls_dowxbphqvk3n`
A :term:`mutable static` whose :term:`type` is not :term:`interiorly mutable`
may reside in read-only memory.

.. rubric:: Undefined Behavior

It is undefined behavior to mutate an :term:`immutable static` that is not
:term:`interiorly mutable`.

.. rubric:: Examples

.. code-block:: text

   static mut GLOBAL: u32 = 0;

Temporaries
-----------

.. rubric:: Legality Rules

A :term:`temporary` is an anonymous :term:`object` that holds the result of some
intermediate computation.

Variables
---------

.. rubric:: Legality Rules

:def_p:`fls_hl5tnd9yy252`
A :term:`variable` is an :term:`object` that is a component of a stack frame.

:def_p:`fls_vgi0gh5zmoiu`
The following :term:`construct`\ s are :term:`variable`\ s:

* :def_p:`fls_3p0sb9ppmg3w`
  An anonymous :term:`temporary`.

* :def_p:`fls_81dlbula47nu`
  A named :term:`binding`.

* :def_p:`fls_adqfhc5k051x`
  A named :term:`function parameter`.

A :term:`local variable` is a :term:`variable` that refers to a :term:`value`
allocated directly on the stack.

:def_p:`fls_r9km9f969bu8`
A :term:`local variable` shall be used only after it has been initialized
through all reachable control flow paths.

.. rubric:: Dynamic Semantics

:def_p:`fls_g8etd5lsgn9j`
A :term:`local variable` is not initialized when allocated.

Constant Evaluation
-------------------

.. rubric:: Legality Rules

:term:`Constant evaluation` is the process of computing the result of a
:term:`constant expression`.

A :term:`constant context` is a :term:`construct` that requires a
:term:`constant expression`. The following :term:`construct`\ s are :term:`const
context`\ s:

* The :term:`constant initializer` of a :term:`constant` or an :term:`associated
  constant`.

* The :term:`discriminant initializer` of an :term:`enum variant`.

* The :term:`static initializer` of a :term:`static`.

* The :term:`size operand` of an :term:`array type`.

* The :term:`size operand` of an :term:`array repetition constructor`.

* The :term:`constant argument` for a :term:`constant generic parameter`.

* The :term:`default value` of a :term:`constant generic parameter`.

The following :term:`expression`\ s are :term:`constant expression`\ s as long
as their :term:`operand`\ s and nested expressions are also :term:`constant
expression`\ s allowed in that context:

* :term:`Literal` expressions.

* :term:`Path expression`\ s that resolve to ``const`` items, :term:`unit
  struct`\ s, :term:`const generic` parameters, or :term:`associated constant`\
  s.

* :term:`Path expression`\ s that resolve to :term:`function`\ s or :term:`tuple
  struct`\ s.

* :term:`Tuple expression`\ s, :term:`array expression`\ s and :term:`record
  struct constructor`\ s.

* :term:`Range expression`\ s.

* :term:`Closure expression`\ s.

* :term:`Block expression`\ s and :term:`unsafe block expression`\ s.

* :term:`Arithmetic expression`\ s, :term:`lazy boolean expression`\ s and
  :term:`bit expression`\ s.

* :term:`Parenthesized expression`\ s.

* :term:`Assignment expression`\ s and :term:`compound assignment expression`\
  s.

* :term:`Index expression`\ s.

* :term:`Field access expression`\ s.

* :def_p:`fls_tvzv7n3x7w31`
  :term:`Immutable borrow expression`\ s, as long as the borrowed type does not
  contain :term:`interior mutability`.

* :def_p:`fls_w1pyskfvs802`
  :term:`Dereference expression`\ s, as long as the dereferenced type is not a
  :term:`raw pointer type`.

* :def_p:`fls_336kkpg5a23b`
  :term:`Infinite loop expression`\ s, :term:`while loop expression`\ s and
  :term:`while let loop expression`\ s.

* :def_p:`fls_cdzn77d30yyt`
  :term:`If expression`\ s, :term:`if let expression`\ s and :term:`match
  expression`\ s.

* :def_p:`fls_419b57mqqw2z`
  :term:`Type cast expression`\ s, except:

  * Function pointer to address casts.

  * Pointer to address casts.

* :def_p:`fls_lbbit0wkatce`
  :term:`Call expression`\ s and :term:`method call expression`\ s, if the
  callee is a :term:`constant function`.

  * This includes the implicit function introduced by tuple struct and variant
    declarations.

:def_p:`fls_6ozenj9t75jg`
Additionally, :term:`path expression`\ s that resolve to static items are
:term:`constant expression`\ s when used in a :term:`static initializer.`

:def_p:`fls_fxyb7cx9xcvg`
It is a static error to create a mutable reference in a :term:`const context`.

An invocation of the ``panic!()`` macro expands to a :term:`constant expression`
allowed in any :term:`const context` and :term:`constant function`, as long as
it is either invoked without arguments, or with a single string literal that
does not capture formatting arguments.

An invocation of the ``addr_of!()`` macro expands to a :term:`constant
expression` allowed in any :term:`const context` and :term:`constant function`,
subject to the same restrictions as a :term:`shared borrow expression`.

Inside the body of a :term:`constant function`, :term:`arithmetic expression`\
s that use floating point values are not considered :term:`constant expression`\
s.

If the evaluation of an expression may result in a value's :term:`destructor` to
be run, the expression is not a :term:`constant expression`.

If an expression results in the invocation of an :term:`associated
trait function`, it is not a constant expression. This includes explicit
:term:`associated trait function` invocations, use of :term:`arithmetic
operator`\ s on non-builtin :term:`type`\ s, which call one of the
:std:`core::ops` :term:`trait`\ s, as well as :term:`field access expression`\ s
and :term:`method call expression`\ s that invoke :std:`core::ops::Deref`.

Within an :term:`explicit enum discriminant`, mentioning a :term:`generic
parameter` of the :term:`enum` is not permitted and shall be reported as a
static error.

Inside an :term:`expression` that denotes the length of an :term:`array type`,
or the length of an :term:`array repeat expression`, operations whose outcome
depends on :term:`generic parameter`\ s are not permitted and shall be reported
as static errors.

Within any :term:`const context`, mentioning any :term:`lifetime parameter`
is not permitted and shall be reported as a static error. Use of ``‘static``
is permitted.

Within a const context, it is permissible to refer to a path whose value is
itself determined by the evaluation of a :term:`constant expression` in a
:term:`const context` (subject to the restrictions specified above).

A static error shall be reported if such path referencing forms a cycle, even if
the cycle does not occur during evaluation.

An implementation shall evaluate every expression in a :term:`const context`
according to the rules laid out in this specification, when the expression's
value is needed.

An implementation shall evaluate the initializer of every :term:`static` and
non-associated :term:`const`, even when the item is unused.

Evaluation of :term:`constant expression`\ s that result in arithmetic overflow
will panic.

If the evaluation of a :term:`constant expression` results in a panic, either
by reaching an invocation of the ``panic!()`` macro, or by invoking one of the
:term:`built-in panic condition`\ s, a static error shall be reported.

.. rubric:: Dynamic Semantics

Expressions used in :term:`constant context`\ s do not have dynamic semantics.
Invocations of :term:`constant function`\ s follow the dynamic semantics of
non-:term:`constant function` invocations.

