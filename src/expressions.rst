.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

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
         | ArrayIndexExpression
         | AwaitExpression
         | BreakExpression
         | CallExpression
         | ClosureExpression
         | ContinueExpression
         | FieldAccessExpression
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


:def_p:`fls_pwut2jbmk66k`
A :def_syntax:`SubjectExpression` is any expression in category
:syntax:`Expression`, except :syntax:`IndexedTupleStructExpression` and
:syntax:`RecordStructExpression`.

:def_p:`fls_361q9ljc6ybz`
A :def_syntax:`SubjectLetExpression` is any expression in category
:syntax:`SubjectExpression`, except :syntax:`LazyBooleanExpression`.

.. rubric:: Legality Rules

:def_p:`fls_h5o6tgul4yor`
An :term:`expression` is a :term:`construct` that produces a :term:`value`, and
may have side effects at run-time.

:def_p:`fls_xmklb3070sp`
An :term:`expression-with-block` is an :term:`expression` whose structure
involves a :term:`block expression`.

:def_p:`fls_p15oeage4j0e`
An :term:`expression-without-block` is an :term:`expression` whose structure
does not involve a :term:`block expression`.

:def_p:`fls_gwgttltgjma4`
An :term:`operand` is an :term:`expression` nested within an :term:`expression`.

:def_p:`fls_1r29rtnjlkql`
A :term:`left operand` is an :term:`operand` that appears on the left-hand side
of a :term:`binary operator`.

:def_p:`fls_qxdpyf4u3hbz`
A :term:`right operand` is an :term:`operand` that appears on the right-hand
side of a :term:`binary operator`.

:def_p:`fls_2j132xueobfv`
A :term:`subject expression` is an :term:`expression` that controls :term:`for
loop`\ s, :term:`if expression`\ s, and :term:`match expression`\ s.

:def_p:`fls_a243nclqqjlu`
A :term:`subject let expression` is an :term:`expression` that controls
:term:`if let expression`\ s and :term:`while let loop`\ s.

.. rubric:: Dynamic Semantics

:term:`Evaluation` is the process by which an :term:`expression` achieves its
runtime effects.

Expression Classification
-------------------------

Assignee Expressions
~~~~~~~~~~~~~~~~~~~~

An :term:`assignee expression` is an :term:`expression` that appears as
the :term:`left operand` of an :term:`assignment expression`. The following
:term:`expression`\ s are :term:`assignee expression`\ s:

* :term:`Place expression`\ s,

* :term:`Underscore expression`\ s,

* :term:`Array expression`\ s of :term:`assignee expression`\ s,

* :term:`Tuple expression`\ s of :term:`assignee expression`\ s,

* :term:`Struct expression`\ s of :term:`assignee expression`\ s,

* :term:`Tuple struct expression`\ s of :term:`assignee expression`\ s,

* :term:`Unit struct expression`\ s.

:term:`Parenthesized expression`\ s are allowed to appear anywhere in
:term:`assignee expression`\ s.

Constant Expressions
~~~~~~~~~~~~~~~~~~~~

:def_p:`fls_1ji7368ieg0b`
A :term:`constant expression` is an :term:`expression` that can be evaluated
statically. The following :term:`construct`\ s are :term:`constant expression`\
s as long as their :term:`operand`\ s are also :term:`constant expression`\ s
and do not involve :term:`type`\ s that require :term:`destruction`:

* :def_p:`fls_y6ore0iwx7e0`
  :term:`Arithmetic expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_xguga84v3j8u`
  :term:`Array expression`\ s,

* :def_p:`fls_rpapnm3afan8`
  :term:`Array index expression`\ s,

* :def_p:`fls_idxf02p7jogu`
  :term:`Assignment expression`\ s,

* :def_p:`fls_6z45ss502alt`
  :term:`Bit expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_wqs0792nud4e`
  :term:`Block expression`\ s,

* :def_p:`fls_8nyu6phm1nji`
  :term:`Closure expression`\ s that do not :term:`capture`,

* :def_p:`fls_8wux08bmpse`
  :term:`Comparison expression`\ s of scalar types,

* :def_p:`fls_ppmnogx8mxk3`
  :term:`Compound assignment expression`\ s,

* :def_p:`fls_6fq6bvxxvhsr`
  :term:`Constant parameter`\ s,

* :def_p:`fls_to4e7imq2c0w`
  :term:`Dereference expression`\ s when the :term:`operand` is not of a
  :term:`raw pointer type`,

* :def_p:`fls_krtbrpwf3mh0`
  :term:`Expression statement`\ s,

* :def_p:`fls_3etom5uu8y4u`
  :term:`Field access expression`\ s,

* :def_p:`fls_qls0wj8bmupz`
  :term:`If expression`\ s,

* :def_p:`fls_b5fraqx07wuo`
  :term:`If let expression`\ s,

* :def_p:`fls_fc62yaqyjpl2`
  :term:`Infinite loop expression`\ s,

* :def_p:`fls_kwg8a351vc7`
  :term:`Lazy boolean expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_7mjv1xd45qr4`
  :term:`Let statement`\ s,

* :def_p:`fls_g7hoyfqy9mu1`
  :term:`Literal expression`\ s,

* :def_p:`fls_br4g7qwfczig`
  :term:`Match expression`\ s,

* :def_p:`fls_y1ezabo61nyk`
  :term:`Negation expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_6tb74n6lu0wf`
  :term:`Parenthesized expression`\ s,

* :def_p:`fls_axwrv7b3zt55`
  :term:`Path expression`\ s that resolve to :term:`constant`\ s,
  :term:`function`\ s, and :term:`static`\ s,

* :def_p:`fls_3bucpdj828bq`
  :term:`Range expression`\ s,

* :def_p:`fls_hkbwa8xx2fwx`
  :term:`Shared borrow`\ s that do not involve :term:`type`\ s with
  :term:`interior mutability`,

* :def_p:`fls_fobs8ebt7dhc`
  :term:`Struct expression`\ s,

* :def_p:`fls_dyo3o1h3keqr`
  :term:`Tuple expression`\ s,

* :def_p:`fls_e0a1e8ddph7`
  :term:`Type cast expression`\ s that are not :term:`pointer-to-address cast`\
  s,  :term:`function-pointer-to-address cast`\ s, and :term:`unsized cast`\ s
  that involve a :term:`trait object type`,

* :def_p:`fls_pbpzkfo1fgtz`
  :term:`While let loop expression`\ s,

* :def_p:`fls_qvofy4wkql0s`
  :term:`While loop expression`\ s.

:def_p:`fls_kjhma680hz3g`
A :term:`constant context` is a :term:`construct` that requires a
:term:`constant expression`. The following :term:`construct`\ s are
:term:`constant context`\ s:

* :def_p:`fls_ljc6jq5ksbcs`
  The :term:`constant initializer` of a :term:`constant`,

* :def_p:`fls_icra98id84mk`
  The :term:`constant parameter` of a :term:`generic`,

* :def_p:`fls_66m2hwkju0vv`
  The :term:`discriminant initializer` of a :term:`discriminant`,

* :def_p:`fls_fsn32kmwg65u`
  The :term:`size operand` of an :term:`array repetition constructor`,

* :def_p:`fls_j6kffhbxdm7o`
  The :term:`size operand` of an :term:`array type`,

* :def_p:`fls_ib8p7dfwddx2`
  The :term:`static initializer` of a :term:`static`.

Place Expressions
~~~~~~~~~~~~~~~~~

:def_p:`fls_qbrcg3cl9td`
A :term:`place expression` is an :term:`expression` that represents a memory
location. The following :term:`expression`\ s are :term:`place expression`\ s:

* :def_p:`fls_lj7x5dgbmg9i`
  :term:`Array index expression`\ s,

* :def_p:`fls_jpmhibm4omm7`
  :term:`Dereference expression`\ s,

* :def_p:`fls_none1dykbn8c`
  :term:`Field access expression`\ s,

* :def_p:`fls_anzidgx02lly`
  :term:`Parenthesized expression`\ s where the :term:`operand` is a
  :term:`place expression`,

* :def_p:`fls_ya05djl1d154`
  :term:`Path expression`\ s that resolve to a :term:`binding` or a
  :term:`static`.

:def_p:`fls_4vxi1ji93dxb`
A :term:`place expression context` is a :term:`construct` that requires a
:term:`place expression`. The following :term:`construct`\ s are :term:`place
expression context`\ s:

* :def_p:`fls_qytgkbhqr5ln`
  The :term:`indexed array operand` of an :term:`array index expression`,

* :def_p:`fls_5gy92rsi2mqm`
  The :term:`assignee operand` of an :term:`assignment expression` or a
  :term:`compound assignment expression`,

* :def_p:`fls_u80htrnr2ebz`
  The :term:`operand` of a :term:`borrow expression`,

* :def_p:`fls_o0feajus3jtu`
  The :term:`operand` of a :term:`dereference expression`,

* :def_p:`fls_ffjx1d5dseo4`
  The :term:`container operand` of :term:`field access expression`,

* :def_p:`fls_9r7dopqf1nzl`
  The :term:`subject let expression` of an :term:`if let expression` or a
  :term:`while let loop expression`,

* :def_p:`fls_ka5b87tkf8t6`
  The initialization :term:`expression` of a :term:`let statement`,

* :def_p:`fls_brwv1zwu37e8`
  The :term:`subject expression` of :term:`a match expression,`

* :def_p:`fls_qewvbxvk81d`
  The :term:`base initializer` of a :term:`struct expression`,

* :def_p:`fls_qaqwmxa3bxw1`
  The :term:`operand` of an :term:`implicit borrow`.

:def_p:`fls_konzgoybhfqm`
A :term:`place expression` can be moved out of when it denotes

* :def_p:`fls_vk1xhvdaakh0`
  A :term:`binding` which is not currently :term:`borrowed`, or

* :def_p:`fls_4bnbv7mqod57`
  A :term:`field` of a :term:`place expression` that can be moved out of and
  does not implement the :codeterm:`core::ops::Drop` :term:`trait`, or

* :def_p:`fls_3xk3p1unbjy5`
  A :term:`temporary` created for a :term:`value expression`.

:def_p:`fls_wuqjaigxdq3r`
After a :term:`place expression` is moved out, the memory location it
represented is deinitialized and shall not be read from until reinitialized.

:def_p:`fls_ku38h562vfyl`
A :term:`mutable place expression` is a :term:`place expression` whose memory
location can be modified. The following :term:`place expression`\ s are
:term:`mutable place expression`\ s:

* :def_p:`fls_bt50fltfqcvn`
  An :term:`array index expression` whose :term:`type` implements the
  :codeterm:`core::ops::IndexMut` :term:`trait`,

* :def_p:`fls_6b4rwkrc1ap6`
  A :term:`dereference expression` whose :term:`type` is ``*mut T``,

* :def_p:`fls_s4bhrpykzmm7`
  A :term:`dereference expression` of a :term:`field` or :term:`binding` whose
  :term:`type` is ``&mut T``,

* :def_p:`fls_1tq2o2huda9l`
  A :term:`dereference expression` whose :term:`type` implements the
  :codeterm:`core::ops::DerefMut` :term:`trait`,

* :def_p:`fls_xm0gm2q27x2e`
  A :term:`field access expression` where the :term:`type` of the
  :term:`container operand` is :term:`mutable`,

* :def_p:`fls_m0gbw9myylv2`
  A :term:`path expression` that resolves to a :term:`mutable binding` that is
  not currently borrowed,

* :def_p:`fls_ilaqmj3hc5uv`
  A :term:`path expression` that resolves to a :term:`mutable static`,

* :def_p:`fls_dcm3yr3y9y0a`
  A :term:`temporary` created for a :term:`value expression`.

.. rubric:: Dynamic Semantics

The :term:`evaluation` of a :term:`place expression` in the context of a
:term:`value expression` or the :term:`evaluation` of a :term:`place expression`
that is bound *by value* in a :term:`pattern` proceeds as follows:

#. The :term:`place expression` denotes the :term:`value` held in that memory
   location.

#. If the :term:`type` of the held :term:`value` implements the
   :codeterm:`core::marker::Copy` :term:`trait`, then the held :term:`value`
   is copied.

#. If the :term:`type` of the held :term:`value` implements the
   :codeterm:`core::marker::Sized` :term:`trait`, then the held :term:`value`
   is moved.

#. Otherwise the :term:`evaluation` results in a static error.

Value Expressions
~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_7q4hrt6yfr9b`
A :term:`value expression` is an :term:`expression` that represents a
:term:`value`.

Literal Expressions
-------------------

.. rubric:: Syntax

.. syntax::

   LiteralExpression ::=
       Literal


.. rubric:: Legality Rules

:def_p:`fls_rbwwczom3agt`
A :term:`literal expression` is an :term:`expression` that denotes a
:term:`literal`.

:def_p:`fls_utbjihhwgxr1`
A :term:`literal expression` is a :term:`value expression`.

:def_p:`fls_w30su9x4q13r`
The :term:`type` of a :term:`literal expression` is the :term:`type` of the
corresponding :term:`literal`.

:def_p:`fls_wdpbg5xzgmwu`
The :term:`value` of a :term:`literal expression` is the :term:`value` of the
corresponding :term:`literal`.

.. rubric:: Dynamic Semantics

:def_p:`fls_g061yzws1m45`
The :term:`evaluation` of a :term:`literal expression` has no effect.

.. rubric:: Examples

.. code-block:: text

   5
   'a'
   "hello"

Path Expressions
----------------

.. rubric:: Syntax

.. syntax::

   PathExpression ::=
       PathInExpression
     | QualifiedPathInExpression

.. rubric:: Legality Rules

:def_p:`fls_gvanx4874ycy`
A :term:`path expression` is an :term:`expression` that denotes a :term:`path`.

:def_p:`fls_t8bdzvtnv249`
A :term:`path expression` that resolves to a :term:`binding` or a :term:`static`
is a :term:`place expression`, otherwise it is a :term:`value expression`.

:def_p:`fls_gz67ju6l7uhn`
A :term:`path expression` that resolves to a :term:`mutable static` shall
require :term:`unsafe context`.

:def_p:`fls_cjywisyiyti6`
The :term:`type` of a :term:`path expression` is the :term:`type` of the
:term:`entity` that it resolved to.

:def_p:`fls_5ifai8nkp5ek`
The :term:`value` of a :term:`path expression` is the :term:`entity` that it
resolved to.

.. rubric:: Dynamic Semantics

:def_p:`fls_ed9w4jwx059`
The :term:`evaluation` of a :term:`path expression` has no effect.

.. rubric:: Examples

.. code-block:: text

   globals::STATIC_VARIABLE
   Vec::<i32>::push

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

:def_p:`fls_nf65p0l0v0gr`
A :term:`block expression` is an :term:`expression` that sequences
:term:`expression`\ s and :term:`statement`\ s.

:def_p:`fls_tn3hj7k2lliu`
A :term:`tail expression` is the last :term:`expression` within a :term:`block
expression`.

:def_p:`fls_wiv8wpw3i79z`
A :term:`block expression` is a :term:`value expression`.

:def_p:`fls_u4gj2lnkq9ub`
The :term:`type` of a :term:`block expression` is determined as follows:

* :def_p:`fls_ob76y2ymdd27`
  If the :term:`block expression` has an :term:`expression`, then the
  :term:`type` is the :term:`type` of the :term:`expression`.

* :def_p:`fls_u0avbm147nyh`
  If the :term:`block expression` does not have an :term:`expression`, then the
  :term:`type` is the :term:`unit type`.

:def_p:`fls_1hzup0sf8l7l`
The :term:`value` of a :term:`block expression` is determined as follows:

* :def_p:`fls_9nmssjseq3jt`
  If the :term:`block expression` has an :term:`expression`, then the
  :term:`value` is the :term:`value` of the :term:`expression`.

* :def_p:`fls_a3ulnvyc1ut`
  If the :term:`block expression` does not have an :term:`expression`, then the
  :term:`value` of the :term:`block expression` is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_elcl73psruxw`
The :term:`evaluation` of a :term:`block expression` proceeds as follows:

#. :def_p:`fls_13b5n127rj92`
   Each :term:`statement` is executed in declarative order.

#. :def_p:`fls_nzdpw59plr2g`
   The :term:`expression` is evaluated.

.. rubric:: Examples

.. code-block:: text

   {
       fn_call();
       42
   }

async Blocks
~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   AsyncBlockExpression ::=
       $$async$$ $$move$$? BlockExpression

.. rubric:: Legality Rules

:def_p:`fls_hhidi5ukxo`
An :term:`async block expression` is a :term:`block expression` that **???**.

:def_p:`fls_eam1cqbjlfs0`
An :term:`async block expression` is a :term:`value expression`.

:def_p:`fls_tzclkasinpoq`
An :term:`async block expression` is subject to :term:`capturing`.

:def_p:`fls_oisws5qykedi`
An :term:`async block expression` denotes a new :term:`control flow boundary`.

:def_p:`fls_ncd0wkgtldem`
The :term:`type` of an :term:`async block expression` shall implement the
:codeterm:`core::future::Future` trait.

:def_p:`fls_pvnofoomgwl5`
The :term:`value` of an :term:`async block expression` is a :term:`future`.

.. rubric:: Dynamic Semantics

:def_p:`fls_9ghp5yet75y6`
The :term:`evaluation` of an :term:`async block expression` produces an
anonymous :term:`object` that captures the related :term:`future`.

.. rubric:: Examples

.. code-block:: text

   async {}

unsafe Blocks
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   UnsafeBlockExpression ::=
       $$unsafe$$ BlockExpression


.. rubric:: Legality Rules

:def_p:`fls_2az5huhcxzzy`
An :term:`unsafe block expression` is a :term:`block expression` that is marked
as :term:`unsafe`.

:def_p:`fls_5ucvvja4dzoc`
An :term:`unsafe block expression` allows :term:`unsafety`.

:def_p:`fls_2nzwo1hbsg9g`
An :term:`unsafe block expression` is a :term:`value expression`.

:def_p:`fls_j3mmg317q442`
The :term:`type` of the :term:`unsafe block expression` is the :term:`type` of
its :term:`block expression`.

:def_p:`fls_nygurv3x3wq6`
The :term:`value` of the :term:`unsafe block expression` is the :term:`value` of
its :term:`block expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_pv5gcy3tbjwo`
The :term:`evaluation` of an :term:`unsafe block expression` evaluates its
:term:`block expression`.

.. rubric:: Examples

.. code-block:: text

   unsafe {
       unsafe_fn_call()
   }

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

:def_p:`fls_ursc5ynymoy`
An :term:`operator expression` is an :term:`expression` that involves an
operator.

Borrow Expression
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BorrowExpression ::=
       $$&$$ $$mut$$? Operand

.. rubric:: Legality Rules

:def_p:`fls_nnqfkl228hjx`
A :term:`borrow expression` is an :term:`expression` that borrows the
:term:`value` of its :term:`operand` and creates a :term:`reference` to the
memory location of its :term:`operand`.

:def_p:`fls_r7ix8webgqlm`
An :term:`immutable borrow expression` is a :term:`borrow expression` that lacks
:term:`keyword` ``mut``.

:def_p:`fls_50j167r4v61b`
A :term:`mutable borrow expression` is a :term:`borrow expression` that has
:term:`keyword` ``mut``.

:def_p:`fls_ya77l2zgtilp`
When the :term:`operand` of a :term:`borrow expression` is a :term:`place
expression`, the :term:`borrow expression` produces a :term:`reference` to the
memory location indicated by the :term:`operand`. The memory location is placed
in a borrowed state, or simply :term:`borrowed`.

:def_p:`fls_8uhfwqurbyqf`
When the :term:`operand` of a :term:`borrow expression` is a :term:`value
expression`, a :term:`temporary` is allocated and the :term:`borrow expression`
produces a :term:`reference` to the memory location of the :term:`temporary`.

:def_p:`fls_xrq41zaq6bza`
A :term:`borrow expression` is a :term:`value expression`.

:def_p:`fls_chr03xll75d`
The :term:`type` of a :term:`borrow expression` is determined as follows:

* :def_p:`fls_5b2x5ri2w54r`
  If the :term:`borrow expression` denotes a :term:`shared reference`, then the
  :term:`type` is ``&T`` where ``T`` is the :term:`type` of the :term:`operand`.

* :def_p:`fls_agl09ia869rk`
  If the :term:`borrow expression` denotes a :term:`mutable reference`,
  then the :term:`type` is ``&mut T`` where ``T`` is the :term:`type` of the
  :term:`operand`.

:def_p:`fls_8cvmee9bzs40`
The :term:`value` of a :term:`borrow expression` is the address of its
:term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_2jd0mgw4zja4`
The :term:`evaluation` of a :term:`borrow expression` evaluates its
:term:`operand`.

.. rubric:: Examples

.. code-block:: text

   let mut answer = 42;


:def_p:`fls_350qejoq9i23`
Mutable borrow.

.. syntax::


   let ref_answer = &mut answer;

Dereference Expression
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   DereferenceExpression ::=
       $$*$$ Operand

.. rubric:: Legality Rules

:def_p:`fls_f6wktzofzdn1`
A :term:`dereference expression` is an :term:`expression` that obtains the
pointed-to memory location of its :term:`operand`.

:def_p:`fls_aeh5pzpcjveq`
When the :term:`operand` of a :term:`dereference expression` is of a
:term:`pointer type`, the :term:`dereference expression` denotes the pointed-to
memory location of the :term:`operand`, or the :term:`dereference` of the
:term:`operand`.

:def_p:`fls_9cc0ml2sru6x`
The :term:`dereference` is assignable when

* :def_p:`fls_m0cc62tcf6b7`
  The :term:`operand` is of :term:`type` ``&mut T`` or ``*mut T``, and

* :def_p:`fls_llzt4s3uwt95`
  The :term:`operand` is a :term:`binding` or a possibly nested :term:`field` of
  a :term:`binding`, or

* :def_p:`fls_908xdt291via`
  The :term:`operand` denotes a :term:`mutable place expression`.

* :def_p:`fls_b96mek2ojcl`
  The :term:`operand` is of another :term:`type` that implements the
  :codeterm:`core::ops::DerefMut` :term:`trait`.

:def_p:`fls_8i4jzksxlrw0`
Dereferencing a :term:`raw pointer` shall require :term:`unsafe context`.

:def_p:`fls_d68ddlse4zp`
If the context of a :term:`dereference expression` is an :term:`immutable
place expression`, then the :term:`dereference expression` is equivalent to
:term:`expression` ``*core::ops::Deref::deref(&operand)``.

:def_p:`fls_g73vguanbs1x`
If the context of a :term:`dereference expression` is a :term:`mutable
place expression`, then the :term:`dereference expression` is equivalent to
:term:`expression` ``*core::ops::DerefMut::deref_mut(&mut operand)``.

:def_p:`fls_8ibfqxtnahzx`
The :term:`type` of a :term:`dereference expression` is determined as follows:

* :def_p:`fls_7e7tka4f2f1a`
  If the :term:`type` of the :term:`operand` is ``&mut T``, ``&T``, ``*mut T``,
  or ``*const T``, then the :term:`type` is ``T``\ ``.``

* :def_p:`fls_y9bc691kkh6v`
  Otherwise the :term:`type` is :term:`associated type`
  :codeterm:`core::ops::Deref::Target`.

:def_p:`fls_gw49nukfveib`
The :term:`value` of a :term:`dereference expression` is determined as follows:

* :def_p:`fls_jjf3sz9ddfhy`
  If the :term:`type` of the :term:`operand` is ``&mut T``, ``&T``, ``*mut T``,
  or ``*const T``, then the :term:`value` is the pointed-to :term:`value`\ ``.``

* :def_p:`fls_fyend8kkpqq4`
  Otherwise the :term:`value` is the result of evaluating :term:`expression`
  ``*core::ops::Deref::deref(&operand)`` or :term:`expression`
  ``*core::ops::DerefMut::deref_mut(&mut operand)`` respectively.

.. rubric:: Dynamic Semantics

:def_p:`fls_72bpdsxxbgeq`
The :term:`evaluation` of a :term:`dereference expression` evaluates its
:term:`operand`.

.. rubric:: Undefined Behavior

:def_p:`fls_9wgldua1u8yt`
It is undefined behavior to dereference a :term:`raw pointer` that is either
:term:`dangling` or unaligned.

.. rubric:: Examples

:def_p:`fls_9ifaterm8nop`
See :p:`6.4.1. <fls_ltflbfba9d5r>` for the declaration of ``ref_answer``.

.. code-block:: text

   let deref_asnwer = *ref_answer;

Error Propagation Expression
~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ErrorPropagationExpression ::=
       Operand ErrorPropagationOperator

   ErrorPropagationOperator ::=
       $$?$$

.. rubric:: Legality Rules

:def_p:`fls_8q59wbumrt5s`
An :term:`error propagation expression` is an :term:`expression` that either
evaluates to a :term:`value` of its :term:`operand` or returns a value to the
next control flow boundary.

:def_p:`fls_mq2h4seoxah`
An :term:`error propagation expression` shall appear within a :term:`control
flow boundary`.

:def_p:`fls_ab4vhq4nwn7f`
The :term:`type` of an :term:`error propagation expression` is :term:`associated
type` :codeterm:`core::ops::Try::Output`.

:def_p:`fls_z4zikxy2b1em`
The :term:`value` of an :term:`error propagation expression` is determined as
follows:

* :def_p:`fls_a09614kgsspt`
  If the :term:`evaluation` of the :term:`error propagation expression`
  executed ``core::ops::Try::branch(operand)``, then the :term:`value` is the
  :term:`value` of the :codeterm:`core::ops::ControlFlow::Continue` variant.

* :def_p:`fls_8df018q7y6g`
  Otherwise control flow is returned to the end of the enclosing :term:`control
  flow boundary`.

:def_p:`fls_9sriwut951xv`
The expression context for the :term:`operand` of the :term:`error propagation
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_alk4qvfprnvy`
The :term:`evaluation` of an :term:`error propagation operator` of the form

.. code-block:: text

   expression?

:def_p:`fls_1nnhjcgy8kdh`
is equivalent to the :term:`evaluation` the following :term:`expression`:

.. syntax::


   match core::ops::Try::branch(expression) {
       core::ops::ControlFlow::Continue(value) =>
           value,

       core::ops::ControlFlow::Break(value) =>
           core::ops::FromResidual::from_residual(value),
   }

.. rubric:: Examples

.. code-block:: text

   fn try_to_parse() -> Result<i32, ParseIntError> {
       "42".parse()?
   }

   fn try_some() -> Option<i32> {
       let val = Some(42)?;
       Some(val)
   }

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

:def_p:`fls_pfa81kv2mru8`
A :term:`negation expression` is an :term:`expression` that negates its
:term:`operand`.

:def_p:`fls_plcut8vzdwox`
The :term:`type` of the :term:`operand` of a :term:`negation expression` with a
:syntax:`BitwiseNegationOperator` shall implement the :codeterm:`core::ops::Not`
:term:`trait`.

:def_p:`fls_ohu0kljfexd3`
The :term:`type` of a :term:`negation expression` with a
:syntax:`BitwiseNegationOperator` is :term:`associated type`
:codeterm:`core::ops::Not::Output`.

:def_p:`fls_ghqvj8q71o97`
The :term:`value` of a :term:`negation expression` with
a :syntax:`BitwiseNegationOperator` is the result of
``core::ops::Not::not(operand)``.

:def_p:`fls_3m4mgqnzqhri`
The :term:`type` of the :term:`operand` of a :term:`negation expression` with
a :syntax:`SignNegationOperator` shall implement the :codeterm:`core::ops::Neg`
:term:`trait`.

:def_p:`fls_u7gzm6n75rzm`
The :term:`type` of a :term:`negation expression` with a
:syntax:`SignNegationOperator` shall be :term:`associated type`
:codeterm:`core::ops::Neg::Output`.

:def_p:`fls_9rmq7iaf092d`
The :term:`value` of a :term:`negation expression`
with a :syntax:`SignNegationOperator` is the result of
``core::ops::Neg::neg(operand)``.

:def_p:`fls_2eou0x2lxmk6`
The expression context for the :term:`operand` of the :term:`negation
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_yzt6pcsvj3a`
The :term:`evaluation` of a :term:`negation expression` with a
:syntax:`BitwiseNegationOperator` proceeds as follows:

#. :def_p:`fls_8tgxtprtifrr`
   The :term:`operand` is evaluated.

#. :def_p:`fls_gn3dnuxm2h8m`
   ``core::ops::Not::not(operand)`` is invoked.

:def_p:`fls_tsou6yz4mfte`
The :term:`evaluation` of a :term:`negation expression` with a
:syntax:`SignNegationOperator` proceeds as follows:

#. :def_p:`fls_zdfgqky85r1f`
   The :term:`operand` is evaluated.

#. :def_p:`fls_uldh10k77sng`
   ``core::ops::Neg::neg(operand)`` is invoked.

.. rubric:: Examples

:def_p:`fls_uo6vv2yf8usx`
Sign negation.

.. code-block:: text

   -42

:def_p:`fls_hbrg0d98jak5`
Bitwise negation.

.. code-block:: text

   !42

:def_p:`fls_kqtr9c3jorvg`
Logical negation.

.. code-block:: text

   !false

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

:def_p:`fls_asibqpe3z95h`
An :term:`arithmetic expression` is an :term:`expression` that computes a
:term:`value` from two :term:`operand`\ s using arithmetic.

:def_p:`fls_dstca76y08ge`
A :term:`division expression` is an :term:`arithmetic expression` that uses
division.

:def_p:`fls_kf41bphvlse3`
A :term:`multiplication expression` is an :term:`arithmetic expression` that
uses multiplication.

:def_p:`fls_3de9ulyzuoa`
A :term:`remainder expression` is an :term:`arithmetic expression` that uses
remainder division.

:def_p:`fls_aalxhbvu8kdi`
A :term:`subtraction expression` is an :term:`arithmetic expression` that uses
subtraction.

:def_p:`fls_8imzo7agyx0k`
The :term:`type` of the :term:`left operand` of an :term:`addition expression`
shall implement the :codeterm:`core::ops::Add` :term:`trait` with the
:term:`type` of the :term:`right operand` as the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_vk17mfv47wk9`
The :term:`type` of an :term:`addition expression` is :term:`associated type`
:codeterm:`core::ops::Add::Output`.

:def_p:`fls_ryzhdpxgm7ii`
The :term:`value` of an :term:`addition expression` is the result of
``core::ops::Add::add(left_operand, right_operand)``.

:def_p:`fls_f1puss9t4btz`
The :term:`type` of the :term:`left operand` of a :term:`division expression`
shall implement the :codeterm:`core::ops::Div` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_5rdrkvspw57z`
The :term:`type` of a :term:`division expression` is :term:`associated type`
:codeterm:`core::ops::Div::Output`.

:def_p:`fls_thyq4h55mx55`
The :term:`value` of a :term:`division expression` is the result of
``core::ops::Div::div(left_operand, right_operand)``.

:def_p:`fls_hrml95g2txcj`
The :term:`type` of the :term:`left operand` of a :term:`multiplication
expression` shall implement the :codeterm:`core::ops::Mul` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_ittf4yggk7do`
The :term:`type` of a :term:`multiplication expression` is :term:`associated
type` :codeterm:`core::ops::Mul::Output`.

:def_p:`fls_ylqm6wucq2sw`
The :term:`value` of a :term:`multiplication expression` is the result of
``core::ops::Mul::mul(left_operand, right_operand)``.

:def_p:`fls_8fbhreyynhid`
The :term:`type` of the :term:`left operand` of a :term:`remainder expression`
shall implement the :codeterm:`core::ops::Rem` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_u3jwnrqun5kl`
The :term:`type` of a :term:`remainder expression` is :term:`associated type`
:codeterm:`core::ops::Rem::Output`.

:def_p:`fls_2ude3wrxji2p`
The :term:`value` of a :term:`remainder expression` is the result of
``core::ops::Rem::rem(left_operand, right_operand)``.

:def_p:`fls_fjcv1nm8tlgf`
The :term:`type` of the :term:`left operand` of a :term:`subtraction expression`
shall implement the :codeterm:`core::ops::Sub` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_9x2i1zlsm364`
The :term:`type` of a :term:`subtraction expression` is :term:`associated type`
:codeterm:`core::ops::Sub::Output`.

:def_p:`fls_v8vekngd27sz`
The :term:`value` of a :term:`subtraction expression` is the result of
``core::ops::Sub::sub(left_operand, right_operand)``.

:def_p:`fls_69r1m88mxzx5`
The expression context for the :term:`operand`\ s of an :term:`arithmetic
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_5nsa9zefz9cv`
The :term:`evaluation` of an :term:`addition expression` proceeds as follows:

#. :def_p:`fls_u3pstd6xe43y`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_jjmc1xgny77`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_cayhj5hcuhcg`
   ``core::ops::Add::add(left_operand, right_operand)`` is invoked.

:def_p:`fls_43knkymqpj7t`
The :term:`evaluation` of a :term:`division expression` proceeds as follows:

#. :def_p:`fls_62gpbubfj30w`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_bveocgaagk1n`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_qd6ggdgq2hob`
   ``core::ops::Div::div(left_operand, right_operand)`` is invoked.

:def_p:`fls_lr2a21v5en59`
The :term:`evaluation` of a :term:`multiplication expression` proceeds as
follows:

#. :def_p:`fls_kpbxcdaflb06`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_b94ojbfukhvd`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_blyr18iao20n`
   ``core::ops::Mul::mul(left_operand, right_operand)`` is invoked.

:def_p:`fls_g28igfbnwfe0`
The :term:`evaluation` of a :term:`remainder expression` proceeds as follows:

#. :def_p:`fls_thcumw8n8xbw`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_gld1u9fnsj6d`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_k7lmxvpkxtub`
   ``core::ops::Rem::rem(left_operand, right_operand)`` is invoked.

:def_p:`fls_bndpd66973ev`
The :term:`evaluation` of a :term:`subtraction expression` proceeds as follows:

#. :def_p:`fls_izmfimd4yg27`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_ad9tc6ki8vcq`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_b9g0r9vc4rou`
   ``core::ops::Rem::rem(left_operand, right_operand)`` is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_8dkygceg0oo`
It is undefined behavior for an :term:`arithmetic operation` to cause overflow
with :term:`value`\ s of :term:`numeric type`\ s.

.. rubric:: Examples

.. code-block:: text

   1 + 2
   4.0 / 3.29
   8.4 * 5.3
   10 % 4
   3 - 2

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

:def_p:`fls_3zd59yuywz6l`
A :term:`bit expression` is an :term:`expression` that computes a :term:`value`
from two :term:`operand`\ s using bit arithmetic.

:def_p:`fls_f6mmva3lbj1i`
A :term:`bit and expression` is a :term:`bit expression` that uses bit and
arithmetic.

:def_p:`fls_3136k1y6x3cu`
A :term:`bit or expression` is a :term:`bit expression` that uses bit or
arithmetic.

:def_p:`fls_j7ujcuthga1i`
A :term:`bit xor expression` is a :term:`bit expression` that uses bit exclusive
or arithmetic.

:def_p:`fls_caxn774ij8lk`
A :term:`shift left expression` is a :term:`bit expression` that uses bit shift
left arithmetic.

:def_p:`fls_t709sl4co3al`
A :term:`shift right expression` is a :term:`bit expression` that uses bit shift
right arithmetic.

:def_p:`fls_cmowpfrcelke`
The :term:`type` of the :term:`left operand` of a :term:`bit and expression`
shall implement the :codeterm:`core::ops::BitAnd` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_kchprk9z6xun`
The :term:`type` of a :term:`bit and expression` is :term:`associated type`
:codeterm:`core::ops::BitAnd::Output`.

:def_p:`fls_dimu987fw4kg`
The :term:`value` of a :term:`bit and expression` is the result of
``core::ops::BitAnd::bitand(left_operand, right_operand)``.

:def_p:`fls_oo2ynd8e1ys6`
The :term:`type` of the :term:`left operand` of a :term:`bit or expression`
shall implement the :codeterm:`core::ops::BitOr` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_s6hkt5fg598y`
The :term:`type` of a :term:`bit or expression` is :term:`associated type`
:codeterm:`core::ops::BitOr::Output`.

:def_p:`fls_osfse0t6ua8a`
The :term:`value` of a :term:`bit or expression` is the result of
``core::ops::BitOr::bitor(left_operand, right_operand)``.

:def_p:`fls_fnywefl9nty2`
The :term:`type` of the :term:`left operand` of a :term:`bit xor expression`
shall implement the :codeterm:`core::ops::BitXor` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_4f24nyx0ix0j`
The :term:`type` of a :term:`bit xor expression` is :term:`associated type`
:codeterm:`core::ops::BitXor::Output`.

:def_p:`fls_8tb22c6zbp3`
The :term:`value` of a :term:`bit xor expression` is the result of
``core::ops::BitXor::bitxor(left_operand, right_operand)``.

:def_p:`fls_1f4pc612f2a8`
The :term:`type` of the :term:`left operand` of a :term:`shift left expression`
shall implement the :codeterm:`core::ops::Shl` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_8trozue35xe4`
The :term:`type` of a :term:`shift left expression` is :term:`associated type`
:codeterm:`core::ops::Shl::Output`.

:def_p:`fls_kqntxbwnc58v`
The :term:`value` of a :term:`shift left expression` is the result of
``core::ops::Shl::shl(left_operand, right_operand)``.

:def_p:`fls_onutb0b9p9zj`
The :term:`type` of the :term:`left operand` of a :term:`shift right operation`
shall implement the :codeterm:`core::ops::Shr` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_yq8rtwfp3nv0`
The :term:`type` of a :term:`shift right operation` is :term:`associated type`
:codeterm:`core::ops::Shr::Output`.

:def_p:`fls_fbazfgd5m1ot`
The :term:`value` of a :term:`shift right operation` is the result of
``core::ops::Shr::shr(left_operand, right_operand)``.

:def_p:`fls_2z6wble3u8ec`
The expression context for the :term:`operand`\ s of a :term:`bit expression` is
a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_f4o8xlu67okn`
The :term:`evaluation` of a :term:`bit and expression` proceeds as follows:

#. :def_p:`fls_kp747xqekyrr`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_m0pdk78dah6n`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_m2hsk41hwm2j`
   ``core::ops::BitAnd::bitand(left_operand, right_operand)`` is invoked.

:def_p:`fls_p9rlmjhbnbao`
The :term:`evaluation` of a :term:`bit or expression` proceeds as follows:

#. :def_p:`fls_vprp53kv64q6`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_d456ummq6vrk`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_n269ufyesndz`
   ``core::ops::BitOr::bitor(left_operand, right_operand)`` is invoked.

:def_p:`fls_i9iqtobheivu`
The :term:`evaluation` of a :term:`bit xor expression` proceeds as follows:

#. :def_p:`fls_htw2tpujktwt`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_gf9tyu1idpjk`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_u5irwqswbsvu`
   ``core::ops::BitXor::bitxor(left_operand, right_operand)`` is invoked.

:def_p:`fls_2kkpr955i4lm`
The :term:`evaluation` of a :term:`shift left expression` proceeds as follows:

#. :def_p:`fls_7p64lgnjxylz`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_ieh1itrkcnf6`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_f0p70y92k14f`
   ``core::ops::Shl::shl(left_operand, right_operand)`` is invoked.

:def_p:`fls_303r0u6ug215`
The :term:`evaluation` of a :term:`shift right expression` proceeds as follows:

#. :def_p:`fls_4gxj18t6cnzq`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_gurl2ve58drz`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_xkyj83mcb9d5`
   ``core::ops::Shr::shr(left_operand, right_operand)`` is invoked.

.. rubric:: Examples

.. code-block:: text

   0b1010 & 0b1100
   0b1010 | 0b0011
   0b1010 ^ 0b1001
   13 << 3
   -10 >> 2

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

:def_p:`fls_yzuceqx6nxwa`
A :term:`comparison expression` is an :term:`expression` that compares the
:term:`value`\ s of two :term:`operand`\ s.

:def_p:`fls_ruyho6cu7rxg`
An :term:`equals expression` is a :term:`comparison expression` that tests
equality.

:def_p:`fls_wapl0ir7uvbp`
A :term:`greater-than expression` is a :term:`comparison expression` that tests
for a greater-than relationship.

:def_p:`fls_7n5gol6a8lod`
A :term:`greater-than-or-equals expression` is a :term:`comparison expression`
that tests for a greater-than-or-equals relationship.

:def_p:`fls_yd4qqi39w248`
A :term:`less-than expression` is a :term:`comparison expression` that tests for
a less-than relationship.

:def_p:`fls_yxwe1o27u6ns`
A :term:`less-than-or-equals expression` is a :term:`comparison expression` that
tests for a less-than-or-equals relationship.

:def_p:`fls_w71j7i3n1kit`
A :term:`not-equals expression` is a :term:`comparison expression` that tests
for inequality.

:def_p:`fls_asfrqemqviad`
A :term:`comparison expression` implicitly takes :term:`shared borrow`\ s of its
:term:`operand`\ s.

:def_p:`fls_9s4re3ujnfis`
The :term:`type` of a :term:`comparison expression` is :term:`type`
:codeterm:`bool`.

:def_p:`fls_8echqk9po1cf`
The :term:`type` of the :term:`left operand` of an :term:`equals expression`
shall implement the :codeterm:`core::cmp::PartialEq` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_d62qfloqk2ub`
The :term:`value` of an :term:`equals expression` is the result of
``core::cmp::PartialEq::eq(&left_operand, &right_operand)``.

:def_p:`fls_x2s6ydvj5zyd`
The :term:`type` of the :term:`left operand` of a :term:`greater-than
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_pso38dowbk91`
The :term:`value` of a :term:`greater-than expression` is the result of
``core::cmp::PartialOrd::gt(&left_operand, &right_operand)``.

:def_p:`fls_hholzcbp5u3n`
The :term:`type` of the :term:`left operand` of a :term:`greater-than-or-equals
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_wytygse41vzm`
The :term:`value` of a :term:`greater-than-or-equals expression` is the result
of ``core::cmp::PartialOrd::ge(&left_operand, &right_operand)``.

:def_p:`fls_ynibdcke3etb`
The :term:`type` of the :term:`left operand` of a :term:`less-than expression`
shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_xmtxkit3qpw7`
The :term:`value` of a :term:`less-than expression` is the result of
``core::cmp::PartialOrd::lt(&left_operand, &right_operand)``.

:def_p:`fls_6dgfieyxdan0`
The :term:`type` of the :term:`left operand` of a :term:`less-than-or-equals
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_7pfsqby2saag`
The :term:`value` of a :term:`less-than-or-equals expression` is the result of
``core::cmp::PartialOrd::le(&left_operand, &right_operand)``.

:def_p:`fls_qzo1torhv5i3`
The :term:`type` of the :term:`left operand` of a :term:`not-equals expression`
shall implement the :codeterm:`core::cmp::PartialEq` :term:`trait` where the
:term:`type` of the :term:`right operand` is the :term:`trait implementation`
:term:`type parameter`.

:def_p:`fls_kodwkh58hmdv`
The :term:`value` of a :term:`not-equals expression` is the result of
``core::cmp::PartialEq::ne(&left_operand, &right_operand)``.

:def_p:`fls_8qbrzb9bxyf`
The expression context for the :term:`operand`\ s of a :term:`comparison
expression` is a :term:`place expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_ydt9zvh0h5ex`
The :term:`evaluation` of an :term:`equals expression` proceeds as follows:

#. :def_p:`fls_4vbrc31r0o60`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_hyy974ksbbrq`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_htrjqxiv3avh`
   ``core::cmp::PartialEq::eq(&left_operand, &right_operand)`` ``is invoked.``

:def_p:`fls_1udbc4aom6ok`
The :term:`evaluation` of a :term:`greater-than expression` proceeds as follows:

#. :def_p:`fls_96mt7gx5ogo0`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_or0i2cqxwl8o`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_udnhkbxpk83m`
   ``core::cmp::PartialOrd::gt(&left_operand, &right_operand)`` is invoked.

:def_p:`fls_mab6yirx77zl`
The :term:`evaluation` of a :term:`greater-than-or-equals expression` proceeds
as follows:

#. :def_p:`fls_2ggb7a7nhrk9`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_ukm97arfzsk1`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_wrftg7onlkmm`
   ``core::cmp::PartialOrd::ge(&left_operand, &right_operand)`` is invoked.

:def_p:`fls_irlqykpbtvd`
The :term:`evaluation` of a :term:`less-than expression` proceeds as follows:

#. :def_p:`fls_udonl4c7f6pz`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_ebvyhqbb921g`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_rfomib80bnn2`
   ``core::cmp::PartialOrd::lt(&left_operand, &right_operand)`` is invoked.

:def_p:`fls_6cb4wg59wmef`
The :term:`evaluation` of a :term:`less-than-or-equals expression` proceeds
as follows:

#. :def_p:`fls_dkbjn7noq8n2`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_kezynx2xc1q7`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_8luq5sellcaq`
   ``core::cmp::PartialOrd::le(&left_operand, &right_operand)`` is invoked.

:def_p:`fls_c93pacid548a`
The :term:`evaluation` of a :term:`not-equals expression` proceeds as follows:

#. :def_p:`fls_gqy6uuowij9e`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_s6sq6p8th5nt`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_kdga59xx4nx3`
   ``core::cmp::PartialEq::ne(&left_operand, &right_operand)`` is invoked.

.. rubric:: Examples

:def_p:`fls_777hlnpac9h1`
12 == 12

:def_p:`fls_xx7ugkxmk06p`
42 > 12

:def_p:`fls_pfym2t99i6x4`
42 >= 35

:def_p:`fls_nnvf94dbxwte`
42 < 109

:def_p:`fls_4h896fhds7jk`
42 <= 42

:def_p:`fls_rm0hk0svq4v7`
12 != 42

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

:def_p:`fls_gpbvus89iy4c`
A :term:`lazy boolean expression` is an :term:`expression` that performs short
circuit Boolean arithmetic.

:def_p:`fls_40jya46h62yi`
A :term:`lazy and expression` is a :term:`lazy boolean expression` that uses
short circuit and arithmetic.

:def_p:`fls_k8u77ow5bb6c`
A :term:`lazy or expression` is a :term:`lazy boolean expression` that uses
short circuit or arithmetic.

:def_p:`fls_u0gwo0s2l0tn`
The :term:`type`\ s of the :term:`operand`\ s of a :term:`lazy boolean
expression` shall be :term:`type` :codeterm:`bool`.

:def_p:`fls_zas0lizgq2hn`
The :term:`type` of a :term:`lazy boolean expression` is :term:`type`
:codeterm:`bool`.

:def_p:`fls_xdgvrd58nkoa`
The :term:`value` of a :term:`lazy boolean expression` is either ``true`` or
``false``.

:def_p:`fls_bov5j5t1bx0a`
The expression context for the :term:`operand`\ s of the :term:`lazy boolean
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_ufre0ko2cwh2`
The :term:`evaluation` of a :term:`lazy and expression` proceeds as follows:

#. :def_p:`fls_jugckad775kq`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_tmfmu3syxp2q`
   If the :term:`left operand` evaluated to ``true``, then

   #. :def_p:`fls_edj00fp6bqdk`
      The :term:`right operand` is evaluated and returned as the :term:`lazy and
      expression`'s :term:`value`.

#. :def_p:`fls_srfv1d4idxy9`
   Otherwise the :term:`lazy and expression` evaluates to ``false``.

:def_p:`fls_tflikh8cmxvc`
The :term:`evaluation` of a :term:`lazy or expression` proceeds as follows:

#. :def_p:`fls_p0rafjsridre`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_yg1348rlziw3`
   If the :term:`left operand` evaluated to ``false``, then

   #. :def_p:`fls_ds8cr5dxc9em`
      The :term:`right operand` is evaluated and returned as the :term:`lazy or
      expression`'s :term:`value`.

#. :def_p:`fls_yffozo2vq5xz`
   Otherwise the :term:`lazy or expression` evaluates to ``true``.

.. rubric:: Examples

.. code-block:: text

   false && panic!()
   this || that

Type Cast Expressions
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TypeCastExpression ::=
       Operand $$as$$ TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:def_p:`fls_ltioqbhl14g0`
A :term:`type cast expression` is an :term:`expression` that changes the
:term:`type` of an :term:`operand`.

:def_p:`fls_99kvyh4puy57`
:term:`Cast` or :term:`casting` is the process of changing the :term:`type` of
an :term:`expression`.

:def_p:`fls_a6midh2m0w0b`
The ``TypeSpecificationWithoutBounds`` describes the :def_term:`target type` of
the :term:`type cast expression`.

:def_p:`fls_otaxe9okhdr1`
A :term:`type cast expression` with the following characteristics performs a
:def_term:`specialized cast`:

* :def_p:`fls_4s69s9pcvbn7`
  An :term:`operand` of a :term:`numeric type` and a target :term:`numeric type`
  perform a :term:`numeric cast`.

* :def_p:`fls_le6bchl25ewz`
  An :term:`operand` of an :term:`enum type` and a target :term:`integer type`
  perform :term:`enum cast`. An\ * enum cast* converts the :term:`operand` to
  its :term:`discriminant`, followed by a :term:`numeric cast`.

* :def_p:`fls_pcromhosmnf0`
  An operand of :term:`type` :codeterm:`bool` or :term:`type` :codeterm:`char`
  and a target :term:`integer type` perform :term:`primitive-to-integer cast`. A
  :def_term:`primitive-to-integer cast`

  * :def_p:`fls_al9f1t7vlsxi`
    Converts an :term:`operand` of :term:`type` :codeterm:`bool` with
    :term:`value` ``false`` to zero.

  * :def_p:`fls_jea17f39fmsg`
    Converts an :term:`operand` of type :codeterm:`bool` with :term:`value`
    ``true`` to one.

  * :def_p:`fls_eb00s8fxlvjb`
    Convert an :term:`operand` of type :codeterm:`char` to the :term:`value` of
    the corresponding :term:`code point`, followed by a :term:`numeric cast`.

* :def_p:`fls_qk5trk8wkvxb`
  An :term:`operand` of :term:`type` :codeterm:`u8` and a target :term:`type`
  :codeterm:`char` performs :term:`u8-to-char cast`. A :def_term:`u8-to-char
  cast` converts an :term:`operand` of :term:`type` :codeterm:`u8` to the
  :term:`value` of the corresponding :term:`code point`.

* :def_p:`fls_t16yzaxro5ew`
  An :term:`operand` of :term:`type` ``*const T`` or ``*mut T``
  and a :term:`target type` ``*const V`` or ``*mut V`` where ``V``
  implements the :codeterm:`core::marker::Sized` :term:`trait` performs
  :term:`pointer-to-pointer cast`.

* :def_p:`fls_i4zsbbmfa2fl`
  An :term:`operand` of :term:`type` ``*const T`` or ``*mut T`` where
  ``T`` implements the :codeterm:`core::marker::Sized` :term:`trait` and
  a target :term:`integer type` perform :term:`pointer-to-address cast`.
  A :def_term:`pointer-to-address cast` produces an :term:`integer` that
  represents the machine address of the referenced memory. If the :term:`integer
  type` is smaller than the :term:`type` of the :term:`operand`, the address
  is truncated.

* :def_p:`fls_59mpteeczzo`
  An :term:`operand` of :term:`integer type` and :term:`target type` ``*const
  V`` or ``*mut V`` where ``V`` implements the :codeterm:`core::marker::Sized`
  :term:`trait` perform :term:`address-to-pointer cast`. An
  :def_term:`address-to-pointer cast` produces a :term:`pointer` that interprets
  the :term:`integer` as a machine address.

* :def_p:`fls_8ccwlliqw9jx`
  An :term:`operand` of :term:`type` ``&mut [T; N]`` and a :term:`target type`
  ``*const T`` perform :term:`array-to-pointer cast`.

* :def_p:`fls_i8txki3htx92`
  An :term:`operand` of a :term:`function item type` and a
  :term:`target type` ``*const V`` or ``*mut V`` where ``V``
  implements the :codeterm:`core::marker::Sized` :term:`trait` perform
  :term:`function-item-to-pointer cast`.

* :def_p:`fls_6hbkvbb1c8aj`
  An :term:`operand` of a :term:`function item type` and a target :term:`integer
  type` perform :term:`function-to-address cast`.

* :def_p:`fls_133j6xw8k4qe`
  An :term:`operand` of a :term:`function pointer type` and a
  :term:`target type` ``*const V`` or ``*mut V`` where ``V``
  implements the :codeterm:`core::marker::Sized` :term:`trait` perform
  :term:`function-pointer-to-pointer cast`.

* :def_p:`fls_bhw2j9wjpf2x`
  An :term:`operand` of a :term:`function pointer type` and a target
  :term:`integer type` perform :term:`function-pointer-to-address cast`.

:def_p:`fls_3ww5gbk9w4ys`
A :term:`cast` is legal when it either performs :term:`type coercion` or is a
:term:`specialized cast`.

:def_p:`fls_hhxawo12cndy`
The :term:`type` of a :term:`type cast expression` is the :term:`target type`.

:def_p:`fls_uuayaksl8059`
The :term:`value` of a :term:`type cast expression` is the :term:`value` of the
:term:`operand` after the :term:`cast`.

.. rubric:: Dynamic Semantics

:def_p:`fls_syk2li8ft3rx`
The :term:`evaluation` of a :term:`type cast expression` evaluates its
:term:`operand`.

:def_p:`fls_uqv32nthva6y`
The :term:`evaluation` of a :def_term:`numeric cast` proceeds as follows:

* :def_p:`fls_kc3gwj9x3jnr`
  Casting an :term:`operand` of an :term:`integer type` to a target
  :term:`integer type` of the same :term:`size` has no effect.

* :def_p:`fls_76eq3bd6birr`
  Casting an :term:`operand` of an :term:`integer type` to a target
  :term:`integer type` with smaller :term:`size` truncates the :term:`value` of
  the :term:`operand`.

* :def_p:`fls_ldiritt32h2w`
  Casting an :term:`operand` of an :term:`integer type` to a target
  :term:`integer type` with a larger :term:`size` either

* :def_p:`fls_h9sxg3pxn7i2`
  Zero-extends the :term:`operand` if the :term:`operand`'s :term:`type` is
  unsigned, or

* :def_p:`fls_shy6e0e30bco`
  Sign-extends the :term:`operand` if the :term:`operand`'s :term:`type` is
  signed.

* :def_p:`fls_4xldaoj5ac6t`
  Casting an :term:`operand` of a :term:`floating-point type` to a target
  :term:`integer type` rounds the :term:`value` of the :term:`operand` towards
  zero. In addition, the :term:`type cast expression`

* :def_p:`fls_50714cvaqkfv`
  Returns zero if the :term:`operand` denotes :codeterm:`f32::NaN` or
  :codeterm:`f64::NaN` respectively.

* :def_p:`fls_g3xbmp8zx1yh`
  Saturates the :term:`value` of the :term:`operand` to the maximum
  :term:`value` of the target :term:`integer type` if the :term:`operand`'s
  :term:`value` exceeds the maximum :term:`value` of the target :term:`integer
  type` or denotes :codeterm:`f32::INFINITY` or :codeterm:`f64::INFINITY`
  respectively.

* :def_p:`fls_hcc5odh52bk7`
  Saturates the :term:`value` of the :term:`operand` to the minimum
  :term:`value` of the target :term:`integer type` if the :term:`operand`'s
  :term:`value` exceeds the minimum :term:`value` of the target
  :term:`integer type` or denotes :codeterm:`f32::NEG_INFINITY` or
  :codeterm:`f64::NEG_INFINITY` respectively.

* :def_p:`fls_o2ep4b6t287z`
  Casting an :term:`operand` of an :term:`integer type` to a target
  :term:`floating-point type` produces the closest possible floating point
  :term:`value`. In addition, the :term:`type cast expression`

* :def_p:`fls_vfofk2aagsj5`
  Rounds the :term:`value` of the :term:`operand` preferring the :term:`value`
  with an even least significant digit if exactly halfway between two floating
  point numbers.

* :def_p:`fls_cx86k8yfjhht`
  Produc	es :codeterm:`f32::INFINITY` or :codeterm:`f64::INFINITY` of the same
  sign as the :term:`value` of the :term:`operand` when the :term:`value` of the
  :term:`operand` causes overflow.

* :def_p:`fls_gzmdwibl5s4w`
  Casting an :term:`operand` of :term:`type` :codeterm:`f32` to a :term:`target
  type` :codeterm:`f64` is perfect and lossless.

* :def_p:`fls_mjqvjt7v8a25`
  Casting an :term:`operand` of :term:`type` :codeterm:`f64` to :term:`target
  type` :codeterm:`f32` produces the closest possible :codeterm:`f32`
  :term:`value`. In addition, the :term:`type cast expression`

* :def_p:`fls_4fd5vkh0jt4`
  Prefers the nearest :term:`value` with an even least significant digit if
  exactly halfway between two floating point numbers.

* :def_p:`fls_2etd73f8jg2n`
  Produces :codeterm:`f32::INFINITY` of the same sign as the :term:`value`
  of the :term:`operand` when the :term:`value` of the :term:`operand` causes
  overflow.

.. rubric:: Examples

:def_p:`fls_vdxkpvmpwl3s`
See :p:`6.4.1. <fls_ltflbfba9d5r>` for the declaration of ``answer``.

.. code-block:: text

   answer as f64

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

:def_p:`fls_nhgexeu2h6wi`
An :term:`assignment expression` is an :term:`expression` that assigns the
:term:`value` of a :term:`value operand` to an :term:`assignee operand`.

:def_p:`fls_bsjw6f4a3wol`
An :term:`assignee operand` is the target :term:`operand` of an
:term:`assignment expression`.

:def_p:`fls_uinh05sslxeo`
A :term:`value operand` is an :term:`operand` that supplies the :term:`value`
that is assigned to an :term:`assignee operand` by an :term:`assignment
expression`.

:def_p:`fls_kh6rp9e0wwl`
An :term:`assignee operand` shall denote a :term:`mutable assignee expression`.

:def_p:`fls_3wragak9hglw`
A :term:`value operand` shall denote a :term:`value expression`.

:def_p:`fls_qengy157fa4a`
The :term:`type` of an :term:`assignment expression` is the :term:`unit type`.

:def_p:`fls_bwwtgqprbxrm`
The :term:`value` of an :term:`assignment expression` is the :term:`unit value`.

Basic Assignment
^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_uhcodvq75nlr`
A :term:`basic assignment` is an :term:`assignment expression` that is not a
:term:`destructuring assignment`.

.. rubric:: Dynamic Semantics

:def_p:`fls_esn5ceoldta`
The :term:`evaluation` of a :term:`basic assignment` proceeds as follows:

#. :def_p:`fls_t8eqzc64ivin`
   The :term:`value operand` is evaluated.

#. :def_p:`fls_b0mqcn5fejhk`
   The :term:`assignee operand` is evaluated.

#. :def_p:`fls_9i0ctuo099bp`
   The :term:`value` denoted by the :term:`assignee operand` is :term:`dropped`,
   unless the :term:`assignee operand` denotes an uninitialized :term:`binding`
   or an uninitialized :term:`field` of a :term:`binding`.

#. :def_p:`fls_hc01gtvlxba`
   The :term:`value` of the :term:`value operand` is :term:`copied` or
   :term:`moved` into the place of the :term:`assignee operand`.

.. rubric:: Examples

.. code-block:: text

   this = 42

Destructuring Assignment
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_2eheo4yo2orm`
A :term:`destructuring assignment` is an :term:`assignment expression` where the
:term:`assignee operand` is either an :term:`array expression`, a :term:`struct
expression`, a :term:`tuple expression`, or a :term:`union expression`.

:def_p:`fls_z8c3b9s9de3x`
The :term:`assignee operand` of a :term:`destructuring assignment` corresponds
to an :term:`assignee pattern` according to its kind, as follows:

* :def_p:`fls_du5eybf8mocy`
  A :term:`place expression` corresponds to an :term:`identifier pattern` with
  a unique :term:`identifier` and without :term:`keyword` ``ref``, keyword
  ``mut``, or a :term:`bound pattern`.

* :def_p:`fls_q90ikfi7ewoi`
  An :term:`underscore expression` corresponds to an :term:`underscore pattern`.

* :def_p:`fls_uydzlfc4hjbx`
  A :term:`tuple expression` corresponds to a :term:`tuple pattern` with all the
  :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\ s.

* :def_p:`fls_fa14yfvxsbx3`
  A :term:`tuple struct expression` corresponds to a :term:`tuple struct
  pattern` with all the :term:`subexpression`\ s lowered to their corresponding
  :term:`pattern`\ s.

* :def_p:`fls_hj6srmzbobid`
  A :term:`struct expression` corresponds to a :term:`struct pattern` with all
  the :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\
  s.

* :def_p:`fls_c4pto819yc8j`
  A :term:`unit struct expression` corresponds to a :term:`unit struct pattern`.

* :def_p:`fls_vqb89rkkjw81`
  A :term:`slice expression` corresponds to a :term:`slice pattern` with all the
  :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\ s.

* :def_p:`fls_vqj7ljrrd7wi`
  A :term:`full range expression` corresponds to a :term:`rest pattern` if
  inside a :term:`slice expression`, otherwise this is a static error.

:def_p:`fls_4bb07tn28ivw`
The :term:`pattern` that corresponds to a :term:`destructuring assignment` shall
be :term:`irrefutable`.

:def_p:`fls_g80a92tr2ser`
A :term:`destructuring assignment` is equivalent to a :term:`block expression`
of the following form:

* :def_p:`fls_u0iqhbw37xvq`
  The first :term:`statement` is a :term:`let statement` with its
  :term:`pattern` equivalent to the lowered :term:`assignee pattern` and its
  :term:`initialization expression` equivalent to the :term:`value operand`.

* :def_p:`fls_wsfhd3udt6fq`
  Then each bound :term:`identifier` of the :term:`assignee pattern` is an
  :term:`assignment expression` used as a :term:`statement`, as follows:

* :def_p:`fls_ll6h6qcaos65`
  The bound :term:`identifier` becomes the :term:`value operand` of the new
  :term:`assignment expression`, and

* :def_p:`fls_ajbdn54qe6wc`
  The corresponding :term:`expression` from the :term:`assignee operand` of the
  :term:`destructuring assignment` becomes the :term:`assignee operand` of the
  new :term:`assignment expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_l4u5hhw8tnvs`
The :term:`evaluation` of a :term:`destructuring assignment` proceeds as
follows:

#. :def_p:`fls_dd62w8c9n9sd`
   The :term:`value operand` is evaluated.

#. :def_p:`fls_jqu2u6mdccgi`
   The :term:`assignee operand` is evaluated by evaluating its :term:`operand`\
   s in a left-to-right order.

#. :def_p:`fls_n7nuj1lvpspc`
   Each :term:`value` denoted by the :term:`assignee operand` is :term:`dropped`
   in left-to-right order, unless the :term:`assignee operand` denotes an
   uninitialized :term:`binding` or an uninitialized field of a :term:`binding`.

#. :def_p:`fls_qb8u5skn8bc4`
   The :term:`value` of the :term:`value operand` is :term:`copied` or
   :term:`moved` into the place of the :term:`assignee operand`.

.. rubric:: Examples

.. code-block:: text

   (four, two) = (4, 2)

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

:def_p:`fls_3bu3g8o5nopc`
A :term:`compound assignment expression` is an expression that first computes
a :term:`value` from two :term:`operand`\ s and then assigns the value to an
:term:`assigned operand`.

:def_p:`fls_w2hbhb989yr4`
A :term:`bit and assignment expression` is a :term:`compound assignment
expression` that uses bit and arithmetic.

:def_p:`fls_ak4g5112jkl`
A :term:`bit or assignment expression` is a :term:`compound assignment
expression` that uses bit or arithmetic.

:def_p:`fls_lkjwyy78m2vx`
A :term:`bit xor assignment expression` is a :term:`compound assignment
expression` that uses bit exclusive or arithmetic.

:def_p:`fls_pkzj0uigfcgm`
A :term:`division assignment expression` is a :term:`compound assignment
expression` that uses division.

:def_p:`fls_ndlv3k9uclz2`
A :term:`multiplication assignment expression` is a :term:`compound assignment
expression` that uses multiplication.

:def_p:`fls_fbp5dojti27r`
A :term:`remainder assignment expression` is a :term:`compound assignment
expression` that uses remainder division.

:def_p:`fls_oy9ur11k78t`
A :term:`shift left assignment expression` is a :term:`compound assignment
expression` that uses bit shift left arithmetic.

:def_p:`fls_s7rey2bndfei`
A :term:`shift right assignment expression` is a :term:`compound assignment
expression` that uses bit shift right arithmetic.

:def_p:`fls_7l7v7vigw3fu`
A :term:`subtraction assignment expression` is a :term:`compound assignment
expression` that uses subtraction.

:def_p:`fls_dvy201zd6oym`
An :term:`assigned operand` is the target :term:`operand` of a :term:`compound
assignment expression`.

:def_p:`fls_9v09ayi2azpe`
A :term:`modifying operand` is an :term:`operand` that supplies the
:term:`value` that is used in the calculation of a :term:`compound assignment
expression`.

:def_p:`fls_row7saf53vwd`
An :term:`assigned operand` shall denote a :term:`mutable assignee expression`.

:def_p:`fls_gulql06xp9f3`
A :term:`modifying operand` shall denote a :term:`value expression`.

:def_p:`fls_xmgcdw9yhb55`
The :term:`type` of a :term:`compound assignment` is the :term:`unit type`.

:def_p:`fls_yeh6mvyvb4dp`
The :term:`value` of a :term:`compound assignment` is the :term:`unit value`.

:def_p:`fls_657knnsobdyu`
The :term:`type` of the :term:`assigned operand` of an :term:`addition
assignment` shall implement the :codeterm:`core::ops::AddAssign` trait where the
type of the right operand is the trait implementation type parameter.

:def_p:`fls_m942dwwmr2cl`
The :term:`type` of the :term:`assigned operand` of a :term:`bit and assignment`
shall implement the :codeterm:`core::ops::BitAndAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_np33oqrz33mp`
The :term:`type` of the :term:`assigned operand` of a :term:`bit or assignment`
shall implement the :codeterm:`core::ops::BitOrAssign` :term:`trait` where
the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_atdpr8be2o2r`
The :term:`type` of the :term:`assigned operand` of a :term:`bit xor assignment`
shall implement the :codeterm:`core::ops::BitXorAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_fbgwb3pdfgz`
The :term:`type` of the :term:`assigned operand` of a :term:`division
assignment` shall implement the :codeterm:`core::ops::DivAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_8tbxq95x06yt`
The :term:`type` of the :term:`assigned operand` of a :term:`multiplication
assignment` shall implement the :codeterm:`core::ops::MulAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_9oy9zo3x3fy3`
The :term:`type` of the :term:`assigned operand` of a :term:`remainder
assignment` shall implement the :codeterm:`core::ops::RemAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_pdgj2xekdead`
The :term:`type` of the :term:`assigned operand` of a :term:`shift left
assignment` shall implement the :codeterm:`core::ops::ShlAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_4uoi6k8r7mvc`
The :term:`type` of the :term:`assigned operand` of a :term:`shift right
assignment` shall implement the :codeterm:`core::ops::ShrAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_fjaz4m90cagr`
The :term:`type` of the :term:`assigned operand` of a :term:`subtraction
assignment` shall implement the :codeterm:`core::ops::SubAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

.. rubric:: Dynamic Semantics

:def_p:`fls_eesn9kuylim`
The :term:`evaluation` of a :term:`compound assignment` proceeds as follows:

#. :def_p:`fls_4nnqz4etisgw`
   If the :term:`type`\ s of both :term:`operand`\ s are :term:`primitive type`\
   s, then

   #. :def_p:`fls_a2g4hs15jpiu`
      The :term:`modifying operand` is evaluated.

   #. :def_p:`fls_kuet16jp6ps9`
      The :term:`assigned operand` is evaluated.

   #. :def_p:`fls_hovju0sni9gr`
      The appropriate :term:`function` is invoked as indicated below.

#. :def_p:`fls_ngimpabunzis`
   Otherwise

   #. :def_p:`fls_4sbpfi12frwe`
      The :term:`assigned operand` is evaluated.

   #. :def_p:`fls_n5ds6ydgckvo`
      The :term:`modifying operand` is evaluated.

   #. :def_p:`fls_xjdu0y1slsg9`
      The appropriate :term:`function` is invoked as indicated below.

:def_p:`fls_ijfmnnrdlu8n`
For an :term:`addition assignment`, ``core::ops::AddAssign::add_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_6x7j9x354pkb`
For a :term:`bit and assignment`, ``core::ops::BitAndAssign::bitand_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_h2cpbz2t74hy`
For a :term:`bit or assignment`, ``core::ops::BitOrAssign::bitor_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_whj50spxz3bh`
For a :term:`bit xor assignment`, ``core::ops::BitXorAssign::bitxor_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_d1cxq1zbt5fq`
For a :term:`division assignment`, ``core::ops::DivAssign::div_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_48i245an2449`
For a :term:`multiplication assignment`, ``core::ops::MulAssign::mul_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_69wr03rt0ali`
For a :term:`remainder assignment`, ``core::ops::RemAssign::rem_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_9d970yfwmj2d`
For a :term:`shift left assignment`, ``core::ops::ShlAssign::shl_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_p9687v3xckps`
For a :term:`shift right assignment`, ``core::ops::ShrAssign::shr_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_8j408kckzzud`
For a :term:`subtraction assignment`, ``core::ops::SubAssign::sub_assign(&mut
assigned_operand, modifying_operand)`` is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_uywamh3nzl6p`
It is undefined behavior for an :term:`addition assignment`, a :term:`division
assignment`, a :term:`multiplication assignment`, a :term:`remainder
assignment`, or a :term:`subtraction assignment` to cause overflow with
:term:`value`\ s of :term:`numeric type`\ s.

.. rubric:: Examples

.. code-block:: text

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

Underscore Expressions
----------------------

.. rubric:: Syntax

.. syntax::

   UnderscoreExpression ::=
       $$_$$

.. rubric:: Legality Rules

:def_p:`fls_pydmv629vfuu`
An :term:`underscore expression` is an :term:`expression` that acts as a
placeholder in a :term:`destructuring assignment`.

:def_p:`fls_wms3dbwjwyu4`
An :term:`underscore expression` shall appear in the :term:`assigned operand` of
a :term:`destructuring statement`.

.. rubric:: Examples

.. code-block:: text

   let pair = (1, 2);
   let mut second = 0;
   (_, second) = pair;

Parenthesized Expressions
-------------------------

.. rubric:: Syntax

.. syntax::

   ParenthesizedExpression ::=
       $$($$ Operand $$)$$

.. rubric:: Legality Rules

:def_p:`fls_jhazc75w5vj`
A :term:`parenthesized expression` is an :term:`expression` that groups other
:term:`expression`\ s.

:def_p:`fls_ew9y5vaseehy`
A :term:`parenthesized expression` is a :term:`place expression` when its
:term:`operand` is a :term:`place expression`.

:def_p:`fls_n4dhc0hvwwfk`
A :term:`parenthesized expression` is a :term:`value expression` when its
:term:`operand` is a :term:`value expression`.

:def_p:`fls_5d66h7naoup6`
The :term:`type` of a :term:`parenthesized expression` is the :term:`type` of
its :term:`operand`.

:def_p:`fls_xp9whcj2obk8`
The :term:`value` of a :term:`parenthesized expression` is the :term:`value` of
its :term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_2po52gv0m021`
The :term:`evaluation` of a :term:`parenthesized expression` evaluates its
:term:`operand`.

.. rubric:: Examples

.. code-block:: text

   (1 + 2) * 3

Array and Array Index Expressions
---------------------------------

Array Expressions
~~~~~~~~~~~~~~~~~

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

:def_p:`fls_ya9res33oxt6`
An :term:`array expression` is an :term:`expression` that constructs an
:term:`array`.

:def_p:`fls_fwtd3b10veiw`
An :term:`array element constructor` is an :term:`array expression` that lists
all elements of the :term:`array` being constructed.

:def_p:`fls_81jf78m5uga4`
An :term:`array repetition constructor` is an :term:`array expression` that
specifies how many times an element is repeated in the :term:`array` being
constructed.

:def_p:`fls_3y69y9ga4at7`
A :term:`repeat operand` is an :term:`operand` that specifies the element being
repeated in an :term:`array repetition constructor`.

:def_p:`fls_2l9objtb23zn`
A :term:`size operand` is an :term:`operand` that specifies the size of an
:term:`array` or an :term:`array type`.

:def_p:`fls_l0gbcyybzdv0`
An :term:`array expression` is a :term:`value expression`.

:def_p:`fls_by21pey7k423`
The :term:`type`\ s of the :term:`operand`\ s of an :term:`array element
constructor` shall be :term:`unifiable`.

:def_p:`fls_x2xu2pynxy1u`
If the :term:`size operand` is greater than one, then the :term:`type` of
the :term:`repeat operand` shall implement the :codeterm:`core::copy::Copy`
:term:`trait` or the :term:`repeat operand` shall be a :term:`path expression`
resolving to a :term:`constant`.

:def_p:`fls_qplsh3pdqitq`
The :term:`type` of the :term:`size operand` shall be :term:`type`
:codeterm:`usize`.

:def_p:`fls_9gmnjvs83d8o`
The :term:`value` of the :term:`size operand` shall be a :term:`constant
expression`.

:def_p:`fls_wmsekin1gd2y`
The :term:`type` of an :term:`array expression` is ``[T; N]``, where ``T`` is
the element type and ``N`` is the size of the array. The :term:`size` of an
:term:`array` is determined as follows:

* :def_p:`fls_2gto5kp9bjw8`
  If the :term:`array expression` appears with an :term:`array element
  constructor`, then the :term:`size` is the number of :term:`operand`\ s in the
  :term:`array element constructor`.

* :def_p:`fls_guop34ayjw2`
  Otherwise the :term:`size` is the :term:`value` of :term:`size operand`.

:def_p:`fls_aj6tbe54v5jl`
The :term:`value` of an :term:`array expression` is the constructed
:term:`array`.

.. rubric:: Dynamic Semantics

:def_p:`fls_t52in1kkyli3`
The :term:`evaluation` of an :term:`array expression` with an :term:`array
element constructor` evaluates its :term:`operand`\ s in left-to-right order.

:def_p:`fls_1kj8nlc5eb8a`
The :term:`evaluation` of an :term:`array expression` with an :term:`array
repetition constructor` proceeds as follows:

#. :def_p:`fls_f3izbkm8607z`
   If the :term:`value` of the :term:`size operand` is greater than zero, then:

   #. :def_p:`fls_qbyysx30pjzs`
      If the :term:`repeat operand` denotes a :term:`constant`, the
      :term:`repeat operand` is evaluated once and its :term:`value` is
      :term:`copied` :term:`size operand`'s :term:`value` times.

   #. :def_p:`fls_1m0laldldh7j`
      Otherwise the :term:`repeat operand` is evaluated :term:`size operand`'s
      :term:`value` times.

#. :def_p:`fls_5cs68nm54l31`
   Otherwise the :term:`repeat operand` is evaluated once.

.. rubric:: Examples

.. code-block:: text

   [1, 2, 3]
   ["one", "two", "three",]

:def_p:`fls_p7hcqryal5cm`
Two dimensional array.

.. syntax::


   [[0, 0], [0, 1], [1, 0], [1, 1]]


:def_p:`fls_izlpt6100gvk`
An array of nine 42s.

.. syntax::


   [42; 9]

Array and Slice Indexing Expressions
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ArrayIndexExpression ::=
       IndexedArrayOperand $$[$$ IndexingOperand $$]$$

   IndexedArrayOperand ::=
       Operand

   IndexingOperand ::=
       Operand

.. rubric:: Legality Rules

:def_p:`fls_42ijvuqqqlvh`
An :term:`array index expression` is an :term:`expression` that indexes into an
:term:`array` or a :term:`slice`.

:def_p:`fls_pc0c22asgzvw`
An :term:`indexed array operand` is an :term:`operand` which indicates the
:term:`array` or :term:`slice` being indexed into by an :term:`array index
expression`.

:def_p:`fls_ff3sgpldn52o`
An :term:`indexing operand` is an :term:`operand` which specifies the index of
the :term:`array` or :term:`slice` being indexed into by an :term:`array index
expression`.

:def_p:`fls_w96p9oyv5mqt`
An :term:`array index expression` is a :term:`constant expression` if the
:term:`indexing operand` is a :term:`constant expression`.

:def_p:`fls_u9sl7h4i8hqu`
The :term:`type` of the :term:`indexing operand` is the :term:`generic
parameter` of the :codeterm:`core::ops::Index` implementation of the
:term:`type` of the :term:`indexed array operand`.

:def_p:`fls_98qeczwv7fmy`
If the :term:`indexed array operand` is evaluated in a :term:`value expression
context`, then

* :def_p:`fls_jxdiknkwglak`
  The :term:`array index expression` is a :term:`value expression`.

* :def_p:`fls_sb2b8gszzaxq`
  The :term:`type` of the :term:`indexed array operand` shall implement the
  :codeterm:`core::ops::Index` :term:`trait`.

* :def_p:`fls_issaykiuha75`
  The :term:`type` of the :term:`array index expression` is ``&T``, where ``T``
  is :term:`associated type` :codeterm:`core::ops::Index::Output`.

:def_p:`fls_y3sduoma6q9v`
If the :term:`indexed array operand` is :term:`mutable` and the :term:`array
index expression` is evaluated in a :term:`mutable place expression context`,
then

* :def_p:`fls_pjmoo8mjgxz3`
  The :term:`array index expression` is a :term:`mutable place expression`.

* :def_p:`fls_ld7lbvqms5i6`
  The :term:`type` of the :term:`indexed array operand` shall implement the
  :codeterm:`core::ops::IndexMut` :term:`trait`.

* :def_p:`fls_nw705fpon79b`
  The :term:`type` of the :term:`array index expression` is ``&mut T``, where
  ``T`` is the element type of the :term:`indexed array operand`'s :term:`type`.

:def_p:`fls_fouu0z3jwoad`
The :term:`value` of an :term:`array index expression` is the indexed memory
location.

.. rubric:: Dynamic Semantics

:def_p:`fls_6sgj0ltt21i`
The :term:`evaluation` of an :term:`array index expression` proceeds as follows:

#. :def_p:`fls_e5l4y3dy69xi`
   The :term:`indexed array operand` is evaluated.

#. :def_p:`fls_fza3omn8yw7s`
   The :term:`indexing operand` is evaluated.

#. :def_p:`fls_ehamppbq4gmg`
   If the :term:`array index expression` is evaluated as a :term:`mutable place
   expression`, then :term:`expression` ``*core::ops::IndexMut::index_mut(&mut
   indexed_array_operand, inexing_operand)`` is evaluated.

#. :def_p:`fls_i68oxj659hc1`
   Otherwise :term:`expression`
   ``*core::ops::Index::index(&indexed_array_operand, inexing_operand)`` is
   evaluated.

.. rubric:: Examples

.. code-block:: text

   let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
   a[1][2]

:def_p:`fls_esvpmh6razg3`
Evaluates to 6.

Tuple Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   TupleExpression ::=
       $$($$ TupleInitializerList? $$)$$
   TupleInitializerList ::=
       ExpressionList

.. rubric:: Legality Rules

:def_p:`fls_87rp1hfwvjel`
A :term:`tuple expression` is an :term:`expression` that constructs a
:term:`tuple`.

:def_p:`fls_581y6jq1eyn8`
A :term:`tuple initializer` is an :term:`operand` that provides the
:term:`value` of a :term:`tuple field` in a :term:`tuple expression`.

:def_p:`fls_3dmehkxewz6d`
A :term:`tuple expression` is a :term:`value expression`.

:def_p:`fls_ljz3sxmfzflm`
The :term:`type` of a :term:`tuple expression` is ``(T1, T2, ..., TN)``, where
``T1`` is the :term:`type` of the first :term:`tuple initializer`, ``T2`` is
the :term:`type` of the second :term:`tuple initializer`, and ``TN`` is the
:term:`type` of the ``N``-th :term:`tuple initializer`.

:def_p:`fls_k2aznqo9j49p`
The :term:`value` of a :term:`tuple expression` is ``(V1, V2, ..., VN)``, where
``V1`` is the :term:`value` of the first :term:`tuple initializer`, ``V2`` is
the :term:`value` of the second :term:`tuple initializer`, and ``VN`` is the
:term:`value` of the ``N``-th :term:`tuple initializer`.

:def_p:`fls_fgthjiu980rr`
The :term:`value` of a :term:`tuple expression` without any :term:`tuple
initializer`\ s is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_waf55yd3mpsq`
The :term:`evaluation` of a :term:`tuple expression` evaluates its :term:`tuple
initializer`\ s in left-to-right order.

.. rubric:: Examples

.. code-block:: text

   ()
   (1.2, 3.4)
   ("hello", 42i16, true)

Struct Expressions
------------------

.. rubric:: Syntax

.. syntax::

   StructExpression ::=
       RecordStructConstructor
     | TupleStructConstructor
     | UnitStructConstructor

   RecordStructConstructor ::=
       ConstructionType $${$$ RecordStructInitializer? $$}$$

   RecordStructInitializer ::=
       BaseInitializer
     | RecordStructFieldInitializerList
     | RecordStructIndexedFieldInitializerList

   BaseInitializer ::=
       $$..$$ Operand

   RecordStructFieldInitializerList ::=
       RecordStructFieldInitializer ($$,$$ RecordStructFieldInitializer)* ($$,$$ BaseInitializer | $$,$$?)

   RecordStructFieldInitializer ::=
       NamedInitializer
     | ShorthandInitializer

   NamedInitializer ::=
       Identifier : Expression

   ShorthandInitializer ::=
       Identifier

   RecordStructIndexedFieldInitializerList ::=
       IndexedInitializer (, IndexedInitializer)* ($$,$$ BaseInitializer | $$,$$?)

   IndexedInitializer ::=
       TupleIndex : Operand

   TupleStructConstructor ::=
       ConstructionType $$($$ TupleStructFieldInitializerList? $$)$$

   TupleStructFieldInitializerList ::=
       PositionalInitializer (, PositionalInitializer)* ,?

   PositionalInitializer ::=
       Operand

   UnitStructConstructor ::=
       ConstructionType

   ConstructionType ::=
       PathInExpression

.. rubric:: Legality Rules

:def_p:`fls_jc45bpgsfnmr`
A :term:`struct expression` is an :term:`expression` that constructs a
:term:`struct` or a :term:`union`.

:def_p:`fls_fan2lkcxk7bg`
A :term:`record struct constructor` is a :term:`struct expression` that
constructs a :term:`record struct`.

:def_p:`fls_1e1rgq9k6n1i`
A :term:`tuple struct constructor` is a :term:`struct expression` that
constructs a :term:`tuple struct`.

:def_p:`fls_9yi0cxjnjlx9`
A :term:`union constructor` is a :term:`struct expression` that constructs
a :term:`union`.

:def_p:`fls_lowrnb3nk318`
A :term:`unit struct constructor` is a :term:`struct expression` that constructs
a :term:`unit struct`.

:def_p:`fls_lb2wr5o9sjm7`
A :term:`base initializer` is a :term:`construct` that specifies a
:term:`struct` to be used as a base for construction in a :term:`struct
expression`.

:def_p:`fls_gfxzhlqlkwmq`
An :term:`indexed initializer` is a :term:`construct` that specifies the index
and initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_auimxoels34d`
A :term:`named initializer` is a :term:`construct` that specifies the name and
initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_9qhnzeoox7o7`
A :term:`positional initializer` is a :term:`construct` that specifies the
initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_1gs6rdyz8as0`
A :term:`shorthand initializer` is a :term:`construct` that specifies the
:term:`name` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_b6idto4dffmr`
The :term:`construction type` indicates the :term:`type` of the :term:`struct`
being constructed by a :term:`struct expression`.

:def_p:`fls_i6dj8n6jmt2a`
A :syntax:`RecordStructConstructor` without a
:syntax:`RecordStructIndexedFieldInitializerList` is a :term:`record struct
constructor`.

:def_p:`fls_fn1tjfjcn8zp`
A :syntax:`UnitStructConstructor` and a :syntax:`RecordStructConstructor`
without a :syntax:`RecordStructInitializer` are :term:`unit struct constructor`\
s.

:def_p:`fls_ub9cu2w16so9`
A :syntax:`TupleStructConstructor` and a :syntax:`RecordStructConstructor`
without a :syntax:`RecordStructFieldInitializerList` are :term:`tuple struct
constructor`\ s.

:def_p:`fls_a1c89b6cb6ya`
A :syntax:`RecordStructConstructor` with a
:syntax:`RecordStructFieldInitializerList` is a :term:`union constructor`.

:def_p:`fls_2pgvvgrs72it`
A :term:`struct expression` is a :term:`value expression`.

:def_p:`fls_fez5ukkeoac6`
The :term:`type` of a :term:`struct expression` is the :term:`construction
type`.

:def_p:`fls_yz7wlyye9e3x`
The :term:`value` of a :term:`struct expression` is the :term:`struct` or
:term:`union` in construction.

:def_p:`fls_sazanondyx35`
The :term:`type` of a :term:`base initializer` is the :term:`type` of its
:term:`operand`. The :term:`type` of a :term:`base initializer` shall be the
same as the :term:`construction type`.

.. rubric:: Dynamic Semantics

:def_p:`fls_yy6qrnohm34`
The :term:`evaluation` of a :term:`struct expression` evaluates its
:term:`operand`\ s in a left-to-right order.

Record Struct Construction
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_cn4qaxx68auy`
The :term:`construction type` of a :term:`struct constructor` shall resolve to a
:term:`struct type`.

:def_p:`fls_gyponpgtgz91`
A :term:`named initializer` matches a :term:`field` of the :term:`construction
type` when its :term:`identifier` and the :term:`name` of the :term:`field` are
the same.

:def_p:`fls_dwn0sqraaifm`
The :term:`type` of a :term:`named initializer` and the :term:`type` of the
matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_aqd0u86gtk6q`
The :term:`value` of a :term:`named initializer` is the :term:`value` of its
:term:`expression`.

:def_p:`fls_i2ip2s14clr2`
A :term:`named initializer` that matches a :term:`field` is referred to as a
:def_term:`matched named initializer`.

:def_p:`fls_ex7vtlyaor8d`
A :term:`shorthand initializer` matches a :term:`field` of the
:term:`construction type` when its :term:`identifier` and the :term:`name` of
the :term:`field` are the same.

:def_p:`fls_2p4542ffuu8x`
The :term:`type` of a :term:`shorthand initializer` and the :term:`type` of the
matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_cdgpb5lgmk7l`
The :term:`value` of a :term:`shorthand initializer` is the :term:`value` of
its :term:`identifier`.

:def_p:`fls_2wbtuiugiuu4`
A :term:`shorthand initializer` that matches a :term:`field` is referred to as a
:def_term:`matched shorthand initializer`.

:def_p:`fls_6at70p40nzdx`
A :term:`shorthand initializer` is equivalent to a :term:`named initializer`
where both the :term:`identifier` and the :term:`expression` of the :term:`named
initializer` denote the :term:`identifier` of the :term:`shorthand initializer`.

:def_p:`fls_b5xkmxd0mqzq`
For each :term:`field` of the :term:`construction type`, the :term:`record
struct constructor` shall either:

* :def_p:`fls_4mftx1nnv3z`
  Contain a :term:`matched named initializer`, or

* :def_p:`fls_jj4x358nm3do`
  Contain a :term:`matched shorthand initializer`, or

* :def_p:`fls_t34yselx5psr`
  Have a :syntax:`RecordStructInitializer` with a :term:`base initializer` or a
  :syntax:`RecordStructFieldInitializerList` with a :term:`base initializer`.

:def_p:`fls_3gav2vg20xgi`
The :term:`value` of a :term:`field` of a :term:`struct` in construction shall
be either:

* :def_p:`fls_eclbzmm2fyx3`
  The :term:`value` of a :term:`matched named initializer`, or

* :def_p:`fls_yhtz73hx66r3`
  The :term:`value` of a :term:`matched shorthand initializer`, or

* :def_p:`fls_hm6uwir33qte`
  The :term:`value` of the corresponding :term:`field` of the :term:`struct`
  indicated by the :term:`base initializer`, where the :term:`value` is either
  :term:`copied` or :term:`moved`.

.. rubric:: Examples

.. code-block:: text

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


:def_p:`fls_44xrpe2k6d6p`
Matched named initializer.

.. syntax::


       name: "Bob".to_string(),

:def_p:`fls_8piw0m60trwg`
Matched shorthand initializer.

.. code-block:: text

       age,

:def_p:`fls_lrjhfc1mnx4a`
Base initializer, equivalent to ``alice.occupation`` and ``alice.compensation``.

.. code-block:: text

       .. alice
   };

Tuple Struct Construction
~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_ciwtgajyevdk`
The :term:`construction type` of a :term:`tuple struct constructor` shall
resolve to a :term:`tuple struct type`.

:def_p:`fls_6t6r55rnjn6n`
A :term:`tuple struct constructor` shall contain one :term:`positional
initializer` for each :term:`field` of the :term:`construction type`.

:def_p:`fls_tofndxu1cwam`
A :term:`positional initializer` matches a :term:`field` of the
:term:`construction type` when the position of the :term:`positional
initializer` and the position of the :term:`field` in the :term:`construction
type` are the same.

:def_p:`fls_wm1zje7mtqht`
The :term:`type` of the :term:`operand` of a :term:`positional initializer` and
the :term:`type` of the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_i8d8mly7wbx7`
The :term:`value` of a :term:`positional initializer` is the :term:`value` of
its :term:`operand`.

:def_p:`fls_wwjecv46x8p8`
A :term:`positional initializer` that matches a :term:`field` is referred to as
a :def_term:`matched positional initializer`.

:def_p:`fls_v59g8ki099ma`
The :syntax:`RecordStructIndexedFieldInitializerList` of a :term:`record struct
constructor` shall:

* :def_p:`fls_wc5090dwk0zf`
  Contain an :term:`indexed initializer` for each :term:`field` of the
  :term:`construction type`, covering all indices of the :term:`construction
  type`, or

* :def_p:`fls_v6c2mpue9v65`
  Have a :syntax:`RecordStructInitializer` with a :term:`base initializer`
  or a :syntax:`RecordStructIndexedFieldInitializerList` with a :term:`base
  initializer`\ ``.``

:def_p:`fls_491ix17yzb6k`
An :term:`indexed initializer` matches a :term:`field` of the
:term:`construction type` when the :term:`tuple index` of the :term:`indexed
initializer` resolves to a valid position of a :term:`field` in the
:term:`construction type`.

:def_p:`fls_kwt27bhyfe39`
The :term:`type` of the :term:`operand` of an :term:`indexed initializer` and
the :term:`type` of the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_ixzyz4np4piy`
The :term:`value` of an :term:`indexed initializer` is the :term:`value` of
its :term:`operand`.

:def_p:`fls_da507av76xq6`
An :term:`indexed initializer` that matches a :term:`field` is referred to as a
:def_term:`matched indexed initializer`.

:def_p:`fls_54rkrj5ajo6`
The :term:`value` of a :term:`field` of a :term:`tuple in construction` is
either:

* :def_p:`fls_r7bzxwxd6364`
  The :term:`value` of a :term:`matched indexed initializer`, or

* :def_p:`fls_mopq56aafcee`
  The :term:`value` of a :term:`matched positional initializer`, or

* :def_p:`fls_kihfh0h7o8nj`
  The :term:`value` of the corresponding :term:`field` of the :term:`tuple
  indicated` by the :term:`base initializer`, where the :term:`value` is either
  :term:`copied` or :term:`moved`.

.. rubric:: Examples

.. syntax::


   struct Point3D (
       f64,
       f64,
       f64
   );

   let positional_point = Point3D (
       0.0,
       1.1,
       2.2
   );

   let indexed_point = Point3D {


:def_p:`fls_voqgzdvwh9k5`
Matched indexed initializer.

.. code-block:: text


       1: 1.1,

:def_p:`fls_gjs024orub2v`
Base initializer, equivalent to ``origin.0`` and ``origin.2``.

.. code-block:: text

       .. origin
   };

Union Construction
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_6oi54di0tndy`
The :term:`construction type` of a :term:`union constructor` shall resolve to a
:term:`union type`.

:def_p:`fls_x8yd82t572hc`
The :syntax:`RecordStructFieldInitializerList` of a :term:`union constructor`
shall contain exactly one :syntax:`RecordStructFieldInitializer` and no
:term:`base initializer`.

:def_p:`fls_paqsgracxc7h`
For the single :term:`field` of the :term:`construction type`, a :term:`unit
constructor` shall either:

* :def_p:`fls_cedzp2z2fk69`
  Contain a :term:`matched named initializer`, or

* :def_p:`fls_ymiajqhy4v43`
  Contain a :term:`matched shorthand initializer`.

.. rubric:: Examples

.. code-block:: text

   union Union {
   	int: u32,
   	float: f32
   }

   let it = Union { int: 0 };
   let it = Union { float: 0.0 };

Unit Struct Construction
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_fyzdvmc4yuzr`
The :term:`construction type` of a :term:`unit struct constructor` shall resolve
to a :term:`unit struct type`.

.. rubric:: Examples

.. syntax::


   struct Empty;

   let unit1 = Empty;
   let unit2 = Empty{};

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

:def_p:`fls_fvgfx17ossd9`
A :term:`call expression` is an :term:`expression` that invokes a
:term:`function`.

:def_p:`fls_jvz5z3eqxb39`
An :term:`argument operand` is an :term:`operand` which is used as an argument
in a :term:`call expression` or a :term:`method call expression`.

:def_p:`fls_7ql1c71eidg8`
A :term:`call operand` is the :term:`function` being invoked by a :term:`call
expression`.

:def_p:`fls_4t6imtiw6kzt`
A :term:`callee type` is either a :term:`function item type`, a
:term:`function pointer type`, or a :term:`type` that implements any
of the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`, or
:codeterm:`core::ops::FnOnce` :term:`trait`\ s.

:def_p:`fls_aafrvlmiwfon`
The :term:`call operand` is subject to :term:`auto dereferencing` until a
:term:`callee type` is found, otherwise this is a static error.

:def_p:`fls_d8rewso3dm6r`
An :term:`adjusted call operand` is a :term:`call operand` with possible
:term:`auto dereferencing` adjustments.

:def_p:`fls_bu6i3mcvnbin`
The :term:`type` of a :term:`call expression` is the :term:`return
type` of the :term:`invoked function` or :term:`associated type`
:codeterm:`core::ops::FnOnce::Output`.

:def_p:`fls_8ljrgdept7s8`
A :term:`call expression` whose :term:`callee type` is either an :term:`external
function item type`, an :term:`unsafe function item type` or an :term:`unsafe
function pointer type` shall require :term:`unsafe context`.

:def_p:`fls_7p6zrjbpj0kl`
The :term:`value` of a :term:`call expression` is determined as follows:

* :def_p:`fls_yrr1s0tucgvh`
  If the :term:`callee type` is a :term:`function item type` or a
  :term:`function pointer type`, then the :term:`value` is the result of
  invoking the corresponding :term:`function` with the :term:`argument operand`\
  s.

* :def_p:`fls_s3q3sej1hgho`
  If the :term:`callee type` implements the :codeterm:`core::ops::Fn`
  :term:`trait`, then the :term:`value` is the result of invoking
  ``core::ops::Fn::call(adjusted_call_operand, argument_operand_tuple)``,
  where ``adjusted_call_operand`` is the :term:`adjusted call operand`, and
  ``argument_operand_tuple`` is a :term:`tuple` that wraps the :term:`argument
  operand`\ s.

* :def_p:`fls_cu2ubdm3tfwb`
  If the :term:`call operand` implements the :codeterm:`core::ops::FnMut`
  :term:`trait`, then the :term:`value` is the result of invoking
  ``core::ops::FnMut::call_mut(adjusted_call_operand, argument_operand_tuple),``
  where ``adjusted_call_operand`` is the :term:`adjusted call operand`, and
  ``argument_operand_tuple`` is a :term:`tuple` that wraps the :term:`argument
  operand`\ s.

* :def_p:`fls_9bbewx1l7h5h`
  If the :term:`call operand` implements the :codeterm:`core::ops::FnOnce`
  :term:`trait`, then the :term:`value` is the result of
  invoking ``core::ops::FnOnce::call_once(adjusted_call_operand,
  argument_operand_tuple),`` where ``adjusted_call_operand`` is the
  :term:`adjusted call operand`, and ``argument_operand_tuple`` is a
  :term:`tuple` that wraps the :term:`argument operand`\ s.

.. rubric:: Dynamic Semantics

:def_p:`fls_ggr5i91vur0r`
The :term:`evaluation` of a :term:`call expression` proceeds as follows:

#. :def_p:`fls_hwalzgdidbfz`
   The :term:`call operand` is evaluated.

#. :def_p:`fls_p52mfvpadu7w`
   The :term:`argument operand`\ s are evaluated in left-to-right order.

#. :def_p:`fls_1cyo5qhbl1j9`
   If the :term:`adjusted call operand` is a :term:`function item type` or
   :term:`function pointer type`, then corresponding :term:`function` is
   invoked.

#. :def_p:`fls_nb0eqky2akzt`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::Fn` :term:`trait`, then
   ``core::ops::Fn::call(adjusted_call_operand, argument_operand_tuple)`` is
   invoked.

#. :def_p:`fls_9lt4wh9ql5ae`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::FnMut` :term:`trait`, then
   ``core::ops::FnMut::call_mut(adjusted_call_operand, argument_operand_tuple)``
   is invoked.

#. :def_p:`fls_ixebnlcccmit`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::FnOnce` :term:`trait`, then
   ``core::ops::FnOnce::call_once(adjusted_call_operand,
   argument_operand_tuple)`` is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_5yeq4oah58dl`
It is undefined behavior to call a :term:`function` with an :term:`ABI` other
than the :term:`ABI` the :term:`function` was defined with.

.. rubric:: Examples

.. code-block:: text

   let three: i32 = add(1, 2);

Method Call Expressions
-----------------------

.. rubric:: Syntax

.. syntax::

   MethodCallExpression ::=
       ReceiverOperand $$.$$ PathExpressionSegment $$($$ ArgumentOperandList? $$)$$

   ReceiverOperand ::=
       Operand

.. rubric:: Legality Rules

:def_p:`fls_b7i26954j1hc`
A :term:`method call expression` is an :term:`expression` that invokes a
:term:`method` of an :term:`object`.

:def_p:`fls_jx3ryre0xs88`
A :term:`receiver operand` is an :term:`operand` that denotes the :term:`value`
whose :term:`method` is being invoked by a :term:`method call expression`.

:def_p:`fls_y7bj7y6davlh`
A :term:`method call expression` is subject to :term:`method resolution`.

:def_p:`fls_11glzggtbgb3`
The :term:`type` of a :term:`method call expression` is the :term:`return type`
of the invoked :term:`method`.

:def_p:`fls_ljvj1f9fv085`
The :term:`value` of a :term:`method call expression` is the :term:`value`
returned by the invoked :term:`method`.

.. rubric:: Dynamic Semantics

:def_p:`fls_oxxk3snd7ya0`
The :term:`evaluation` of a :term:`method call expression` proceeds as follows:

#. :def_p:`fls_gmpq15g77o20`
   The :term:`receiver operand` is evaluated.

#. :def_p:`fls_pu0n9hakkym2`
   The :term:`argument operand`\ s are evaluated in left-to-right order.

#. :def_p:`fls_cawdkgvvd1x6`
   The :term:`method` is invoked.

.. rubric:: Examples

.. code-block:: text

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

:def_p:`fls_hr8qvwlhd9ts`
A :term:`field access expression` is an :term:`expression` that accesses a
:term:`field` of an :term:`object`.

:def_p:`fls_s2vpn4ihenpe`
A :term:`container operand` is an :term:`operand` that indicates the
:term:`object` whose :term:`field` is selected in a :term:`field access
expression`.

:def_p:`fls_yeuayil6uxzx`
A :term:`field selector` is a :term:`construct` that selects the :term:`field`
to be accessed in a :term:`field access expression`.

:def_p:`fls_qqrconpa92i3`
A :term:`selected field` is a :term:`field` that is selected by a :term:`field
access expression`.

:def_p:`fls_t9xakmda134a`
A :term:`field access expression` with an :syntax:`IndexedFieldSelector` is
referred to as an :def_term:`indexed field access`.

:def_p:`fls_dch5i39ycw7s`
A :term:`field access expression` with a :syntax:`NamedFieldSelector` is
referred to as a :def_term:`named field access`.

:def_p:`fls_1l92izxtm1t8`
A :term:`field access expression` is a :term:`place expression`.

:def_p:`fls_1bbbw0qj0h0q`
A :term:`field access expression` is a :term:`mutable place expression` when its
:term:`container operand` is :term:`mutable`.

:def_p:`fls_fovs9il2h9xg`
The :term:`type` of a :term:`field access expression` is the :term:`type` of the
:term:`selected field`.

:def_p:`fls_r1b4n12i93pg`
The :term:`value` of a :term:`field access expression` is the :term:`value` of
the :term:`selected field`.

.. rubric:: Dynamic Semantics

:def_p:`fls_6uzouesw2sod`
The :term:`evaluation` of a :term:`field access expression` evaluates its
:term:`container operand`.

Named Field Access
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_kddnnz8uc15b`
Reading the :term:`selected field` of a :term:`union` shall require
:term:`unsafe context`.

:def_p:`fls_an3no949lvfw`
Writing to the :term:`selected field` of a :term:`union` where the :term:`type`
of the :term:`selected field` implements the :codeterm:`core::marker::Copy`
:term:`trait` or the :codeterm:`core::mem::ManuallyDrop` :term:`trait` shall
require :term:`safe context`.

:def_p:`fls_t6xmsm2nk1bc`
Writing to and then reading from the :term:`selected field` of a :term:`union`
subject to :term:`attribute` :codeterm:`repr` is equivalent to invoking
:term:`function` ``core::mem::transmute<write_type, read_type>(field_bits)``
where ``write_type`` is the :term:`type` used at the time of writing the
:term:`selected field`, ``read_type`` is the :term:`type` used at the time of
reading the :term:`selected field`, and ``field_bits`` is the bit representation
of the :term:`selected field`.

.. rubric:: Undefined Behavior

:def_p:`fls_sdnafipirg8w`
It is undefined behavior when the :term:`type` of the :term:`container operand`
is a :term:`union type` and the :term:`selected field` contains invalid data.

.. rubric:: Examples

:def_p:`fls_x27yayh4z787`
See :p:`6.8.1. <fls_hv4grs2tcuiw>` for the declaration of ``alice``.

.. code-block:: text

   alice.name

Indexed Field Access
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_zexojym4ak6f`
The :term:`decimal literal` of an :term:`indexed field access` shall denote a
valid index of a :term:`field` of the :term:`container operand`'s :term:`type`.

.. rubric:: Examples

:def_p:`fls_dimto84ifanr`
The following indexed field access evaluates to ``42``.

.. code-block:: text

   ("hello", 42i16, true).1

Closure Expressions
-------------------

.. rubric:: Syntax

.. syntax::

   ClosureExpression ::=
       $$move$$? $$|$$ ClosureParameterList? $$|$$ (ClosureBody | ClosureBodyWithReturnType)

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

:def_p:`fls_2d141c9a0yui`
A :term:`closure expression` is an :term:`expression` that defines a
:term:`closure type`.

:def_p:`fls_srbl7ptknjyk`
A :term:`closure body` is a :term:`construct` that represents the executable
portion of a :term:`closure expression`.

:def_p:`fls_oey0ivaiu1l`
A :term:`closure body` denotes a new :term:`control flow boundary`.

:def_p:`fls_fg8lx0yyt6oq`
A :term:`closure body` is subject to :term:`capturing`.

:def_p:`fls_yn30xuejcfxo`
The :term:`type` of a :term:`closure expression` is the anonymous unique
:term:`closure type` defined by it.

:def_p:`fls_sje6cdvifgv5`
The :term:`value` of a :term:`closure expression` is the :term:`value` of
the anonymous unique :term:`closure type` instantiated with the selected
:term:`capture`\ s.

.. rubric:: Dynamic Semantics

:def_p:`fls_f59fw17gsasn`
The :term:`evaluation` of a :term:`closure expression` proceeds as follows:

#. :def_p:`fls_7w15ccc1zzxl`
   An anonymous :term:`value` of an anonymous unique :term:`closure type` is
   created.

#. :def_p:`fls_b8w9y73pvdnm`
   The :term:`closure body` is evaluated.

.. rubric:: Examples

.. code-block:: text

   fn do_ten_times<F>(consumer: F) where F: Fn(i32) {
       for times in 0 .. 10 {
           consumer(times);
       }
   }

   do_ten_times(|value: i32| { println!("value: {}", value)});

Loop Expressions
----------------

.. rubric:: Syntax

.. syntax::

   LoopExpression ::=
       Label? LoopContent

   LoopContent ::=
       ForLoopExpression
     | InfiniteLoopExpression
     | WhileLetLoopExpression
     | WhileLoopExpression

.. rubric:: Legality Rules

:def_p:`fls_y1d8kd1bdlmx`
A :term:`loop expression` is an :term:`expression` that evaluates a :term:`block
expression` continuously as long as some criterion holds true.

:def_p:`fls_eg93m93gvwal`
An :term:`anonymous loop` is a :term:`loop expression` without a :term:`label`.

:def_p:`fls_phpoq9ho8f1v`
A :term:`named loop` is a :term:`loop expression` with a :term:`label`.

:def_p:`fls_b314wjbv0zwe`
The :term:`type` of a :term:`loop expression` is determined as follows:

* :def_p:`fls_rpedapxnv8w3`
  If the :term:`loop expression` does not contain a :term:`break expression`,
  then the :term:`type` is the :term:`never type`.

* :def_p:`fls_wf11yp1jwf53`
  If the :term:`loop expression` contains at least one :term:`break expression`,
  then the :term:`type` is the :term:`unified type` of the :term:`break type`\ s
  of all :term:`break expression`\ s.

:def_p:`fls_q3qpcf2fz7h`
The :term:`value` of a :term:`loop expression` is determined as follows:

* :def_p:`fls_2ulbzmuuny3g`
  If the :term:`loop expression` does not contain a :term:`break expression`,
  then the :term:`value` is the :term:`unit value`.

* :def_p:`fls_99imks9hj3kp`
  If the :term:`loop expression` contains at least one :term:`break expression`,
  then the :term:`value` is the :term:`break value` of the :term:`break
  expression` that broke out of the :term:`loop expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_aw6qczl4zpko`
A :term:`loop expression` is :term:`terminated` when its :term:`block
expression` is no longer evaluated.

For Loops
~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ForLoopExpression ::=
       $$for$$ Pattern $$in$$ SubjectExpression BlockExpression

.. rubric:: Legality Rules

:def_p:`fls_1bh2alh37frz`
A :term:`for loop expression` is a :term:`loop expression` that continues to
evaluate its :term:`block expression` as long as its :term:`subject expression`
yields a :term:`value`.

:def_p:`fls_fkgbin6ydkm4`
The :term:`type` of a :term:`subject expression` shall implement the
:codeterm:`core::iter::IntoIterator` :term:`trait`.

.. rubric:: Dynamic Semantics

:def_p:`fls_kuxo0on3vit6`
The :term:`evaluation` of a :term:`for loop expression` of the form

.. code-block:: text

   'label: for pattern in subject_expression {
       /* loop body */
   }

:def_p:`fls_2lrzrtjhsdes`
is equivalent to the :term:`evaluation` of the following :term:`block
expression`:

.. code-block:: text

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

.. code-block:: text

   let favorite_fruits = &["apples", "pears", "strawberries"];

   for fruit in favorite_fruits {
       println!("I like eating {}.", fruit);
   }

Infinite Loops
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   InfiniteLoopExpression ::=
       $$loop$$ BlockExpression

.. rubric:: Legality Rules

:def_p:`fls_p11qw6mtxlda`
An :term:`infinite loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` indefinitely unless :term:`terminated`
with a :term:`break expression` or a :term:`return expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_w4tj5gofwih1`
The :term:`evaluation` of an :term:`infinite loop expression` proceeds as
follows:

#. :def_p:`fls_pg3r6nyl865`
   The :term:`block expression` is evaluated.

#. :def_p:`fls_lp15ilkul2uv`
   Control restarts the :term:`evaluation` of the :term:`infinite loop
   expression`.

.. rubric:: Examples

.. code-block:: text

   loop {
       println!("I am alive!");
   }

While Loops
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WhileLoopExpression ::=
       $$while$$ IterationExpression BlockExpression

   IterationExpression ::=
       Expression

.. rubric:: Legality Rules

:def_p:`fls_ajby242tnu7c`
A :term:`while loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` as long as its :term:`iteration
expression` holds true.

:def_p:`fls_13hmhzqz82v6`
An :term:`iteration expression` is an :term:`expression` that provides the
criterion of a :term:`while loop expression`.

:def_p:`fls_d7ofrq3777kq`
The :term:`type` of an :term:`iteration expression` shall be :term:`type`
:codeterm:`bool`.

.. rubric:: Dynamic Semantics

:def_p:`fls_1i7hm645h7ox`
The :term:`evaluation` of a :term:`while loop expression` proceeds as follows:

#. :def_p:`fls_5x0du3u1jwd3`
   The :term:`iteration expression` is evaluated.

#. :def_p:`fls_23uluvhhoct6`
   If the :term:`iteration expression` evaluated to ``true``, then:

   #. :def_p:`fls_k7g4cac93617`
      The :term:`block expression` is evaluated.

   #. :def_p:`fls_j08k3brdpgno`
      Control restarts the :term:`evaluation` of the :term:`while loop
      expression`.

.. rubric:: Examples

.. code-block:: text

   let mut counter = 0;

   while counter < 5 {
       counter += 1;
       println("iteration {}", counter);
   }

While Let Loops
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WhileLetLoopExpression ::=
       $$while$$ $$let$$ Pattern $$=$$ SubjectLetExpression BlockExpression

.. rubric:: Legality Rules

:def_p:`fls_fmdlyp9r9zl7`
A :term:`while let loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` as long as its :term:`subject
let expression` yields a :term:`value` that can be matched against its
:term:`pattern`.

.. rubric:: Dynamic Semantics

:def_p:`fls_z2ht5iaat5ag`
The :term:`evaluation` of a :term:`while let loop expression` of the form

.. code-block:: text

   'label: let pattern = subject_let_expression {
       /* loop body */
   }

:def_p:`fls_pacf1uavh1qt`
shall be equivalent to the :term:`evaluation` the following :term:`infinite
loop`:

.. code-block:: text

   'label: loop {
       match subject_let_expression {
           pattern => { /* loop body */ },
           _ => break
       }
   }

.. rubric:: Examples

.. code-block:: text

   let mut favorite_animals = vec!["cats", "dogs", "otters"];

   while let Some(animal) = favorite_animals.pop() {
       println!("I like petting {}", animal);
   }

Loop Labels
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   LabelIndication ::=
       $$'$$ NonKeywordIdentifier

.. rubric:: Legality Rules

:def_p:`fls_tx5u743391h7`
A :term:`label indication` is a :term:`construct` that indicates a
:term:`label`.

:def_p:`fls_7hc8yboeaho0`
A :term:`label indication` shall indicate a :term:`label` of an enclosing
:term:`named loop` that does not pass a :term:`control flow boundary` in order
to reach the enclosing :term:`named loop`.

Break Expressions
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BreakExpression ::=
       $$break$$ LabelIndication? Operand?

.. rubric:: Legality Rules

:def_p:`fls_i5ko1t2wbgxe`
A :term:`break expression` is an :term:`expression` that terminates a
:term:`loop expression`.

:def_p:`fls_jiykbp51909f`
A :term:`break expression` shall appear within a :term:`loop expression`.

:def_p:`fls_7frvr2nm2mcj`
The :term:`label indication` of a :term:`break expression` shall resolve to the
:term:`label` of an enclosing :term:`named loop`.

:def_p:`fls_ghxns2nggffj`
A :term:`break expression` without a :term:`label indication` is associated with
the innermost enclosing :term:`loop expression`.

:def_p:`fls_54d5uydc87td`
A :term:`break expression` with a :term:`label indication` is associated with
a :term:`named loop` whose :term:`label` is indicated by the :term:`label
indication`.

:def_p:`fls_6x15ig8drne8`
A :term:`break expression` shall have an :term:`operand` only when it is
associated with an :term:`infinite loop`.

:def_p:`fls_dnnq1zym8ii0`
The :term:`type` of a :term:`break expression` is the :term:`never type`.

:def_p:`fls_1wdybpfldj7q`
:term:`Break type` is the :term:`type` of the :term:`operand` of a :term:`break
expression`.

:def_p:`fls_8yore99adr22`
The :term:`break type` is determined as follows:

* :def_p:`fls_60imbzwg3e2x`
  If the :term:`break expression` lacks an :term:`operand`, then the
  :term:`break type` is the :term:`unit type`.

* :def_p:`fls_l0c05wa9q97w`
  If the :term:`break expression` has an :term:`operand`, then the :term:`break
  type` is the :term:`type` of its :term:`operand`.

:def_p:`fls_bgd7d5q69q0g`
:term:`Break value` is the :term:`value` of the :term:`operand` of a
:term:`break expression`.

:def_p:`fls_yb8jv4mkmki0`
The :term:`break value` is determined as follows:

* :def_p:`fls_d7l1y2qbe8br`
  If the :term:`break expression` lacks an :term:`operand`, then the
  :term:`break value` is the :term:`unit value`.

* :def_p:`fls_56szfyilc06`
  If the :term:`break expression` has an :term:`operand`, then the :term:`break
  value` is the :term:`value` of its :term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_jnpx8mx1oa7n`
The :term:`evaluation` of a :term:`break expression` proceeds as follows:

#. :def_p:`fls_l2kp8mw6bjj0`
   The :term:`operand` is evaluated.

#. :def_p:`fls_2nmadhe3ismj`
   All enclosing :term:`loop expression`\ s upto and including the associated
   :term:`loop expression` are :term:`terminated`.

.. rubric:: Examples

:def_p:`fls_32fwis9pxh77`
The following break expression terminates both the inner and the outer loop.

.. code-block:: text

   'outer: loop {
       'inner: loop {
           break 'outer;
       }
   }

Continue Expressions
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ContinueExpression ::=
       $$continue$$ LabelIndication?

.. rubric:: Legality Rules

:def_p:`fls_wzs6kz9ffqzt`
A :term:`continue expression` shall appear within a :term:`loop expression`.

:def_p:`fls_r5ke7b9n7k3s`
A :term:`continue expression` without a :term:`label indication` is associated
with the innermost enclosing :term:`loop expression`.

:def_p:`fls_ckm6i9c3s6j8`
A :term:`continue expression` with a :term:`label indication` is associated
with a :term:`named loop` whose :term:`label` is indicated by the :term:`label
indication`.

:def_p:`fls_d0bmw8xiw5nk`
The :term:`type` of a :term:`continue expression` is the :term:`never type`.

:def_p:`fls_b7m0h2i3mot1`
The :term:`value` of a :term:`continue expression` is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_vmyuuptfnwek`
The :term:`evaluation` of a :term:`continue expression` proceeds as follows:

#. :def_p:`fls_gm74eo754rq9`
   If the :term:`continue expression` appears with a :term:`label indication`,
   then all enclosing :term:`loop expression`\ s upto and including the
   associated :term:`loop expression` are :term:`terminated`.

#. :def_p:`fls_gvuesa5ekeif`
   The :term:`evaluation` of the associated :term:`loop expression` is
   restarted.

.. rubric:: Examples

:def_p:`fls_767gv7zhqamh`
The following continue expression terminates and restarts ``game_loop``.

.. code-block:: text

   'game_loop: loop {
       if is_paused() {
           continue;
       }
       . . .
   }

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

:def_p:`fls_bi82rusji8g0`
A :term:`range expression` is an :term:`expression` that constructs a range.

:def_p:`fls_msyv4oyk5zp9`
A :term:`range expression low bound` is an :term:`operand` that specifies the
start of a range.

:def_p:`fls_f648uuxxh4vk`
A :term:`range expression high bound` is an :term:`operand` that specifies the
end of a range.

:def_p:`fls_9pl4629t54yq`
If a :term:`range expression` has two :term:`operand`\ s, then the :term:`type`\
s of the :term:`operand`\ s shall be :term:`unifiable`.

:def_p:`fls_xaumwogwbv3g`
A :term:`range-from expression` is a :term:`range expression` that specifies an
included :term:`range expression low bound`.

:def_p:`fls_exa2ufugnpgc`
The :term:`type` of a :term:`range-from expression` is
:codeterm:`core::ops::RangeFrom`.

:def_p:`fls_jqy0p155btca`
The :term:`value` of a :term:`range-from expression` is ``core::ops::RangeFrom {
start: range_expression_low_bound }``.

:def_p:`fls_ppustuqdji7b`
A :term:`range-from-to expression` is a :term:`range expression` that specifies
an included :term:`range expression low bound` and an excluded :term:`range
expression high bound`.

:def_p:`fls_ke2fpgodq84u`
The :term:`type` of a :term:`range-from-to expression` is
:codeterm:`core::ops::Range`.

:def_p:`fls_zb6jk6qykun6`
The :term:`value` of a :term:`range-from-to expression` is ``core::ops::Range {
start: range_expression_low_bound, end: range_expression_high_bound }``.

:def_p:`fls_x67xo25n0qlz`
A :term:`range-full expression` is a :term:`range expression` that covers the
whole range of a :term:`type`.

:def_p:`fls_m6n0gvg3ct1b`
The :term:`type` of a :term:`range-full expression` is
:codeterm:`core::ops::RangeFull`.

:def_p:`fls_yvh5cdgzevni`
The :term:`value` of a :term:`range-full expression` is ``core::ops::RangeFull
{}``.

:def_p:`fls_lh9my7g8oflq`
A :term:`range-inclusive expression` is a :term:`range expression` that
specifies an included :term:`range expression low bound` and an included
:term:`range expression high bound`.

:def_p:`fls_livflk52xaj9`
The :term:`type` of a :term:`range-inclusive expression` is
:codeterm:`core::ops::RangeInclusive`.

:def_p:`fls_vj213j9bj61y`
The :term:`value` of a :term:`range-inclusive expression` is
``core::ops::RangeInclusive::new(range_expression_low_bound,
range_expression_high_bound)``.

:def_p:`fls_5a1uivj19kob`
A :term:`range-to expression` is a :term:`range expression` that specifies an
excluded :term:`range expression high bound`.

:def_p:`fls_k611yoc8hk0n`
The :term:`type` of a :term:`range-to expression` is
:codeterm:`core::ops::RangeTo`.

:def_p:`fls_m0slikrulnvd`
The :term:`value` of a :term:`range-to expression` is ``core::ops::RangeTo {
end: range_expression_high_bound }``.

:def_p:`fls_1gc436ee1nzm`
A :term:`range-to-inclusive expression` is a :term:`range expression` that
specifies an included :term:`range expression high bound`.

:def_p:`fls_8sfjw83irpre`
The :term:`type` of a :term:`range-to-inclusive expression` is
:codeterm:`core::ops::RangeToInclusive`.

:def_p:`fls_5xw4opkbxhsc`
The :term:`value` of a :term:`range-to-inclusive expression` is
``core::ops::RangeToInclusive { end: range_expression_high_bound }``.

.. rubric:: Dynamic Semantics

:def_p:`fls_ehseim1p479z`
The :term:`evaluation` of a :term:`range expression` evaluates its
:term:`operand`\ s in left-to-right order.

.. rubric:: Examples

.. code-block:: text

   1 ..
   42 .. 86
   ..
   dawn ..= dusk
   ..= 5

If and If Let Expressions
-------------------------

If Expressions
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IfExpression ::=
       $$if$$ SubjectExpression BlockExpression ElseExpression?

   ElseExpression ::=
       $$else$$ (BlockExpression | IfExpression | IfLetExpression)

.. rubric:: Legality Rules

:def_p:`fls_2i4fbxbbvpf1`
An :term:`if expression` is an :term:`expression` that evaluates either
a :term:`block expression` or an :term:`else expression` depending on the
:term:`value` of its :term:`subject expression`.

:def_p:`fls_5azwlk7hav1k`
An :term:`else expression` is an :term:`expression` that represents either
a :term:`block expression`, an :term:`if expression`, or an :term:`if let
expression`.

:def_p:`fls_r7gzxo16esri`
The :term:`type` of the :term:`subject expression` of an :term:`if expression`
shall be :term:`type` :codeterm:`bool`.

:def_p:`fls_iv9t4nfs4f6w`
The :term:`type` of an :term:`if expression` is the :term:`type` of its
:term:`block expression`.

:def_p:`fls_i9sxf2q5jjqt`
The :term:`value` of an :term:`if expression` is the :term:`value` of its
:term:`block expression`.

:def_p:`fls_1e8qer6bh2f3`
The :term:`type` of an :term:`else expression` is the :term:`type` of its
:term:`block expression`, :term:`if expression`, or :term:`if let expression`.

:def_p:`fls_p5pjxk5xfcbx`
The :term:`value` of an :term:`else expression` is the :term:`value` of its
:term:`block expression`, :term:`if expression`, or :term:`if let expression`.

:def_p:`fls_mpq7gicosgkt`
The :term:`type` of an :term:`if expression` and the :term:`type` of an
:term:`else expression` shall be :term:`unifiable`.

.. rubric:: Dynamic Semantics

:def_p:`fls_yhlyzef9h97q`
The :term:`evaluation` of an :term:`if expression` proceeds as follows:

#. :def_p:`fls_w7lq4dkoyuf7`
   The :term:`subject expression` is evaluated.

#. :def_p:`fls_5udx9zyeg5ga`
   If the :term:`subject expression` evaluated to ``true``, then the
   :term:`block expression` is evaluated.

#. :def_p:`fls_67l4j48n6p7o`
   If the :term:`subject expression` evaluated to ``false`` and the :term:`if
   expression` has an :term:`else expression`, then the :term:`else expressio`\
   n is evaluated.

:def_p:`fls_e8gd5lzcaifw`
The :term:`evaluation` of an :term:`else expression` evaluates its :term:`block
expression`, :term:`if expression`, or :term:`if let expression`.

.. rubric:: Examples

.. code-block:: text

   if age <= 14 {
       println!("child");
   } else if age <= 24 {
       println!("youth");
   } else if age <=64 {
       println!("adult");
   } else {
       println!("senior");
   }

If Let Expressions
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IfLetExpression ::=
       $$if$$ $$let$$ Pattern $$=$$ SubjectLetExpression BlockExpression ElseExpression?

.. rubric:: Legality Rules

:def_p:`fls_dsrjup2umr9`
An :term:`if let expression` is an :term:`expression` that evaluates either a
:term:`block expression` or an :term:`else expression` depending on whether its
:term:`pattern` can be matched against its :term:`subject let expression`.

:def_p:`fls_4vyrufo4qdeg`
The :term:`type` of an :term:`if let expression` is the :term:`type` of its
:term:`block expression`.

:def_p:`fls_qfnwwvzxsl3`
The :term:`value` of an :term:`if let expression` is the :term:`value` of its
:term:`block expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_ijo73wtz1sy`
The :term:`evaluation` of an :term:`if let expression` of the form

.. code-block:: text

   if let pattern = subject_let_expression {
       /* body */
   }

:def_p:`fls_qeho5iqiy59`
is equivalent to the :term:`evaluation` of the following :term:`match
expression`:

.. code-block:: text

   match subject_let_expression {
       pattern => { /* body */ },
       _ => ()
   }

:def_p:`fls_nhngr8y850dt`
The :term:`evaluation` of an :term:`if let expression` of the form

.. code-block:: text

   if let pattern = subject_let_expression {
       /* body */
   } else {
       /* else */
   }

:def_p:`fls_8fg2ufaxjkv5`
is equivalent to the :term:`evaluation` of the following :term:`match
expression`:

.. code-block:: text

   match subject_let_expression {
       pattern => { /* body */ },
       _ => { /* else */ }
   }

.. rubric:: Examples

.. code-block:: text

   let dish = ("Ham", "Eggs");

   if let ("Ham", side) = dish {
       println!("Ham is served with {}", side);
   }

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
       MatchArmMatcher $$=>$$ $$($$ ExpressionWithBlock $$,$$? | ExpressionWithoutBlock $$,$$ $$)$$

   FinalMatchArm ::=
       MatchArmMatcher $$=>$$ Operand $$,$$?

   MatchArmMatcher ::=
       OuterAttributeOrDoc* Pattern MatchArmGuard?

   MatchArmGuard ::=
       $$if$$ Operand

.. rubric:: Legality Rules

:def_p:`fls_ei4pbeksd1v8`
A :term:`match expression` is an :term:`expression` that tries to match one of
its multiple :term:`pattern`\ s against its :term:`subject expression` and if it
succeeds, evaluates an :term:`operand`.

:def_p:`fls_l45i24ikfavm`
A :term:`match arm` is a :term:`construct` that consists of a :term:`match arm
matcher` and a :term:`match arm body`.

:def_p:`fls_d9gerg12hm2d`
An :term:`intermediate match arm` is any non-:term:`final match arm` of a
:term:`match expression`.

:def_p:`fls_oj8dg28xw5yp`
A :term:`final match arm` is the last :term:`match arm` of a :term:`match
expression`.

:def_p:`fls_lrdrtedyz28i`
A :term:`match arm matcher` is a :term:`construct` that consists of a
:term:`pattern` and a :term:`match arm guard`.

:def_p:`fls_8wjdichfxp0y`
A :term:`match arm body` is the :term:`operand` of a :term:`match arm`.

:def_p:`fls_hs1rr54hu18w`
A :term:`match arm guard` is a :term:`construct` that provides additional
filtering to a :term:`match arm matcher`.

:def_p:`fls_s4483f30nwf`
A :term:`match expression` is a :term:`place expression` when its :term:`subject
expression` is a :term:`place expression`. When a :term:`match expression`
is a :term:`place expression`, the :term:`value` produced by evaluating its
:term:`subject expression` is :term:`copied` or :term:`moved`.

:def_p:`fls_9t5pmb9wzmpy`
A :term:`match expression` is a :term:`value expression` when its :term:`subject
expression` is a :term:`value expression`. When the :term:`match expression`
is a :term:`value expression`, the :term:`value` produced by evaluating its
:term:`subject expression` is captured in a :term:`temporary`.

:def_p:`fls_knv1affr2o8t`
The :term:`type` of the :term:`subject expression` and the :term:`type`\
s of all :term:`pattern`\ s of all :term:`match arm matcher`\ s shall be
:term:`unifiable`.

:def_p:`fls_bzhz5wjd90ii`
The :term:`type` of the :term:`operand` of a :term:`match arm guard` shall be
:term:`type` :codeterm:`bool`.

:def_p:`fls_17ag0wzdbxv6`
The :term:`type`\ s of all :term:`match arm bodies` shall be :term:`unifiable`.

:def_p:`fls_5w964phrru82`
The :term:`type` of a :term:`match expression` is the :term:`unified type` of
the :term:`type`\ s of the :term:`operand`\ s of all :term:`match arm`\ s.

:def_p:`fls_g6xyz0beps3o`
A :term:`match arm` is selected when its :term:`pattern` matches the
:term:`subject expression` and its :term:`match arm guard` (if any) evaluates
to ``true``.

:def_p:`fls_8dba4o5qg8js`
:term:`Match arm` selection happens in declarative order.

:def_p:`fls_e02um1gb89d0`
The :term:`pattern`\ s of all :term:`match arm`\ s taken together shall
exhaustively match the :term:`subject expression`'s :term:`type`.

:def_p:`fls_4sh2yrslszvb`
The :term:`value` of a :term:`match expression` is the :term:`value` of the
:term:`operand` of the selected :term:`match arm`.

.. rubric:: Dynamic Semantics

:def_p:`fls_g551l8r8yh6d`
The :term:`evaluation` of a :term:`match expression` proceeds as follows:

#. :def_p:`fls_y44jzkbv74bv`
   The :term:`subject expression` is evaluated.

#. :def_p:`fls_jwxykea99psw`
   Each :term:`match arm` is evaluated in declarative order as follows:

   #. :def_p:`fls_pgulnjeoxwtj`
      The :term:`match arm marcher` of the :term:`match arm` is evaluated.

   #. :def_p:`fls_2dg7wl68z7ar`
      If the :term:`match arm marcher` succeeds, then

      #. :def_p:`fls_yv11febo0kyb`
         The :term:`operand` of the :term:`match arm` is evaluated.

      #. :def_p:`fls_mvi9z1x836qu`
         Control stops the :term:`evaluation` of the :term:`match expression`.

   #. :def_p:`fls_81nnizrxgrsm`
      Otherwise control proceeds with the :term:`evaluation` of the next
      :term:`match arm`.

:def_p:`fls_4dv7x9nh2h4e`
The :term:`evaluation` of a :term:`match arm marcher` proceeds as follows:

#. :def_p:`fls_k7kliy101m0f`
   The :term:`pattern` of the :term:`match arm marcher` is evaluated.

#. :def_p:`fls_k68zkb6jv0vz`
   If the :term:`pattern` succeeds, then

   #. :def_p:`fls_gbb6wbmher5z`
      If the :term:`match arm marcher` has a :term:`match arm guard`, then

      #. :def_p:`fls_jl4av757yx8j`
         The :term:`match arm guard` is evaluated.

      #. :def_p:`fls_wkh5wztauwhu`
         If the :term:`match arm guard` evaluates to ``true``, then the
         :term:`match arm marcher` succeeds.

   #. :def_p:`fls_f5f0x8jstp1g`
      Otherwise the :term:`match arm marcher` fails.

#. :def_p:`fls_yk8l9zjh7i0d`
   Otherwise the :term:`match arm marcher` fails.

:def_p:`fls_sbtx1l6n2tp2`
The :term:`evaluation` of a :term:`match arm guard` evaluates its
:term:`operand`. A :term:`match arm guard` evaluates to ``true`` when its
:term:`operand` evaluates to ``true``, otherwise it evaluates to ``false``.

.. rubric:: Examples

.. code-block:: text

   fn quantify(number_of_things: i32) {
       match number_of_things {
           0 | 1 => println!("not many"),
           2 ..= 9 => println!("a few"),
           _ if number_of_things < 0 => println!("you owe me"),
           _ => println!("lots")
       }
   }

Return Expressions
------------------

.. rubric:: Syntax

.. syntax::

   ReturnExpression ::=
       $$return$$ Expression?

.. rubric:: Legality Rules

:def_p:`fls_u7jk4j8gkho`
A :term:`return expression` is an :term:`expression` that optionally yields a
:term:`value` and causes control flow to return to the caller.

:def_p:`fls_5v3j5ghhw8j8`
A :term:`return expression` shall appear within a :term:`control flow boundary`.

:def_p:`fls_m4e00bju2dy4`
The :term:`type` of a :term:`return expression` is determined as follows:

* :def_p:`fls_xpp027s2m7ue`
  If the :term:`return expression` has an :term:`operand`, then the :term:`type`
  is the :term:`type` of the :term:`operand`.

* :def_p:`fls_cqduumpsjfut`
  If the :term:`return expression` does not have an :term:`operand`, then the
  :term:`type` is the :term:`never type`.

:def_p:`fls_r610t5vsi7bx`
The :term:`value` of a :term:`return expression` is determined as follows:

* :def_p:`fls_njndlx2rps2k`
  If the :term:`return expression` has an :term:`operand`, then the
  :term:`value` is the :term:`value` of the :term:`operand`.

* :def_p:`fls_tjksia7prao1`
  If the :term:`return expression` does not have an :term:`operand`, then the
  :term:`value` is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_bqmwlona6l5w`
The :term:`evaluation` of a :term:`return expression` proceeds as follows:

#. :def_p:`fls_d9avvfi548t7`
   If the :term:`return expression` has an :term:`operand`, then

   #. :def_p:`fls_o3fc1z2mn8zc`
      The :term:`operand` is evaluated.

   #. :def_p:`fls_bbf54ukld7j9`
      The :term:`value` of the :term:`operand` is :term:`moved` into the
      designated output location of the enclosing control flow boundary.

#. :def_p:`fls_99ea30a5mulj`
   Control destroys the current activation frame.

#. :def_p:`fls_ubwj8uj6sbgt`
   Control is transferred to the caller frame.

.. rubric:: Examples

.. code-block:: text

   fn max(left: i32, right: i32) -> i32 {
       if left > right {
           return left;
       }
       return right;
   }

Await Expressions
-----------------

.. rubric:: Syntax

.. syntax::

   AwaitExpression ::=
       FutureOperand $$.$$ $$await$$

   FutureOperand ::=
       Operand

.. rubric:: Legality Rules

:def_p:`fls_sjz5s71hwm7l`
An :term:`await expression` is an :term:`expression` that polls a
:term:`future`, suspending the :term:`execution` of the :term:`future` until the
:term:`future` is ready.

:def_p:`fls_vhchgab59jvd`
A :term:`future operand` is an :term:`operand` whose :term:`future` is being
awaited by an :term:`await expression`.

:def_p:`fls_k9pncajmhgk1`
An :term:`await expression` shall appear within

* :def_p:`fls_xilkqy5piyh0`
  An :term:`async block expression`.

* :def_p:`fls_cr61i8so7cty`
  An :term:`async function`.

:def_p:`fls_9uw5pd7kbrx3`
The :term:`type` of a :term:`future operand` shall implement the
:codeterm:`core::future::Future` :term:`trait`.

:def_p:`fls_c6mxfzef2qop`
The :term:`type` of an :term:`await expression` is ``<_ as
core::future::Future>::Output``.

:def_p:`fls_l396mo6k9ox7`
The :term:`value` of an :term:`await expression` is the :term:`value` held by
:codeterm:`core::task::Poll::Ready`.

.. rubric:: Dynamic Semantics

:def_p:`fls_1ppywe40s62c`
The :term:`evaluation` of an :term:`await expression` proceeds as follows:

#. :def_p:`fls_eymcs2rgv3qw`
   The :term:`future operand` is evaluated to a :term:`temporary`.

#. :def_p:`fls_qshnnpirkasz`
   The :term:`temporary` is pinned using
   :codeterm:`core::pin::Pin::new_unchecked`.

#. :def_p:`fls_umevprl99y6c`
   The pinned :term:`temporary` is polled using
   :codeterm:`core::future::Future::poll`, passing in the
   :codeterm:`core::task::Context` of the current task.

#. :def_p:`fls_k76d8ady623m`
   If :codeterm:`core::future::Future::poll` returns
   :codeterm:`core::task::Poll::Pending`, then the current :term:`future`
   yields.

#. :def_p:`fls_frwtufwe8dya`
   If :codeterm:`core::future::Future::poll` returns
   :codeterm:`core::task::Poll::Ready`, then

   #. :def_p:`fls_u72ylhlmqge3`
      The :term:`value` held within is unwrapped.

   #. :def_p:`fls_tn3vwidct3ks`
      Control stops the evaluation of the :term:`await expression`.

.. rubric:: Examples

:def_p:`fls_la9boykzmfac`
**provide an example**

Expression Precedence
---------------------

.. rubric:: Legality Rules

:def_p:`fls_cwt7afsbgs7w`
Certain :term:`expression`\ s are subject to :term:`precedence` and
:term:`associativity`.

:def_p:`fls_ya23jjg5wjl`
:term:`Precedence` is the order by which :term:`expression`\ s are evaluated in
the presence of other :term:`expression`\ s.

:def_p:`fls_bezkcuwp5qol`
:term:`Associativity` is the order by which :term:`operand`\ s are evaluated
within a single :term:`expression`.

:def_p:`fls_48br7odx6nke`
The :term:`precedence` and :term:`associativity` of qualifying
:term:`expression`\ s are as follows:

.. list-table::

   * - .. rubric:: Expression
     - .. rubric:: Precedence
     - .. rubric:: Associativity
   * - :def_p:`fls_jtdnf0vmn6xt`
       :term:`Array expression`

       :def_p:`fls_pko7c3suw18p`
       :term:`Block expression`

       :def_p:`fls_5ernwvbjzd6d`
       :term:`Continue expression`

       :def_p:`fls_xc0bqiptpur4`
       :term:`If expression`

       :def_p:`fls_p1xm2f9xkyro`
       :term:`If let expression`

       :def_p:`fls_3hbf564rvp0k`
       :term:`Literal expression`

       :def_p:`fls_w62578xv5jkv`
       :term:`Loop expression`

       :def_p:`fls_o8hd0vlldsm9`
       :term:`Match expression`

       :def_p:`fls_u44p4fz3rkll`
       :term:`Parenthesized expression`

       :def_p:`fls_uj567walo9o8`
       :term:`Path expression`

       :def_p:`fls_ljs7zaa6n5d7`
       :term:`Struct expression`

       :def_p:`fls_bnxs8i53x3i9`
       :term:`Tuple expression`

       :def_p:`fls_r5acfjshejmh`
       :term:`Underscore expression`
     - :def_p:`fls_1i7atfw6z0c6`
       highest
     - :def_p:`fls_qoiys31tbe6p`
       none
   * - :def_p:`fls_qurz25skmryg`
       :term:`Method call expression`
     -
     - :def_p:`fls_ixo8jlfl90wj`
       none
   * - :def_p:`fls_ywqn5nixelkz`
       \ | :term:`Await expression`
       | :term:`Field access expression`
     -
     - :def_p:`fls_kha0vjr5shwe`
       left-to-right
   * - :def_p:`fls_k3ohh8k888c`
       \ | :term:`Array index expression`
       | :term:`Call expression`
     -
     - :def_p:`fls_kif1ywa2cw3x`
       none
   * - :def_p:`fls_41n6z85h1z47`
       :term:`Error propagation expression`
     -
     - :def_p:`fls_5radv61jswjq`
       none
   * - :def_p:`fls_f39rzauxrlcl`
       \ | :term:`Borrow expression`
       | :term:`Dereference expression`
       | :term:`Negation expression`
     -
     - :def_p:`fls_dcw9fdgvu8xb`
       none
   * - :def_p:`fls_djphr5mk0t6f`
       :term:`Type cast expression`
     -
     - :def_p:`fls_t4s6plcgrwfv`
       left-to-right
   * - :def_p:`fls_sif2aqky96j6`
       \ | :term:`Division expression`
       | :term:`Multiplication expression`

       :def_p:`fls_9igfpqwxex7l`
       :term:`Remainder expression`
     -
     - :def_p:`fls_e3loqntvo54t`
       left-to-right
   * - :def_p:`fls_d7x817v8xzea`
       :term:`Addition expression`

       :def_p:`fls_dfipk06akxgr`
       :term:`Subtraction expression`
     -
     - :def_p:`fls_96ojqb37wzud`
       left-to-right
   * - :def_p:`fls_1f5ibdkz3l51`
       :term:`Shift left expression`

       :def_p:`fls_52j8skuhqw9s`
       :term:`Shift right expression`
     -
     - :def_p:`fls_aw4n1ufi93ey`
       left-to-right
   * - :def_p:`fls_t1zqnab8a752`
       :term:`Bit and expression`
     -
     - :def_p:`fls_h9ck3hljx7w2`
       left-to-right
   * - :def_p:`fls_f6in3h5cj8i6`
       :term:`Bit xor expression`
     -
     - :def_p:`fls_mjbkpccftmk2`
       left-to-right
   * - :def_p:`fls_hxa1avitfvrq`
       :term:`Bit or expression`
     -
     - :def_p:`fls_6puftczanrgb`
       left-to-right
   * - :def_p:`fls_sy2xzzq06i0x`
       :term:`Comparison expression`
     -
     - :def_p:`fls_k248rk69wkny`
       requires parentheses
   * - :def_p:`fls_hish3qfmawd`
       :term:`Lazy and expression`
     -
     - :def_p:`fls_l5x6434yce6h`
       left-to-right
   * - :def_p:`fls_ruy7e6yccsqk`
       :term:`Lazy or expression`
     -
     - :def_p:`fls_mkpiht18rd1n`
       left-to-right
   * - :def_p:`fls_9qcm0dx9rolw`
       :term:`Range expression`
     -
     - :def_p:`fls_xt6ol3nlmyra`
       requires parentheses
   * - :def_p:`fls_1l3rgtm6o54s`
       :term:`Assignment expression`

       :def_p:`fls_9q1d9t3eoh1z`
       :term:`Compound assignment expression`
     -
     - :def_p:`fls_i7nmdvd04rca`
       right-to-left
   * - :def_p:`fls_hr4kokysrjop`
       :term:`Break expression`

       :def_p:`fls_mi2sttbjkqrh`
       :term:`Closure expression`

       :def_p:`fls_35v53lpzvzqu`
       :term:`Return expression`
     - :def_p:`fls_rkhos8dg9te6`
       lowest
     - :def_p:`fls_f245hom9s6m2`
       none

Capturing
---------

.. rubric:: Legality Rules

:def_p:`fls_iamnzlm430ef`
A :term:`capturing expression` is either an :term:`async block expression` or a
:term:`closure expression`.

:def_p:`fls_eca6tl7j0afx`
A :term:`capture target` is either a :term:`binding` or a :term:`field` of
a :term:`binding`.

:def_p:`fls_e70ywb8191h`
The :term:`capturing environment` of a :term:`capturing expression` consists
of all :term:`capture target`\ s that are defined outside the :term:`capture
expression`.

:def_p:`fls_1y2ttb466m9c`
:term:`Capturing` is the process of saving the :term:`capture target`\ s of a
:term:`capturing expression`'s :term:`capturing environment`.

:def_p:`fls_ip81lt2mm940`
A :term:`capturing target` requires :term:`capturing` when it is part of the
:term:`capturing expression`'s :term:`capture environment` and it is used by
the :term:`capturing expression`. Such a :term:`capturing target` is said to be
:def_term:`captured`.

:def_p:`fls_y9n1i4hbq8sf`
:term:`Capture mode` is the mechanism by which a :term:`capture target` is
captured.

:def_p:`fls_t695ps4lfh6z`
The :term:`capture mode` is determined based on the use of the :term:`capture
target` within the :term:`capturing expression`, as follows:

#. :def_p:`fls_6j8cr7d5zs1l`
   If the :term:`capturing expression` is subject to :term:`keyword` ``move``,
   then

   #. :def_p:`fls_dd8sc7y2vi3u`
      If the :term:`type` of the :term:`capture target` is a :term:`by copy
      type`, then the :term:`capture mode` is :term:`by copy mode`.

   #. :def_p:`fls_sq1wam8j1d0a`
      Otherwise the :term:`capture mode` is :term:`by move mode`.

#. :def_p:`fls_l8e98pyhm08g`
   Otherwise the :term:`capture mode` is determined based on the following
   precedence:

   #. :def_p:`fls_33hfay24hx8u`
      :term:`By immutable borrow mode`, if the :term:`capture target` is
      immutably borrowed.

   #. :def_p:`fls_wmxsd0i2yemf`
      :term:`By unique immutable borrow mode`, if the :term:`capture target` is
      a :term:`mutable reference` that is being modified.

   #. :def_p:`fls_lu779ufqhggl`
      :term:`By mutable borrow mode`, if the :term:`capture target` is mutably
      borrowed.

   #. :def_p:`fls_uqy5w9uc8gla`
      If the :term:`type` of the :term:`capture target` is a :term:`by copy
      type`, then the :term:`capture mode` is :term:`by copy mode`, otherwise it
      is :term:`by move mode`.

:def_p:`fls_wvob7114tfat`
A tool selects the first :term:`capture mode` that is compatible with the use of
the :term:`capture target`.

:def_p:`fls_c3hla8dqymvn`
The :term:`capture mode` dictates the :term:`capturing` of the :term:`capture
target` as follows:

* :def_p:`fls_60g4jyiphzs`
  If the :term:`capture mode` is :def_term:`by copy mode`, then the
  :term:`capture target` is transferred :term:`by copy`.

* :def_p:`fls_1juvkmh2aoyo`
  If the :term:`capture mode` is :def_term:`by move mode`, then the
  :term:`capture target` is transferred :term:`by move`.

* :def_p:`fls_ctvot5k6jsdx`
  If the :term:`capture mode` is :def_term:`by immutable borrow mode`, then the
  :term:`capture target` is immutably borrowed.

* :def_p:`fls_teyva6i05akb`
  If the :term:`capture mode` is :def_term:`by unique immutable borrow mode`,
  then the :term:`capture target` is uniquely immutably borrowed.

* :def_p:`fls_wik2g15r7vye`
  If the :term:`capture mode` is :def_term:`by mutable borrow mode`, then the
  :term:`capture target` is mutably borrowed.

