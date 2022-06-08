.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Values and Objects
==================

.. rubric:: Legality Rules

:def_p:`fls_saaxhw2r8msw`
A :term:`value` is a memory location that is interpreted based on
some :term:`type`.

:def_p:`fls_jmwhiz1qrtmy`
An :term:`object` relates a :term:`value` to a :term:`name`, and dictates how
the :term:`value` is initialized and modified.

.. rubric:: Undefined Behavior

:def_p:`fls_6lg0oaaopc26`
It is undefined behavior to create a :term:`value` from uninitialized memory.

Constants
---------

.. rubric:: Syntax

.. syntax::

   ConstantDeclaration ::=
       $$const$$ (Name | $$_$$) TypeAscription ConstantInitializer $$;$$

   ConstantInitializer ::=
   $$    =$$ Expression

.. rubric:: Legality Rules

:def_p:`fls_5o5iu4j8in4l`
A :term:`constant` is an immutable :term:`object` that is not associated with
a specific memory location. The address of a :term:`constant` may differ from
other :term:`object`\ s derived from the same :term:`constant`.

:def_p:`fls_3mhj0kkupwuz`
An :term:`unnamed constant` is a :term:`constant` declared with character 0x5F
(low line).

:def_p:`fls_ka4y2yd100dx`
The :term:`type specification` of a :term:`constant` shall have a :term:`static
lifetime`. If a :term:`constant` refers to another :term:`constant`,
the :term:`lifetime` of the :term:`constant` may be :term:`elided`.

:def_p:`fls_ndmfqxjpvsqy`
A :term:`constant initializer` is a :term:`construct` that provides
the :term:`value` of its related :term:`constant`.

:def_p:`fls_vnc3ttnid1qr`
The :term:`expression` of a :term:`constant initializer` shall be
a :term:`constant expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_ndobth7s92if`
A :term:`path` that refers to a :term:`constant` is replaced with
the :term:`value` of the :term:`constant`.

.. rubric:: Undefined Behavior

:def_p:`fls_64mjyviikkgn`
It is undefined behavior to mutate a :term:`constant`.

.. rubric:: Examples

.. code-block:: text

   const ZERO: u32 = 0;

Statics
-------

.. rubric:: Syntax

.. syntax::


   StaticDeclaration ::=
       $$static$$ $$mut$$? Name TypeAscription StaticInitializer $$;$$

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

:def_p:`fls_doi4z6u55bi7`
A :term:`mutable static` is a :term:`static` whose :term:`value` can be
modified.

:def_p:`fls_74hp208pto22`
Access to a :term:`mutable static` shall require :term:`unsafe context`.

:def_p:`fls_jfde2vg6mtww`
An :term:`immutable static` is a :term:`static` whose :term:`value` cannot be
modified.

:def_p:`fls_k4tyqb1j6zjo`
The type of an :term:`immutable static` shall implement
the :codeterm:`core::marker::Sync` :term:`trait`.

:def_p:`fls_t17h5h6a6v4c`
A :term:`static initializer` is a :term:`construct` that provides
the :term:`value` of its related :term:`static`.

:def_p:`fls_vgidvfwzm4ks`
The :term:`expression` of a :term:`static initializer` shall be
a :term:`constant expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_eeocxst9vafn`
All :term:`path`\ s that refer to a :term:`static` refer to the same memory
location.

:def_p:`fls_47khd5ljsxeq`
A :term:`static` is not :term:`dropped` during :term:`destruction`. (**revisit
this once chapter 16 has settled**)

:def_p:`fls_dowxbphqvk3n`
A :term:`mutable static` whose :term:`type` is not :term:`interiorly mutable`
may reside in read-only memory.

.. rubric:: Examples

.. code-block:: text

   static mut GLOBAL: u32 = 0;

Variables
---------

.. rubric:: Legality Rules

:def_p:`fls_hl5tnd9yy252`
A :term:`variable` is an :term:`object` that is a component of a stack frame.

:def_p:`fls_u5akysriyedt`
A :term:`local variable` is a :term:`variable` that refers to a :term:`value`
allocated directly on the stack.

:def_p:`fls_vgi0gh5zmoiu`
The following :term:`construct`\ s are :term:`variable`\ s:

* :def_p:`fls_3p0sb9ppmg3w`
  An anonymous :term:`temporary`.

* :def_p:`fls_81dlbula47nu`
  A named :term:`binding`.

* :def_p:`fls_adqfhc5k051x`
  A named :term:`function parameter`.

:def_p:`fls_r9km9f969bu8`
A :term:`local variable` shall be used only after it has been initialized
through all reachable control flow paths.

.. rubric:: Dynamic Semantics

:def_p:`fls_g8etd5lsgn9j`
A :term:`local variable` is not initialized when allocated.

Constant Evaluation
-------------------

.. rubric:: Legality Rules

:def_p:`fls_p12zpuvig9pf`
A :term:`const context` is a syntactic position in which a :term:`constant
expression` is required.

:def_p:`fls_c0yzl4h73s3k`
The :term:`const context`\ s are:

* :def_p:`fls_5rgi3pph9wc4`
  The initializing expression of a :term:`static` item.

* :def_p:`fls_9kswtqn5yr1n`
  The initializing expression of a :term:`const` item or an :term:`associated
  constant`.

* :def_p:`fls_123mr3h7et07`
  An :term:`explicit enum discriminant` expression.

* :def_p:`fls_88t4r97xmlwx`
  The length of an :term:`array` type.

* :def_p:`fls_fuypzi33am77`
  The length of an :term:`array repeat expression`.

* :def_p:`fls_6wkdqn69969h`
  The argument for a :term:`const generic` parameter.

:def_p:`fls_vstagez3scsh`
Additionally, all expressions used in the body of a ``const fn`` must
be :term:`constant expression`\ s.

:def_p:`fls_nz5x0sl1aaxr`
The following expressions are :term:`constant expression`\ s which are allowed
in any :term:`const context`, including the body of a ``const fn``, as long
as all operands and nested expressions are also :term:`constant expression`\ s
allowed in that context:

* :def_p:`fls_i0183x4oe1g7`
  :term:`Literal` expressions.

* :def_p:`fls_ocu2eh6vzbzu`
  :term:`Path expression`\ s that resolve to ``const`` items, :term:`const
  generic` parameters, or :term:`associated constant`\ s (but see the
  restrictions below).

* :def_p:`fls_xoddawfbxna7`
  :term:`Path expression`\ s that resolve to :term:`function`\ s.

* :def_p:`fls_o9q26ap7z5kx`
  Tuple, array, and record literal expressions.

* :def_p:`fls_yxu6pxhn0ksv`
  All :term:`range expression`\ s.

* :def_p:`fls_lx4xungn86t`
  :term:`Closure` expressions.

* :def_p:`fls_6o8qbmqjrml8`
  :term:`Block expression`\ s and :term:```unsafe`` block` expressions.

* :def_p:`fls_azrc7aihmg9`
  :term:`Arithmetic expression`\ s and :term:`logical operator`\ s, including
  the short-circuiting operators ``&&`` and ``||``.

* :def_p:`fls_jyrdnob28p0r`
  :term:`Parenthesized expression`\ s.

* :def_p:`fls_gawa45ahn5at`
  :term:`Assignment` and :term:`compound assignment` expressions.

* :def_p:`fls_1q3ste2v3kxb`
  :term:`Index expression`\ s.

* :def_p:`fls_gwkj0qhdfd0y`
  :term:`Field access`\ es.

* :def_p:`fls_3bp5vni0d6gw`
  Shared :term:`borrow expression`\ s (``&x``), as long as the borrowed type
  does not contain :term:`interior mutability`.

* :def_p:`fls_85rmkfgi252a`
  The :term:`dereference operator` (``*x``), as long as the dereferenced type is
  not a :term:`raw pointer type`.

* :def_p:`fls_hnzzz91b15mc`
  ``loop``, ``while``, and ``while let`` expressions.

* :def_p:`fls_8mrx3wty4dqf`
  ``if``, ``if let``, and ``match`` expressions.

* :def_p:`fls_8oijo84tmaj`
  :term:`Cast expression`\ s, except:

*    * :def_p:`fls_qgszzqyktxtu`
       Function pointer to address casts.

*    * :def_p:`fls_a4bumngtkxcj`
       Pointer to address casts.

* :def_p:`fls_5xkzors182ol`
  Method and function :term:`call expression`\ s, if the callee is a ``const
  fn``.

*    * :def_p:`fls_76fo88iyb5sh`
       This includes the implicit function introduced by tuple struct and
       variant declarations.

:def_p:`fls_yjh0htcbsnyb`
:term:`Constant expression`\ s that are only allowed in the initializing
expression of a ``static`` item are:

* :def_p:`fls_amowygskdiar`
  :term:`Path expression`\ s that resolve to ``static`` items.

:def_p:`fls_f2vgtfvq20h7`
An invocation of the ``panic!()`` macro shall expand to a :term:`constant
expression` allowed in any :term:`const context` and ``const fn``, as long as it
is either invoked without arguments, or with a single string literal that does
not capture formatting arguments.

:def_p:`fls_cgpygdptucx`
An invocation of the ``addr_of!()`` macro shall expand to a :term:`constant
expression` allowed in any :term:`const context` and const fn, subject to the
same restrictions as a shared borrow expression ``&x``.

:def_p:`fls_v945h87j5fbx`
Inside the body of a const fn, :term:`arithmetic expression`\ s that use
floating point values are not considered :term:`constant expression`\ s.

:def_p:`fls_8fldf6pj9tb`
If the evaluation of an expression may result in a value's :term:`destructor` to
be run, the expression is not a :term:`constant expression`.

:def_p:`fls_1pucpqf80ksc`
If an expression results in the invocation of a :term:`trait method`, it is
not a constant expression. This includes explicit trait method invocations,
use of arithmetic operators on non-builtin types, which call one of
the :std:`core::ops` traits, as well as field accesses and method calls that
invoke :std:`core::ops::Deref`.

:def_p:`fls_xwqbqpij9la6`
Within an :term:`explicit enum discriminant`, mentioning a generic parameter of
the enum is not permitted and shall be reported as a static error.

:def_p:`fls_atpvxxruvhaf`
Inside an expression that denotes the length of an array type, or the length
of an array repeat expression, operations whose outcome depends on generic
parameters are not permitted and shall be reported as static errors.

:def_p:`fls_6gq9g613r8cs`
Within any :term:`const context`, mentioning any :term:`lifetime parameter`
is not permitted and shall be reported as a static error. Use of ``‘static``
is permitted.

:def_p:`fls_sdko5mhyn3kd`
Within a const context, it is permissible to refer to a path whose value
is itself determined by the evaluation of a :term:`constant expression` in
a :term:`const context` (subject to the restrictions specified above).

:def_p:`fls_bbooy7dcotjb`
A static error shall be reported if such references form a cycle, even if the
cycle does not occur during evaluation.

An implementation shall evaluate every expression in a :term:`const context`
according to the rules laid out in this specification, when the expression's
value is needed.

:def_p:`fls_frqm5xckz67a`
An implementation shall evaluate the initializer of every ``static`` and non-
associated ``const`` item, even when the item is unused.

:def_p:`fls_lqzxxznqkhwo`
Evaluation of :term:`constant expression`\ s shall be performed with overflow
checks enabled, so that any overflowing operation causes a panic.

If the evaluation of a :term:`constant expression` results in a panic, either
by reaching an invocation of the ``panic!()`` macro, or by invoking one of
the :term:`built-in panic condition`\ s, a static error shall be reported.

.. rubric:: Dynamic Semantics

:def_p:`fls_ql3nzmnvvfr`
Expressions used in :term:`constant context`\ s do not have dynamic semantics.
Invocations of ``const fn``\ s follow the dynamic semantics of non-``const``
function invocations.

