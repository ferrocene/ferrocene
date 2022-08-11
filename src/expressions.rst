.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_ckvjj4tt1hh2:

Expressions
===========

.. rubric:: Syntax

.. syntax::

   Expression ::=
       ExpressionWithBlock
     | ExpressionWithoutBlock

   ExpressionWithBlock ::=
       OuterAttributeOrDoc* (
           AsyncBlockExpression
         | BlockExpression
         | IfExpression
         | IfLetExpression
         | LoopExpression
         | MatchExpression
         | UnsafeBlockExpression
       )

   ExpressionWithoutBlock ::=
       OuterAttributeOrDoc* (
           ArrayExpression
         | AwaitExpression
         | BreakExpression
         | CallExpression
         | ClosureExpression
         | ContinueExpression
         | FieldAccessExpression
         | IndexExpression
         | LiteralExpression
         | MethodCallExpression
         | MacroInvocation
         | OperatorExpression
         | ParenthesizedExpression
         | PathExpression
         | RangeExpression
         | ReturnExpression
         | StructExpression
         | TupleExpression
         | UnderscoreExpression
       )

   ExpressionList ::=
       Expression ($$,$$ Expression)* $$,$$?

   Operand ::=
       Expression

   LeftOperand ::=
       Operand

   RightOperand ::=
       Operand

:dp:`fls_pwut2jbmk66k`
A :ds:`SubjectExpression` is any expression in category :s:`Expression`, except
:s:`StructExpression`.

:dp:`fls_361q9ljc6ybz`
A :ds:`SubjectLetExpression` is any expression in category
:s:`SubjectExpression`, except :s:`LazyBooleanExpression`.

.. rubric:: Legality Rules

:dp:`fls_h5o6tgul4yor`
An :t:`expression` is a :t:`construct` that produces a :t:`value`, and may have
side effects at run-time.

:dp:`fls_xmklb3070sp`
An :t:`expression-with-block` is an :t:`expression` whose structure involves a
:t:`block expression`.

:dp:`fls_p15oeage4j0e`
An :t:`expression-without-block` is an :t:`expression` whose structure does not
involve a :t:`block expression`.

:dp:`fls_gwgttltgjma4`
An :t:`operand` is an :t:`expression` nested within an :t:`expression`.

:dp:`fls_1r29rtnjlkql`
A :t:`left operand` is an :t:`operand` that appears on the left-hand side of a
:t:`binary operator`.

:dp:`fls_qxdpyf4u3hbz`
A :t:`right operand` is an :t:`operand` that appears on the right-hand side of a
:t:`binary operator`.

:dp:`fls_2j132xueobfv`
A :t:`subject expression` is an :t:`expression` that controls :t:`[for loop]s`,
:t:`[if expression]s`, and :t:`[match expression]s`.

:dp:`fls_a243nclqqjlu`
A :t:`subject let expression` is an :t:`expression` that controls :t:`[if let
expression]s` and :t:`[while let loop]s`.

.. rubric:: Dynamic Semantics

:dp:`fls_1223lwh4nq49`
:t:`Evaluation` is the process by which an :t:`expression` achieves its runtime
effects.

.. _fls_isyftqu120l:

Expression Classification
-------------------------

.. _fls_3ut3biyra4r9:

Assignee Expressions
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_oqj7s9fi3j3j`
An :t:`assignee expression` is an :t:`expression` that appears as the :t:`left
operand` of an :t:`assignment expression`. The following :t:`[expression]s` are
:t:`[assignee expression]s`:

* :dp:`fls_skopz71arbwa`
  :t:`[Place expression]s`,

* :dp:`fls_vxrg6preh46x`
  :t:`[Underscore expression]s`,

* :dp:`fls_yso6dmog0an2`
  :t:`[Array expression]s` of :t:`[assignee expression]s`,

* :dp:`fls_1tsdlpgkgb2u`
  :t:`[Struct expression]s` of :t:`[assignee expression]s`.

* :dp:`fls_hier3b8knpuq`
  :t:`[Tuple expression]s` of :t:`[assignee expression]s`,

:dp:`fls_1smb3tj9pxsq`
:t:`[Parenthesized expression]s` are allowed to appear anywhere in :t:`[assignee
expression]s`.

.. _fls_66m4rnbssgig:

Constant Expressions
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_1ji7368ieg0b`
A :t:`constant expression` is an :t:`expression` that can be evaluated
statically. The following :t:`[construct]s` are :t:`[constant expression]s` as
long as their :t:`[operand]s` are also :t:`[constant expression]s` and do not
involve :t:`[type]s` that require :t:`destruction`:

* :dp:`fls_y6ore0iwx7e0`
  :t:`[Arithmetic expression]s` of :t:`[scalar type]s`,

* :dp:`fls_xguga84v3j8u`
  :t:`[Array expression]s`,

* :dp:`fls_idxf02p7jogu`
  :t:`[Assignment expression]s`,

* :dp:`fls_6z45ss502alt`
  :t:`[Bit expression]s` of :t:`[scalar type]s`,

* :dp:`fls_wqs0792nud4e`
  :t:`[Block expression]s`,

* :dp:`fls_490a1b74fut6`
  :t:`[Call expression]s` where the callee is a :t:`constant function`,

* :dp:`fls_8nyu6phm1nji`
  :t:`[Closure expression]s` that do not :t:`capture`,

* :dp:`fls_8wux08bmpse`
  :t:`[Comparison expression]s` of :t:`[scalar type]s`,

* :dp:`fls_v1bnk7neb82a`
  :t:`[Compound assignment expression]s`,

* :dp:`fls_6fq6bvxxvhsr`
  :t:`[Constant parameter]s`,

* :dp:`fls_to4e7imq2c0w`
  :t:`[Dereference expression]s` when the :t:`operand` is not of a :t:`raw
  pointer type`,

* :dp:`fls_krtbrpwf3mh0`
  :t:`[Expression statement]s`,

* :dp:`fls_3etom5uu8y4u`
  :t:`[Field access expression]s` that do not invoke the :std:`core::ops::Deref`
  :t:`trait`,

* :dp:`fls_qls0wj8bmupz`
  :t:`[If expression]s`,

* :dp:`fls_b5fraqx07wuo`
  :t:`[If let expression]s`,

* :dp:`fls_6g7c1kjrmfnr`
  :t:`[Immutable borrow expression]s` where the borrowed :t:`type` is not
  subject to :t:`interior mutability`.

* :dp:`fls_rpapnm3afan8`
  :t:`[Index expression]s`,

* :dp:`fls_fc62yaqyjpl2`
  :t:`[Infinite loop expression]s`,

* :dp:`fls_kwg8a351vc7`
  :t:`[Lazy boolean expression]s` of :t:`[scalar type]s`,

* :dp:`fls_7mjv1xd45qr4`
  :t:`[Let statement]s`,

* :dp:`fls_g7hoyfqy9mu1`
  :t:`[Literal expression]s`,

* :dp:`fls_br4g7qwfczig`
  :t:`[Match expression]s`,

* :dp:`fls_w4lpq9bs8tsc`
  :t:`[Method call expression]s` where the callee is a :t:`constant function` or
  do not invoke the :std:`core::ops::Deref` :t:`trait`,

* :dp:`fls_y1ezabo61nyk`
  :t:`[Negation expression]s` of :t:`[scalar type]s`,

* :dp:`fls_6tb74n6lu0wf`
  :t:`[Parenthesized expression]s`,

* :dp:`fls_axwrv7b3zt55`
  :t:`[Path expression]s` that resolve to :t:`[associated constant]s`,
  :t:`[constant]s`, :t:`[constant parameter]s`, :t:`[function]s`,
  :t:`[static]s`, :t:`[tuple struct]s`, and  :t:`[unit struct]s`,

* :dp:`fls_3bucpdj828bq`
  :t:`[Range expression]s`,

* :dp:`fls_hkbwa8xx2fwx`
  :t:`[Shared borrow]s` that do not involve :t:`[type]s` with :t:`interior
  mutability`,

* :dp:`fls_fobs8ebt7dhc`
  :t:`[Struct expression]s`,

* :dp:`fls_dyo3o1h3keqr`
  :t:`[Tuple expression]s`,

* :dp:`fls_e0a1e8ddph7`
  :t:`[Type cast expression]s` that are not :t:`[pointer-to-address cast]s`,
  :t:`[function-pointer-to-address cast]s`, and :t:`[unsized cast]s` that
  involve a :t:`trait object type`,

* :dp:`fls_zcuzhw7qkzkr`
  :t:`[Unsafe block expression]s`,

* :dp:`fls_pbpzkfo1fgtz`
  :t:`[While let loop expression]s`,

* :dp:`fls_qvofy4wkql0s`
  :t:`[While loop expression]s`.

:dp:`fls_3i7efddbsmn0`
An :t:`expression` is not considered a :t:`constant expression` when it
explicitly invokes an :t:`associated trait function` or uses :t:`[arithmetic
operator]s` of non-builtin :t:`[type]s` that invoke :std:`core::ops`
:t:`[trait]s`.

:dp:`fls_9mrrosm8jnn7`
An :t:`arithmetic expression` that operates with :t:`[floating-point value]s` is
not considered a :t:`constant expression` when it appears inside the
:t:`function body` of a :t:`constant function`.

:dp:`fls_fmqar6o1bwqk`
It is a static error if the :t:`size operand` of an :t:`array repetition
constructor` or an :t:`array type` depends on :t:`[generic parameter]s`.

:dp:`fls_kjhma680hz3g`
A :t:`constant context` is a :t:`construct` that requires a :t:`constant
expression`. The following :t:`[construct]s` are :t:`[constant context]s`:

* :dp:`fls_ljc6jq5ksbcs`
  The :t:`constant initializer` of an :t:`associated constant` or a
  :t:`constant`,

* :dp:`fls_3of516eo0kkx`
  The :t:`constant argument` for a :t:`constant parameter`,

* :dp:`fls_yiks5bvojncc`
  The :t:`default value` of a :t:`constant parameter`,

* :dp:`fls_66m2hwkju0vv`
  The :t:`discriminant initializer` of an :t:`enum variant`,

* :dp:`fls_fsn32kmwg65u`
  The :t:`size operand` of an :t:`array repetition constructor`,

* :dp:`fls_j6kffhbxdm7o`
  The :t:`size operand` of an :t:`array type`,

* :dp:`fls_ib8p7dfwddx2`
  The :t:`static initializer` of a :t:`static`.

:dp:`fls_ox6sgl9n43g2`
It is a static error to create a :t:`mutable reference` in a :t:`constant
context`.

:dp:`fls_od0h3v40kjp6`
An invocation of the ``addr_of!()`` :t:`macro` expands to a :t:`constant
expression` allowed in any :t:`constant context` and :t:`constant function`,
subject to the same restrictions as a :t:`mutable borrow expression`.

:dp:`fls_6sc556tz4oxd`
An invocation of the ``panic!()`` :t:`macro` expands to a :t:`constant
expression` allowed in any :t:`constant context` and :t:`constant function`,
as long as the :t:`macro` is either invoked without arguments, or
with a single :t:`string literal` that does not :t:`capture` formatting
arguments.

:dp:`fls_b1vfpvsdv38`
A :t:`constant expression` is evaluated statically whenever its :t:`value` is
needed.

:dp:`fls_b46nyamfqxdu`
The :t:`evaluation` of a :t:`constant expression` that results in
:t:`arithmetic overflow` :t:`[panic]s`.

:dp:`fls_ms9vey2wymqp`
It is a static error if a :t:`constant expression` either :t:`[panic]s` or
control reaches the invocation of :t:`macro` :std:`core::panic`.

.. rubric:: Dynamic Semantics

:dp:`fls_tg0kya5125jt`
The invocation of a :t:`constant function` follows the dynamic semantics of a
:t:`non-[constant function]` invocation.

.. _fls_6ydylimiv553:

Place Expressions
~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_qbrcg3cl9td`
A :t:`place expression` is an :t:`expression` that represents a memory location.
The following :t:`[expression]s` are :t:`[place expression]s`:

* :dp:`fls_jpmhibm4omm7`
  :t:`[Dereference expression]s`,

* :dp:`fls_none1dykbn8c`
  :t:`[Field access expression]s`,

* :dp:`fls_lj7x5dgbmg9i`
  :t:`[Index expression]s`,

* :dp:`fls_anzidgx02lly`
  :t:`[Parenthesized expression]s` where the :t:`operand` is a :t:`place
  expression`,

* :dp:`fls_ya05djl1d154`
  :t:`[Path expression]s` that resolve to a :t:`static` or a :t:`variable`.

:dp:`fls_4vxi1ji93dxb`
A :t:`place expression context` is a :t:`construct` that requires a :t:`place
expression`. The following :t:`[construct]s` are :t:`[place expression
context]s`:

* :dp:`fls_qytgkbhqr5ln`
  The :t:`indexed operand` of an :t:`index expression`,

* :dp:`fls_5gy92rsi2mqm`
  The :t:`assignee operand` of an :t:`assignment expression` or a :t:`compound
  assignment expression`,

* :dp:`fls_u80htrnr2ebz`
  The :t:`operand` of a :t:`borrow expression`,

* :dp:`fls_o0feajus3jtu`
  The :t:`operand` of a :t:`dereference expression`,

* :dp:`fls_ffjx1d5dseo4`
  The :t:`container operand` of :t:`field access expression`,

* :dp:`fls_9r7dopqf1nzl`
  The :t:`subject let expression` of an :t:`if let expression` or a :t:`while
  let loop expression`,

* :dp:`fls_ka5b87tkf8t6`
  The initialization :t:`expression` of a :t:`let statement`,

* :dp:`fls_brwv1zwu37e8`
  The :t:`subject expression` of a :t:`match expression`,

* :dp:`fls_qewvbxvk81d`
  The :t:`base initializer` of a :t:`struct expression`,

* :dp:`fls_qaqwmxa3bxw1`
  The :t:`operand` of an :t:`implicit borrow`.

:dp:`fls_konzgoybhfqm`
A :t:`place expression` can be moved out of when it denotes

* :dp:`fls_4bnbv7mqod57`
  A :t:`field` of a :t:`place expression` that can be moved out of and does not
  implement the :std:`core::ops::Drop` :t:`trait`, or

* :dp:`fls_3xk3p1unbjy5`
  A :t:`temporary` created for a :t:`value expression`, or

* :dp:`fls_vk1xhvdaakh0`
  A :t:`variable` which is not currently :t:`borrowed`.

:dp:`fls_wuqjaigxdq3r`
After a :t:`place expression` is moved out, the memory location it represented
is deinitialized and shall not be read from until reinitialized.

:dp:`fls_cPEMHZtPkctX`
An :t:`immutable place expression` is a :t:`place expression` whose memory
location cannot be modified.

:dp:`fls_ku38h562vfyl`
A :t:`mutable place expression` is a :t:`place expression` whose memory location
can be modified. The following :t:`[place expression]s` are :t:`[mutable place
expression]s`:

* :dp:`fls_bt50fltfqcvn`
  An :t:`index expression` whose :t:`type` implements the
  :std:`core::ops::IndexMut` :t:`trait`,

* :dp:`fls_6b4rwkrc1ap6`
  A :t:`dereference expression` whose :t:`type` is ``*mut T``,

* :dp:`fls_s4bhrpykzmm7`
  A :t:`dereference expression` of a :t:`field` or a :t:`variable` whose
  :t:`type` is ``&mut T``,

* :dp:`fls_1tq2o2huda9l`
  A :t:`dereference expression` whose :t:`type` implements the
  :std:`core::ops::DerefMut` :t:`trait`,

* :dp:`fls_xm0gm2q27x2e`
  A :t:`field access expression` where the :t:`type` of the :t:`container
  operand` is :t:`mutable`,

* :dp:`fls_ilaqmj3hc5uv`
  A :t:`path expression` that resolves to a :t:`mutable static`,

* :dp:`fls_m0gbw9myylv2`
  A :t:`path expression` that resolves to a :t:`mutable variable` that is not
  currently borrowed,

* :dp:`fls_dcm3yr3y9y0a`
  A :t:`temporary` created for a :t:`value expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_malm0kcczgyg`
The :t:`evaluation` of a :t:`place expression` in the context of a :t:`value
expression` or the :t:`evaluation` of a :t:`place expression` that is bound *by
value* in a :t:`pattern` proceeds as follows:

#. :dp:`fls_iuxjvxd91h06`
   The :t:`place expression` denotes the :t:`value` held in that memory
   location.

#. :dp:`fls_oq11btd97wpz`
   If the :t:`type` of the held :t:`value` implements the
   :std:`core::marker::Copy` :t:`trait`, then the held :t:`value` is copied.

#. :dp:`fls_zada4g3qmjqo`
   If the :t:`type` of the held :t:`value` implements the
   :std:`core::marker::Sized` :t:`trait`, then the held :t:`value` is moved.

#. :dp:`fls_gq35gqagw35`
   Otherwise the :t:`evaluation` results in a static error.

.. _fls_e7zgqroy2qxn:

Value Expressions
~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_7q4hrt6yfr9b`
A :t:`value expression` is an :t:`expression` that represents a :t:`value`.

.. _fls_h0dvogc64tfh:

Literal Expressions
-------------------

.. rubric:: Syntax

.. syntax::

   LiteralExpression ::=
       Literal

.. rubric:: Legality Rules

:dp:`fls_rbwwczom3agt`
A :t:`literal expression` is an :t:`expression` that denotes a :t:`literal`.

:dp:`fls_utbjihhwgxr1`
A :t:`literal expression` is a :t:`value expression`.

:dp:`fls_w30su9x4q13r`
The :t:`type` of a :t:`literal expression` is the :t:`type` of the corresponding
:t:`literal`.

:dp:`fls_wdpbg5xzgmwu`
The :t:`value` of a :t:`literal expression` is the :t:`value` of the
corresponding :t:`literal`.

.. rubric:: Dynamic Semantics

:dp:`fls_g061yzws1m45`
The :t:`evaluation` of a :t:`literal expression` has no effect.

.. rubric:: Examples

.. code-block:: rust

   5
   'a'
   "hello"

.. _fls_6l60b5hwnjbm:

Path Expressions
----------------

.. rubric:: Syntax

.. syntax::

   PathExpression ::=
       PathInExpression
     | QualifiedPathInExpression

.. rubric:: Legality Rules

:dp:`fls_gvanx4874ycy`
A :t:`path expression` is an :t:`expression` that denotes a :t:`path`.

:dp:`fls_t8bdzvtnv249`
A :t:`path expression` that resolves to a :t:`static` or a :t:`variable` is a
:t:`place expression`, otherwise it is a :t:`value expression`.

:dp:`fls_gz67ju6l7uhn`
A :t:`path expression` that resolves to a :t:`mutable static` shall require
:t:`unsafe context`.

:dp:`fls_cjywisyiyti6`
The :t:`type` of a :t:`path expression` is the :t:`type` of the :t:`entity` that
it resolved to.

:dp:`fls_5ifai8nkp5ek`
The :t:`value` of a :t:`path expression` is the :t:`entity` that it resolved to.

.. rubric:: Dynamic Semantics

:dp:`fls_ed9w4jwx059`
The :t:`evaluation` of a :t:`path expression` has no effect.

.. rubric:: Examples

.. code-block:: rust

   globals::STATIC_VARIABLE
   Vec::<i32>::push

.. _fls_hndm19t57wby:

Block Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   BlockExpression ::=
       $${$$
         InnerAttributeOrDoc*
         StatementList
       $$}$$

   StatementList ::=
       Statement* Expression?

.. rubric:: Legality Rules

:dp:`fls_nf65p0l0v0gr`
A :t:`block expression` is an :t:`expression` that sequences :t:`[expression]s`
and :t:`[statement]s`.

:dp:`fls_tn3hj7k2lliu`
A :t:`tail expression` is the last :t:`expression` within a :t:`block
expression`.

:dp:`fls_wiv8wpw3i79z`
A :t:`block expression` is a :t:`value expression`.

:dp:`fls_u4gj2lnkq9ub`
The :t:`type` of a :t:`block expression` is determined as follows:

* :dp:`fls_ob76y2ymdd27`
  If the :t:`block expression` has an :t:`expression`, then the :t:`type` is the
  :t:`type` of the :t:`expression`.

* :dp:`fls_u0avbm147nyh`
  If the :t:`block expression` does not have an :t:`expression`, then the
  :t:`type` is the :t:`unit type`.

:dp:`fls_1hzup0sf8l7l`
The :t:`value` of a :t:`block expression` is determined as follows:

* :dp:`fls_9nmssjseq3jt`
  If the :t:`block expression` has an :t:`expression`, then the :t:`value` is
  the :t:`value` of the :t:`expression`.

* :dp:`fls_a3ulnvyc1ut`
  If the :t:`block expression` does not have an :t:`expression`, then the
  :t:`value` of the :t:`block expression` is the :t:`unit value`.

.. rubric:: Dynamic Semantics

:dp:`fls_elcl73psruxw`
The :t:`evaluation` of a :t:`block expression` proceeds as follows:

#. :dp:`fls_13b5n127rj92`
   Each :t:`statement` is executed in declarative order.

#. :dp:`fls_nzdpw59plr2g`
   The :t:`expression` is evaluated.

.. rubric:: Examples

.. code-block:: rust

   {
       fn_call();
       42
   }

.. _fls_aadan19t5006:

async Blocks
~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   AsyncBlockExpression ::=
       $$async$$ $$move$$? BlockExpression

.. rubric:: Legality Rules

:dp:`fls_hhidi5ukxo`
An :t:`async block expression` is a :t:`block expression` that is specified
with :t:`keyword` ``async`` and encapsulates behavior which is executed in
asynchronous manner.

:dp:`fls_eam1cqbjlfs0`
An :t:`async block expression` is a :t:`value expression`.

:dp:`fls_tzclkasinpoq`
An :t:`async block expression` is subject to :t:`capturing`.

:dp:`fls_oisws5qykedi`
An :t:`async block expression` denotes a new :t:`control flow boundary`.

:dp:`fls_ncd0wkgtldem`
The :t:`type` of an :t:`async block expression` shall implement the
:std:`core::future::Future` trait.

:dp:`fls_pvnofoomgwl5`
The :t:`value` of an :t:`async block expression` is a :t:`future`.

.. rubric:: Dynamic Semantics

:dp:`fls_9ghp5yet75y6`
The :t:`evaluation` of an :t:`async block expression` produces a :t:`temporary`
that captures the related :t:`future`.

.. rubric:: Examples

.. code-block:: rust

   async {
       42
   }

.. _fls_8wnyln2nmg4y:

unsafe Blocks
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   UnsafeBlockExpression ::=
       $$unsafe$$ BlockExpression

.. rubric:: Legality Rules

:dp:`fls_2az5huhcxzzy`
An :t:`unsafe block expression` is a :t:`block expression` that is specified
with :t:`keyword` ``unsafe``.

:dp:`fls_5ucvvja4dzoc`
An :t:`unsafe block expression` allows :t:`unsafety`.

:dp:`fls_2nzwo1hbsg9g`
An :t:`unsafe block expression` is a :t:`value expression`.

:dp:`fls_j3mmg317q442`
The :t:`type` of the :t:`unsafe block expression` is the :t:`type` of its
:t:`block expression`.

:dp:`fls_nygurv3x3wq6`
The :t:`value` of the :t:`unsafe block expression` is the :t:`value` of its
:t:`block expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_pv5gcy3tbjwo`
The :t:`evaluation` of an :t:`unsafe block expression` evaluates its :t:`block
expression`.

.. rubric:: Examples

.. code-block:: rust

   unsafe {
       unsafe_fn_call()
   }

.. _fls_izdv9i4spokw:

Operator Expressions
--------------------

.. rubric:: Syntax

.. syntax::

   OperatorExpression ::=
       ArithmeticExpression
     | AssignmentExpression
     | BitExpression
     | BorrowExpression
     | ComparisonExpression
     | CompoundAssignmentExpression
     | DereferenceExpression
     | ErrorPropagationExpression
     | LazyBooleanExpression
     | NegationExpression
     | TypeCastExpression

.. rubric:: Legality Rules

:dp:`fls_ursc5ynymoy`
An :t:`operator expression` is an :t:`expression` that involves an operator.

.. _fls_qztk0bkju9u:

Borrow Expression
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BorrowExpression ::=
       $$&$$ $$mut$$? Operand

.. rubric:: Legality Rules

:dp:`fls_nnqfkl228hjx`
A :t:`borrow expression` is an :t:`expression` that borrows the :t:`value` of
its :t:`operand` and creates a :t:`reference` to the memory location of its
:t:`operand`.

:dp:`fls_r7ix8webgqlm`
An :t:`immutable borrow expression` is a :t:`borrow expression` that lacks
:t:`keyword` ``mut``.

:dp:`fls_50j167r4v61b`
A :t:`mutable borrow expression` is a :t:`borrow expression` that has
:t:`keyword` ``mut``.

:dp:`fls_ya77l2zgtilp`
When the :t:`operand` of a :t:`borrow expression` is a :t:`place expression`,
the :t:`borrow expression` produces a :t:`reference` to the memory location
indicated by the :t:`operand`. The memory location is placed in a borrowed
state, or simply :t:`borrowed`.

:dp:`fls_8uhfwqurbyqf`
When the :t:`operand` of a :t:`borrow expression` is a :t:`value expression`,
a :t:`temporary` is allocated and the :t:`borrow expression` produces a
:t:`reference` to the memory location of the :t:`temporary`.

:dp:`fls_xrq41zaq6bza`
A :t:`borrow expression` is a :t:`value expression`.

:dp:`fls_chr03xll75d`
The :t:`type` of a :t:`borrow expression` is determined as follows:

* :dp:`fls_5b2x5ri2w54r`
  If the :t:`borrow expression` denotes a :t:`shared reference`, then the
  :t:`type` is ``&T`` where ``T`` is the :t:`type` of the :t:`operand`.

* :dp:`fls_agl09ia869rk`
  If the :t:`borrow expression` denotes a :t:`mutable reference`, then the
  :t:`type` is ``&mut T`` where ``T`` is the :t:`type` of the :t:`operand`.

:dp:`fls_8cvmee9bzs40`
The :t:`value` of a :t:`borrow expression` is the address of its :t:`operand`.

.. rubric:: Dynamic Semantics

:dp:`fls_2jd0mgw4zja4`
The :t:`evaluation` of a :t:`borrow expression` evaluates its :t:`operand`.

.. rubric:: Examples

.. code-block:: rust

   let mut answer = 42;

:dp:`fls_350qejoq9i23`
Mutable borrow.

.. syntax::

   let ref_answer = &mut answer;

.. _fls_5cm4gkt55hjh:

Dereference Expression
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   DereferenceExpression ::=
       $$*$$ Operand

.. rubric:: Legality Rules

:dp:`fls_f6wktzofzdn1`
A :t:`dereference expression` is an :t:`expression` that obtains the pointed-to
memory location of its :t:`operand`.

:dp:`fls_aeh5pzpcjveq`
When the :t:`operand` of a :t:`dereference expression` is of a :t:`pointer
type`, the :t:`dereference expression` denotes the pointed-to memory location of
the :t:`operand`, or the :t:`dereference` of the :t:`operand`.

:dp:`fls_9cc0ml2sru6x`
The :t:`dereference` is assignable when

* :dp:`fls_m0cc62tcf6b7`
  The :t:`operand` is of :t:`type` ``&mut T`` or ``*mut T``, and

* :dp:`fls_llzt4s3uwt95`
  The :t:`operand` is a :t:`variable` or a possibly nested :t:`field` of a
  :t:`variable`, or

* :dp:`fls_908xdt291via`
  The :t:`operand` denotes a :t:`mutable place expression`.

* :dp:`fls_b96mek2ojcl`
  The :t:`operand` is of another :t:`type` that implements the
  :std:`core::ops::DerefMut` :t:`trait`.

:dp:`fls_8i4jzksxlrw0`
Dereferencing a :t:`raw pointer` shall require :t:`unsafe context`.

:dp:`fls_d68ddlse4zp`
If the context of a :t:`dereference expression` is an :t:`immutable
place expression`, then the :t:`dereference expression` is equivalent to
:t:`expression` ``*core::ops::Deref::deref(&operand)``.

:dp:`fls_g73vguanbs1x`
If the context of a :t:`dereference expression` is a :t:`mutable place
expression`, then the :t:`dereference expression` is equivalent to
:t:`expression` ``*core::ops::DerefMut::deref_mut(&mut operand)``.

:dp:`fls_8ibfqxtnahzx`
The :t:`type` of a :t:`dereference expression` is determined as follows:

* :dp:`fls_7e7tka4f2f1a`
  If the :t:`type` of the :t:`operand` is ``&mut T``, ``&T``, ``*mut T``, or
  ``*const T``, then the :t:`type` is ``T``\ ``.``

* :dp:`fls_y9bc691kkh6v`
  Otherwise the :t:`type` is :t:`associated type`
  :std:`core::ops::Deref::Target`.

:dp:`fls_gw49nukfveib`
The :t:`value` of a :t:`dereference expression` is determined as follows:

* :dp:`fls_jjf3sz9ddfhy`
  If the :t:`type` of the :t:`operand` is ``&mut T``, ``&T``, ``*mut T``, or
  ``*const T``, then the :t:`value` is the pointed-to :t:`value`\ ``.``

* :dp:`fls_fyend8kkpqq4`
  Otherwise the :t:`value` is the result of evaluating :t:`expression`
  ``*core::ops::Deref::deref(&operand)`` or :t:`expression`
  ``*core::ops::DerefMut::deref_mut(&mut operand)`` respectively.

.. rubric:: Dynamic Semantics

:dp:`fls_72bpdsxxbgeq`
The :t:`evaluation` of a :t:`dereference expression` evaluates its :t:`operand`.

.. rubric:: Undefined Behavior

:dp:`fls_9wgldua1u8yt`
It is undefined behavior to dereference a :t:`raw pointer` that is either
:t:`dangling` or unaligned.

.. rubric:: Examples

:dp:`fls_9ifaterm8nop`
See :p:`6.4.1. <fls_ltflbfba9d5r>` for the declaration of ``ref_answer``.

.. code-block:: rust

   let deref_asnwer = *ref_answer;

.. _fls_pocsh1neugpc:

Error Propagation Expression
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ErrorPropagationExpression ::=
       Operand $$?$$

.. rubric:: Legality Rules

:dp:`fls_8q59wbumrt5s`
An :t:`error propagation expression` is an :t:`expression` that either evaluates
to a :t:`value` of its :t:`operand` or returns a value to the enclosing control
flow boundary.

:dp:`fls_mq2h4seoxah`
An :t:`error propagation expression` shall appear within a :t:`control flow
boundary`.

:dp:`fls_ab4vhq4nwn7f`
The :t:`type` of an :t:`error propagation expression` is :t:`associated type`
:std:`core::ops::Try::Output`.

:dp:`fls_z4zikxy2b1em`
The :t:`value` of an :t:`error propagation expression` is determined as follows:

* :dp:`fls_a09614kgsspt`
  If the :t:`evaluation` of the :t:`error propagation expression` executed
  ``core::ops::Try::branch(operand)``, then the :t:`value` is the :t:`value` of
  the :std:`core::ops::ControlFlow::Continue` variant.

* :dp:`fls_8df018q7y6g`
  Otherwise control flow is returned to the end of the enclosing :t:`control
  flow boundary`.

:dp:`fls_9sriwut951xv`
The expression context for the :t:`operand` of the :t:`error propagation
expression` is a :t:`value expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_alk4qvfprnvy`
The :t:`evaluation` of an :t:`error propagation expression` of the form

.. code-block:: rust

   expression?

:dp:`fls_1nnhjcgy8kdh`
is equivalent to the :t:`evaluation` the following :t:`expression`:

.. code-block:: rust

   match core::ops::Try::branch(expression) {
       core::ops::ControlFlow::Continue(value) =>
           value,

       core::ops::ControlFlow::Break(value) =>
           core::ops::FromResidual::from_residual(value),
   }

.. rubric:: Examples

.. code-block:: rust

   fn try_to_parse() -> Result<i32, ParseIntError> {
       "42".parse()?
   }

   fn try_some() -> Option<i32> {
       let val = Some(42)?;
       Some(val)
   }

.. _fls_wrecura8u5ar:

Negation Expression
~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   NegationExpression ::=
       NegationOperator Operand

   NegationOperator ::=
       BitwiseNegationOperator
     | SignNegationOperator

   BitwiseNegationOperator ::=
       $$!$$

   SignNegationOperator ::=
       $$-$$

.. rubric:: Legality Rules

:dp:`fls_pfa81kv2mru8`
A :t:`negation expression` is an :t:`expression` that negates its :t:`operand`.

:dp:`fls_plcut8vzdwox`
The :t:`type` of the :t:`operand` of a :t:`negation expression` with a
:s:`BitwiseNegationOperator` shall implement the :std:`core::ops::Not`
:t:`trait`.

:dp:`fls_ohu0kljfexd3`
The :t:`type` of a :t:`negation expression` with a :s:`BitwiseNegationOperator`
is :t:`associated type` :std:`core::ops::Not::Output`.

:dp:`fls_ghqvj8q71o97`
The :t:`value` of a :t:`negation expression` with a :s:`BitwiseNegationOperator`
is the result of ``core::ops::Not::not(operand)``.

:dp:`fls_3m4mgqnzqhri`
The :t:`type` of the :t:`operand` of a :t:`negation expression` with a
:s:`SignNegationOperator` shall implement the :std:`core::ops::Neg` :t:`trait`.

:dp:`fls_u7gzm6n75rzm`
The :t:`type` of a :t:`negation expression` with a :s:`SignNegationOperator`
shall be :t:`associated type` :std:`core::ops::Neg::Output`.

:dp:`fls_9rmq7iaf092d`
The :t:`value` of a :t:`negation expression` with a :s:`SignNegationOperator` is
the result of ``core::ops::Neg::neg(operand)``.

:dp:`fls_2eou0x2lxmk6`
The expression context for the :t:`operand` of the :t:`negation expression` is a
:t:`value expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_yzt6pcsvj3a`
The :t:`evaluation` of a :t:`negation expression` with a
:s:`BitwiseNegationOperator` proceeds as follows:

#. :dp:`fls_8tgxtprtifrr`
   The :t:`operand` is evaluated.

#. :dp:`fls_gn3dnuxm2h8m`
   ``core::ops::Not::not(operand)`` is invoked.

:dp:`fls_tsou6yz4mfte`
The :t:`evaluation` of a :t:`negation expression` with a
:s:`SignNegationOperator` proceeds as follows:

#. :dp:`fls_zdfgqky85r1f`
   The :t:`operand` is evaluated.

#. :dp:`fls_uldh10k77sng`
   ``core::ops::Neg::neg(operand)`` is invoked.

.. rubric:: Examples

:dp:`fls_uo6vv2yf8usx`
Sign negation.

.. code-block:: rust

   -42

:dp:`fls_hbrg0d98jak5`
Bitwise negation.

.. code-block:: rust

   !42

:dp:`fls_kqtr9c3jorvg`
Logical negation.

.. code-block:: rust

   !false

.. _fls_1k9mkv7rbezi:

Arithmetic Expressions
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ArithmeticExpression ::=
       AdditionExpression
     | DivisionExpression
     | MultiplicationExpression
     | RemainderExpression
     | SubtractionExpression

   AdditionExpression ::=
       LeftOperand $$+$$ RightOperand

   DivisionExpression ::=
       LeftOperand $$/$$ RightOperand

   MultiplicationExpression ::=
       LeftOperand $$*$$ RightOperand

   RemainderExpression ::=
       LeftOperand $$%$$ RightOperand

   SubtractionExpression ::=
       LeftOperand $$-$$ RightOperand

.. rubric:: Legality Rules

:dp:`fls_asibqpe3z95h`
An :t:`arithmetic expression` is an :t:`expression` that computes a :t:`value`
from two :t:`[operand]s` using arithmetic.

:dp:`fls_dstca76y08ge`
A :t:`division expression` is an :t:`arithmetic expression` that uses division.

:dp:`fls_kf41bphvlse3`
A :t:`multiplication expression` is an :t:`arithmetic expression` that uses
multiplication.

:dp:`fls_3de9ulyzuoa`
A :t:`remainder expression` is an :t:`arithmetic expression` that uses remainder
division.

:dp:`fls_aalxhbvu8kdi`
A :t:`subtraction expression` is an :t:`arithmetic expression` that uses
subtraction.

:dp:`fls_8imzo7agyx0k`
The :t:`type` of the :t:`left operand` of an :t:`addition expression` shall
implement the :std:`core::ops::Add` :t:`trait` with the :t:`type` of the
:t:`right operand` as the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_vk17mfv47wk9`
The :t:`type` of an :t:`addition expression` is :t:`associated type`
:std:`core::ops::Add::Output`.

:dp:`fls_ryzhdpxgm7ii`
The :t:`value` of an :t:`addition expression` is the result of
``core::ops::Add::add(left_operand, right_operand)``.

:dp:`fls_f1puss9t4btz`
The :t:`type` of the :t:`left operand` of a :t:`division expression` shall
implement the :std:`core::ops::Div` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_5rdrkvspw57z`
The :t:`type` of a :t:`division expression` is :t:`associated type`
:std:`core::ops::Div::Output`.

:dp:`fls_thyq4h55mx55`
The :t:`value` of a :t:`division expression` is the result of
``core::ops::Div::div(left_operand, right_operand)``.

:dp:`fls_hrml95g2txcj`
The :t:`type` of the :t:`left operand` of a :t:`multiplication expression`
shall implement the :std:`core::ops::Mul` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_ittf4yggk7do`
The :t:`type` of a :t:`multiplication expression` is :t:`associated type`
:std:`core::ops::Mul::Output`.

:dp:`fls_ylqm6wucq2sw`
The :t:`value` of a :t:`multiplication expression` is the result of
``core::ops::Mul::mul(left_operand, right_operand)``.

:dp:`fls_8fbhreyynhid`
The :t:`type` of the :t:`left operand` of a :t:`remainder expression` shall
implement the :std:`core::ops::Rem` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_u3jwnrqun5kl`
The :t:`type` of a :t:`remainder expression` is :t:`associated type`
:std:`core::ops::Rem::Output`.

:dp:`fls_2ude3wrxji2p`
The :t:`value` of a :t:`remainder expression` is the result of
``core::ops::Rem::rem(left_operand, right_operand)``.

:dp:`fls_fjcv1nm8tlgf`
The :t:`type` of the :t:`left operand` of a :t:`subtraction expression` shall
implement the :std:`core::ops::Sub` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_9x2i1zlsm364`
The :t:`type` of a :t:`subtraction expression` is :t:`associated type`
:std:`core::ops::Sub::Output`.

:dp:`fls_v8vekngd27sz`
The :t:`value` of a :t:`subtraction expression` is the result of
``core::ops::Sub::sub(left_operand, right_operand)``.

:dp:`fls_69r1m88mxzx5`
The expression context for the :t:`[operand]s` of an :t:`arithmetic expression`
is a :t:`value expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_5nsa9zefz9cv`
The :t:`evaluation` of an :t:`addition expression` proceeds as follows:

#. :dp:`fls_u3pstd6xe43y`
   The :t:`left operand` is evaluated.

#. :dp:`fls_jjmc1xgny77`
   The :t:`right operand` is evaluated.

#. :dp:`fls_cayhj5hcuhcg`
   ``core::ops::Add::add(left_operand, right_operand)`` is invoked.

:dp:`fls_43knkymqpj7t`
The :t:`evaluation` of a :t:`division expression` proceeds as follows:

#. :dp:`fls_62gpbubfj30w`
   The :t:`left operand` is evaluated.

#. :dp:`fls_bveocgaagk1n`
   The :t:`right operand` is evaluated.

#. :dp:`fls_qd6ggdgq2hob`
   ``core::ops::Div::div(left_operand, right_operand)`` is invoked.

:dp:`fls_lr2a21v5en59`
The :t:`evaluation` of a :t:`multiplication expression` proceeds as follows:

#. :dp:`fls_kpbxcdaflb06`
   The :t:`left operand` is evaluated.

#. :dp:`fls_b94ojbfukhvd`
   The :t:`right operand` is evaluated.

#. :dp:`fls_blyr18iao20n`
   ``core::ops::Mul::mul(left_operand, right_operand)`` is invoked.

:dp:`fls_g28igfbnwfe0`
The :t:`evaluation` of a :t:`remainder expression` proceeds as follows:

#. :dp:`fls_thcumw8n8xbw`
   The :t:`left operand` is evaluated.

#. :dp:`fls_gld1u9fnsj6d`
   The :t:`right operand` is evaluated.

#. :dp:`fls_k7lmxvpkxtub`
   ``core::ops::Rem::rem(left_operand, right_operand)`` is invoked.

:dp:`fls_bndpd66973ev`
The :t:`evaluation` of a :t:`subtraction expression` proceeds as follows:

#. :dp:`fls_izmfimd4yg27`
   The :t:`left operand` is evaluated.

#. :dp:`fls_ad9tc6ki8vcq`
   The :t:`right operand` is evaluated.

#. :dp:`fls_b9g0r9vc4rou`
   ``core::ops::Rem::rem(left_operand, right_operand)`` is invoked.

:dp:`fls_35oSMqAMFYWl`
If :t:`arithmetic overflow` occurs, the computed :t:`value` shall wrap around or the program
shall either :t:`panic` or :t:`abort` execution.

.. rubric:: Examples

.. code-block:: rust

   1 + 2
   4.0 / 3.29
   8.4 * 5.3
   10 % 4
   3 - 2

.. _fls_abp6tjbz8tpn:

Bit Expressions
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BitExpression ::=
       BitAndExpression
     | BitOrExpression
     | BitXOrExpression
     | ShiftLeftExpression
     | ShiftRightExpression

   BitAndExpression ::=
       LeftOperand $$&$$ RightOperand

   BitOrExpression ::=
       LeftOperand $$|$$ RightOperand

   BitXorExpression ::=
       LeftOperand $$^$$ RightOperand

   ShiftLeftExpression ::=
       LeftOperand $$<<$$ RightOperand

   ShiftRightExpression ::=
       LeftOperand $$>>$$ RightOperand

.. rubric:: Legality Rules

:dp:`fls_3zd59yuywz6l`
A :t:`bit expression` is an :t:`expression` that computes a :t:`value` from two
:t:`[operand]s` using bit arithmetic.

:dp:`fls_f6mmva3lbj1i`
A :t:`bit and expression` is a :t:`bit expression` that uses bit and arithmetic.

:dp:`fls_3136k1y6x3cu`
A :t:`bit or expression` is a :t:`bit expression` that uses bit or arithmetic.

:dp:`fls_j7ujcuthga1i`
A :t:`bit xor expression` is a :t:`bit expression` that uses bit exclusive or
arithmetic.

:dp:`fls_caxn774ij8lk`
A :t:`shift left expression` is a :t:`bit expression` that uses bit shift left
arithmetic.

:dp:`fls_t709sl4co3al`
A :t:`shift right expression` is a :t:`bit expression` that uses bit shift right
arithmetic.

:dp:`fls_cmowpfrcelke`
The :t:`type` of the :t:`left operand` of a :t:`bit and expression` shall
implement the :std:`core::ops::BitAnd` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_kchprk9z6xun`
The :t:`type` of a :t:`bit and expression` is :t:`associated type`
:std:`core::ops::BitAnd::Output`.

:dp:`fls_dimu987fw4kg`
The :t:`value` of a :t:`bit and expression` is the result of
``core::ops::BitAnd::bitand(left_operand, right_operand)``.

:dp:`fls_oo2ynd8e1ys6`
The :t:`type` of the :t:`left operand` of a :t:`bit or expression` shall
implement the :std:`core::ops::BitOr` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_s6hkt5fg598y`
The :t:`type` of a :t:`bit or expression` is :t:`associated type`
:std:`core::ops::BitOr::Output`.

:dp:`fls_osfse0t6ua8a`
The :t:`value` of a :t:`bit or expression` is the result of
``core::ops::BitOr::bitor(left_operand, right_operand)``.

:dp:`fls_fnywefl9nty2`
The :t:`type` of the :t:`left operand` of a :t:`bit xor expression` shall
implement the :std:`core::ops::BitXor` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_4f24nyx0ix0j`
The :t:`type` of a :t:`bit xor expression` is :t:`associated type`
:std:`core::ops::BitXor::Output`.

:dp:`fls_8tb22c6zbp3`
The :t:`value` of a :t:`bit xor expression` is the result of
``core::ops::BitXor::bitxor(left_operand, right_operand)``.

:dp:`fls_1f4pc612f2a8`
The :t:`type` of the :t:`left operand` of a :t:`shift left expression` shall
implement the :std:`core::ops::Shl` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_8trozue35xe4`
The :t:`type` of a :t:`shift left expression` is :t:`associated type`
:std:`core::ops::Shl::Output`.

:dp:`fls_kqntxbwnc58v`
The :t:`value` of a :t:`shift left expression` is the result of
``core::ops::Shl::shl(left_operand, right_operand)``.

:dp:`fls_onutb0b9p9zj`
The :t:`type` of the :t:`left operand` of a :t:`shift right expression` shall
implement the :std:`core::ops::Shr` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_yq8rtwfp3nv0`
The :t:`type` of a :t:`shift right expression` is :t:`associated type`
:std:`core::ops::Shr::Output`.

:dp:`fls_fbazfgd5m1ot`
The :t:`value` of a :t:`shift right expression` is the result of
``core::ops::Shr::shr(left_operand, right_operand)``.

:dp:`fls_2z6wble3u8ec`
The expression context for the :t:`[operand]s` of a :t:`bit expression` is a
:t:`value expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_f4o8xlu67okn`
The :t:`evaluation` of a :t:`bit and expression` proceeds as follows:

#. :dp:`fls_kp747xqekyrr`
   The :t:`left operand` is evaluated.

#. :dp:`fls_m0pdk78dah6n`
   The :t:`right operand` is evaluated.

#. :dp:`fls_m2hsk41hwm2j`
   ``core::ops::BitAnd::bitand(left_operand, right_operand)`` is invoked.

:dp:`fls_p9rlmjhbnbao`
The :t:`evaluation` of a :t:`bit or expression` proceeds as follows:

#. :dp:`fls_vprp53kv64q6`
   The :t:`left operand` is evaluated.

#. :dp:`fls_d456ummq6vrk`
   The :t:`right operand` is evaluated.

#. :dp:`fls_n269ufyesndz`
   ``core::ops::BitOr::bitor(left_operand, right_operand)`` is invoked.

:dp:`fls_i9iqtobheivu`
The :t:`evaluation` of a :t:`bit xor expression` proceeds as follows:

#. :dp:`fls_htw2tpujktwt`
   The :t:`left operand` is evaluated.

#. :dp:`fls_gf9tyu1idpjk`
   The :t:`right operand` is evaluated.

#. :dp:`fls_u5irwqswbsvu`
   ``core::ops::BitXor::bitxor(left_operand, right_operand)`` is invoked.

:dp:`fls_2kkpr955i4lm`
The :t:`evaluation` of a :t:`shift left expression` proceeds as follows:

#. :dp:`fls_7p64lgnjxylz`
   The :t:`left operand` is evaluated.

#. :dp:`fls_ieh1itrkcnf6`
   The :t:`right operand` is evaluated.

#. :dp:`fls_f0p70y92k14f`
   ``core::ops::Shl::shl(left_operand, right_operand)`` is invoked.

:dp:`fls_303r0u6ug215`
The :t:`evaluation` of a :t:`shift right expression` proceeds as follows:

#. :dp:`fls_4gxj18t6cnzq`
   The :t:`left operand` is evaluated.

#. :dp:`fls_gurl2ve58drz`
   The :t:`right operand` is evaluated.

#. :dp:`fls_xkyj83mcb9d5`
   ``core::ops::Shr::shr(left_operand, right_operand)`` is invoked.

.. rubric:: Examples

.. code-block:: rust

   0b1010 & 0b1100
   0b1010 | 0b0011
   0b1010 ^ 0b1001
   13 << 3
   -10 >> 2

.. _fls_nsvzzbldhq53:

Comparison Expressions
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ComparisonExpression ::=
       EqualsExpression
     | GreaterThanExpression
     | GreaterThanOrEqualsExpression
     | LessThanExpression
     | LessThanOrEqualsExpression
     | NotEqualsExpression

   EqualsExpression ::=
       LeftOperand $$==$$ RightOperand

   GreaterThanExpression ::=
       LeftOperand $$>$$ RightOperand

   GreaterThanOrEqualsExpression ::=
       LeftOperand $$>=$$ RightOperand

   LessThanExpression ::=
       LeftOperand $$<$$ RightOperand

   LessThanOrEqualsExpression ::=
       LeftOperand $$<=$$ RightOperand

   NotEqualsExpression ::=
       LeftOperand $$!=$$ RightOperand

.. rubric:: Legality Rules

:dp:`fls_yzuceqx6nxwa`
A :t:`comparison expression` is an :t:`expression` that compares the
:t:`[value]s` of two :t:`[operand]s`.

:dp:`fls_ruyho6cu7rxg`
An :t:`equals expression` is a :t:`comparison expression` that tests equality.

:dp:`fls_wapl0ir7uvbp`
A :t:`greater-than expression` is a :t:`comparison expression` that tests for a
greater-than relationship.

:dp:`fls_7n5gol6a8lod`
A :t:`greater-than-or-equals expression` is a :t:`comparison expression` that
tests for a greater-than-or-equals relationship.

:dp:`fls_yd4qqi39w248`
A :t:`less-than expression` is a :t:`comparison expression` that tests for a
less-than relationship.

:dp:`fls_yxwe1o27u6ns`
A :t:`less-than-or-equals expression` is a :t:`comparison expression` that tests
for a less-than-or-equals relationship.

:dp:`fls_w71j7i3n1kit`
A :t:`not-equals expression` is a :t:`comparison expression` that tests for
inequality.

:dp:`fls_asfrqemqviad`
A :t:`comparison expression` implicitly takes :t:`[shared borrow]s` of its
:t:`[operand]s`.

:dp:`fls_9s4re3ujnfis`
The :t:`type` of a :t:`comparison expression` is :t:`type` :c:`bool`.

:dp:`fls_8echqk9po1cf`
The :t:`type` of the :t:`left operand` of an :t:`equals expression` shall
implement the :std:`core::cmp::PartialEq` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_d62qfloqk2ub`
The :t:`value` of an :t:`equals expression` is the result of
``core::cmp::PartialEq::eq(&left_operand, &right_operand)``.

:dp:`fls_x2s6ydvj5zyd`
The :t:`type` of the :t:`left operand` of a :t:`greater-than expression` shall
implement the :std:`core::cmp::PartialOrd` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_pso38dowbk91`
The :t:`value` of a :t:`greater-than expression` is the result of
``core::cmp::PartialOrd::gt(&left_operand, &right_operand)``.

:dp:`fls_hholzcbp5u3n`
The :t:`type` of the :t:`left operand` of a :t:`greater-than-or-equals
expression` shall implement the :std:`core::cmp::PartialOrd` :t:`trait` where
the :t:`type` of the :t:`right operand` is the :t:`trait implementation`
:t:`type parameter`.

:dp:`fls_wytygse41vzm`
The :t:`value` of a :t:`greater-than-or-equals expression` is the result of
``core::cmp::PartialOrd::ge(&left_operand, &right_operand)``.

:dp:`fls_ynibdcke3etb`
The :t:`type` of the :t:`left operand` of a :t:`less-than expression` shall
implement the :std:`core::cmp::PartialOrd` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_xmtxkit3qpw7`
The :t:`value` of a :t:`less-than expression` is the result of
``core::cmp::PartialOrd::lt(&left_operand, &right_operand)``.

:dp:`fls_6dgfieyxdan0`
The :t:`type` of the :t:`left operand` of a :t:`less-than-or-equals expression`
shall implement the :std:`core::cmp::PartialOrd` :t:`trait` where the :t:`type`
of the :t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_7pfsqby2saag`
The :t:`value` of a :t:`less-than-or-equals expression` is the result of
``core::cmp::PartialOrd::le(&left_operand, &right_operand)``.

:dp:`fls_qzo1torhv5i3`
The :t:`type` of the :t:`left operand` of a :t:`not-equals expression` shall
implement the :std:`core::cmp::PartialEq` :t:`trait` where the :t:`type` of the
:t:`right operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_kodwkh58hmdv`
The :t:`value` of a :t:`not-equals expression` is the result of
``core::cmp::PartialEq::ne(&left_operand, &right_operand)``.

:dp:`fls_8qbrzb9bxyf`
The expression context for the :t:`[operand]s` of a :t:`comparison expression`
is a :t:`place expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_ydt9zvh0h5ex`
The :t:`evaluation` of an :t:`equals expression` proceeds as follows:

#. :dp:`fls_4vbrc31r0o60`
   The :t:`left operand` is evaluated.

#. :dp:`fls_hyy974ksbbrq`
   The :t:`right operand` is evaluated.

#. :dp:`fls_htrjqxiv3avh`
   ``core::cmp::PartialEq::eq(&left_operand, &right_operand)`` ``is invoked.``

:dp:`fls_1udbc4aom6ok`
The :t:`evaluation` of a :t:`greater-than expression` proceeds as follows:

#. :dp:`fls_96mt7gx5ogo0`
   The :t:`left operand` is evaluated.

#. :dp:`fls_or0i2cqxwl8o`
   The :t:`right operand` is evaluated.

#. :dp:`fls_udnhkbxpk83m`
   ``core::cmp::PartialOrd::gt(&left_operand, &right_operand)`` is invoked.

:dp:`fls_mab6yirx77zl`
The :t:`evaluation` of a :t:`greater-than-or-equals expression` proceeds as
follows:

#. :dp:`fls_2ggb7a7nhrk9`
   The :t:`left operand` is evaluated.

#. :dp:`fls_ukm97arfzsk1`
   The :t:`right operand` is evaluated.

#. :dp:`fls_wrftg7onlkmm`
   ``core::cmp::PartialOrd::ge(&left_operand, &right_operand)`` is invoked.

:dp:`fls_irlqykpbtvd`
The :t:`evaluation` of a :t:`less-than expression` proceeds as follows:

#. :dp:`fls_udonl4c7f6pz`
   The :t:`left operand` is evaluated.

#. :dp:`fls_ebvyhqbb921g`
   The :t:`right operand` is evaluated.

#. :dp:`fls_rfomib80bnn2`
   ``core::cmp::PartialOrd::lt(&left_operand, &right_operand)`` is invoked.

:dp:`fls_6cb4wg59wmef`
The :t:`evaluation` of a :t:`less-than-or-equals expression` proceeds as
follows:

#. :dp:`fls_dkbjn7noq8n2`
   The :t:`left operand` is evaluated.

#. :dp:`fls_kezynx2xc1q7`
   The :t:`right operand` is evaluated.

#. :dp:`fls_8luq5sellcaq`
   ``core::cmp::PartialOrd::le(&left_operand, &right_operand)`` is invoked.

:dp:`fls_c93pacid548a`
The :t:`evaluation` of a :t:`not-equals expression` proceeds as follows:

#. :dp:`fls_gqy6uuowij9e`
   The :t:`left operand` is evaluated.

#. :dp:`fls_s6sq6p8th5nt`
   The :t:`right operand` is evaluated.

#. :dp:`fls_kdga59xx4nx3`
   ``core::cmp::PartialEq::ne(&left_operand, &right_operand)`` is invoked.

.. rubric:: Examples

.. code-block:: rust

   12 == 12
   42 > 12
   42 >= 35
   42 < 109
   42 <= 42
   12 != 42

.. _fls_lstusiu2c8lu:

Lazy Boolean Expressions
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   LazyBooleanExpression ::=
       LazyAndExpression
     | LazyOrExpression

   LazyAndExpression ::=
       LeftOperand $$&&$$ RightOperand

   LazyOrExpression ::=
       LeftOperand $$||$$ RightOperand

.. rubric:: Legality Rules

:dp:`fls_gpbvus89iy4c`
A :t:`lazy boolean expression` is an :t:`expression` that performs short circuit
Boolean arithmetic.

:dp:`fls_40jya46h62yi`
A :t:`lazy and expression` is a :t:`lazy boolean expression` that uses short
circuit and arithmetic.

:dp:`fls_k8u77ow5bb6c`
A :t:`lazy or expression` is a :t:`lazy boolean expression` that uses short
circuit or arithmetic.

:dp:`fls_u0gwo0s2l0tn`
The :t:`[type]s` of the :t:`[operand]s` of a :t:`lazy boolean expression` shall
be :t:`type` :c:`bool`.

:dp:`fls_zas0lizgq2hn`
The :t:`type` of a :t:`lazy boolean expression` is :t:`type` :c:`bool`.

:dp:`fls_xdgvrd58nkoa`
The :t:`value` of a :t:`lazy boolean expression` is either ``true`` or
``false``.

:dp:`fls_bov5j5t1bx0a`
The expression context for the :t:`[operand]s` of the :t:`lazy boolean
expression` is a :t:`value expression` context.

.. rubric:: Dynamic Semantics

:dp:`fls_ufre0ko2cwh2`
The :t:`evaluation` of a :t:`lazy and expression` proceeds as follows:

#. :dp:`fls_jugckad775kq`
   The :t:`left operand` is evaluated.

#. :dp:`fls_tmfmu3syxp2q`
   If the :t:`left operand` evaluated to ``true``, then

   #. :dp:`fls_edj00fp6bqdk`
      The :t:`right operand` is evaluated and returned as the :t:`[lazy and
      expression]'s` :t:`value`.

#. :dp:`fls_srfv1d4idxy9`
   Otherwise the :t:`lazy and expression` evaluates to ``false``.

:dp:`fls_tflikh8cmxvc`
The :t:`evaluation` of a :t:`lazy or expression` proceeds as follows:

#. :dp:`fls_p0rafjsridre`
   The :t:`left operand` is evaluated.

#. :dp:`fls_yg1348rlziw3`
   If the :t:`left operand` evaluated to ``false``, then

   #. :dp:`fls_ds8cr5dxc9em`
      The :t:`right operand` is evaluated and returned as the :t:`[lazy or
      expression]'s` :t:`value`.

#. :dp:`fls_yffozo2vq5xz`
   Otherwise the :t:`lazy or expression` evaluates to ``true``.

.. rubric:: Examples

.. code-block:: rust

   false && panic!()
   this || that

.. _fls_1qhsun1vyarz:

Type Cast Expressions
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TypeCastExpression ::=
       Operand $$as$$ TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:dp:`fls_ltioqbhl14g0`
A :t:`type cast expression` is an :t:`expression` that changes the :t:`type` of
an :t:`operand`.

:dp:`fls_99kvyh4puy57`
:t:`Cast` or :t:`casting` is the process of changing the :t:`type` of an
:t:`expression`.

:dp:`fls_a6midh2m0w0b`
The ``TypeSpecificationWithoutBounds`` describes the :dt:`target type` of the
:t:`type cast expression`.

:dp:`fls_otaxe9okhdr1`
A :t:`type cast expression` with the following characteristics performs a
:dt:`specialized cast`:

* :dp:`fls_4s69s9pcvbn7`
  An :t:`operand` of a :t:`numeric type` and a target :t:`numeric type` perform
  a :t:`numeric cast`.

* :dp:`fls_le6bchl25ewz`
  An :t:`operand` of an :t:`enum type` and a target :t:`integer type`
  perform :t:`enum cast`. An :dt:`enum cast` converts the :t:`operand` to its
  :t:`discriminant`, followed by a :t:`numeric cast`.

* :dp:`fls_pcromhosmnf0`
  An operand of :t:`type` :c:`bool` or :t:`type` :c:`char` and a
  target :t:`integer type` perform :t:`primitive-to-integer cast`. A
  :dt:`primitive-to-integer cast`

  * :dp:`fls_al9f1t7vlsxi`
    Converts an :t:`operand` of :t:`type` :c:`bool` with :t:`value` ``false``
    to zero.

  * :dp:`fls_jea17f39fmsg`
    Converts an :t:`operand` of :t:`type` :c:`bool` with :t:`value` ``true``
    to one.

  * :dp:`fls_eb00s8fxlvjb`
    Convert an :t:`operand` of :t:`type` :c:`char` to the :t:`value` of the
    corresponding :t:`code point`, followed by a :t:`numeric cast`.

* :dp:`fls_qk5trk8wkvxb`
  An :t:`operand` of :t:`type` :c:`u8` and a target :t:`type` :c:`char` performs
  :t:`u8-to-char cast`. A :dt:`u8-to-char cast` converts an :t:`operand` of
  :t:`type` :c:`u8` to the :t:`value` of the corresponding :t:`code point`.

* :dp:`fls_t16yzaxro5ew`
  An :t:`operand` of :t:`type` ``*const T`` or ``*mut T`` and a
  :t:`target type` ``*const V`` or ``*mut V`` where ``V`` implements the
  :std:`core::marker::Sized` :t:`trait` performs pointer-to-pointer cast.

* :dp:`fls_i4zsbbmfa2fl`
  An :t:`operand` of :t:`type` ``*const T`` or ``*mut T`` where ``T`` implements
  the :std:`core::marker::Sized` :t:`trait` and a target :t:`integer type`
  perform :t:`pointer-to-address cast`. A :dt:`pointer-to-address cast` produces
  an integer that represents the machine address of the referenced memory. If
  the :t:`integer type` is smaller than the :t:`type` of the :t:`operand`, the
  address is truncated.

* :dp:`fls_59mpteeczzo`
  An :t:`operand` of :t:`integer type` and :t:`target type` ``*const V`` or
  ``*mut V`` where ``V`` implements the :std:`core::marker::Sized` :t:`trait`
  perform :t:`address-to-pointer cast`. An :dt:`address-to-pointer cast`
  produces a :t:`pointer` that interprets the integer as a machine address.

* :dp:`fls_8ccwlliqw9jx`
  An :t:`operand` of :t:`type` ``&mut [T; N]`` and a :t:`target type` ``*const
  T`` perform array-to-pointer cast.

* :dp:`fls_i8txki3htx92`
  An :t:`operand` of a :t:`function item type` and a :t:`target type` ``*const
  V`` or ``*mut V`` where ``V`` implements the :std:`core::marker::Sized`
  :t:`trait` perform function-item-to-pointer cast.

* :dp:`fls_6hbkvbb1c8aj`
  An :t:`operand` of a :t:`function item type` and a target :t:`integer type`
  perform function-to-address cast.

* :dp:`fls_133j6xw8k4qe`
  An :t:`operand` of a :t:`function pointer type` and a :t:`target
  type` ``*const V`` or ``*mut V`` where ``V`` implements the
  :std:`core::marker::Sized` :t:`trait` perform function-pointer-to-pointer
  cast.

* :dp:`fls_bhw2j9wjpf2x`
  An :t:`operand` of a :t:`function pointer type` and a target :t:`integer type`
  perform function-pointer-to-address cast.

:dp:`fls_3ww5gbk9w4ys`
A :t:`cast` is legal when it either performs :t:`type coercion` or is a
:t:`specialized cast`.

:dp:`fls_hhxawo12cndy`
The :t:`type` of a :t:`type cast expression` is the :t:`target type`.

:dp:`fls_uuayaksl8059`
The :t:`value` of a :t:`type cast expression` is the :t:`value` of the
:t:`operand` after the :t:`cast`.

.. rubric:: Dynamic Semantics

:dp:`fls_syk2li8ft3rx`
The :t:`evaluation` of a :t:`type cast expression` evaluates its :t:`operand`.

:dp:`fls_uqv32nthva6y`
The :t:`evaluation` of a :dt:`numeric cast` proceeds as follows:

* :dp:`fls_kc3gwj9x3jnr`
  Casting an :t:`operand` of an :t:`integer type` to a target :t:`integer type`
  of the same :t:`size` has no effect.

* :dp:`fls_76eq3bd6birr`
  Casting an :t:`operand` of an :t:`integer type` to a target :t:`integer type`
  with smaller :t:`size` truncates the :t:`value` of the :t:`operand`.

* :dp:`fls_ldiritt32h2w`
  Casting an :t:`operand` of an :t:`integer type` to a target :t:`integer type`
  with a larger :t:`size` either

* :dp:`fls_h9sxg3pxn7i2`
  Zero-extends the :t:`operand` if the :t:`[operand]'s` :t:`type` is unsigned,
  or

* :dp:`fls_shy6e0e30bco`
  Sign-extends the :t:`operand` if the :t:`[operand]'s` :t:`type` is signed.

* :dp:`fls_4xldaoj5ac6t`
  Casting an :t:`operand` of a :t:`floating-point type` to a target :t:`integer
  type` rounds the :t:`value` of the :t:`operand` towards zero. In addition, the
  :t:`type cast expression`

* :dp:`fls_50714cvaqkfv`
  Returns zero if the :t:`operand` denotes :std:`f32::NaN` or :std:`f64::NaN`
  respectively.

* :dp:`fls_g3xbmp8zx1yh`
  Saturates the :t:`value` of the :t:`operand` to the maximum :t:`value`
  of the target :t:`integer type` if the :t:`[operand]'s` :t:`value`
  exceeds the maximum :t:`value` of the target :t:`integer type` or denotes
  :std:`f32::INFINITY` or :std:`f64::INFINITY` respectively.

* :dp:`fls_hcc5odh52bk7`
  Saturates the :t:`value` of the :t:`operand` to the minimum :t:`value`
  of the target :t:`integer type` if the :t:`[operand]'s` :t:`value`
  exceeds the minimum :t:`value` of the target :t:`integer type` or denotes
  :std:`f32::NEG_INFINITY` or :std:`f64::NEG_INFINITY` respectively.

* :dp:`fls_o2ep4b6t287z`
  Casting an :t:`operand` of an :t:`integer type` to a target :t:`floating-point
  type` produces the closest possible floating-point :t:`value`. In addition,
  the :t:`type cast expression`

* :dp:`fls_vfofk2aagsj5`
  Rounds the :t:`value` of the :t:`operand` preferring the :t:`value` with an
  even least significant digit if exactly halfway between two floating-point
  numbers.

* :dp:`fls_cx86k8yfjhht`
  Produces :std:`f32::INFINITY` or :std:`f64::INFINITY` of the same sign as the
  :t:`value` of the :t:`operand` when the :t:`value` of the :t:`operand` causes
  :t:`arithmetic overflow`.

* :dp:`fls_gzmdwibl5s4w`
  Casting an :t:`operand` of :t:`type` :c:`f32` to a :t:`target type` :c:`f64`
  is perfect and lossless.

* :dp:`fls_mjqvjt7v8a25`
  Casting an :t:`operand` of :t:`type` :c:`f64` to :t:`target type` :c:`f32`
  produces the closest possible :c:`f32` :t:`value`. In addition, the :t:`type
  cast expression`

* :dp:`fls_4fd5vkh0jt4`
  Prefers the nearest :t:`value` with an even least significant digit if exactly
  halfway between two floating-point numbers.

* :dp:`fls_2etd73f8jg2n`
  Produces :std:`f32::INFINITY` of the same sign as the :t:`value` of the
  :t:`operand` when the :t:`value` of the :t:`operand` causes
  :t:`arithmetic overflow`.

.. rubric:: Examples

:dp:`fls_vdxkpvmpwl3s`
See :p:`6.4.1. <fls_ltflbfba9d5r>` for the declaration of ``answer``.

.. code-block:: rust

   answer as f64

.. _fls_y4by2i8dl05o:

Assignment Expressions
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   AssignmentExpression ::=
       AssigneeOperand $$=$$ ValueOperand

   AssigneeOperand ::=
       Operand

   ValueOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_nhgexeu2h6wi`
An :t:`assignment expression` is an :t:`expression` that assigns the :t:`value`
of a :t:`value operand` to an :t:`assignee operand`.

:dp:`fls_bsjw6f4a3wol`
An :t:`assignee operand` is the target :t:`operand` of an :t:`assignment
expression`.

:dp:`fls_uinh05sslxeo`
A :t:`value operand` is an :t:`operand` that supplies the :t:`value` that is
assigned to an :t:`assignee operand` by an :t:`assignment expression`.

:dp:`fls_kh6rp9e0wwl`
An :t:`assignee operand` shall denote a :t:`mutable assignee expression`.
LUKAS, what is a "mutable assignee expression"?

:dp:`fls_3wragak9hglw`
A :t:`value operand` shall denote a :t:`value expression`.

:dp:`fls_qengy157fa4a`
The :t:`type` of an :t:`assignment expression` is the :t:`unit type`.

:dp:`fls_bwwtgqprbxrm`
The :t:`value` of an :t:`assignment expression` is the :t:`unit value`.

.. _fls_nnqlae9zp80s:

Basic Assignment
^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_uhcodvq75nlr`
A :t:`basic assignment` is an :t:`assignment expression` that is not a
:t:`destructuring assignment`.

.. rubric:: Dynamic Semantics

:dp:`fls_esn5ceoldta`
The :t:`evaluation` of a :t:`basic assignment` proceeds as follows:

#. :dp:`fls_t8eqzc64ivin`
   The :t:`value operand` is evaluated.

#. :dp:`fls_b0mqcn5fejhk`
   The :t:`assignee operand` is evaluated.

#. :dp:`fls_9i0ctuo099bp`
   The :t:`value` denoted by the :t:`assignee operand` is :t:`dropped`, unless
   the :t:`assignee operand` denotes an uninitialized :t:`variable` or an
   uninitialized :t:`field` of a :t:`variable`.

#. :dp:`fls_hc01gtvlxba`
   The :t:`value` of the :t:`value operand` is passed :t:`by value` into the
   place of the :t:`assignee operand`.

.. rubric:: Examples

.. code-block:: rust

   this = 42

.. _fls_9beohh5475s2:

Destructuring Assignment
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_2eheo4yo2orm`
A :t:`destructuring assignment` is an :t:`assignment expression` where
the :t:`assignee operand` is either an :t:`array expression`, a :t:`struct
expression`, or a :t:`tuple expression`.

:dp:`fls_z8c3b9s9de3x`
The :t:`assignee operand` of a :t:`destructuring assignment` corresponds to an
:t:`assignee pattern` according to its kind, as follows:
LUKAS, that is an "assignee pattern"?

* :dp:`fls_du5eybf8mocy`
  A :t:`place expression` corresponds to an :t:`identifier pattern` with a
  unique :t:`identifier` and without :t:`keyword` ``ref``, keyword ``mut``, or a
  :t:`bound pattern`.

* :dp:`fls_q90ikfi7ewoi`
  An :t:`underscore expression` corresponds to an :t:`wildcard pattern`.

* :dp:`fls_uydzlfc4hjbx`
  A :t:`tuple expression` corresponds to a :t:`tuple pattern` with all the
  :t:`[subexpression]s` lowered to their corresponding :t:`[pattern]s`.

* :dp:`fls_hj6srmzbobid`
  A :t:`struct expression` corresponds to a :t:`struct pattern` with all the
  :t:`[subexpression]s` lowered to their corresponding :t:`[pattern]s`.

* :dp:`fls_vqb89rkkjw81`
  A :t:`slice expression` corresponds to a :t:`slice pattern` with all the
  :t:`[subexpression]s` lowered to their corresponding :t:`[pattern]s`.
  LUKAS, there is no such thing as "slice expression". Did you mean "array
  expression"?

* :dp:`fls_vqj7ljrrd7wi`
  A :t:`full range expression` corresponds to a :t:`rest pattern` if inside a
  :t:`slice expression`, otherwise this is a static error.

:dp:`fls_4bb07tn28ivw`
The :t:`pattern` that corresponds to a :t:`destructuring assignment` shall be
:t:`irrefutable`.

:dp:`fls_g80a92tr2ser`
A :t:`destructuring assignment` is equivalent to a :t:`block expression` of the
following form:

* :dp:`fls_u0iqhbw37xvq`
  The first :t:`statement` is a :t:`let statement` with its :t:`pattern`
  equivalent to the lowered :t:`assignee pattern` and its :t:`initialization
  expression` equivalent to the :t:`value operand`.

* :dp:`fls_wsfhd3udt6fq`
  Then each bound :t:`identifier` of the :t:`assignee pattern` is an
  :t:`assignment expression` used as a :t:`statement`, as follows:

* :dp:`fls_ll6h6qcaos65`
  The bound :t:`identifier` becomes the :t:`value operand` of the new
  :t:`assignment expression`, and

* :dp:`fls_ajbdn54qe6wc`
  The corresponding :t:`expression` from the :t:`assignee operand` of the
  :t:`destructuring assignment` becomes the :t:`assignee operand` of the new
  :t:`assignment expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_l4u5hhw8tnvs`
The :t:`evaluation` of a :t:`destructuring assignment` proceeds as follows:

#. :dp:`fls_dd62w8c9n9sd`
   The :t:`value operand` is evaluated.

#. :dp:`fls_jqu2u6mdccgi`
   The :t:`assignee operand` is evaluated by evaluating its :t:`[operand]s` in a
   left-to-right order.

#. :dp:`fls_n7nuj1lvpspc`
   Each :t:`value` denoted by the :t:`assignee operand` is :t:`dropped`
   in left-to-right order, unless the :t:`assignee operand` denotes an
   uninitialized :t:`variable` or an uninitialized field of a :t:`variable`.

#. :dp:`fls_qb8u5skn8bc4`
   The :t:`value` of the :t:`value operand` is passed :t:`by value` into the
   place of the :t:`assignee operand`.

.. rubric:: Examples

.. code-block:: rust

   (four, two) = (4, 2)

.. _fls_290jmzfh7x4e:

Compound Assignment Expressions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   CompoundAssignmentExpression ::=
       AdditionAssignmentExpression
     | BitAndAssignmentExpression
     | BitOrAssignmentExpression
     | BitXorAssignmentExpression
     | DivisionAssignmentExpression
     | MultiplicationAssignmentExpression
     | RemainderAssignmentExpression
     | ShiftLeftAssignmentExpression
     | ShiftRightAssignmentExpression
     | SubtractionAssignmentExpression

   AdditionAssignmentExpression ::=
       AssignedOperand $$+=$$ ModifyingOperand

   BitAndAssignmentExpression ::=
       AssignedOperand $$&=$$ ModifyingOperand

   BitOrAssignmentExpression ::=
       AssignedOperand $$|=$$ ModifyingOperand

   BitXorAssignmentExpression ::=
       AssignedOperand $$^=$$ ModifyingOperand

   DivisionAssignmentExpression ::=
       AssignedOperand $$/=$$ ModifyingOperand

   MultiplicationAssignmentExpression ::=
       AssignedOperand $$*=$$ ModifyingOperand

   RemainderAssignmentExpression ::=
       AssignedOperand $$%=$$ ModifyingOperand

   ShiftLeftAssignmentExpression ::=
       AssignedOperand $$<<=$$ ModifyingOperand

   ShiftRightAssignmentExpression ::=
       AssignedOperand $$>>=$$ ModifyingOperand

   SubtractionAssignmentExpression ::=
       AssignedOperand $$-=$$ ModifyingOperand

   AssignedOperand ::=
       Operand

   ModifyingOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_3bu3g8o5nopc`
A :t:`compound assignment expression` is an expression that first computes
a :t:`value` from two :t:`[operand]s` and then assigns the value to an
:t:`assigned operand`.

:dp:`fls_w2hbhb989yr4`
A :t:`bit and assignment expression` is a :t:`compound assignment expression`
that uses bit and arithmetic.

:dp:`fls_ak4g5112jkl`
A :t:`bit or assignment expression` is a :t:`compound assignment expression`
that uses bit or arithmetic.

:dp:`fls_lkjwyy78m2vx`
A :t:`bit xor assignment expression` is a :t:`compound assignment expression`
that uses bit exclusive or arithmetic.

:dp:`fls_pkzj0uigfcgm`
A :t:`division assignment expression` is a :t:`compound assignment expression`
that uses division.

:dp:`fls_ndlv3k9uclz2`
A :t:`multiplication assignment expression` is a :t:`compound assignment
expression` that uses multiplication.

:dp:`fls_fbp5dojti27r`
A :t:`remainder assignment expression` is a :t:`compound assignment expression`
that uses remainder division.

:dp:`fls_oy9ur11k78t`
A :t:`shift left assignment expression` is a :t:`compound assignment expression`
that uses bit shift left arithmetic.

:dp:`fls_s7rey2bndfei`
A :t:`shift right assignment expression` is a :t:`compound assignment
expression` that uses bit shift right arithmetic.

:dp:`fls_7l7v7vigw3fu`
A :t:`subtraction assignment expression` is a :t:`compound assignment
expression` that uses subtraction.

:dp:`fls_dvy201zd6oym`
An :t:`assigned operand` is the target :t:`operand` of a :t:`compound assignment
expression`.

:dp:`fls_9v09ayi2azpe`
A :t:`modifying operand` is an :t:`operand` that supplies the :t:`value` that is
used in the calculation of a :t:`compound assignment expression`.

:dp:`fls_row7saf53vwd`
An :t:`assigned operand` shall denote a :t:`mutable assignee expression`.

:dp:`fls_gulql06xp9f3`
A :t:`modifying operand` shall denote a :t:`value expression`.

:dp:`fls_xmgcdw9yhb55`
The :t:`type` of a :t:`compound assignment` is the :t:`unit type`.

:dp:`fls_yeh6mvyvb4dp`
The :t:`value` of a :t:`compound assignment` is the :t:`unit value`.

:dp:`fls_657knnsobdyu`
The :t:`type` of the :t:`assigned operand` of an :t:`addition assignment` shall
implement the :std:`core::ops::AddAssign` trait where the type of the right
operand is the trait implementation type parameter.

:dp:`fls_m942dwwmr2cl`
The :t:`type` of the :t:`assigned operand` of a :t:`bit and assignment` shall
implement the :std:`core::ops::BitAndAssign` :t:`trait` where the :t:`type` of
the :t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_np33oqrz33mp`
The :t:`type` of the :t:`assigned operand` of a :t:`bit or assignment` shall
implement the :std:`core::ops::BitOrAssign` :t:`trait` where the :t:`type` of
the :t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_atdpr8be2o2r`
The :t:`type` of the :t:`assigned operand` of a :t:`bit xor assignment` shall
implement the :std:`core::ops::BitXorAssign` :t:`trait` where the :t:`type` of
the :t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_fbgwb3pdfgz`
The :t:`type` of the :t:`assigned operand` of a :t:`division assignment` shall
implement the :std:`core::ops::DivAssign` :t:`trait` where the :t:`type` of the
:t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_8tbxq95x06yt`
The :t:`type` of the :t:`assigned operand` of a :t:`multiplication assignment`
shall implement the :std:`core::ops::MulAssign` :t:`trait` where the :t:`type`
of the :t:`modifying operand` is the :t:`trait implementation` :t:`type
parameter`.

:dp:`fls_9oy9zo3x3fy3`
The :t:`type` of the :t:`assigned operand` of a :t:`remainder assignment` shall
implement the :std:`core::ops::RemAssign` :t:`trait` where the :t:`type` of the
:t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_pdgj2xekdead`
The :t:`type` of the :t:`assigned operand` of a :t:`shift left assignment` shall
implement the :std:`core::ops::ShlAssign` :t:`trait` where the :t:`type` of the
:t:`modifying operand` is the :t:`trait implementation` :t:`type parameter`.

:dp:`fls_4uoi6k8r7mvc`
The :t:`type` of the :t:`assigned operand` of a :t:`shift right assignment`
shall implement the :std:`core::ops::ShrAssign` :t:`trait` where the :t:`type`
of the :t:`modifying operand` is the :t:`trait implementation` :t:`type
parameter`.

:dp:`fls_fjaz4m90cagr`
The :t:`type` of the :t:`assigned operand` of a :t:`subtraction assignment`
shall implement the :std:`core::ops::SubAssign` :t:`trait` where the :t:`type`
of the :t:`modifying operand` is the :t:`trait implementation` :t:`type
parameter`.

.. rubric:: Dynamic Semantics

:dp:`fls_eesn9kuylim`
The :t:`evaluation` of a :t:`compound assignment` proceeds as follows:

#. :dp:`fls_4nnqz4etisgw`
   If the :t:`[type]s` of both :t:`[operand]s` are :t:`[primitive type]s`, then

   #. :dp:`fls_a2g4hs15jpiu`
      The :t:`modifying operand` is evaluated.

   #. :dp:`fls_kuet16jp6ps9`
      The :t:`assigned operand` is evaluated.

   #. :dp:`fls_hovju0sni9gr`
      The appropriate :t:`function` is invoked as indicated below.

#. :dp:`fls_ngimpabunzis`
   Otherwise

   #. :dp:`fls_4sbpfi12frwe`
      The :t:`assigned operand` is evaluated.

   #. :dp:`fls_n5ds6ydgckvo`
      The :t:`modifying operand` is evaluated.

   #. :dp:`fls_xjdu0y1slsg9`
      The appropriate :t:`function` is invoked as indicated below.

:dp:`fls_ijfmnnrdlu8n`
For an :t:`addition assignment`, ``core::ops::AddAssign::add_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_6x7j9x354pkb`
For a :t:`bit and assignment`, ``core::ops::BitAndAssign::bitand_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_h2cpbz2t74hy`
For a :t:`bit or assignment`, ``core::ops::BitOrAssign::bitor_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_whj50spxz3bh`
For a :t:`bit xor assignment`, ``core::ops::BitXorAssign::bitxor_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_d1cxq1zbt5fq`
For a :t:`division assignment`, ``core::ops::DivAssign::div_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_48i245an2449`
For a :t:`multiplication assignment`, ``core::ops::MulAssign::mul_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_69wr03rt0ali`
For a :t:`remainder assignment`, ``core::ops::RemAssign::rem_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_9d970yfwmj2d`
For a :t:`shift left assignment`, ``core::ops::ShlAssign::shl_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_p9687v3xckps`
For a :t:`shift right assignment`, ``core::ops::ShrAssign::shr_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:dp:`fls_8j408kckzzud`
For a :t:`subtraction assignment`, ``core::ops::SubAssign::sub_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

.. rubric:: Examples

.. code-block:: rust

   let mut result = 42;
   result += 1
   result &= 59
   result /= 3
   result ^= 2
   result *= 81
   result |= 9402
   result %= 7
   result <<= 2
   result >>= 3
   result -= 0

.. _fls_tpwp86mronn2:

Underscore Expressions
----------------------

.. rubric:: Syntax

.. syntax::

   UnderscoreExpression ::=
       $$_$$

.. rubric:: Legality Rules

:dp:`fls_pydmv629vfuu`
An :t:`underscore expression` is an :t:`expression` that acts as a placeholder
in a :t:`destructuring assignment`.

:dp:`fls_wms3dbwjwyu4`
An :t:`underscore expression` shall appear in the :t:`assigned operand` of a
:t:`destructuring assignment`.

.. rubric:: Examples

.. code-block:: rust

   let pair = (1, 2);
   let mut second = 0;
   (_, second) = pair;

.. _fls_g0uyl7qw4c7w:

Parenthesized Expressions
-------------------------

.. rubric:: Syntax

.. syntax::

   ParenthesizedExpression ::=
       $$($$ Operand $$)$$

.. rubric:: Legality Rules

:dp:`fls_jhazc75w5vj`
A :t:`parenthesized expression` is an :t:`expression` that groups other
:t:`[expression]s`.

:dp:`fls_ew9y5vaseehy`
A :t:`parenthesized expression` is a :t:`place expression` when its :t:`operand`
is a :t:`place expression`.

:dp:`fls_n4dhc0hvwwfk`
A :t:`parenthesized expression` is a :t:`value expression` when its :t:`operand`
is a :t:`value expression`.

:dp:`fls_5d66h7naoup6`
The :t:`type` of a :t:`parenthesized expression` is the :t:`type` of its
:t:`operand`.

:dp:`fls_xp9whcj2obk8`
The :t:`value` of a :t:`parenthesized expression` is the :t:`value` of its
:t:`operand`.

.. rubric:: Dynamic Semantics

:dp:`fls_2po52gv0m021`
The :t:`evaluation` of a :t:`parenthesized expression` evaluates its
:t:`operand`.

.. rubric:: Examples

.. code-block:: rust

   (1 + 2) * 3

.. _fls_xinykul167l:

Array Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   ArrayExpression ::=
       $$[$$ ArrayElementExpression? $$]$$

   ArrayElementExpression ::=
       ArrayElementConstructor
     | ArrayRepetitionConstructor

   ArrayElementConstructor ::=
       ExpressionList

   ArrayRepetitionConstructor ::=
       RepeatOperand $$;$$ SizeOperand

   RepeatOperand ::=
       Operand

   SizeOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_ya9res33oxt6`
An :t:`array expression` is an :t:`expression` that constructs an :t:`array`.

:dp:`fls_fwtd3b10veiw`
An :t:`array element constructor` is an :t:`array expression` that lists all
elements of the :t:`array` being constructed.

:dp:`fls_81jf78m5uga4`
An :t:`array repetition constructor` is an :t:`array expression` that specifies
how many times an element is repeated in the :t:`array` being constructed.

:dp:`fls_3y69y9ga4at7`
A :t:`repeat operand` is an :t:`operand` that specifies the element being
repeated in an :t:`array repetition constructor`.

:dp:`fls_2l9objtb23zn`
A :t:`size operand` is an :t:`operand` that specifies the size of an :t:`array`
or an :t:`array type`.

:dp:`fls_l0gbcyybzdv0`
An :t:`array expression` is a :t:`value expression`.

:dp:`fls_by21pey7k423`
The :t:`[type]s` of the :t:`[operand]s` of an :t:`array element constructor`
shall be :t:`unifiable`.

:dp:`fls_x2xu2pynxy1u`
If the :t:`size operand` is greater than one, then the :t:`type` of the
:t:`repeat operand` shall implement the :std:`core::copy::Copy` :t:`trait`
or the :t:`repeat operand` shall be a :t:`path expression` resolving to a
:t:`constant`.

:dp:`fls_qplsh3pdqitq`
The :t:`type` of the :t:`size operand` shall be :t:`type` :c:`usize`.

:dp:`fls_9gmnjvs83d8o`
The :t:`value` of the :t:`size operand` shall be a :t:`constant expression`.

:dp:`fls_wmsekin1gd2y`
The :t:`type` of an :t:`array expression` is ``[T; N]``, where ``T`` is the
element type and ``N`` is the size of the array. The :t:`size` of an :t:`array`
is determined as follows:

* :dp:`fls_2gto5kp9bjw8`
  If the :t:`array expression` appears with an :t:`array element constructor`,
  then the :t:`size` is the number of :t:`[operand]s` in the :t:`array element
  constructor`.

* :dp:`fls_guop34ayjw2`
  Otherwise the :t:`size` is the :t:`value` of :t:`size operand`.

:dp:`fls_aj6tbe54v5jl`
The :t:`value` of an :t:`array expression` is the constructed :t:`array`.

.. rubric:: Dynamic Semantics

:dp:`fls_t52in1kkyli3`
The :t:`evaluation` of an :t:`array expression` with an :t:`array element
constructor` evaluates its :t:`[operand]s` in left-to-right order.

:dp:`fls_1kj8nlc5eb8a`
The :t:`evaluation` of an :t:`array expression` with an :t:`array repetition
constructor` proceeds as follows:

#. :dp:`fls_f3izbkm8607z`
   If the :t:`value` of the :t:`size operand` is greater than zero, then:

   #. :dp:`fls_qbyysx30pjzs`
      If the :t:`repeat operand` denotes a :t:`constant`, the :t:`repeat
      operand` is evaluated once and its :t:`value` is passed :t:`by copy`
      :t:`[size operand]'s` :t:`value` times.

   #. :dp:`fls_1m0laldldh7j`
      Otherwise the :t:`repeat operand` is evaluated :t:`[size operand]'s`
      :t:`value` times.

#. :dp:`fls_5cs68nm54l31`
   Otherwise the :t:`repeat operand` is evaluated once.

.. rubric:: Examples

.. code-block:: rust

   [1, 2, 3]
   ["one", "two", "three",]

:dp:`fls_p7hcqryal5cm`
Two dimensional array.

.. syntax::

   [[0, 0], [0, 1], [1, 0], [1, 1]]

:dp:`fls_izlpt6100gvk`
An array of nine 42s.

.. syntax::

   [42; 9]

.. _fls_sxcr4aa098i6:

Indexing Expressions
--------------------

.. rubric:: Syntax

.. syntax::

   IndexExpression ::=
       IndexedOperand $$[$$ IndexingOperand $$]$$

   IndexedOperand ::=
       Operand

   IndexingOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_X9kdEAPTqsAe`
A :t:`indexable type` is a :t:`type` that implements :std:`core::ops::Index`.

:dp:`fls_42ijvuqqqlvh`
An :t:`index expression` is an :t:`expression` that indexes into a :t:`value`
of an :t:`indexable type`.

:dp:`fls_pc0c22asgzvw`
An :t:`indexed operand` is an :t:`operand` which indicates the :t:`value`
being indexed into by an :t:`index expression`.

:dp:`fls_ff3sgpldn52o`
An :t:`indexing operand` is an :t:`operand` which specifies the index of an :t:`index expression`.

:dp:`fls_w96p9oyv5mqt`
An :t:`index expression` is a :t:`constant expression` if the :t:`indexing
operand` and :t:`indexed operand` are :t:`[constant expression]s`.

:dp:`fls_u9sl7h4i8hqu`
The :t:`type` of the :t:`indexing operand` is the :t:`generic parameter` of the
:std:`core::ops::Index` implementation of the :t:`type` of the
:t:`indexed operand`.

:dp:`fls_98qeczwv7fmy`
If the :t:`indexed operand` is evaluated in a :t:`value expression context`,
then
LUKAS, what is a "value expression context"?

* :dp:`fls_jxdiknkwglak`
  The :t:`index expression` is a :t:`value expression`.

* :dp:`fls_sb2b8gszzaxq`
  The :t:`type` of the :t:`indexed operand` shall implement the
  :std:`core::ops::Index` :t:`trait`.

* :dp:`fls_issaykiuha75`
  The :t:`type` of the :t:`index expression` is ``&T``, where ``T`` is
  :t:`associated type` :std:`core::ops::Index::Output`.

:dp:`fls_y3sduoma6q9v`
If the :t:`indexed operand` is :t:`mutable` and the :t:`index expression` is
evaluated in a :t:`mutable place expression context`, then
LUKAS, what is a "mutable place expression context"?

* :dp:`fls_pjmoo8mjgxz3`
  The :t:`index expression` is a :t:`mutable place expression`.

* :dp:`fls_ld7lbvqms5i6`
  The :t:`type` of the :t:`indexed operand` shall implement the
  :std:`core::ops::IndexMut` :t:`trait`.

* :dp:`fls_nw705fpon79b`
  The :t:`type` of the :t:`index expression` is ``&mut T``, where ``T`` is
  the element type of the :t:`[indexed operand]'s` :t:`type`.

:dp:`fls_fouu0z3jwoad`
The :t:`value` of an :t:`index expression` is the indexed memory location.

.. rubric:: Dynamic Semantics

:dp:`fls_6sgj0ltt21i`
The :t:`evaluation` of an :t:`index expression` proceeds as follows:

#. :dp:`fls_e5l4y3dy69xi`
   The :t:`indexed operand` is evaluated.

#. :dp:`fls_fza3omn8yw7s`
   The :t:`indexing operand` is evaluated.

#. :dp:`fls_ehamppbq4gmg`
   If the :t:`index expression` is evaluated as a :t:`mutable place
   expression`, then :t:`expression` ``*core::ops::IndexMut::index_mut(&mut
   indexed_operand, indexing_operand)`` is evaluated.

#. :dp:`fls_i68oxj659hc1`
   Otherwise :t:`expression` ``*core::ops::Index::index(&indexed_operand,
   indexing_operand)`` is evaluated.

.. rubric:: Examples

.. code-block:: rust

   let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
   a[1][2]

:dp:`fls_esvpmh6razg3`
Evaluates to 6.

.. _fls_k64tfywtn0g8:

Tuple Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   TupleExpression ::=
       $$($$ TupleInitializerList? $$)$$

   TupleInitializerList ::=
       ExpressionList

.. rubric:: Legality Rules

:dp:`fls_87rp1hfwvjel`
A :t:`tuple expression` is an :t:`expression` that constructs a :t:`tuple`.

:dp:`fls_581y6jq1eyn8`
A :t:`tuple initializer` is an :t:`operand` that provides the :t:`value` of a
:t:`tuple field` in a :t:`tuple expression`.

:dp:`fls_3dmehkxewz6d`
A :t:`tuple expression` is a :t:`value expression`.

:dp:`fls_ljz3sxmfzflm`
The :t:`type` of a :t:`tuple expression` is ``(T1, T2, ..., TN)``, where ``T1``
is the :t:`type` of the first :t:`tuple initializer`, ``T2`` is the :t:`type` of
the second :t:`tuple initializer`, and ``TN`` is the :t:`type` of the ``N``-th
:t:`tuple initializer`.

:dp:`fls_k2aznqo9j49p`
The :t:`value` of a :t:`tuple expression` is ``(V1, V2, ..., VN)``, where ``V1``
is the :t:`value` of the first :t:`tuple initializer`, ``V2`` is the :t:`value`
of the second :t:`tuple initializer`, and ``VN`` is the :t:`value` of the
``N``-th :t:`tuple initializer`.

:dp:`fls_fgthjiu980rr`
The :t:`value` of a :t:`tuple expression` without any :t:`[tuple initializer]s`
is the :t:`unit value`.

.. rubric:: Dynamic Semantics

:dp:`fls_waf55yd3mpsq`
The :t:`evaluation` of a :t:`tuple expression` evaluates its :t:`[tuple
initializer]s` in left-to-right order.

.. rubric:: Examples

.. code-block:: rust

   ()
   (1.2, 3.4)
   ("hello", 42i16, true)

.. _fls_8tsynkj2cufj:

Struct Expressions
------------------

.. rubric:: Syntax

.. syntax::

   StructExpression ::=
       Constructee $${$$ StructExpressionContent? $$}$$

   Constructee ::=
       PathInExpression

   StructExpressionContent ::=
       BaseInitializer
     | FieldInitializerList ($$,$$ BaseInitializer | $$,$$?)

   BaseInitializer ::=
       $$..$$ Operand

   FieldInitializerList ::=
       FieldInitializer ($$,$$ FieldInitializer)*

   FieldInitializer ::=
       IndexedInitializer
     | NamedInitializer
     | ShorthandInitializer

   IndexedInitializer ::=
       TupleIndex : Expression

   NamedInitializer ::=
       Identifier : Expression

   ShorthandInitializer ::=
       Identifier

.. rubric:: Legality Rules

:dp:`fls_ij8rebvupb85`
A :t:`struct expression` is an :t:`expression` that constructs an :t:`enum value`, a
:t:`struct value`, or a :t:`union value`.

:dp:`fls_4z91ymz3ciup`
A :t:`constructee` indicates the :t:`enum variant`, :t:`struct` or :t:`union` whose value is
being constructed by a :t:`struct expression`.

:dp:`fls_uib1ml41mfrn`
A :t:`base initializer` is a :t:`construct` that specifies an :t:`enum value`,
a :t:`struct value`, or a :t:`union value` to be used as a base for construction in a
:t:`struct expression`.

:dp:`fls_gfu267bpl9ql`
The :t:`type` of a :t:`base initializer` is the :t:`type` of its :t:`operand`.
The :t:`type` of a :t:`base initializer` shall be the same as the :t:`type` of
the :t:`constructee`.

:dp:`fls_ph7fsphbpbv4`
An :t:`indexed initializer` is a :t:`construct` that specifies the index and
initial :t:`value` of a :t:`field` in a :t:`struct expression`.

:dp:`fls_y3p6rtm7ek3l`
An :t:`indexed initializer` matches a :t:`field` of the :t:`constructee`
when the :t:`tuple index` of the :t:`indexed initializer` resolves to a valid
position of a :t:`field` in the :t:`constructee`. Such an :t:`indexed
initializer` is a :dt:`matched indexed initializer`.

:dp:`fls_dfajs3xaxbv`
The :t:`type` of the :t:`operand` of an :t:`indexed initializer` and the
:t:`type` of the matched :t:`field` shall be :t:`unifiable`.

:dp:`fls_e5b9n910z1cp`
The :t:`value` of an :t:`indexed initializer` is the :t:`value` of its
:t:`operand`.

:dp:`fls_lwyq3vyc91rn`
A :t:`named initializer` is a :t:`construct` that specifies the name and initial
:t:`value` of a :t:`field` in a :t:`struct expression`.

:dp:`fls_qed1pps827dv`
A :t:`named initializer` matches a :t:`field` of the :t:`constructee` when
its :t:`identifier` and the :t:`name` of the :t:`field` are the same. Such a
:t:`named initializer` is a :dt:`matched named initializer`.

:dp:`fls_b60omrhc7t73`
The :t:`type` of a :t:`named initializer` and the :t:`type` of the matched
:t:`field` shall be :t:`unifiable`.

:dp:`fls_z3gj1v6g605r`
The :t:`value` of a :t:`named initializer` is the :t:`value` of its
:t:`expression`.

:dp:`fls_57t368kema7h`
A :t:`shorthand initializer` is a :t:`construct` that specifies the :t:`name` of
a :t:`field` in a :t:`struct expression`.

:dp:`fls_sm2hx8sh4agb`
A :t:`shorthand initializer` is equivalent to a :t:`named initializer` where
both the :t:`identifier` and the :t:`expression` of the :t:`named initializer`
denote the :t:`identifier` of the :t:`shorthand initializer`.

:dp:`fls_yjx1t3x6qpfg`
A :t:`shorthand initializer` matches a :t:`field` of the :t:`constructee`
when its :t:`identifier` and the :t:`name` of the :t:`field` are the same. Such
a :t:`shorthand initializer` is a :dt:`matched shorthand initializer`.

:dp:`fls_2dajkhq58cdp`
The :t:`type` of a :t:`shorthand initializer` and the :t:`type` of the matched
:t:`field` shall be :t:`unifiable`.

:dp:`fls_9s4znhi0u3ys`
The :t:`value` of a :t:`shorthand initializer` is the :t:`value` its
:t:`identifier` resolves to.

:dp:`fls_3dialm6x0x7a`
A :t:`struct expression` is a :t:`value expression`.

:dp:`fls_i31rodt42m0z`
The :t:`type` of a :t:`struct expression` is the :t:`type` of the
:t:`constructee`.

:dp:`fls_sjwd8o5mknjo`
The :t:`value` of a :t:`struct expression` is the :t:`enum value`, :t:`struct value`, or
:t:`union value` in construction.

:dp:`fls_ccqomsereni2`
If the :t:`constructee` is a :t:`record enum variant` or a :t:`record struct`,
then

* :dp:`fls_pivpdyr4seez`
  For each :t:`field` of the :t:`constructee`, the :t:`struct expression` shall
  either:

  * :dp:`fls_bbmm5vir9xos`
    Contain at most one :t:`matched named initializer`, or

  * :dp:`fls_9370n5xkkzce`
    Contain at most one :t:`matched shorthand initializer`, or

  * :dp:`fls_rclgwzdhfjj`
    Have exactly one :t:`base initializer`.

* :dp:`fls_lmxz5768v5d8`
  A :t:`base initializer` is allowed even if all :t:`[field]s` of the
  :t:`constructee` have been matched.

:dp:`fls_939cugbxju5e`
If the :t:`constructee` is a :t:`tuple enum variant` or a :t:`tuple struct`,
then

* :dp:`fls_c34qwhaq2asm`
  For each :t:`field` of the :t:`constructee`, the :t:`struct expression` shall
  either:

  * :dp:`fls_j2kmp1fee0g4`
    Contain at most one :t:`matched indexed initializer`, or

  * :dp:`fls_90q7krxazc6u`
    Have exactly one :t:`base initializer`.

* :dp:`fls_qo05owpmtag0`
  A :t:`base initializer` is allowed even if all :t:`[field]s` of the
  :t:`constructee` have been matched.

:dp:`fls_ywh3nk6emwmw`
If the :t:`constructee` is a :t:`union type`, then

* :dp:`fls_5w9lj5dc84p`
  The :t:`struct expression` shall not contain a :t:`base initializer`.

* :dp:`fls_5zceer19mhdu`
  For the single :t:`field` of the :t:`constructee`, the :t:`struct expression`
  shall either:

  * :dp:`fls_mq80i8fof7sx`
    Contain exactly one :t:`matched named initializer`, or

  * :dp:`fls_raon1c1vrhx7`
    Contain exactly one :t:`matched shorthand initializer`.

:dp:`fls_njder5r7y5fg`
If the :t:`constructee` is a :t:`unit enum variant` or a :t:`unit struct`, then
the :t:`struct expression` shall have at most one :t:`base initializer`.

:dp:`fls_w7x9wy6t0qcp`
If a :t:`base initializer` is supplied, then for each :t:`field` that was not
matched in the :t:`struct expression`:

* :dp:`fls_24kqbc9oytaq`
  If the :t:`type` of the :t:`field` is a :t:`by copy type`, then the :t:`value`
  of the :t:`field` is copied and the copy becomes the initial :t:`value` of the
  :t:`field` of the :t:`constructee`, or

* :dp:`fls_rsc4c09tuqx9`
  If the :t:`type` of the :t:`field` is a :t:`by move type`, then the :t:`value`
  of the :t:`field` is moved and becomes the initial :t:`value` of the
  :t:`field` of the :t:`constructee`.

.. rubric:: Dynamic Semantics

:dp:`fls_vsxsbqps64o`
The :t:`evaluation` of a :t:`struct expression` evaluates its :t:`[operand]s` in
a left-to-right order.

.. rubric:: Examples

.. code-block:: rust

   enum Occupation {
       Engineer,
       Gardener
   }

   struct Employee {
       name: String,
       age: u16,
       occupation: Occupation
       compensation: u32
   }

   let alice = Employee {
       name: "Alice".to_string(),
       age: 23,
       occupation: Occupation::Engineer
       compensation: 250_000
   };

   let age = 45;

   let bob = Employee {
       name: "Bob".to_string(), // matched named initializer
       age, // matched shorthand initializer
       .. alice // equivalent to alice.occupation, alice.compensation
   };

   union Union {
   	int: u32,
   	float: f32
   }

   let u1 = Union { int: 0 };
   let u2 = Union { float: 0.0 };

.. _fls_xa4nbfas01cj:

Call Expressions
----------------

.. rubric:: Syntax

.. syntax::

   CallExpression ::=
       CallOperand $$($$ ArgumentOperandList? $$)$$

   CallOperand ::=
       Operand

   ArgumentOperandList ::=
       ExpressionList

.. rubric:: Legality Rules

:dp:`fls_fvgfx17ossd9`
A :t:`call expression` is an :t:`expression` that invokes a :t:`function` or
constructs a :t:`tuple struct value` or :t:`tuple enum variant value`.

:dp:`fls_jvz5z3eqxb39`
An :t:`argument operand` is an :t:`operand` which is used as an argument in a
:t:`call expression` or a :t:`method call expression`.

:dp:`fls_7ql1c71eidg8`
A :t:`call operand` is the :t:`function` being invoked or the
:t:`tuple struct value` or :t:`tuple enum variant value` being constructed by a
:t:`call expression`.

:dp:`fls_4t6imtiw6kzt`
A :t:`callee type` is either a :t:`function item type`,
a :t:`function pointer type`, a :t:`tuple struct type`,
a :t:`tuple enum variant` or a :t:`type` that implements any of the
:std:`core::ops::Fn`, :std:`core::ops::FnMut`, or :std:`core::ops::FnOnce`
:t:`[trait]s`.

:dp:`fls_aafrvlmiwfon`
The :t:`call operand` is subject to :t:`auto dereferencing` until a
:t:`callee type` is found, otherwise this is a static error.

:dp:`fls_d8rewso3dm6r`
An :t:`adjusted call operand` is a :t:`call operand` with possible
:t:`auto dereferencing` adjustments.

:dp:`fls_bu6i3mcvnbin`
The :t:`type` of a :t:`call expression` is the :t:`return type` of the invoked
:t:`function`, the :t:`type` of the :t:`tuple struct` or the
:t:`tuple enum variant` being constructed, or :t:`associated type`
:std:`core::ops::FnOnce::Output`.

:dp:`fls_8ljrgdept7s8`
A :t:`call expression` whose :t:`callee type` is either an :t:`external function
item type`, an :t:`unsafe function item type` or an :t:`unsafe function pointer
type` shall require :t:`unsafe context`.

:dp:`fls_7p6zrjbpj0kl`
The :t:`value` of a :t:`call expression` is determined as follows:

* :dp:`fls_yrr1s0tucgvh`
  If the :t:`callee type` is a :t:`function item type` or a :t:`function
  pointer type`, then the :t:`value` is the result of invoking the corresponding
  :t:`function` with the :t:`[argument operand]s`.

* :dp:`fls_RZjFs9koNOk8`
  If the :t:`callee type` is a :t:`tuple struct type` or a :t:`tuple enum variant`,
  then the :t:`value` is the result of constructing the :t:`tuple struct` or :t:`tuple enum variant`
  with the :t:`[argument operand]s`.

* :dp:`fls_s3q3sej1hgho`
  If the :t:`callee type` implements the :std:`core::ops::Fn`
  :t:`trait`, then the :t:`value` is the result of invoking
  ``core::ops::Fn::call(adjusted_call_operand, argument_operand_tuple)``,
  where ``adjusted_call_operand`` is the :t:`adjusted call operand`, and
  ``argument_operand_tuple`` is a :t:`tuple` that wraps the :t:`[argument
  operand]s`.

* :dp:`fls_cu2ubdm3tfwb`
  If the :t:`call operand` implements the :std:`core::ops::FnMut`
  :t:`trait`, then the :t:`value` is the result of invoking
  ``core::ops::FnMut::call_mut(adjusted_call_operand, argument_operand_tuple),``
  where ``adjusted_call_operand`` is the :t:`adjusted call operand`, and
  ``argument_operand_tuple`` is a :t:`tuple` that wraps the :t:`[argument
  operand]s`.

* :dp:`fls_9bbewx1l7h5h`
  If the :t:`call operand` implements the :std:`core::ops::FnOnce`
  :t:`trait`, then the :t:`value` is the result of invoking
  ``core::ops::FnOnce::call_once(adjusted_call_operand,
  argument_operand_tuple),`` where ``adjusted_call_operand`` is the :t:`adjusted
  call operand`, and ``argument_operand_tuple`` is a :t:`tuple` that wraps the
  :t:`[argument operand]s`.

.. rubric:: Dynamic Semantics

:dp:`fls_ggr5i91vur0r`
The :t:`evaluation` of a :t:`call expression` proceeds as follows:

#. :dp:`fls_hwalzgdidbfz`
   The :t:`call operand` is evaluated.

#. :dp:`fls_p52mfvpadu7w`
   The :t:`[argument operand]s` are evaluated in left-to-right order.

#. :dp:`fls_1cyo5qhbl1j9`
   If the :t:`adjusted call operand` is a :t:`function item type` or
   :t:`function pointer type`, then corresponding :t:`function` is invoked.

#. :dp:`fls_nb0eqky2akzt`
   If the :t:`type` of the :t:`call operand` implements the :std:`core::ops::Fn`
   :t:`trait`, then ``core::ops::Fn::call(adjusted_call_operand,
   argument_operand_tuple)`` is invoked.

#. :dp:`fls_9lt4wh9ql5ae`
   If the :t:`type` of the :t:`call operand` implements
   the :std:`core::ops::FnMut` :t:`trait`, then
   ``core::ops::FnMut::call_mut(adjusted_call_operand, argument_operand_tuple)``
   is invoked.

#. :dp:`fls_ixebnlcccmit`
   If the :t:`type` of the :t:`call operand` implements
   the :std:`core::ops::FnOnce` :t:`trait`, then
   ``core::ops::FnOnce::call_once(adjusted_call_operand,
   argument_operand_tuple)`` is invoked.

.. rubric:: Undefined Behavior

:dp:`fls_5yeq4oah58dl`
It is undefined behavior to call a :t:`function` with an :t:`ABI` other than the
:t:`ABI` the :t:`function` was defined with.

.. rubric:: Examples

.. code-block:: rust

   let three: i32 = add(1, 2);

.. _fls_z7q8kbjwdc7g:

Method Call Expressions
-----------------------

.. rubric:: Syntax

.. syntax::

   MethodCallExpression ::=
       ReceiverOperand $$.$$ PathInExpressionSegment $$($$ ArgumentOperandList? $$)$$

   ReceiverOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_b7i26954j1hc`
A :t:`method call expression` is an :t:`expression` that invokes a :t:`method`
of a :t:`variable`.

:dp:`fls_jx3ryre0xs88`
A :t:`receiver operand` is an :t:`operand` that denotes the :t:`value` whose
:t:`method` is being invoked by a :t:`method call expression`.

:dp:`fls_y7bj7y6davlh`
A :t:`method call expression` is subject to :t:`method resolution`.

:dp:`fls_11glzggtbgb3`
The :t:`type` of a :t:`method call expression` is the :t:`return type` of the
invoked :t:`method`.

:dp:`fls_ljvj1f9fv085`
The :t:`value` of a :t:`method call expression` is the :t:`value` returned by
the invoked :t:`method`.

.. rubric:: Dynamic Semantics

:dp:`fls_oxxk3snd7ya0`
The :t:`evaluation` of a :t:`method call expression` proceeds as follows:

#. :dp:`fls_gmpq15g77o20`
   The :t:`receiver operand` is evaluated.

#. :dp:`fls_pu0n9hakkym2`
   The :t:`[argument operand]s` are evaluated in left-to-right order.

#. :dp:`fls_cawdkgvvd1x6`
   The :t:`method` is invoked.

.. rubric:: Examples

.. code-block:: rust

   trait Command {
       fn execute(&self);
   }

   struct ClickCommand { ... }

   impl ClickCommand for Command {
       fn execute(&self) {
           println!("Someone clicked me!")
       }
   }

   let click = ClickCommand { ... };
   click.execute();

.. _fls_18k3uajrgq5f:

Field Access Expressions
------------------------

.. rubric:: Syntax

.. syntax::

   FieldAccessExpression ::=
       ContainerOperand $$.$$ FieldSelector

   ContainerOperand ::=
       Operand

   FieldSelector ::=
       IndexedFieldSelector
     | NamedFieldSelector

   IndexedFieldSelector ::=
       DecimalLiteral

   NamedFieldSelector ::=
       Identifier

.. rubric:: Legality Rules

:dp:`fls_hr8qvwlhd9ts`
A :t:`field access expression` is an :t:`expression` that accesses a :t:`field`
of a :t:`value`.

:dp:`fls_s2vpn4ihenpe`
A :t:`container operand` is an :t:`operand` that indicates the :t:`value` whose
:t:`field` is selected in a :t:`field access expression`.

:dp:`fls_yeuayil6uxzx`
A :t:`field selector` is a :t:`construct` that selects the :t:`field` to be
accessed in a :t:`field access expression`.

:dp:`fls_qqrconpa92i3`
A :t:`selected field` is a :t:`field` that is selected by a :t:`field access
expression`.

:dp:`fls_t9xakmda134a`
A :t:`field access expression` with an :s:`IndexedFieldSelector` is referred to
as an :dt:`indexed field access`.

:dp:`fls_dch5i39ycw7s`
A :t:`field access expression` with a :s:`NamedFieldSelector` is referred to as
a :dt:`named field access`.

:dp:`fls_1l92izxtm1t8`
A :t:`field access expression` is a :t:`place expression`.

:dp:`fls_1bbbw0qj0h0q`
A :t:`field access expression` is a :t:`mutable place expression` when its
:t:`container operand` is :t:`mutable`.

:dp:`fls_fovs9il2h9xg`
The :t:`type` of a :t:`field access expression` is the :t:`type` of the
:t:`selected field`.

:dp:`fls_r1b4n12i93pg`
The :t:`value` of a :t:`field access expression` is the :t:`value` of the
:t:`selected field`.

.. rubric:: Dynamic Semantics

:dp:`fls_6uzouesw2sod`
The :t:`evaluation` of a :t:`field access expression` evaluates its
:t:`container operand`.

.. _fls_yx65ucoaimdp:

Named Field Access
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_kddnnz8uc15b`
Reading the :t:`selected field` of a :t:`union` shall require :t:`unsafe
context`.

:dp:`fls_an3no949lvfw`
Writing to the :t:`selected field` of a :t:`union` where the :t:`type` of the
:t:`selected field` implements the :std:`core::marker::Copy` :t:`trait` or the
:std:`core::mem::ManuallyDrop` :t:`trait` shall not require :t:`unsafe context`.

:dp:`fls_t6xmsm2nk1bc`
Writing to and then reading from the :t:`selected field` of a :t:`union`
subject to :t:`attribute` :c:`repr` is equivalent to invoking :t:`function`
``core::mem::transmute<write_type, read_type>(field_bits)`` where ``write_type``
is the :t:`type` used at the time of writing the :t:`selected field`,
``read_type`` is the :t:`type` used at the time of reading the :t:`selected
field`, and ``field_bits`` is the bit representation of the :t:`selected field`.

.. rubric:: Undefined Behavior

:dp:`fls_sdnafipirg8w`
It is undefined behavior when the :t:`type` of the :t:`container operand` is a
:t:`union type` and the :t:`selected field` contains invalid data.

.. rubric:: Examples

:dp:`fls_x27yayh4z787`
See :p:`6.8.1. <fls_hv4grs2tcuiw>` for the declaration of ``alice``.

.. code-block:: rust

   alice.name

.. _fls_e4q0018ch25g:

Indexed Field Access
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_zexojym4ak6f`
The :t:`decimal literal` of an :t:`indexed field access` shall denote a valid
index of a :t:`field` of the :t:`[container operand]'s` :t:`type`.

.. rubric:: Examples

:dp:`fls_dimto84ifanr`
The following indexed field access evaluates to ``42``.

.. code-block:: rust

   ("hello", 42i16, true).1

.. _fls_tjyexqrx0fx5:

Closure Expressions
-------------------

.. rubric:: Syntax

.. syntax::

   ClosureExpression ::=
       $$move$$? $$|$$ ClosureParameterList? $$|$$
         (ClosureBody | ClosureBodyWithReturnType)

   ClosureBody ::=
       Expression

   ClosureBodyWithReturnType ::=
       ReturnTypeWithoutBounds BlockExpression

   ReturnTypeWithoutBounds ::=
       $$->$$ TypeSpecificationWithoutBounds

   ClosureParameterList ::=
       ClosureParameter ($$,$$ ClosureParameter)* $$,$$?

   ClosureParameter ::=
       OuterAttributeOrDoc* PatternWithoutAlternation TypeAscription?

.. rubric:: Legality Rules

:dp:`fls_2d141c9a0yui`
A :t:`closure expression` is an :t:`expression` that defines a :t:`closure
type`.

:dp:`fls_srbl7ptknjyk`
A :t:`closure body` is a :t:`construct` that represents the executable portion
of a :t:`closure expression`.

:dp:`fls_oey0ivaiu1l`
A :t:`closure body` denotes a new :t:`control flow boundary`.

:dp:`fls_fg8lx0yyt6oq`
A :t:`closure body` is subject to :t:`capturing`.

:dp:`fls_yn30xuejcfxo`
The :t:`type` of a :t:`closure expression` is the anonymous unique :t:`closure
type` defined by it.

:dp:`fls_sje6cdvifgv5`
The :t:`value` of a :t:`closure expression` is the :t:`value` of the anonymous
unique :t:`closure type` instantiated with the selected :t:`[capture target]s`.

.. rubric:: Dynamic Semantics

:dp:`fls_f59fw17gsasn`
The :t:`evaluation` of a :t:`closure expression` proceeds as follows:

#. :dp:`fls_7w15ccc1zzxl`
   An anonymous :t:`value` of an anonymous unique :t:`closure type` is created.

#. :dp:`fls_b8w9y73pvdnm`
   The :t:`closure body` is evaluated.

.. rubric:: Examples

.. code-block:: rust

   fn do_ten_times<F>(consumer: F) where F: Fn(i32) {
       for times in 0 .. 10 {
           consumer(times);
       }
   }

   do_ten_times(|value: i32| { println!("value: {}", value)});

.. _fls_rr908hgunja7:

Loop Expressions
----------------

.. rubric:: Syntax

.. syntax::

   LoopExpression ::=
       Label? LoopContent

   Label ::=
       $$'$$ NonKeywordIdentifier

   LoopContent ::=
       ForLoopExpression
     | InfiniteLoopExpression
     | WhileLetLoopExpression
     | WhileLoopExpression

.. rubric:: Legality Rules

:dp:`fls_y1d8kd1bdlmx`
A :t:`loop expression` is an :t:`expression` that evaluates a :t:`block
expression` continuously as long as some criterion holds true.

:dp:`fls_eg93m93gvwal`
An :t:`anonymous loop` is a :t:`loop expression` without a :t:`label`.

:dp:`fls_phpoq9ho8f1v`
A :t:`named loop` is a :t:`loop expression` with a :t:`label`.

:dp:`fls_b314wjbv0zwe`
The :t:`type` of a :t:`loop expression` is determined as follows:

* :dp:`fls_rpedapxnv8w3`
  If the :t:`loop expression` does not contain a :t:`break expression`, then the
  :t:`type` is the :t:`never type`.

* :dp:`fls_wf11yp1jwf53`
  If the :t:`loop expression` contains at least one :t:`break expression`,
  then the :t:`type` is the :t:`unified type` of the :t:`[break type]s` of all
  :t:`[break expression]s`.

:dp:`fls_q3qpcf2fz7h`
The :t:`value` of a :t:`loop expression` is determined as follows:

* :dp:`fls_2ulbzmuuny3g`
  If the :t:`loop expression` does not contain a :t:`break expression`, then the
  :t:`value` is the :t:`unit value`.

* :dp:`fls_99imks9hj3kp`
  If the :t:`loop expression` contains at least one :t:`break expression`, then
  the :t:`value` is the :t:`break value` of the :t:`break expression` that broke
  out of the :t:`loop expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_aw6qczl4zpko`
A :t:`loop expression` is :t:`terminated` when its :t:`block expression` is no
longer evaluated.

.. _fls_onfyolkcbeh3:

For Loops
~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ForLoopExpression ::=
       $$for$$ Pattern $$in$$ SubjectExpression BlockExpression

.. rubric:: Legality Rules

:dp:`fls_1bh2alh37frz`
A :t:`for loop expression` is a :t:`loop expression` that continues to evaluate
its :t:`block expression` as long as its :t:`subject expression` yields a
:t:`value`.

:dp:`fls_fkgbin6ydkm4`
The :t:`type` of a :t:`subject expression` shall implement the
:std:`core::iter::IntoIterator` :t:`trait`.

.. rubric:: Dynamic Semantics

:dp:`fls_kuxo0on3vit6`
The :t:`evaluation` of a :t:`for loop expression` of the form

.. code-block:: rust

   'label: for pattern in subject_expression {
       /* loop body */
   }

:dp:`fls_2lrzrtjhsdes`
is equivalent to the :t:`evaluation` of the following :t:`block expression`:

.. code-block:: rust

   {
       let result =
           match core::iter::IntoIterator::into_iter
               (subject_expression)
       {
           mut iter => 'label: loop {
               let mut next_value;
               match core::iter::Iterator::next(&mut iter) {
                   Option::Some(value) => next_value = value,
                   Option::None => break
               };
               let pattern = next_value;
               let () = { /* loop body */ };
           }
       };
       result
   }

.. rubric:: Examples

.. code-block:: rust

   let favorite_fruits = &["apples", "pears", "strawberries"];

   for fruit in favorite_fruits {
       println!("I like eating {}.", fruit);
   }

.. _fls_sf4qnd43z2wc:

Infinite Loops
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   InfiniteLoopExpression ::=
       $$loop$$ BlockExpression

.. rubric:: Legality Rules

:dp:`fls_p11qw6mtxlda`
An :t:`infinite loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`block expression` indefinitely unless :t:`terminated` with a
:t:`break expression` or a :t:`return expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_w4tj5gofwih1`
The :t:`evaluation` of an :t:`infinite loop expression` proceeds as follows:

#. :dp:`fls_pg3r6nyl865`
   The :t:`block expression` is evaluated.

#. :dp:`fls_lp15ilkul2uv`
   Control restarts the :t:`evaluation` of the :t:`infinite loop expression`.

.. rubric:: Examples

.. code-block:: rust

   loop {
       println!("I am alive!");
   }

.. _fls_5jjm1kt43axd:

While Loops
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WhileLoopExpression ::=
       $$while$$ IterationExpression BlockExpression

   IterationExpression ::=
       Expression

.. rubric:: Legality Rules

:dp:`fls_ajby242tnu7c`
A :t:`while loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`block expression` as long as its :t:`iteration expression`
holds true.

:dp:`fls_13hmhzqz82v6`
An :t:`iteration expression` is an :t:`expression` that provides the criterion
of a :t:`while loop expression`.

:dp:`fls_d7ofrq3777kq`
The :t:`type` of an :t:`iteration expression` shall be :t:`type` :c:`bool`.

.. rubric:: Dynamic Semantics

:dp:`fls_1i7hm645h7ox`
The :t:`evaluation` of a :t:`while loop expression` proceeds as follows:

#. :dp:`fls_5x0du3u1jwd3`
   The :t:`iteration expression` is evaluated.

#. :dp:`fls_23uluvhhoct6`
   If the :t:`iteration expression` evaluated to ``true``, then:

   #. :dp:`fls_k7g4cac93617`
      The :t:`block expression` is evaluated.

   #. :dp:`fls_j08k3brdpgno`
      Control restarts the :t:`evaluation` of the :t:`while loop expression`.

.. rubric:: Examples

.. code-block:: rust

   let mut counter = 0;

   while counter < 5 {
       counter += 1;
       println("iteration {}", counter);
   }

.. _fls_m6kd5i9dy8dx:

While Let Loops
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WhileLetLoopExpression ::=
       $$while$$ $$let$$ Pattern $$=$$ SubjectLetExpression BlockExpression

.. rubric:: Legality Rules

:dp:`fls_fmdlyp9r9zl7`
A :t:`while let loop expression` is a :t:`loop expression` that continues to
evaluate its :t:`block expression` as long as its :t:`subject let expression`
yields a :t:`value` that can be matched against its :t:`pattern`.

.. rubric:: Dynamic Semantics

:dp:`fls_z2ht5iaat5ag`
The :t:`evaluation` of a :t:`while let loop expression` of the form

.. code-block:: rust

   'label: let pattern = subject_let_expression {
       /* loop body */
   }

:dp:`fls_pacf1uavh1qt`
shall be equivalent to the :t:`evaluation` the following :t:`infinite loop`:

.. code-block:: rust

   'label: loop {
       match subject_let_expression {
           pattern => { /* loop body */ },
           _ => break
       }
   }

.. rubric:: Examples

.. code-block:: rust

   let mut favorite_animals = vec!["cats", "dogs", "otters"];

   while let Some(animal) = favorite_animals.pop() {
       println!("I like petting {}", animal);
   }

.. _fls_uusi0zej55is:

Loop Labels
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   LabelIndication ::=
       $$'$$ NonKeywordIdentifier

.. rubric:: Legality Rules

:dp:`fls_tx5u743391h7`
A :t:`label indication` is a :t:`construct` that indicates a :t:`label`.

:dp:`fls_7hc8yboeaho0`
A :t:`label indication` shall indicate a :t:`label` of an enclosing :t:`named
loop` that does not pass a :t:`control flow boundary` in order to reach the
enclosing :t:`named loop`.

.. _fls_jr4tpuyksr75:

Break Expressions
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BreakExpression ::=
       $$break$$ LabelIndication? Operand?

.. rubric:: Legality Rules

:dp:`fls_i5ko1t2wbgxe`
A :t:`break expression` is an :t:`expression` that terminates a :t:`loop
expression`.

:dp:`fls_jiykbp51909f`
A :t:`break expression` shall appear within a :t:`loop expression`.

:dp:`fls_7frvr2nm2mcj`
The :t:`label indication` of a :t:`break expression` shall resolve to the
:t:`label` of an enclosing :t:`named loop`.

:dp:`fls_ghxns2nggffj`
A :t:`break expression` without a :t:`label indication` is associated with the
innermost enclosing :t:`loop expression`.

:dp:`fls_54d5uydc87td`
A :t:`break expression` with a :t:`label indication` is associated with a
:t:`named loop` whose :t:`label` is indicated by the :t:`label indication`.

:dp:`fls_6x15ig8drne8`
A :t:`break expression` shall have an :t:`operand` only when it is associated
with an :t:`infinite loop`.

:dp:`fls_dnnq1zym8ii0`
The :t:`type` of a :t:`break expression` is the :t:`never type`.

:dp:`fls_1wdybpfldj7q`
:t:`Break type` is the :t:`type` of the :t:`operand` of a :t:`break expression`.

:dp:`fls_8yore99adr22`
The :t:`break type` is determined as follows:

* :dp:`fls_60imbzwg3e2x`
  If the :t:`break expression` lacks an :t:`operand`, then the :t:`break type`
  is the :t:`unit type`.

* :dp:`fls_l0c05wa9q97w`
  If the :t:`break expression` has an :t:`operand`, then the :t:`break type` is
  the :t:`type` of its :t:`operand`.

:dp:`fls_bgd7d5q69q0g`
:t:`Break value` is the :t:`value` of the :t:`operand` of a :t:`break
expression`.

:dp:`fls_yb8jv4mkmki0`
The :t:`break value` is determined as follows:

* :dp:`fls_d7l1y2qbe8br`
  If the :t:`break expression` lacks an :t:`operand`, then the :t:`break value`
  is the :t:`unit value`.

* :dp:`fls_56szfyilc06`
  If the :t:`break expression` has an :t:`operand`, then the :t:`break value` is
  the :t:`value` of its :t:`operand`.

.. rubric:: Dynamic Semantics

:dp:`fls_jnpx8mx1oa7n`
The :t:`evaluation` of a :t:`break expression` proceeds as follows:

#. :dp:`fls_l2kp8mw6bjj0`
   The :t:`operand` is evaluated.

#. :dp:`fls_2nmadhe3ismj`
   All enclosing :t:`[loop expression]s` upto and including the associated
   :t:`loop expression` are :t:`terminated`.

.. rubric:: Examples

:dp:`fls_32fwis9pxh77`
The following break expression terminates both the inner and the outer loop.

.. code-block:: rust

   'outer: loop {
       'inner: loop {
           break 'outer;
       }
   }

.. _fls_sjwrlwvpulp:

Continue Expressions
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ContinueExpression ::=
       $$continue$$ LabelIndication?

.. rubric:: Legality Rules

:dp:`fls_wzs6kz9ffqzt`
A :t:`continue expression` shall appear within a :t:`loop expression`.

:dp:`fls_r5ke7b9n7k3s`
A :t:`continue expression` without a :t:`label indication` is associated with
the innermost enclosing :t:`loop expression`.

:dp:`fls_ckm6i9c3s6j8`
A :t:`continue expression` with a :t:`label indication` is associated with a
:t:`named loop` whose :t:`label` is indicated by the :t:`label indication`.

:dp:`fls_d0bmw8xiw5nk`
The :t:`type` of a :t:`continue expression` is the :t:`never type`.

:dp:`fls_b7m0h2i3mot1`
The :t:`value` of a :t:`continue expression` is the :t:`unit value`.

.. rubric:: Dynamic Semantics

:dp:`fls_vmyuuptfnwek`
The :t:`evaluation` of a :t:`continue expression` proceeds as follows:

#. :dp:`fls_gm74eo754rq9`
   If the :t:`continue expression` appears with a :t:`label indication`, then
   all enclosing :t:`[loop expression]s` upto and including the associated
   :t:`loop expression` are :t:`terminated`.

#. :dp:`fls_gvuesa5ekeif`
   The :t:`evaluation` of the associated :t:`loop expression` is restarted.

.. rubric:: Examples

:dp:`fls_767gv7zhqamh`
The following continue expression terminates and restarts ``game_loop``.

.. code-block:: rust

   'game_loop: loop {
       if is_paused() {
           continue;
       }
       . . .
   }

.. _fls_18swodqqzadj:

Range Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   RangeExpression ::=
       RangeFromExpression
     | RangeFromToExpression
     | RangeFullExpression
     | RangeInclusiveExpression
     | RangeToExpression
     | RangeToInclusiveExpression

   RangeFromExpression ::=
       RangeExpressionLowBound $$..$$

   RangeFromToExpression ::=
       RangeExpressionLowBound $$..$$ RangeExpressionHighBound

   RangeFullExpression ::=
       $$..$$

   RangeInclusiveExpression ::=
       RangeExpressionLowBound $$..=$$ RangeExpressionHighBound

   RangeToExpression ::=
       $$..$$ RangeExpressionHighBound

   RangeToInclusiveExpression ::=
       $$..=$$ RangeExpressionHighBound

   RangeExpressionLowBound ::=
       Operand

   RangeExpressionHighBound ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_bi82rusji8g0`
A :t:`range expression` is an :t:`expression` that constructs a range.

:dp:`fls_msyv4oyk5zp9`
A :t:`range expression low bound` is an :t:`operand` that specifies the start of
a range.

:dp:`fls_f648uuxxh4vk`
A :t:`range expression high bound` is an :t:`operand` that specifies the end of
a range.

:dp:`fls_9pl4629t54yq`
If a :t:`range expression` has two :t:`[operand]s`, then the :t:`[type]s` of the
:t:`[operand]s` shall be :t:`unifiable`.

:dp:`fls_xaumwogwbv3g`
A :t:`range-from expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound`.

:dp:`fls_exa2ufugnpgc`
The :t:`type` of a :t:`range-from expression` is :std:`core::ops::RangeFrom`.

:dp:`fls_jqy0p155btca`
The :t:`value` of a :t:`range-from expression` is ``core::ops::RangeFrom {
start: range_expression_low_bound }``.

:dp:`fls_ppustuqdji7b`
A :t:`range-from-to expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound` and an excluded :t:`range expression
high bound`.

:dp:`fls_ke2fpgodq84u`
The :t:`type` of a :t:`range-from-to expression` is :std:`core::ops::Range`.

:dp:`fls_zb6jk6qykun6`
The :t:`value` of a :t:`range-from-to expression` is ``core::ops::Range { start:
range_expression_low_bound, end: range_expression_high_bound }``.

:dp:`fls_x67xo25n0qlz`
A :t:`range-full expression` is a :t:`range expression` that covers the whole
range of a :t:`type`.

:dp:`fls_m6n0gvg3ct1b`
The :t:`type` of a :t:`range-full expression` is :std:`core::ops::RangeFull`.

:dp:`fls_yvh5cdgzevni`
The :t:`value` of a :t:`range-full expression` is ``core::ops::RangeFull {}``.

:dp:`fls_lh9my7g8oflq`
A :t:`range-inclusive expression` is a :t:`range expression` that specifies an
included :t:`range expression low bound` and an included :t:`range expression
high bound`.

:dp:`fls_livflk52xaj9`
The :t:`type` of a :t:`range-inclusive expression` is
:std:`core::ops::RangeInclusive`.

:dp:`fls_vj213j9bj61y`
The :t:`value` of a :t:`range-inclusive expression` is
``core::ops::RangeInclusive::new(range_expression_low_bound,
range_expression_high_bound)``.

:dp:`fls_5a1uivj19kob`
A :t:`range-to expression` is a :t:`range expression` that specifies an excluded
:t:`range expression high bound`.

:dp:`fls_k611yoc8hk0n`
The :t:`type` of a :t:`range-to expression` is :std:`core::ops::RangeTo`.

:dp:`fls_m0slikrulnvd`
The :t:`value` of a :t:`range-to expression` is ``core::ops::RangeTo { end:
range_expression_high_bound }``.

:dp:`fls_1gc436ee1nzm`
A :t:`range-to-inclusive expression` is a :t:`range expression` that specifies
an included :t:`range expression high bound`.

:dp:`fls_8sfjw83irpre`
The :t:`type` of a :t:`range-to-inclusive expression` is
:std:`core::ops::RangeToInclusive`.

:dp:`fls_5xw4opkbxhsc`
The :t:`value` of a :t:`range-to-inclusive expression` is
``core::ops::RangeToInclusive { end: range_expression_high_bound }``.

.. rubric:: Dynamic Semantics

:dp:`fls_ehseim1p479z`
The :t:`evaluation` of a :t:`range expression` evaluates its :t:`[operand]s` in
left-to-right order.

.. rubric:: Examples

.. code-block:: rust

   1 ..
   42 .. 86
   ..
   dawn ..= dusk
   ..= 5

.. _fls_nlzksiu8y3z9:

If and If Let Expressions
-------------------------

.. _fls_mkut7gut49gi:

If Expressions
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IfExpression ::=
       $$if$$ SubjectExpression BlockExpression ElseExpression?

   ElseExpression ::=
       $$else$$ (BlockExpression | IfExpression | IfLetExpression)

.. rubric:: Legality Rules

:dp:`fls_2i4fbxbbvpf1`
An :t:`if expression` is an :t:`expression` that evaluates either a :t:`block
expression` or an :t:`else expression` depending on the :t:`value` of its
:t:`subject expression`.

:dp:`fls_5azwlk7hav1k`
An :t:`else expression` is an :t:`expression` that represents either a :t:`block
expression`, an :t:`if expression`, or an :t:`if let expression`.

:dp:`fls_r7gzxo16esri`
The :t:`type` of the :t:`subject expression` of an :t:`if expression` shall be
:t:`type` :c:`bool`.

:dp:`fls_iv9t4nfs4f6w`
The :t:`type` of an :t:`if expression` is the :t:`type` of its :t:`block
expression`.

:dp:`fls_i9sxf2q5jjqt`
The :t:`value` of an :t:`if expression` is the :t:`value` of its :t:`block
expression`.

:dp:`fls_1e8qer6bh2f3`
The :t:`type` of an :t:`else expression` is the :t:`type` of its :t:`block
expression`, :t:`if expression`, or :t:`if let expression`.

:dp:`fls_p5pjxk5xfcbx`
The :t:`value` of an :t:`else expression` is the :t:`value` of its :t:`block
expression`, :t:`if expression`, or :t:`if let expression`.

:dp:`fls_mpq7gicosgkt`
The :t:`type` of an :t:`if expression` and the :t:`type` of an :t:`else
expression` shall be :t:`unifiable`.

.. rubric:: Dynamic Semantics

:dp:`fls_yhlyzef9h97q`
The :t:`evaluation` of an :t:`if expression` proceeds as follows:

#. :dp:`fls_w7lq4dkoyuf7`
   The :t:`subject expression` is evaluated.

#. :dp:`fls_5udx9zyeg5ga`
   If the :t:`subject expression` evaluated to ``true``, then the :t:`block
   expression` is evaluated.

#. :dp:`fls_67l4j48n6p7o`
   If the :t:`subject expression` evaluated to ``false`` and the :t:`if
   expression` has an :t:`else expression`, then the :t:`else expression`
   is evaluated.

:dp:`fls_e8gd5lzcaifw`
The :t:`evaluation` of an :t:`else expression` evaluates its :t:`block
expression`, :t:`if expression`, or :t:`if let expression`.

.. rubric:: Examples

.. code-block:: rust

   if age <= 14 {
       println!("child");
   } else if age <= 24 {
       println!("youth");
   } else if age <=64 {
       println!("adult");
   } else {
       println!("senior");
   }

.. _fls_p0t1ch115tra:

If Let Expressions
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IfLetExpression ::=
       $$if$$ $$let$$ Pattern $$=$$ SubjectLetExpression BlockExpression
         ElseExpression?

.. rubric:: Legality Rules

:dp:`fls_dsrjup2umr9`
An :t:`if let expression` is an :t:`expression` that evaluates either a
:t:`block expression` or an :t:`else expression` depending on whether its
:t:`pattern` can be matched against its :t:`subject let expression`.

:dp:`fls_4vyrufo4qdeg`
The :t:`type` of an :t:`if let expression` is the :t:`type` of its :t:`block
expression`.

:dp:`fls_qfnwwvzxsl3`
The :t:`value` of an :t:`if let expression` is the :t:`value` of its :t:`block
expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_ijo73wtz1sy`
The :t:`evaluation` of an :t:`if let expression` of the form

.. code-block:: rust

   if let pattern = subject_let_expression {
       /* body */
   }

:dp:`fls_qeho5iqiy59`
is equivalent to the :t:`evaluation` of the following :t:`match expression`:

.. code-block:: rust

   match subject_let_expression {
       pattern => { /* body */ },
       _ => ()
   }

:dp:`fls_nhngr8y850dt`
The :t:`evaluation` of an :t:`if let expression` of the form

.. code-block:: rust

   if let pattern = subject_let_expression {
       /* body */
   } else {
       /* else */
   }

:dp:`fls_8fg2ufaxjkv5`
is equivalent to the :t:`evaluation` of the following :t:`match expression`:

.. code-block:: rust

   match subject_let_expression {
       pattern => { /* body */ },
       _ => { /* else */ }
   }

.. rubric:: Examples

.. code-block:: rust

   let dish = ("Ham", "Eggs");

   if let ("Ham", side) = dish {
       println!("Ham is served with {}", side);
   }

.. _fls_e5td0fa92fay:

Match Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   MatchExpression ::=
       $$match$$ SubjectExpression $${$$
         InnerAttributeOrDoc*
         MatchArmList?
       $$}$$

   MatchArmList ::=
       IntermediateMatchArm* FinalMatchArm

   IntermediateMatchArm ::=
       MatchArmMatcher $$=>$$
         $$($$ ExpressionWithBlock $$,$$? | ExpressionWithoutBlock $$,$$ $$)$$

   FinalMatchArm ::=
       MatchArmMatcher $$=>$$ Operand $$,$$?

   MatchArmMatcher ::=
       OuterAttributeOrDoc* Pattern MatchArmGuard?

   MatchArmGuard ::=
       $$if$$ Operand

.. rubric:: Legality Rules

:dp:`fls_ei4pbeksd1v8`
A :t:`match expression` is an :t:`expression` that tries to match one of its
multiple :t:`[pattern]s` against its :t:`subject expression` and if it succeeds,
evaluates an :t:`operand`.

:dp:`fls_l45i24ikfavm`
A :t:`match arm` is a :t:`construct` that consists of a :t:`match arm matcher`
and a :t:`match arm body`.

:dp:`fls_d9gerg12hm2d`
An :t:`intermediate match arm` is any :t:`non-[final match arm]` of a :t:`match
expression`.

:dp:`fls_oj8dg28xw5yp`
A :t:`final match arm` is the last :t:`match arm` of a :t:`match expression`.

:dp:`fls_lrdrtedyz28i`
A :t:`match arm matcher` is a :t:`construct` that consists of a :t:`pattern` and
a :t:`match arm guard`.

:dp:`fls_8wjdichfxp0y`
A :t:`match arm body` is the :t:`operand` of a :t:`match arm`.

:dp:`fls_hs1rr54hu18w`
A :t:`match arm guard` is a :t:`construct` that provides additional filtering to
a :t:`match arm matcher`.

:dp:`fls_s4483f30nwf`
A :t:`match expression` is a :t:`place expression` when its :t:`subject
expression` is a :t:`place expression`. When a :t:`match expression` is a
:t:`place expression`, the :t:`value` produced by evaluating its :t:`subject
expression` is passed :t:`by value`.

:dp:`fls_9t5pmb9wzmpy`
A :t:`match expression` is a :t:`value expression` when its :t:`subject
expression` is a :t:`value expression`. When the :t:`match expression` is a
:t:`value expression`, the :t:`value` produced by evaluating its :t:`subject
expression` is captured in a :t:`temporary`.

:dp:`fls_knv1affr2o8t`
The :t:`type` of the :t:`subject expression` and the :t:`[type]s` of all
:t:`[pattern]s` of all :t:`[match arm matcher]s` shall be :t:`unifiable`.

:dp:`fls_bzhz5wjd90ii`
The :t:`type` of the :t:`operand` of a :t:`match arm guard` shall be :t:`type`
:c:`bool`.

:dp:`fls_17ag0wzdbxv6`
The :t:`[type]s` of all :t:`[match arm body]ies` shall be :t:`unifiable`.

:dp:`fls_5w964phrru82`
The :t:`type` of a :t:`match expression` is the :t:`unified type` of the
:t:`[type]s` of the :t:`[operand]s` of all :t:`[match arm]s`.

:dp:`fls_g6xyz0beps3o`
A :t:`match arm` is selected when its :t:`pattern` matches the :t:`subject
expression` and its :t:`match arm guard` (if any) evaluates to ``true``.

:dp:`fls_8dba4o5qg8js`
:t:`Match arm` selection happens in declarative order.

:dp:`fls_e02um1gb89d0`
The :t:`[pattern]s` of all :t:`[match arm]s` taken together shall exhaustively
match the :t:`[subject expression]'s` :t:`type`.

:dp:`fls_4sh2yrslszvb`
The :t:`value` of a :t:`match expression` is the :t:`value` of the :t:`operand`
of the selected :t:`match arm`.

.. rubric:: Dynamic Semantics

:dp:`fls_g551l8r8yh6d`
The :t:`evaluation` of a :t:`match expression` proceeds as follows:

#. :dp:`fls_y44jzkbv74bv`
   The :t:`subject expression` is evaluated.

#. :dp:`fls_jwxykea99psw`
   Each :t:`match arm` is evaluated in declarative order as follows:

   #. :dp:`fls_pgulnjeoxwtj`
      The :t:`match arm matcher` of the :t:`match arm` is evaluated.

   #. :dp:`fls_2dg7wl68z7ar`
      If the :t:`match arm matcher` succeeds, then

      #. :dp:`fls_yv11febo0kyb`
         The :t:`operand` of the :t:`match arm` is evaluated.

      #. :dp:`fls_mvi9z1x836qu`
         Control stops the :t:`evaluation` of the :t:`match expression`.

   #. :dp:`fls_81nnizrxgrsm`
      Otherwise control proceeds with the :t:`evaluation` of the next :t:`match
      arm`.

:dp:`fls_4dv7x9nh2h4e`
The :t:`evaluation` of a :t:`match arm matcher` proceeds as follows:

#. :dp:`fls_k7kliy101m0f`
   The :t:`pattern` of the :t:`match arm matcher` is evaluated.

#. :dp:`fls_k68zkb6jv0vz`
   If the :t:`pattern` succeeds, then

   #. :dp:`fls_gbb6wbmher5z`
      If the :t:`match arm matcher` has a :t:`match arm guard`, then

      #. :dp:`fls_jl4av757yx8j`
         The :t:`match arm guard` is evaluated.

      #. :dp:`fls_wkh5wztauwhu`
         If the :t:`match arm guard` evaluates to ``true``, then the
         :t:`match arm matcher` succeeds.

   #. :dp:`fls_f5f0x8jstp1g`
      Otherwise the :t:`match arm matcher` fails.

#. :dp:`fls_yk8l9zjh7i0d`
   Otherwise the :t:`match arm matcher` fails.

:dp:`fls_sbtx1l6n2tp2`
The :t:`evaluation` of a :t:`match arm guard` evaluates its :t:`operand`. A
:t:`match arm guard` evaluates to ``true`` when its :t:`operand` evaluates to
``true``, otherwise it evaluates to ``false``.

.. rubric:: Examples

.. code-block:: rust

   fn quantify(number_of_things: i32) {
       match number_of_things {
           0 | 1 => println!("not many"),
           2 ..= 9 => println!("a few"),
           _ if number_of_things < 0 => println!("you owe me"),
           _ => println!("lots")
       }
   }

.. _fls_8l74abhlxzdl:

Return Expressions
------------------

.. rubric:: Syntax

.. syntax::

   ReturnExpression ::=
       $$return$$ Expression?

.. rubric:: Legality Rules

:dp:`fls_u7jk4j8gkho`
A :t:`return expression` is an :t:`expression` that optionally yields a
:t:`value` and causes control flow to return to the end of the enclosing
:t:`control flow boundary`.

:dp:`fls_5v3j5ghhw8j8`
A :t:`return expression` shall appear within a :t:`control flow boundary`.

:dp:`fls_m4e00bju2dy4`
The :t:`type` of a :t:`return expression` is determined as follows:

* :dp:`fls_xpp027s2m7ue`
  If the :t:`return expression` has an :t:`operand`, then the :t:`type` is the
  :t:`type` of the :t:`operand`.

* :dp:`fls_cqduumpsjfut`
  If the :t:`return expression` does not have an :t:`operand`, then the
  :t:`type` is the :t:`never type`.

:dp:`fls_r610t5vsi7bx`
The :t:`value` of a :t:`return expression` is determined as follows:

* :dp:`fls_njndlx2rps2k`
  If the :t:`return expression` has an :t:`operand`, then the :t:`value` is the
  :t:`value` of the :t:`operand`.

* :dp:`fls_tjksia7prao1`
  If the :t:`return expression` does not have an :t:`operand`, then the
  :t:`value` is the :t:`unit value`.

.. rubric:: Dynamic Semantics

:dp:`fls_bqmwlona6l5w`
The :t:`evaluation` of a :t:`return expression` proceeds as follows:

#. :dp:`fls_d9avvfi548t7`
   If the :t:`return expression` has an :t:`operand`, then

   #. :dp:`fls_o3fc1z2mn8zc`
      The :t:`operand` is evaluated.

   #. :dp:`fls_bbf54ukld7j9`
      The :t:`value` of the :t:`operand` is passed :t:`by move` into the
      designated output location of the enclosing :t:`control flow boundary`.

#. :dp:`fls_99ea30a5mulj`
   Control destroys the current activation frame.

#. :dp:`fls_ubwj8uj6sbgt`
   Control is transferred to the caller frame.

.. rubric:: Examples

.. code-block:: rust

   fn max(left: i32, right: i32) -> i32 {
       if left > right {
           return left;
       }
       return right;
   }

.. _fls_hyrbmfmf6r8g:

Await Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   AwaitExpression ::=
       FutureOperand $$.$$ $$await$$

   FutureOperand ::=
       Operand

.. rubric:: Legality Rules

:dp:`fls_sjz5s71hwm7l`
An :t:`await expression` is an :t:`expression` that polls a :t:`future`,
suspending the :t:`execution` of the :t:`future` until the :t:`future` is ready.

:dp:`fls_vhchgab59jvd`
A :t:`future operand` is an :t:`operand` whose :t:`future` is being awaited by
an :t:`await expression`.

:dp:`fls_k9pncajmhgk1`
An :t:`await expression` shall appear within an :t:`async control flow boundary`.

:dp:`fls_xtG3Lw2XgGS2`
An :dt:`async control flow boundary` is a :t:`control flow boundary` with
asynchronous control flow.

:dp:`fls_Rr2Wl6mp4oXS`
The following constructs are :t:`[async control flow boundary]ies`:

* :dp:`fls_xilkqy5piyh0`
  An :t:`async block expression`.

* :dp:`fls_cr61i8so7cty`
  The :t:`function body`` of an :t:`async function`.

:dp:`fls_9uw5pd7kbrx3`
The :t:`type` of a :t:`future operand` shall implement the
:std:`core::future::Future` :t:`trait`.

:dp:`fls_c6mxfzef2qop`
The :t:`type` of an :t:`await expression` is ``<_ as
core::future::Future>::Output``.

:dp:`fls_l396mo6k9ox7`
The :t:`value` of an :t:`await expression` is the :t:`value` held by
:std:`core::task::Poll::Ready`.

.. rubric:: Dynamic Semantics

:dp:`fls_1ppywe40s62c`
The :t:`evaluation` of an :t:`await expression` proceeds as follows:

#. :dp:`fls_eymcs2rgv3qw`
   The :t:`future operand` is evaluated to a :t:`temporary`.

#. :dp:`fls_qshnnpirkasz`
   The :t:`temporary` is pinned using :std:`core::pin::Pin::new_unchecked`.

#. :dp:`fls_umevprl99y6c`
   The pinned :t:`temporary` is polled using :std:`core::future::Future::poll`,
   passing in the :std:`core::task::Context` of the current task.

#. :dp:`fls_k76d8ady623m`
   If :std:`core::future::Future::poll` returns
   :std:`core::task::Poll::Pending`, then the current :t:`future` yields.

#. :dp:`fls_frwtufwe8dya`
   If :std:`core::future::Future::poll` returns :std:`core::task::Poll::Ready`,
   then

   #. :dp:`fls_u72ylhlmqge3`
      The :t:`value` held within is unwrapped.

   #. :dp:`fls_tn3vwidct3ks`
      Control stops the evaluation of the :t:`await expression`.

.. rubric:: Examples

.. code-block:: rust

   let future = async { expensive_function() };
   future.await;

.. _fls_kw25194gpael:

Expression Precedence
---------------------

.. rubric:: Legality Rules

:dp:`fls_cwt7afsbgs7w`
Certain :t:`[expression]s` are subject to :t:`precedence` and
:t:`associativity`.

:dp:`fls_ya23jjg5wjl`
:t:`Precedence` is the order by which :t:`[expression]s` are evaluated in the
presence of other :t:`[expression]s`.

:dp:`fls_bezkcuwp5qol`
:t:`Associativity` is the order by which :t:`[operand]s` are evaluated within a
single :t:`expression`.

:dp:`fls_48br7odx6nke`
The :t:`precedence` and :t:`associativity` of qualifying :t:`[expression]s` are
as follows:

.. list-table::

   * - :dp:`fls_mk2yk99p6nvp`
     - **Expression**
     - **Precedence**
     - **Associativity**
   * - :dp:`fls_jtdnf0vmn6xt`
     - :t:`Array expression`

       :t:`Block expression`

       :t:`Continue expression`

       :t:`If expression`

       :t:`If let expression`

       :t:`Literal expression`

       :t:`Loop expression`

       :t:`Match expression`

       :t:`Parenthesized expression`

       :t:`Path expression`

       :t:`Struct expression`

       :t:`Tuple expression`

       :t:`Underscore expression`
     - highest
     - none
   * - :dp:`fls_qurz25skmryg`
     - :t:`Method call expression`
     -
     - none
   * - :dp:`fls_ywqn5nixelkz`
     - :t:`Await expression`

       :t:`Field access expression`
     -
     - left-to-right
   * - :dp:`fls_k3ohh8k888c`
     - :t:`Call expression`

       :t:`Index expression`
     -
     - none
   * - :dp:`fls_41n6z85h1z47`
     - :t:`Error propagation expression`
     -
     - none
   * - :dp:`fls_f39rzauxrlcl`
     - :t:`Borrow expression`

       :t:`Dereference expression`

       :t:`Negation expression`
     -
     - none
   * - :dp:`fls_djphr5mk0t6f`
     - :t:`Type cast expression`
     -
     - left-to-right
   * - :dp:`fls_sif2aqky96j6`
     - :t:`Division expression`

       :t:`Multiplication expression`

       :t:`Remainder expression`
     -
     - left-to-right
   * - :dp:`fls_d7x817v8xzea`
     - :t:`Addition expression`

       :t:`Subtraction expression`
     -
     - left-to-right
   * - :dp:`fls_1f5ibdkz3l51`
     - :t:`Shift left expression`

       :t:`Shift right expression`
     -
     - left-to-right
   * - :dp:`fls_t1zqnab8a752`
     - :t:`Bit and expression`
     -
     - left-to-right
   * - :dp:`fls_f6in3h5cj8i6`
     - :t:`Bit xor expression`
     -
     - left-to-right
   * - :dp:`fls_hxa1avitfvrq`
     - :t:`Bit or expression`
     -
     - left-to-right
   * - :dp:`fls_sy2xzzq06i0x`
     - :t:`Comparison expression`
     -
     - requires parentheses
   * - :dp:`fls_hish3qfmawd`
     - :t:`Lazy and expression`
     -
     - left-to-right
   * - :dp:`fls_ruy7e6yccsqk`
     - :t:`Lazy or expression`
     -
     - left-to-right
   * - :dp:`fls_9qcm0dx9rolw`
     - :t:`Range expression`
     -
     - requires parentheses
   * - :dp:`fls_1l3rgtm6o54s`
     - :t:`Assignment expression`

       :t:`Compound assignment expression`
     -
     - right-to-left
   * - :dp:`fls_hr4kokysrjop`
     - :t:`Break expression`

       :t:`Closure expression`

       :t:`Return expression`
     - lowest
     - none

.. _fls_jmjn8jkbzujm:

Capturing
---------

.. rubric:: Legality Rules

:dp:`fls_iamnzlm430ef`
A :t:`capturing expression` is either an :t:`async block expression` or a
:t:`closure expression`.

:dp:`fls_eca6tl7j0afx`
A :t:`capture target` is either a :t:`variable` or a :t:`field` of a
:t:`variable`.

:dp:`fls_e70ywb8191h`
The :t:`capturing environment` of a :t:`capturing expression` consists of all
:t:`[capture target]s` that are defined outside the :t:`capturing expression`.

:dp:`fls_1y2ttb466m9c`
:t:`Capturing` is the process of saving the :t:`[capture target]s` of a
:t:`[capturing expression]'s` :t:`capturing environment`.

:dp:`fls_ip81lt2mm940`
A :t:`capture target` requires :t:`capturing` when it is part of the
:t:`[capturing expression]'s` :t:`capturing environment` and it is used by
the :t:`capturing expression`. Such a :t:`capture target` is said to be
:dt:`captured`.

:dp:`fls_y9n1i4hbq8sf`
:t:`Capture mode` is the mechanism by which a :t:`capture target` is captured.

:dp:`fls_t695ps4lfh6z`
The :t:`capture mode` is determined based on the use of the :t:`capture target`
within the :t:`capturing expression`, as follows:

#. :dp:`fls_6j8cr7d5zs1l`
   If the :t:`capturing expression` is subject to :t:`keyword` ``move``, then

   #. :dp:`fls_dd8sc7y2vi3u`
      If the :t:`type` of the :t:`capture target` is a :t:`by copy type`, then
      the :t:`capture mode` is :t:`by copy`.

   #. :dp:`fls_sq1wam8j1d0a`
      Otherwise the :t:`capture mode` is :t:`by move`.

#. :dp:`fls_l8e98pyhm08g`
   Otherwise the :t:`capture mode` is determined based on the following
   precedence:

   #. :dp:`fls_33hfay24hx8u`
      :t:`By immutable reference` mode.

   #. :dp:`fls_wmxsd0i2yemf`
      :t:`By unique immutable reference` mode, if the :t:`capture target` is a
      :t:`mutable reference` that is being modified.

   #. :dp:`fls_lu779ufqhggl`
      :t:`By mutable reference` mode.

   #. :dp:`fls_uqy5w9uc8gla`
      If the :t:`type` of the :t:`capture target` is a :t:`by copy type`, then
      the :t:`capture mode` is :t:`by copy`, otherwise it is :t:`by move`.

:dp:`fls_wvob7114tfat`
A tool selects the first :t:`capture mode` that is compatible with the use of
the :t:`capture target`.

