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

   SubjectExpression ::=

:def_p:`fls_92p74qxbdux8`
``    ``\ Any expression in category ``Expression``, except
``IndexedTupleStructExpression`` and ``RecordStructExpression``.

.. syntax::

   SubjectLetExpression ::=

:def_p:`fls_pws8eq5elmo5`
``    ``\ Any expression in category ``SubjectExpression``, except
``LazyBooleanExpression``.

.. rubric:: Legality Rules

:def_p:`fls_rmufrljw7ege`
An :term:`expression` is a :term:`construct` that produces a :term:`value`, and
may have side effects at run-time.

:def_p:`fls_lizijopk5515`
An :term:`expression-with-block` is an :term:`expression` whose structure
involves a :term:`block expression`.

:def_p:`fls_aynma1ci51mv`
An :term:`expression-without-block` is an :term:`expression` whose structure
does not involve a :term:`block expression`.

:def_p:`fls_cx70ms4dxh9r`
An :term:`operand` is an :term:`expression` nested within an :term:`expression`.

:def_p:`fls_9nx4z5eyy6aw`
A :term:`left operand` is an :term:`operand` that appears on the left-hand side
of a :term:`binary operator`.

:def_p:`fls_l8q6jvc5fhca`
A :term:`right operand` is an :term:`operand` that appears on the right-hand
side of a :term:`binary operator`.

:def_p:`fls_6cg3jr5t2ens`
A :term:`subject expression` is an :term:`expression` that controls :term:`for
loop`\ s, :term:`if expression`\ s, and :term:`match expression`\ s.

:def_p:`fls_tzxogrv42d5p`
A :term:`subject let expression` is an :term:`expression` that
controls :term:`if let expression`\ s and :term:`while let loop`\ s.

:def_p:`fls_1gim4ra5crct`
A :term:`constant expression` is an :term:`expression` that can be evaluated
statically. The following :term:`construct`\ s are :term:`constant expression`\
s as long as their :term:`operand`\ s are also :term:`constant expression`\ s
and do not involve :term:`type`\ s that require :term:`destruction`:

* :def_p:`fls_l15yk75sk3b2`
  :term:`Arithmetic expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_te2ifvfnffpn`
  :term:`Array expression`\ s,

* :def_p:`fls_v05n9figoh4k`
  :term:`Array index expression`\ s,

* :def_p:`fls_h50e68t42w3w`
  :term:`Assignment expression`\ s,

* :def_p:`fls_mjc51gilujh4`
  :term:`Bit expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_2pgmsrqnxu4b`
  :term:`Block expression`\ s,

* :def_p:`fls_c0nrb4h2jxn7`
  :term:`Closure expression`\ s that do not :term:`capture`,

* :def_p:`fls_tjmilikitvo2`
  :term:`Comparison expression`\ s of scalar types,

* :def_p:`fls_aiixormlibij`
  :term:`Compound assignment expression`\ s,

* :def_p:`fls_yd5thsd79q6u`
  :term:`Constant parameter`\ s,

* :def_p:`fls_9s9bintcqmm0`
  :term:`Dereference expression`\ s when the :term:`operand` is not of
  a :term:`raw pointer type`,

* :def_p:`fls_oaaktofns9rh`
  :term:`Expression statement`\ s,

* :def_p:`fls_x3yz9hsmm0rx`
  :term:`Field access expression`\ s,

* :def_p:`fls_fc93r9m6aawg`
  :term:`If expression`\ s,

* :def_p:`fls_wuq9mxkdvtoa`
  :term:`If let expression`\ s,

* :def_p:`fls_szo29anhxn0t`
  :term:`Infinite loop expression`\ s,

* :def_p:`fls_bazp3fjacy3b`
  :term:`Lazy boolean expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_upj66bq5rw40`
  :term:`Let statement`\ s,

* :def_p:`fls_5ahddupdy2ha`
  :term:`Literal expression`\ s,

* :def_p:`fls_t30r8od0zxdt`
  :term:`Match expression`\ s,

* :def_p:`fls_ttexlz445xsa`
  :term:`Negation expression`\ s of :term:`scalar type`\ s,

* :def_p:`fls_1i5qh265rkrh`
  :term:`Parenthesized expression`\ s,

* :def_p:`fls_gymgn3mb8oxh`
  :term:`Path expression`\ s that resolve to :term:`constant`\
  s, :term:`function`\ s, and :term:`static`\ s,

* :def_p:`fls_48gcj1p3qeu6`
  :term:`Range expression`\ s,

* :def_p:`fls_u5irsh33e5i`
  :term:`Shared borrow`\ s that do not involve :term:`type`\ s
  with :term:`interior mutability`,

* :def_p:`fls_pwbukef57qf0`
  :term:`Struct expression`\ s,

* :def_p:`fls_i7b0y89zyb0o`
  :term:`Tuple expression`\ s,

* :def_p:`fls_ey0ylj4vocpx`
  :term:`Type cast expression`\ s that are not :term:`pointer-to-address cast`\
  s,  :term:`function-pointer-to-address cast`\ s, and :term:`unsized cast`\ s
  that involve a :term:`trait object type`,

* :def_p:`fls_4ptfki2z0ft9`
  :term:`While let loop expression`\ s,

* :def_p:`fls_yr9uzrnv40xj`
  :term:`While loop expression`\ s.

:def_p:`fls_gurc3crz2dtv`
A :term:`constant context` is a :term:`construct` that requires
a :term:`constant expression`. The following :term:`construct`\ s
are :term:`constant context`\ s:

* :def_p:`fls_qpyjtm6sphms`
  The :term:`constant initializer` of a :term:`constant`,

* :def_p:`fls_o7loxmd4k0k9`
  The :term:`constant parameter` of a :term:`generic`,

* :def_p:`fls_nyssg5o3e8wm`
  The :term:`discriminant initializer` of a :term:`discriminant`,

* :def_p:`fls_dmoscfy2avp0`
  The :term:`size operand` of an :term:`array repetition constructor`,

* :def_p:`fls_5klhuld3g4f3`
  The :term:`size operand` of an :term:`array type`,

* :def_p:`fls_g7ux1rgglzk`
  The :term:`static initializer` of a :term:`static`.

:def_p:`fls_raypvrbk3qjn`
A :term:`place expression` is an :term:`expression` that represents a memory
location. The following :term:`expression`\ s are :term:`place expression`\ s:

* :def_p:`fls_5rehh9r7j9sn`
  :term:`Array expression`\ s,

* :def_p:`fls_rpmakjtqgt11`
  :term:`Dereference expression`\ s,

* :def_p:`fls_l9x8mpk0ue2`
  :term:`Field access expression`\ s,

* :def_p:`fls_4e21f69i9vhf`
  :term:`Parenthesized expression`\ s where the :term:`operand` is
  a :term:`place expression`,

* :def_p:`fls_wsdamwze5k0b`
  :term:`Path expression`\ s that resolve to a :term:`binding` or
  a :term:`static`.

:def_p:`fls_fks9gnu0o17`
A :term:`place expression context` is a :term:`construct` that requires
a :term:`place expression`. The following :term:`construct`\ s are :term:`place
expression context`\ s:

* :def_p:`fls_5jm2qjkoxenw`
  The :term:`indexed array operand` of an :term:`array index expression`,

* :def_p:`fls_xj6wqom6t0pw`
  The :term:`assignee operand` of an :term:`assignment expression` or
  a :term:`compound assignment expression`,

* :def_p:`fls_70azfjz3m9x2`
  The :term:`operand` of a :term:`borrow expression`,

* :def_p:`fls_8gpvldbtjprt`
  The :term:`operand` of a :term:`dereference expression`,

* :def_p:`fls_resz6ifgwn5u`
  The :term:`container operand` of :term:`field access expression`,

* :def_p:`fls_1w7i48nu0owe`
  The :term:`subject let expression` of an :term:`if let expression` or
  a :term:`while let loop expression`,

* :def_p:`fls_bcm5zf96ro0i`
  The initialization :term:`expression` of a :term:`let statement`,

* :def_p:`fls_7p5zp4f4vwm3`
  The :term:`subject expression` of :term:`a match expression,`

* :def_p:`fls_qgbu2ci0e36l`
  The :term:`base initializer` of a :term:`struct expression`,

* :def_p:`fls_mu13vxqv4sos`
  The :term:`operand` of an :term:`implicit borrow`.

:def_p:`fls_lpmwd3v9qf4d`
A :term:`place expression` can be moved out of when it denotes

* :def_p:`fls_tzdw69neidfr`
  A :term:`binding` which is not currently :term:`borrowed`, or

* :def_p:`fls_o4a7wh2h67o7`
  A :term:`field` of a :term:`place expression` that can be moved out of and
  does not implement the :codeterm:`core::ops::Drop` :term:`trait`, or

* :def_p:`fls_lekoje9g6t7y`
  A :term:`temporary` created for a :term:`value expression`.

:def_p:`fls_l7wpjtyc6qgv`
After a :term:`place expression` is moved out, the memory location it
represented is deinitialized and shall not be read from until reinitialized.

:def_p:`fls_lo4zakddjj8t`
A :term:`mutable place expression` is a :term:`place expression` whose
memory location can be modified. The following :term:`place expression`\ s
are :term:`mutable place expression`\ s:

* :def_p:`fls_xsax0j1hf7s7`
  An :term:`array expression` whose :term:`type` implements
  the :codeterm:`core::ops::IndexMut` :term:`trait`,

* :def_p:`fls_bg2dlb7vru1l`
  A :term:`dereference expression` whose :term:`type` is ``*mut T``,

* :def_p:`fls_q7ayrvbp3yjb`
  A :term:`dereference expression` of a :term:`field` or :term:`binding`
  whose :term:`type` is ``&mut T``,

* :def_p:`fls_jtagdpr7lnux`
  A :term:`dereference expression` whose :term:`type` implements
  the :codeterm:`core::ops::DerefMut` :term:`trait`,

* :def_p:`fls_scl0ifgs74l8`
  A :term:`field access expression` where the :term:`type` of
  the :term:`container operand` is :term:`mutable`,

* :def_p:`fls_v1o9o91k703z`
  A :term:`path expression` that resolves to a :term:`mutable binding` that is
  not currently borrowed,

* :def_p:`fls_lnid671za9z`
  A :term:`path expression` that resolves to a :term:`mutable static`,

* :def_p:`fls_i37c5lukclg`
  A :term:`temporary` created for a :term:`value expression`.

:def_p:`fls_mm3gonxd2lat`
An :term:`assignee expression` is an :term:`expression` that appears
as the :term:`left operand` of an :term:`assignment expression`. The
following :term:`expression`\ s are :term:`assignee expression`\ s:

* :def_p:`fls_6emyzjtwipxk`
  :term:`Place expression`\ s,

* :def_p:`fls_4qnf7itzpw8n`
  :term:`Underscore expression`\ s,

* :def_p:`fls_oyh7elv4efkl`
  :term:`Array expression`\ s of :term:`assignee expression`\ s,

* :def_p:`fls_u4fxv7cbia1`
  :term:`Tuple expression`\ s of :term:`assignee expression`\ s,

* :def_p:`fls_4v5s80lzn8cq`
  :term:`Struct expression`\ s of :term:`assignee expression`\ s,

* :def_p:`fls_516zen997s27`
  :term:`Tuple struct expression`\ s of :term:`assignee expression`\ s,

* :def_p:`fls_uqzluxgw4le`
  :term:`Unit struct expression`\ s.

:def_p:`fls_sl1u3cp8rwtc`
:term:`Parenthesized expression`\ s are allowed to appear anywhere
in :term:`assignee expression`\ s.

.. rubric:: Dynamic Semantics

:def_p:`fls_lvs0yplb2mu1`
:term:`Evaluation` is the process by which an :term:`expression` achieves its
runtime effects.

:def_p:`fls_xm46ns409rv`
The :term:`evaluation` of a :term:`place expression` in the context of
a :term:`value expression` or the :term:`evaluation` of a :term:`place
expression` that is bound *by value* in a :term:`pattern` proceeds as follows:

#. :def_p:`fls_sewpsbj99pa7`
   The :term:`place expression` denotes the :term:`value` held in that memory
   location.

#. :def_p:`fls_g1rj1wiqcds`
   If the :term:`type` of the held :term:`value` implements
   the :codeterm:`core::marker::Copy` :term:`trait`, then the held :term:`value`
   is copied.

#. :def_p:`fls_t7t8bywcmgbt`
   If the :term:`type` of the held :term:`value` implements
   the :codeterm:`core::marker::Sized` :term:`trait`, then the
   held :term:`value` is moved.

#. :def_p:`fls_qja61qipqonu`
   Otherwise the :term:`evaluation` results in a static error.

Literal Expressions
-------------------

.. rubric:: Syntax

.. syntax::

   LiteralExpression ::=
       Literal


.. rubric:: Legality Rules

:def_p:`fls_t2arr0q4lu1i`
A :term:`literal expression` is an :term:`expression` that denotes
a :term:`literal`.

:def_p:`fls_154bq8kn11bu`
A :term:`literal expression` is a :term:`value expression`.

:def_p:`fls_hs8f6gly1e5v`
The :term:`type` of a :term:`literal expression` is the :term:`type` of the
corresponding :term:`literal`.

:def_p:`fls_5b3ks0an2cqg`
The :term:`value` of a :term:`literal expression` is the :term:`value` of the
corresponding :term:`literal`.

.. rubric:: Dynamic Semantics

:def_p:`fls_9zvcqqxlpx6z`
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

:def_p:`fls_xu2qgd26n1pp`
A :term:`path expression` is an :term:`expression` that denotes a :term:`path`.

:def_p:`fls_c6iqt5kf6ux4`
A :term:`path expression` that resolves to a :term:`binding` or a :term:`static`
is a :term:`place expression`, otherwise it is a :term:`value expression`.

:def_p:`fls_4xn5d94l6e56`
A :term:`path expression` that resolves to a :term:`mutable static` shall
require :term:`unsafe context`.

:def_p:`fls_9cbd3dlgkfe9`
The :term:`type` of a :term:`path expression` is the :term:`type` of
the :term:`entity` that it resolved to.

:def_p:`fls_3vecv802mjd`
The :term:`value` of a :term:`path expression` is the :term:`entity` that it
resolved to.

.. rubric:: Dynamic Semantics

:def_p:`fls_t8nw4qrj7y3b`
The :term:`evaluation` of a :term:`path expression` has no effect.

.. rubric:: Examples

.. code-block:: text

   globals::STATIC_VARIABLE
   Vec::<i32>::push

6.3. Block Expressions
----------------------

.. rubric:: Syntax

.. syntax::

   BlockExpression ::=
       $${$$
         InnerAttributeOrDoc*
         StatementList
       $$}$$
   StatementList ::=
       ExpressionWithoutBlock
     | Statement+
     | StatementListWithExpression

   StatementListWithExpression ::=
       Statement+ ExpressionWithoutBlock


.. rubric:: Legality Rules

:def_p:`fls_finpsjxhsypb`
A :term:`block expression` is an :term:`expression` that
sequences :term:`expression`\ s and :term:`statement`\ s.

:def_p:`fls_xi7comov8h8f`
A :term:`tail expression` is the last :term:`expression` within a :term:`block
expression`.

:def_p:`fls_sde8ptwa7xay`
A :term:`block expression` is a :term:`value expression`.

:def_p:`fls_wbu4cmu1s52r`
The :term:`type` of a :term:`block expression` is determined as follows:

* :def_p:`fls_dajjk13u5po8`
  If the :term:`block expression` has an :term:`expression-without-block`, then
  the :term:`type` is the :term:`type` of the :term:`expression-without-block`.

* :def_p:`fls_augjp7fgxs`
  If the :term:`block expression` does not have an :term:`expression-without-
  block`, then the :term:`type` is the :term:`unit type`.

:def_p:`fls_s7wl3eryevzj`
The :term:`value` of a :term:`block expression` is determined as follows:

* :def_p:`fls_wi23eki92zy1`
  If the :term:`block expression` has an :term:`expression-without-block`,
  then the :term:`value` is the :term:`value` of the :term:`expression-without-
  block`.

* :def_p:`fls_xp1yuwtj0t8x`
  If the :term:`block expression` does not have an :term:`expression-
  without-block`, then the :term:`value` of the :term:`block expression` is
  the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_yhvh8gfwwhrs`
The :term:`evaluation` of a :term:`block expression` proceeds as follows:

#. :def_p:`fls_283hm4wfei4c`
   Each :term:`statement` is executed in declarative order.

#. :def_p:`fls_uos3yyahab5q`
   The :term:`expression-without-block` is evaluated.

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

:def_p:`fls_59i7pbvsc00s`
An :term:`async block expression` is a :term:`block expression` that **???**.

:def_p:`fls_hbyep9yqksq1`
An :term:`async block expression` is a :term:`value expression`.

:def_p:`fls_il8ewcwls65z`
An :term:`async block expression` is subject to :term:`capturing`.

:def_p:`fls_bdql2gtjp36j`
An :term:`async block expression` denotes a new :term:`control flow boundary`.

:def_p:`fls_619uyeq6ysv4`
The :term:`type` of an :term:`async block expression` shall implement
the :codeterm:`core::future::Future` trait.

:def_p:`fls_oe999cee8wc7`
The :term:`value` of an :term:`async block expression` is a :term:`future`.

.. rubric:: Dynamic Semantics

:def_p:`fls_2wya4voirk78`
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

:def_p:`fls_tpr9u7obcvxd`
An :term:`unsafe block expression` is a :term:`block expression` that is marked
as :term:`unsafe`.

:def_p:`fls_dxsae33asd5b`
An :term:`unsafe block expression` allows :term:`unsafety`.

:def_p:`fls_mq7qii8r5upt`
An :term:`unsafe block expression` is a :term:`value expression`.

:def_p:`fls_t9fi02md63ru`
The :term:`type` of the :term:`unsafe block expression` is the :term:`type` of
its :term:`block expression`.

:def_p:`fls_xih5u6r4l4mh`
The :term:`value` of the :term:`unsafe block expression` is the :term:`value` of
its :term:`block expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_bl8cbljlhgvq`
The :term:`evaluation` of an :term:`unsafe block expression` evaluates
its :term:`block expression`.

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
     | BorrowExpression
     | ComparisonExpression
     | CompoundAssignmentExpression
     | DereferenceExpression
     | ErrorPropagationExpression
     | LazyBooleanExpression
     | BitExpression
     | NegationExpression
     | TypeCastExpression


.. rubric:: Legality Rules

:def_p:`fls_ci9pyru4m3h9`
An :term:`operator expression` is an :term:`expression` that involves an
operator.

Borrow Expression
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BorrowExpression ::=
       $$&$$ $$mut$$? Operand

.. rubric:: Legality Rules

:def_p:`fls_qdp22i6tvznu`
A :term:`borrow expression` is an :term:`expression` that borrows
the :term:`value` of its :term:`operand` and creates a :term:`reference` to the
memory location of its :term:`operand`.

:def_p:`fls_22785s5n6udx`
A :term:`shared borrow` is a :term:`borrow` produced by evaluating
a :term:`borrow expression` without :term:`keyword` **``mut``**.

:def_p:`fls_3abn0szdewlb`
A :term:`mutable borrow` is a :term:`borrow` produced by evaluating
a :term:`borrow expression` with :term:`keyword` **``mut``**.

:def_p:`fls_bh2umdq4780j`
When the :term:`operand` of a :term:`borrow expression` is a :term:`place
expression`, the :term:`borrow expression` produces a :term:`reference` to the
memory location indicated by the :term:`operand`. The memory location is placed
in a borrowed state, or simply :term:`borrowed`.

:def_p:`fls_h9lycvsm9lp7`
When the :term:`operand` of a :term:`borrow expression` is a :term:`value
expression`, a :term:`temporary` is allocated and the :term:`borrow expression`
produces a :term:`reference` to the memory location of the :term:`temporary`.

:def_p:`fls_tlst032o0xp8`
A :term:`borrow expression` is a :term:`value expression`.

:def_p:`fls_c8r2p62sfvuq`
An :def_term:`implicit borrow` is a :term:`borrow` that **???**.
An :term:`implicit borrow` occurs in the following contexts:

* :def_p:`fls_hixgs5l7b687`
  The :term:`indexed array operand` of an :term:`array index expression`,

* :def_p:`fls_pldrhmkkk011`
  The :term:`call operand` of a :term:`call expression`,

* :def_p:`fls_eb7859x0i97n`
  The :term:`assigned operand` of a :term:`compound assignment expression`,

* :def_p:`fls_kl5q92rw5iat`
  The :term:`operand`\ s of a :term:`comparison expression`,

* :def_p:`fls_9e1taql46wy5`
  The :term:`operand` of a :term:`field access expression`,

* :def_p:`fls_z21th8tky04y`
  The :term:`operand` of a :term:`dereference expression`,

* :def_p:`fls_i6knvvl849kk`
  The :term:`receiver operand` of a :term:`method call expression`.

:def_p:`fls_2n363anl8q8k`
Borrowing a :term:`field` of a :term:`union type` borrows all
remaining :term:`field`\ s using the same :term:`lifetime`.

:def_p:`fls_s9x71ir7bs28`
The :term:`type` of a :term:`borrow expression` is determined as follows:

* :def_p:`fls_9i4f6pdx1474`
  If the :term:`borrow expression` denotes a :term:`shared reference`,
  then the :term:`type` is ``&T`` where ``T`` is the :term:`type` of
  the :term:`operand`.

* :def_p:`fls_a2d5itokqsi1`
  If the :term:`borrow expression` denotes a :term:`mutable reference`,
  then the :term:`type` is ``&mut T`` where ``T`` is the :term:`type` of
  the :term:`operand`.

:def_p:`fls_7a3uvqv6l71u`
The :term:`value` of a :term:`borrow expression` is the address of
its :term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_1cvhvq6zhjc7`
The :term:`evaluation` of a :term:`borrow expression` evaluates
its :term:`operand`.

.. rubric:: Examples

.. code-block:: text

   let mut answer = 42;


:def_p:`fls_gri2udh2a9kb`
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

:def_p:`fls_44a4ym2lm3lw`
A :term:`dereference expression` is an :term:`expression` that obtains the
pointed-to memory location of its :term:`operand`.

:def_p:`fls_mbpovcuekt8v`
When the :term:`operand` of a :term:`dereference expression` is of
a :term:`pointer type`, the :term:`dereference expression` denotes the pointed-
to memory location of the :term:`operand`, or the :term:`dereference` of
the :term:`operand`.

:def_p:`fls_fk7ockwpd871`
The :term:`dereference` is assignable when

* :def_p:`fls_of3j12qj2t2i`
  The :term:`operand` is of :term:`type` ``&mut T`` or ``*mut T``, and

* :def_p:`fls_ckcejs6kj0hn`
  The :term:`operand` is a :term:`binding` or a possibly nested :term:`field` of
  a :term:`binding`, or

* :def_p:`fls_1h8mjuj5pukl`
  The :term:`operand` denotes a :term:`mutable place expression`.

* :def_p:`fls_jqml0qqpwl93`
  The :term:`operand` is of another :term:`type` that implements
  the :codeterm:`core::ops::DerefMut` :term:`trait`.

:def_p:`fls_ylubpgghlo0b`
Dereferencing a :term:`raw pointer` shall require :term:`unsafe context`.

:def_p:`fls_75dm5yehiwln`
If the context of a :term:`dereference expression` is an :term:`immutable
place expression`, then the :term:`dereference expression` is equivalent
to :term:`expression` ``*:term:`core::ops::Deref::deref`\ (&operand)``.

:def_p:`fls_cnsbobkkkfgp`
If the context of a :term:`dereference expression` is a :term:`mutable
place expression`, then the :term:`dereference expression` is equivalent
to :term:`expression` ``*:term:`core::ops::DerefMut::deref_mut`\ (&mut
operand)``.

:def_p:`fls_92pyhrv9mprz`
The :term:`type` of a :term:`dereference expression` is determined as follows:

* :def_p:`fls_axngn5tgxgtb`
  If the :term:`type` of the :term:`operand` is ``&mut T``, ``&T``, ``*mut T``,
  or ``*const T``, then the :term:`type` is ``T``\ ``.``

* :def_p:`fls_3fkl8q9v6qsm`
  Otherwise the :term:`type` is :term:`associated
  type` :codeterm:`core::ops::Deref::Target`.

:def_p:`fls_2x16x17u8dag`
The :term:`value` of a :term:`dereference expression` is determined as follows:

* :def_p:`fls_k1009ctsqlw`
  If the :term:`type` of the :term:`operand` is ``&mut T``, ``&T``, ``*mut T``,
  or ``*const T``, then the :term:`value` is the pointed-to :term:`value`\ ``.``

* :def_p:`fls_sy0q0jgdacfi`
  Otherwise the :term:`value` is the result of evaluating :term:`expression`
  ``*:term:`core::ops::Deref::deref`\ (&operand)`` or :term:`expression`
  ``*:term:`core::ops::DerefMut::deref_mut`\ (&mut operand)`` respectively.

.. rubric:: Dynamic Semantics

:def_p:`fls_7yod90ny039i`
The :term:`evaluation` of a :term:`dereference expression` evaluates
its :term:`operand`.

.. rubric:: Undefined Behavior

:def_p:`fls_xecb8kz4imsf`
It is undefined behavior to dereference a :term:`raw pointer` that is
either :term:`dangling` or unaligned.

.. rubric:: Examples

:def_p:`fls_jfxv8azi7u3c`
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

:def_p:`fls_huxle5883n1r`
An :term:`error propagation expression` is an :term:`expression` that either
returns the :term:`value` of its :term:`operand` or propagates an error up the
call stack. (**better explanation?**)

:def_p:`fls_isojdacjfibg`
An :term:`error propagation expression` shall appear within a :term:`control
flow boundary`.

:def_p:`fls_gqbmyn8xanqg`
The :syntax:`ErrorPropagationOperator` shall not be overloadable. **(unstable
item, needs to be reviewed)**

:def_p:`fls_e5gorkufm2op`
The :term:`type` of an :term:`error propagation expression` is :term:`associated
type` :codeterm:`core::ops::Try::Output`. **(review this when we determine the
version the document is for, Try is changing constantly as it is unstable)**

:def_p:`fls_fr6451g9fzkp`
The :term:`value` of an :term:`error propagation expression` is determined as
follows:

* :def_p:`fls_v9ngkvuw3mlq`
  If the :term:`evaluation` of the :term:`error propagation expression` executed
  ``:term:`core::ops::Try::branch`\ (operand)``, then the :term:`value` is
  the :term:`value` of the :codeterm:`core::ops::ControlFlow::Continue` variant.

* :def_p:`fls_yl6vfqrsfmar`
  Otherwise control flow is returned to the end of the enclosing :term:`control
  flow boundary`.

:def_p:`fls_2zy1nfj3ek47`
The expression context for the :term:`operand` of the :term:`error propagation
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_bfq9iktx37qv`
The :term:`evaluation` of an :term:`error propagation operator` of the form

.. code-block:: text

   expression?

:def_p:`fls_n8eraxesgwo7`
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

:def_p:`fls_fchu9oz9tr2i`
A :term:`negation expression` is an :term:`expression` that negates
its :term:`operand`.

:def_p:`fls_rv093cfi9ukr`
The :term:`type` of the :term:`operand` of a :term:`negation
expression` with a :syntax:`BitwiseNegationOperator` shall implement
the :codeterm:`core::ops::Not` :term:`trait`.

:def_p:`fls_wvl42j4gllg5`
The :term:`type` of a :term:`negation expression` with
a :syntax:`BitwiseNegationOperator` is :term:`associated
type` :codeterm:`core::ops::Not::Output`.

:def_p:`fls_w6pbophm3ce`
The :term:`value` of a :term:`negation expression` with
a :syntax:`BitwiseNegationOperator` is the result of
``:term:`core::ops::Not::not`\ (operand)``.

:def_p:`fls_9irod01syejg`
The :term:`type` of the :term:`operand` of a :term:`negation
expression` with a :syntax:`SignNegationOperator` shall implement
the :codeterm:`core::ops::Neg` :term:`trait`.

:def_p:`fls_zgab96ojq0uy`
The :term:`type` of a :term:`negation expression` with
a :syntax:`SignNegationOperator` shall be :term:`associated
type` :codeterm:`core::ops::Neg::Output`.

:def_p:`fls_vd24tvs1uvrd`
The :term:`value` of a :term:`negation expression` with
a :syntax:`SignNegationOperator` is the result of ``:term:`core::ops::Neg::neg`\
(operand)``.

:def_p:`fls_i0w50l3qr0nt`
The expression context for the :term:`operand` of the :term:`negation
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_c6ey1mffhwl1`
The :term:`evaluation` of a :term:`negation expression` with
a :syntax:`BitwiseNegationOperator` proceeds as follows:

#. :def_p:`fls_ipmbqmpb4hf6`
   The :term:`operand` is evaluated.

#. :def_p:`fls_c8aytm39ce3f`
   ``:term:`core::ops::Not::not`\ (operand)`` is invoked.

:def_p:`fls_3em2xlag4xoh`
The :term:`evaluation` of a :term:`negation expression` with
a :syntax:`SignNegationOperator` proceeds as follows:

#. :def_p:`fls_rga2v0fdbtmv`
   The :term:`operand` is evaluated.

#. :def_p:`fls_yhisko9we432`
   ``:term:`core::ops::Neg::neg`\ (operand)`` is invoked.

.. rubric:: Examples

:def_p:`fls_w556dob4qayw`
Sign negation.

.. code-block:: text

   -42

:def_p:`fls_wn452yw7h4yb`
Bitwise negation.

.. code-block:: text

   !42

:def_p:`fls_t0ajn047zje4`
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

:def_p:`fls_dtu5tqg8vlzx`
An :term:`arithmetic expression` is an :term:`expression` that computes
a :term:`value` from two :term:`operand`\ s using arithmetic.

:def_p:`fls_entl3cazg3fa`
A :term:`division expression` is an :term:`arithmetic expression` that uses
division.

:def_p:`fls_ylqgaolt9put`
A :term:`multiplication expression` is an :term:`arithmetic expression` that
uses multiplication.

:def_p:`fls_jnw5qmmidzj`
A :term:`remainder expression` is an :term:`arithmetic expression` that uses
remainder division.

:def_p:`fls_p9hicu7blixc`
A :term:`subtraction expression` is an :term:`arithmetic expression` that uses
subtraction.

:def_p:`fls_ieqyt450tate`
The :term:`type` of the :term:`left operand` of an :term:`addition
expression` shall implement the :codeterm:`core::ops::Add` :term:`trait`
with the :term:`type` of the :term:`right operand` as the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_vhqal4gu3fb`
The :term:`type` of an :term:`addition expression` is :term:`associated
type` :codeterm:`core::ops::Add::Output`.

:def_p:`fls_3ytucymwpghs`
The :term:`value` of an :term:`addition expression` is the result of
``:term:`core::ops::Add::add`\ (left_operand, right_operand)``.

:def_p:`fls_v0amu946wriz`
The :term:`type` of the :term:`left operand` of a :term:`division
expression` shall implement the :codeterm:`core::ops::Div` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_1llb94scqnd7`
The :term:`type` of a :term:`division expression` is :term:`associated
type` :codeterm:`core::ops::Div::Output`.

:def_p:`fls_paskhh8abapy`
The :term:`value` of a :term:`division expression` is the result of
``:term:`core::ops::Div::div`\ (left_operand, right_operand)``.

:def_p:`fls_ubgqldpns1br`
The :term:`type` of the :term:`left operand` of a :term:`multiplication
expression` shall implement the :codeterm:`core::ops::Mul` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_errqd2w5df3j`
The :term:`type` of a :term:`multiplication expression` is :term:`associated
type` :codeterm:`core::ops::Mul::Output`.

:def_p:`fls_qapvj3wy918h`
The :term:`value` of a :term:`multiplication expression` is the result of
``:term:`core::ops::Mul::mul`\ (left_operand, right_operand)``.

:def_p:`fls_uen65b14p0pi`
The :term:`type` of the :term:`left operand` of a :term:`remainder
expression` shall implement the :codeterm:`core::ops::Rem` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_j2ybyozh6boj`
The :term:`type` of a :term:`remainder expression` is :term:`associated
type` :codeterm:`core::ops::Rem::Output`.

:def_p:`fls_1xvk8wrqfzzy`
The :term:`value` of a :term:`remainder expression` is the result of
``:term:`core::ops::Rem::rem`\ (left_operand, right_operand)``.

:def_p:`fls_9bcz9y4hnari`
The :term:`type` of the :term:`left operand` of a :term:`subtraction
expression` shall implement the :codeterm:`core::ops::Sub` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_f36mzxtoepbo`
The :term:`type` of a :term:`subtraction expression` is :term:`associated
type` :codeterm:`core::ops::Sub::Output`.

:def_p:`fls_xi12qrvdkq7z`
The :term:`value` of a :term:`subtraction expression` is the result of
``:term:`core::ops::Sub::sub`\ (left_operand, right_operand)``.

:def_p:`fls_n4f0obg0uqmn`
The expression context for the :term:`operand`\ s of an :term:`arithmetic
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_674k6h2p2ge`
The :term:`evaluation` of an :term:`addition expression` proceeds as follows:

#. :def_p:`fls_3jevd428zxun`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_e9j7einv01p`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_7bs6sq83fqug`
   ``:term:`core::ops::Add::add`\ (left_operand, right_operand)`` is invoked.

:def_p:`fls_adsvsbwvjri4`
The :term:`evaluation` of a :term:`division expression` proceeds as follows:

#. :def_p:`fls_zf1e3684e5dd`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_vdond19eqgb7`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_ewsizsixa6m1`
   ``:term:`core::ops::Div::div`\ (left_operand, right_operand)`` is invoked.

:def_p:`fls_36tretnnq6k9`
The :term:`evaluation` of a :term:`multiplication expression` proceeds as
follows:

#. :def_p:`fls_ki6rbiiph1cb`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_covolgcvhux9`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_tk04y3lxvqby`
   ``:term:`core::ops::Mul::mul`\ (left_operand, right_operand)`` is invoked.

:def_p:`fls_ih2spu1rnij2`
The :term:`evaluation` of a :term:`remainder expression` proceeds as follows:

#. :def_p:`fls_t9naln501bn`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_p7vlvou62nbv`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_vqp87p96pnht`
   ``:term:`core::ops::Rem::rem`\ (left_operand, right_operand)`` is invoked.

:def_p:`fls_hkrrivi6ra8d`
The :term:`evaluation` of a :term:`subtraction expression` proceeds as follows:

#. :def_p:`fls_zfh3ocos7aab`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_rupphsikb1sb`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_cj4eeioqpe72`
   ``:term:`core::ops::Rem::rem`\ (left_operand, right_operand)`` is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_ywq504j3mhn4`
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

:def_p:`fls_i3t72pewqqm7`
A :term:`bit expression` is an :term:`expression` that computes a :term:`value`
from two :term:`operand`\ s using bit arithmetic.

:def_p:`fls_cbnjpg2234vw`
A :term:`bit and expression` is a :term:`bit expression` that uses bit and
arithmetic.

:def_p:`fls_ax2ltcyjq7fg`
A :term:`bit or expression` is a :term:`bit expression` that uses bit or
arithmetic.

:def_p:`fls_tghdye8bnq64`
A :term:`bit xor expression` is a :term:`bit expression` that uses bit exclusive
or arithmetic.

:def_p:`fls_rt4xx4h6dtpq`
A :term:`shift left expression` is a :term:`bit expression` that uses bit shift
left arithmetic.

:def_p:`fls_f8rn0u6iptb`
A :term:`shift right expression` is a :term:`bit expression` that uses bit shift
right arithmetic.

:def_p:`fls_qrhhu6ta1sns`
The :term:`type` of the :term:`left operand` of a :term:`bit and
expression` shall implement the :codeterm:`core::ops::BitAnd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_62vxcdn3u374`
The :term:`type` of a :term:`bit and expression` is :term:`associated
type` :codeterm:`core::ops::BitAnd::Output`.

:def_p:`fls_vkxbvqrk6rgb`
The :term:`value` of a :term:`bit and expression` is the result of
``:term:`core::ops::BitAnd::bitand`\ (left_operand, right_operand)``.

:def_p:`fls_lplx3a767c2t`
The :term:`type` of the :term:`left operand` of a :term:`bit or expression`
shall implement the :codeterm:`core::ops::BitOr` :term:`trait` where
the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_nk921c8ykajw`
The :term:`type` of a :term:`bit or expression` is :term:`associated
type` :codeterm:`core::ops::BitOr::Output`.

:def_p:`fls_4yf17g2q600b`
The :term:`value` of a :term:`bit or expression` is the result of
``:term:`core::ops::BitOr::bitor`\ (left_operand, right_operand)``.

:def_p:`fls_fg4jugltr6og`
The :term:`type` of the :term:`left operand` of a :term:`bit xor
expression` shall implement the :codeterm:`core::ops::BitXor` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_h8yyacdawka0`
The :term:`type` of a :term:`bit xor expression` is :term:`associated
type` :codeterm:`core::ops::BitXor::Output`.

:def_p:`fls_cb9a2f5vot7n`
The :term:`value` of a :term:`bit xor expression` is the result of
``:term:`core::ops::BitXor::bitxor`\ (left_operand, right_operand)``.

:def_p:`fls_n2c8x9bxe2xk`
The :term:`type` of the :term:`left operand` of a :term:`shift left
expression` shall implement the :codeterm:`core::ops::Shl` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_hdrlnohi7tmd`
The :term:`type` of a :term:`shift left expression` is :term:`associated
type` :codeterm:`core::ops::Shl::Output`.

:def_p:`fls_v45auqh19q2u`
The :term:`value` of a :term:`shift left expression` is the result of
``:term:`core::ops::Shl::shl`\ (left_operand, right_operand)``.

:def_p:`fls_orogiybgb100`
The :term:`type` of the :term:`left operand` of a :term:`shift right
operation` shall implement the :codeterm:`core::ops::Shr` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_btrieb96kkq9`
The :term:`type` of a :term:`shift right operation` is :term:`associated
type` :codeterm:`core::ops::Shr::Output`.

:def_p:`fls_etpzvxxa90di`
The :term:`value` of a :term:`shift right operation` is the result of
``:term:`core::ops::Shr::shr`\ (left_operand, right_operand)``.

:def_p:`fls_kv1vs7otuw93`
The expression context for the :term:`operand`\ s of a :term:`bit expression` is
a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_nuctzmv4eq5j`
The :term:`evaluation` of a :term:`bit and expression` proceeds as follows:

#. :def_p:`fls_xw9f119b1ad4`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_tdmhtjmfmwdr`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_sm1y7eemsphb`
   ``:term:`core::ops::BitAnd::bitand`\ (left_operand, right_operand)`` is
   invoked.

:def_p:`fls_yo1cunuih4mh`
The :term:`evaluation` of a :term:`bit or expression` proceeds as follows:

#. :def_p:`fls_ejaa7zgzb87f`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_v5mfpxc9l5x0`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_yj7xjzst21gg`
   ``:term:`core::ops::BitOr::bitor`\ (left_operand, right_operand)`` is
   invoked.

:def_p:`fls_cmogovidfzac`
The :term:`evaluation` of a :term:`bit xor expression` proceeds as follows:

#. :def_p:`fls_otp453fdpeq7`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_dzjqvvsqkzf0`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_lx7zo9uu33i`
   ``:term:`core::ops::BitXor::bitxor`\ (left_operand, right_operand)`` is
   invoked.

:def_p:`fls_fsyfey9zs6u5`
The :term:`evaluation` of a :term:`shift left expression` proceeds as follows:

#. :def_p:`fls_ggl8l8wcmfsb`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_guw0d1c5u9ay`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_ucy60ssaae39`
   ``:term:`core::ops::Shl::shl`\ (left_operand, right_operand)`` is invoked.

:def_p:`fls_x0oj2ahm0tml`
The :term:`evaluation` of a :term:`shift right expression` proceeds as follows:

#. :def_p:`fls_f6zck0x48ko5`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_ft8d0k7z6v1v`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_1cmnk3spqr3`
   ``:term:`core::ops::Shr::shr`\ (left_operand, right_operand)`` is invoked.

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

:def_p:`fls_62auyjq8bx31`
A :term:`comparison expression` is an :term:`expression` that compares
the :term:`value`\ s of two :term:`operand`\ s.

:def_p:`fls_esa7ju1yfnrw`
An :term:`equals expression` is a :term:`comparison expression` that tests
equality.

:def_p:`fls_42xw6twn0sv7`
A :term:`greater-than expression` is a :term:`comparison expression` that tests
for a greater-than relationship.

:def_p:`fls_p8ynzeaomkf5`
A :term:`greater-than-or-equals expression` is a :term:`comparison expression`
that tests for a greater-than-or-equals relationship.

:def_p:`fls_h6ymcmndrzbi`
A :term:`less-than expression` is a :term:`comparison expression` that tests for
a less-than relationship.

:def_p:`fls_bzftqgvq5sz2`
A :term:`less-than-or-equals expression` is a :term:`comparison expression` that
tests for a less-than-or-equals relationship.

:def_p:`fls_7eyd1ldjm88c`
A :term:`not-equals expression` is a :term:`comparison expression` that tests
for inequality.

:def_p:`fls_m3wnughbzehm`
A :term:`comparison expression` implicitly takes :term:`shared borrow`\ s of
its :term:`operand`\ s.

:def_p:`fls_4ccbsxylkwmj`
The :term:`type` of a :term:`comparison expression`
is :term:`type` :codeterm:`bool`.

:def_p:`fls_n0aqca8wu54f`
The :term:`type` of the :term:`left operand` of an :term:`equals expression`
shall implement the :codeterm:`core::cmp::PartialEq` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_waw05dg2m9tk`
The :term:`value` of an :term:`equals expression` is the result of
``:term:`core::cmp::PartialEq::eq`\ (&left_operand, &right_operand)``.

:def_p:`fls_7lnckq84hwd`
The :term:`type` of the :term:`left operand` of a :term:`greater-than
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_n3yo9qeeo1ud`
The :term:`value` of a :term:`greater-than expression` is the result of
``:term:`core::cmp::PartialOrd::gt`\ (&left_operand, &right_operand)``.

:def_p:`fls_lpcccw6d2b8x`
The :term:`type` of the :term:`left operand` of a :term:`greater-than-or-equals
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_73za6h5ady39`
The :term:`value` of a :term:`greater-than-or-equals expression` is the result
of ``:term:`core::cmp::PartialOrd::ge`\ (&left_operand, &right_operand)``.

:def_p:`fls_qiikbx17f47z`
The :term:`type` of the :term:`left operand` of a :term:`less-than expression`
shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_x8wynfvb929w`
The :term:`value` of a :term:`less-than expression` is the result of
``:term:`core::cmp::PartialOrd::lt`\ (&left_operand, &right_operand)``.

:def_p:`fls_9sbwx8n6mapz`
The :term:`type` of the :term:`left operand` of a :term:`less-than-or-equals
expression` shall implement the :codeterm:`core::cmp::PartialOrd` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_uq2o8hwlgzr5`
The :term:`value` of a :term:`less-than-or-equals expression` is the result of
``:term:`core::cmp::PartialOrd::le`\ (&left_operand, &right_operand)``.

:def_p:`fls_ccsrr3ops2vb`
The :term:`type` of the :term:`left operand` of a :term:`not-equals
expression` shall implement the :codeterm:`core::cmp::PartialEq` :term:`trait`
where the :term:`type` of the :term:`right operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_nxg2f089s5y4`
The :term:`value` of a :term:`not-equals expression` is the result of
``:term:`core::cmp::PartialEq::ne`\ (&left_operand, &right_operand)``.

:def_p:`fls_z8fwh0qcjnqj`
The expression context for the :term:`operand`\ s of a :term:`comparison
expression` is a :term:`place expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_ovndhs5v94fs`
The :term:`evaluation` of an :term:`equals expression` proceeds as follows:

#. :def_p:`fls_rc9b652jipqj`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_gwi1z6lfbhlc`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_uoo0uqruactp`
   ``:term:`core::cmp::PartialEq::eq`\ (&left_operand, &right_operand)`` ``is
   invoked.``

:def_p:`fls_r9chfejy39t`
The :term:`evaluation` of a :term:`greater-than expression` proceeds as follows:

#. :def_p:`fls_nyk0hsa5rokm`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_y35teqcydr4g`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_yt4w9gb9400p`
   ``:term:`core::cmp::PartialOrd::gt`\ (&left_operand, &right_operand)`` is
   invoked.

:def_p:`fls_h8syj5x7pz9u`
The :term:`evaluation` of a :term:`greater-than-or-equals expression` proceeds
as follows:

#. :def_p:`fls_eqlldhso8cqd`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_xt17kpxz33qb`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_rvj8om9qro8p`
   ``:term:`core::cmp::PartialOrd::ge`\ (&left_operand, &right_operand)`` is
   invoked.

:def_p:`fls_7s4100yj4zh6`
The :term:`evaluation` of a :term:`less-than expression` proceeds as follows:

#. :def_p:`fls_onz9rvfngc5l`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_99os36q5y6pz`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_7n25yb62dohd`
   ``:term:`core::cmp::PartialOrd::lt`\ (&left_operand, &right_operand)`` is
   invoked.

:def_p:`fls_wgpe68gbqlz6`
The :term:`evaluation` of a :term:`less-than-or-equals expression` proceeds
as follows:

#. :def_p:`fls_s8fxnl87s9m8`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_d028x1sn6ntl`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_3nkwhbdbasad`
   ``:term:`core::cmp::PartialOrd::le`\ (&left_operand, &right_operand)`` is
   invoked.

:def_p:`fls_2yugsqls9m7`
The :term:`evaluation` of a :term:`not-equals expression` proceeds as follows:

#. :def_p:`fls_lpct1cp37kb7`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_9xzwfkrjxw3o`
   The :term:`right operand` is evaluated.

#. :def_p:`fls_k6s8f75t8njc`
   ``:term:`core::cmp::PartialEq::ne`\ (&left_operand, &right_operand)`` is
   invoked.

.. rubric:: Examples

:def_p:`fls_z4gnyldw8ufi`
12 == 12

:def_p:`fls_ns9pnuajd87j`
42 > 12

:def_p:`fls_ca0jdb8nw5dk`
42 >= 35

:def_p:`fls_rs4w2rtv8249`
42 < 109

:def_p:`fls_c3v2sjdt7sxr`
42 <= 42

:def_p:`fls_sejpan48w3sl`
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

:def_p:`fls_9nsr0ihym6m8`
A :term:`lazy boolean expression` is an :term:`expression` that performs short
circuit Boolean arithmetic.

:def_p:`fls_2fn849gcxjbb`
A :term:`lazy and expression` is a :term:`lazy boolean expression` that uses
short circuit and arithmetic.

:def_p:`fls_wtpsulfhofhv`
A :term:`lazy or expression` is a :term:`lazy boolean expression` that uses
short circuit or arithmetic.

:def_p:`fls_bndskylci0ar`
The :term:`type`\ s of the :term:`operand`\ s of a :term:`lazy boolean
expression` shall be :term:`type` :codeterm:`bool`.

:def_p:`fls_nhm8bpr0u2xl`
The :term:`type` of a :term:`lazy boolean expression`
is :term:`type` :codeterm:`bool`.

:def_p:`fls_o94kuw9hp7us`
The :term:`value` of a :term:`lazy boolean expression` is either ``true`` or
``false``.

:def_p:`fls_3ahnv3upc4vs`
The expression context for the :term:`operand`\ s of the :term:`lazy boolean
expression` is a :term:`value expression` context.

.. rubric:: Dynamic Semantics

:def_p:`fls_mzje88sjh8uz`
The :term:`evaluation` of a :term:`lazy and expression` proceeds as follows:

#. :def_p:`fls_jvswmb58ha28`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_1rlxywepgadq`
   If the :term:`left operand` evaluated to ``true``, then

#.    #. :def_p:`fls_o327m0rhvq5a`
         The :term:`right operand` is evaluated and returned as the :term:`lazy
         and expression`'s :term:`value`.

#. :def_p:`fls_lm1i3921lja2`
   Otherwise the :term:`lazy and expression` evaluates to ``false``.

:def_p:`fls_eqxjpzhw3k24`
The :term:`evaluation` of a :term:`lazy or expression` proceeds as follows:

#. :def_p:`fls_hxkxtjkny5ws`
   The :term:`left operand` is evaluated.

#. :def_p:`fls_sifevyjl1pjn`
   If the :term:`left operand` evaluated to ``false``, then

#.    #. :def_p:`fls_kal860yhg4c3`
         The :term:`right operand` is evaluated and returned as the :term:`lazy
         or expression`'s :term:`value`.

#. :def_p:`fls_smufeyrajj9r`
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

:def_p:`fls_ofwsxsvtpxma`
A :term:`type cast expression` is an :term:`expression` that changes
the :term:`type` of an :term:`operand`.

:def_p:`fls_3xa9wnb01xw0`
:term:`Cast` or :term:`casting` is the process of changing the :term:`type` of
an :term:`expression`.

:def_p:`fls_tm80ig2w763q`
The ``TypeSpecificationWithoutBounds`` describes the :def_term:`target type` of
the :term:`type cast expression`.

:def_p:`fls_svmr0pt2ibyl`
A :term:`type cast expression` with the following characteristics performs
a :def_term:`specialized cast`:

* :def_p:`fls_6hyuwlvm91s0`
  An :term:`operand` of a :term:`numeric type` and a target :term:`numeric type`
  perform a :term:`numeric cast`.

* :def_p:`fls_xyy2ve91j04g`
  An :term:`operand` of an :term:`enum type` and a target :term:`integer
  type` perform :term:`enum cast`. An* :term:`enum cast`\ * converts
  the :term:`operand` to its :term:`discriminant`, followed by a :term:`numeric
  cast`.

* :def_p:`fls_8pav1kz4yyvh`
  An operand of :term:`type` :codeterm:`bool` or :term:`type` :codeterm:`char`
  and a target :term:`integer type` perform :term:`primitive-to-integer cast`.
  A :def_term:`primitive-to-integer cast`

*    * :def_p:`fls_q7pv2u7f08s9`
       Converts an :term:`operand` of :term:`type` :codeterm:`bool`
       with :term:`value` ``false`` to zero.

*    * :def_p:`fls_dnfeh6cc2aa1`
       Converts an :term:`operand` of type :codeterm:`bool` with :term:`value`
       ``true`` to one.

*    * :def_p:`fls_plsgljewtta1`
       Convert an :term:`operand` of type :codeterm:`char` to the :term:`value`
       of the corresponding :term:`code point`, followed by a :term:`numeric
       cast`.

* :def_p:`fls_p4helift7lfl`
  An :term:`operand` of :term:`type` :codeterm:`u8` and a
  target :term:`type` :codeterm:`char` performs :term:`u8-to-char
  cast`. A :def_term:`u8-to-char cast` converts an :term:`operand`
  of :term:`type` :codeterm:`u8` to the :term:`value` of the
  corresponding :term:`code point`.

* :def_p:`fls_m5pemt2xk9yg`
  An :term:`operand` of :term:`type` ``*const T`` or ``*mut T`` and
  a :term:`target type` ``*const V`` or ``*mut V`` where ``V`` implements
  the :codeterm:`core::marker::Sized` :term:`trait` performs :term:`pointer-to-
  pointer cast`.

* :def_p:`fls_blys0phz4snk`
  An :term:`operand` of :term:`type` ``*const T`` or ``*mut T`` where
  ``T`` implements the :codeterm:`core::marker::Sized` :term:`trait` and
  a target :term:`integer type` perform :term:`pointer-to-address cast`.
  A :def_term:`pointer-to-address cast` produces an :term:`integer` that
  represents the machine address of the referenced memory. If the :term:`integer
  type` is smaller than the :term:`type` of the :term:`operand`, the address
  is truncated.

* :def_p:`fls_ucbjhvvi59b1`
  An :term:`operand` of :term:`integer type` and :term:`target
  type` ``*const V`` or ``*mut V`` where ``V`` implements
  the :codeterm:`core::marker::Sized` :term:`trait` perform :term:`address-
  to-pointer cast`. An :def_term:`address-to-pointer cast` produces
  a :term:`pointer` that interprets the :term:`integer` as a machine address.

* :def_p:`fls_lmqsue25egs9`
  An :term:`operand` of :term:`type` ``&mut [T; N]`` and a :term:`target type`
  ``*const T`` perform :term:`array-to-pointer cast`.

* :def_p:`fls_kte9b7v7sxl`
  An :term:`operand` of a :term:`function item type` and a :term:`target
  type` ``*const V`` or ``*mut V`` where ``V`` implements
  the :codeterm:`core::marker::Sized` :term:`trait` perform :term:`function-
  item-to-pointer cast`.

* :def_p:`fls_xnaeqtlppz1s`
  An :term:`operand` of a :term:`function item type` and a target :term:`integer
  type` perform :term:`function-to-address cast`.

* :def_p:`fls_o7o8r0rvkbds`
  An :term:`operand` of a :term:`function pointer type` and
  a :term:`target type` ``*const V`` or ``*mut V`` where ``V`` implements
  the :codeterm:`core::marker::Sized` :term:`trait` perform :term:`function-
  pointer-to-pointer cast`.

* :def_p:`fls_qo9srxkc2b9d`
  An :term:`operand` of a :term:`function pointer type` and a
  target :term:`integer type` perform :term:`function-pointer-to-address cast`.

:def_p:`fls_51smmcl4l4po`
A :term:`cast` is legal when it either performs :term:`type coercion` or is
a :term:`specialized cast`.

:def_p:`fls_cfq0b0f7yjjn`
The :term:`type` of a :term:`type cast expression` is the :term:`target type`.

:def_p:`fls_tq02d5wppi63`
The :term:`value` of a :term:`type cast expression` is the :term:`value` of
the :term:`operand` after the :term:`cast`.

.. rubric:: Dynamic Semantics

:def_p:`fls_fosuabsemcja`
The :term:`evaluation` of a :term:`type cast expression` evaluates
its :term:`operand`.

:def_p:`fls_pgj5ywxqzg8n`
The :term:`evaluation` of a :def_term:`numeric cast` proceeds as follows:

* :def_p:`fls_j4qo2m8gr5no`
  Casting an :term:`operand` of an :term:`integer type` to a
  target :term:`integer type` of the same :term:`size` has no effect.

* :def_p:`fls_87p7lp8q1hya`
  Casting an :term:`operand` of an :term:`integer type` to a
  target :term:`integer type` with smaller :term:`size` truncates
  the :term:`value` of the :term:`operand`.

* :def_p:`fls_2v9fo7odycpl`
  Casting an :term:`operand` of an :term:`integer type` to a
  target :term:`integer type` with a larger :term:`size` either

* :def_p:`fls_9naij73tqmfr`
  Zero-extends the :term:`operand` if the :term:`operand`'s :term:`type` is
  unsigned, or

* :def_p:`fls_9was8jx3c91q`
  Sign-extends the :term:`operand` if the :term:`operand`'s :term:`type` is
  signed.

* :def_p:`fls_61yegfuj2n3x`
  Casting an :term:`operand` of a :term:`floating-point type` to a
  target :term:`integer type` rounds the :term:`value` of the :term:`operand`
  towards zero. In addition, the :term:`type cast expression`

* :def_p:`fls_hr00zm3wvoj9`
  Returns zero if the :term:`operand` denotes :codeterm:`f32::NaN`
  or :codeterm:`f64::NaN` respectively.

* :def_p:`fls_xfm70eod14le`
  Saturates the :term:`value` of the :term:`operand` to the
  maximum :term:`value` of the target :term:`integer type` if
  the :term:`operand`'s :term:`value` exceeds the maximum :term:`value`
  of the target :term:`integer type` or denotes :codeterm:`f32::INFINITY`
  or :codeterm:`f64::INFINITY` respectively.

* :def_p:`fls_9xh03uq6nb8g`
  Saturates the :term:`value` of the :term:`operand` to the
  minimum :term:`value` of the target :term:`integer type` if
  the :term:`operand`'s :term:`value` exceeds the minimum :term:`value` of
  the target :term:`integer type` or denotes :codeterm:`f32::NEG_INFINITY`
  or :codeterm:`f64::NEG_INFINITY` respectively.

* :def_p:`fls_ctaj14m896u3`
  Casting an :term:`operand` of an :term:`integer type` to a
  target :term:`floating-point type` produces the closest possible floating
  point :term:`value`. In addition, the :term:`type cast expression`

* :def_p:`fls_iac4u5mdi2w1`
  Rounds the :term:`value` of the :term:`operand` preferring the :term:`value`
  with an even least significant digit if exactly halfway between two floating
  point numbers.

* :def_p:`fls_payefszfsswt`
  Produc	es :codeterm:`f32::INFINITY` or :codeterm:`f64::INFINITY` of the same
  sign as the :term:`value` of the :term:`operand` when the :term:`value` of
  the :term:`operand` causes overflow.

* :def_p:`fls_3ggicpgbus79`
  Casting an :term:`operand` of :term:`type` :codeterm:`f32` to a :term:`target
  type` :codeterm:`f64` is perfect and lossless.

* :def_p:`fls_dify2krq29ea`
  Casting an :term:`operand` of :term:`type` :codeterm:`f64`
  to :term:`target type` :codeterm:`f32` produces the closest
  possible :codeterm:`f32` :term:`value`. In addition, the :term:`type cast
  expression`

* :def_p:`fls_hftupotk27cz`
  Prefers the nearest :term:`value` with an even least significant digit if
  exactly halfway between two floating point numbers.

* :def_p:`fls_kbc5hhjgyq8`
  Produces :codeterm:`f32::INFINITY` of the same sign as the :term:`value`
  of the :term:`operand` when the :term:`value` of the :term:`operand` causes
  overflow.

.. rubric:: Examples

:def_p:`fls_57cwaj70sb91`
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

:def_p:`fls_7yg3x3xfjfa1`
An :term:`assignment expression` is an :term:`expression` that assigns
the :term:`value` of a :term:`value operand` to an :term:`assignee operand`.

:def_p:`fls_esr1gnf4t0og`
An :term:`assignee operand` is the target :term:`operand` of
an :term:`assignment expression`.

:def_p:`fls_nseu7nz946ox`
A :term:`value operand` is an :term:`operand` that supplies the :term:`value`
that is assigned to an :term:`assignee operand` by an :term:`assignment
expression`.

:def_p:`fls_k2l5mgpbvvi6`
An :term:`assignee operand` shall denote a :term:`mutable assignee expression`.

:def_p:`fls_oq34zoxe1h93`
A :term:`value operand` shall denote a :term:`value expression`.

:def_p:`fls_4kig1ee9vvyt`
The :term:`type` of an :term:`assignment expression` is the :term:`unit type`.

:def_p:`fls_nkvmede1po84`
The :term:`value` of an :term:`assignment expression` is the :term:`unit value`.

Basic Assignment
^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_7p5u7mdv5pda`
A :term:`basic assignment` is an :term:`assignment expression` that is not
a :term:`destructuring assignment`.

.. rubric:: Dynamic Semantics

:def_p:`fls_84qkhuqqwcv6`
The :term:`evaluation` of a :term:`basic assignment` proceeds as follows:

#. :def_p:`fls_vdz1ieu3ebfa`
   The :term:`value operand` is evaluated.

#. :def_p:`fls_wxwgh5a78xe`
   The :term:`assignee operand` is evaluated.

#. :def_p:`fls_qnuf9no2h2j0`
   The :term:`value` denoted by the :term:`assignee operand` is :term:`dropped`,
   unless the :term:`assignee operand` denotes an uninitialized :term:`binding`
   or an uninitialized :term:`field` of a :term:`binding`.

#. :def_p:`fls_qlfxc6jczl6o`
   The :term:`value` of the :term:`value operand` is :term:`copied`
   or :term:`moved` into the place of the :term:`assignee operand`.

.. rubric:: Examples

.. code-block:: text

   this = 42

Destructuring Assignment
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_kjqwemg1wfqn`
A :term:`destructuring assignment` is an :term:`assignment expression`
where the :term:`assignee operand` is either an :term:`array expression`,
a :term:`struct expression`, a :term:`tuple expression`, or a :term:`union
expression`.

:def_p:`fls_5x11ajkjtric`
The :term:`assignee operand` of a :term:`destructuring assignment` corresponds
to an :term:`assignee pattern` according to its kind, as follows:

* :def_p:`fls_2y30udx03tiv`
  A :term:`place expression` corresponds to an :term:`identifier pattern` with
  a unique :term:`identifier` and without :term:`keyword` **``ref``**, keyword
  **``mut``**, or a :term:`bound pattern`.

* :def_p:`fls_wp563c9ms3dq`
  An :term:`underscore expression` corresponds to an :term:`underscore pattern`.

* :def_p:`fls_rfdn8hxidvo5`
  A :term:`tuple expression` corresponds to a :term:`tuple pattern` with all
  the :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\
  s.

* :def_p:`fls_btlvvgchcf6l`
  A :term:`tuple struct expression` corresponds to a :term:`tuple
  struct pattern` with all the :term:`subexpression`\ s lowered to their
  corresponding :term:`pattern`\ s.

* :def_p:`fls_ucyg7kxqyp34`
  A :term:`struct expression` corresponds to a :term:`struct pattern` with all
  the :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\
  s.

* :def_p:`fls_t57bpvukbg62`
  A :term:`unit struct expression` corresponds to a :term:`unit struct pattern`.

* :def_p:`fls_25uq9ii6t4pr`
  A :term:`slice expression` corresponds to a :term:`slice pattern` with all
  the :term:`subexpression`\ s lowered to their corresponding :term:`pattern`\
  s.

* :def_p:`fls_l4timuq56fcg`
  A :term:`full range expression` corresponds to a :term:`rest pattern` if
  inside a :term:`slice expression`, otherwise this is a static error.

:def_p:`fls_ijcx8lj91d9g`
The :term:`pattern` that corresponds to a :term:`destructuring assignment` shall
be :term:`irrefutable`.

:def_p:`fls_no1iwx7etx02`
A :term:`destructuring assignment` is equivalent to a :term:`block expression`
of the following form:

* :def_p:`fls_dv0cqqmaeed`
  The first :term:`statement` is a :term:`let statement` with
  its :term:`pattern` equivalent to the lowered :term:`assignee pattern` and
  its :term:`initialization expression` equivalent to the :term:`value operand`.

* :def_p:`fls_1gvtzo8k3vyc`
  Then each bound identifier in the assignee pattern is an expression statement
  of an assignment as follows:

* :def_p:`fls_2gv1hf94icyx`
  The bound identifier becomes the value operand, and

* :def_p:`fls_5wybluxuyed4`
  The corresponding expression from the destructuring assignment's assignee
  operand becomes the assignee operand of the new assignment.

.. rubric:: Dynamic Semantics

:def_p:`fls_emkf51ugo5l1`
The :term:`evaluation` of a :term:`destructuring assignment` proceeds as
follows:

#. :def_p:`fls_hqzt60g28dqt`
   The :term:`value operand` is evaluated.

#. :def_p:`fls_8f7vh1hu6n80`
   The :term:`assignee operand` is evaluated by evaluating its :term:`operand`\
   s in a left-to-right order.

#. :def_p:`fls_dqnbk8aq2tay`
   Each :term:`value` denoted by the :term:`assignee operand` is :term:`dropped`
   in left-to-right order, unless the :term:`assignee operand` denotes an
   uninitialized :term:`binding` or an uninitialized field of a :term:`binding`.

#. :def_p:`fls_wq2lilgl2ye1`
   The :term:`value` of the :term:`value operand` is :term:`copied`
   or :term:`moved` into the place of the :term:`assignee operand`.

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

:def_p:`fls_dg34rw4e94u8`
A :term:`compound assignment expression` is an expression that first computes
a :term:`value` from two :term:`operand`\ s and then assigns the value to
an :term:`assigned operand`.

:def_p:`fls_oo73w3qpgtwy`
A :term:`bit and assignment expression` is a :term:`compound assignment
expression` that uses bit and arithmetic.

:def_p:`fls_ljpyo36zh5lb`
A :term:`bit or assignment expression` is a :term:`compound assignment
expression` that uses bit or arithmetic.

:def_p:`fls_jhd4zd7ktgqg`
A :term:`bit xor assignment expression` is a :term:`compound assignment
expression` that uses bit exclusive or arithmetic.

:def_p:`fls_6qjdnyy5iyuk`
A :term:`division assignment expression` is a :term:`compound assignment
expression` that uses division.

:def_p:`fls_hhnsfl1be5lk`
A :term:`multiplication assignment expression` is a :term:`compound assignment
expression` that uses multiplication.

:def_p:`fls_sncjawsya8i7`
A :term:`remainder assignment expression` is a :term:`compound assignment
expression` that uses remainder division.

:def_p:`fls_g35wq7paoff2`
A :term:`shift left assignment expression` is a :term:`compound assignment
expression` that uses bit shift left arithmetic.

:def_p:`fls_7mspjhc8k34l`
A :term:`shift right assignment expression` is a :term:`compound assignment
expression` that uses bit shift right arithmetic.

:def_p:`fls_zblx7hnqjep2`
A :term:`subtraction assignment expression` is a :term:`compound assignment
expression` that uses subtraction.

:def_p:`fls_bi4zmt4hw23u`
An :term:`assigned operand` is the target :term:`operand` of a :term:`compound
assignment expression`.

:def_p:`fls_o14v7fwqm15q`
A :term:`modifying operand` is an :term:`operand` that supplies
the :term:`value` that is used in the calculation of a :term:`compound
assignment expression`.

:def_p:`fls_askwo0qmu6n3`
An :term:`assigned operand` shall denote a :term:`mutable assignee expression`.

:def_p:`fls_bw285iheixol`
A :term:`modifying operand` shall denote a :term:`value expression`.

:def_p:`fls_of8ds5huz6f3`
The :term:`type` of a :term:`compound assignment` is the :term:`unit type`.

:def_p:`fls_fkiz6hgtty3r`
The :term:`value` of a :term:`compound assignment` is the :term:`unit value`.

:def_p:`fls_ggri9g80atdz`
The :term:`type` of the :term:`assigned operand` of an :term:`addition
assignment` shall implement the :codeterm:`core::ops::AddAssign` trait where the
type of the right operand is the trait implementation type parameter.

:def_p:`fls_ojxvr7bch45m`
The :term:`type` of the :term:`assigned operand` of a :term:`bit and assignment`
shall implement the :codeterm:`core::ops::BitAndAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_owskjaapwuie`
The :term:`type` of the :term:`assigned operand` of a :term:`bit or assignment`
shall implement the :codeterm:`core::ops::BitOrAssign` :term:`trait` where
the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_3yyzizbudxn2`
The :term:`type` of the :term:`assigned operand` of a :term:`bit xor assignment`
shall implement the :codeterm:`core::ops::BitXorAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_i09un6yi2num`
The :term:`type` of the :term:`assigned operand` of a :term:`division
assignment` shall implement the :codeterm:`core::ops::DivAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_5ctsgjmrmtph`
The :term:`type` of the :term:`assigned operand` of a :term:`multiplication
assignment` shall implement the :codeterm:`core::ops::MulAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_diqxah22oui2`
The :term:`type` of the :term:`assigned operand` of a :term:`remainder
assignment` shall implement the :codeterm:`core::ops::RemAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_3k6b1ljdjyic`
The :term:`type` of the :term:`assigned operand` of a :term:`shift left
assignment` shall implement the :codeterm:`core::ops::ShlAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_t9z5crww08mu`
The :term:`type` of the :term:`assigned operand` of a :term:`shift right
assignment` shall implement the :codeterm:`core::ops::ShrAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

:def_p:`fls_58le9uwtg95m`
The :term:`type` of the :term:`assigned operand` of a :term:`subtraction
assignment` shall implement the :codeterm:`core::ops::SubAssign` :term:`trait`
where the :term:`type` of the :term:`modifying operand` is the :term:`trait
implementation` :term:`type parameter`.

.. rubric:: Dynamic Semantics

:def_p:`fls_oqbctldcvscd`
The :term:`evaluation` of a :term:`compound assignment` proceeds as follows:

#. :def_p:`fls_w3be95jn61ag`
   If the :term:`type`\ s of both :term:`operand`\ s are :term:`primitive type`\
   s, then

#.    #. :def_p:`fls_16ycfcc4sada`
         The :term:`modifying operand` is evaluated.

#.    #. :def_p:`fls_801ycpsxezha`
         The :term:`assigned operand` is evaluated.

#.    #. :def_p:`fls_6yu66ilql5ix`
         The appropriate :term:`function` is invoked as indicated below.

#. :def_p:`fls_j36wyfs449zp`
   Otherwise

#.    #. :def_p:`fls_ewuqlx4thw8p`
         The :term:`assigned operand` is evaluated.

#.    #. :def_p:`fls_yz0fftv4u3i1`
         The :term:`modifying operand` is evaluated.

#.    #. :def_p:`fls_ki9k53vs1kw2`
         The appropriate :term:`function` is invoked as indicated below.

:def_p:`fls_akajeq4ie2lq`
For an :term:`addition assignment`, ``:term:`core::ops::AddAssign::add_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_z9fddlbuh0hc`
For a :term:`bit and assignment`,
``:term:`core::ops::BitAndAssign::bitand_assign`\ (&mut assigned_operand,
modifying_operand)`` is invoked.

:def_p:`fls_l6wp0gx0wgoi`
For a :term:`bit or assignment`, ``:term:`core::ops::BitOrAssign::bitor_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_nt5jdy89nrle`
For a :term:`bit xor assignment`,
``:term:`core::ops::BitXorAssign::bitxor_assign`\ (&mut assigned_operand,
modifying_operand)`` is invoked.

:def_p:`fls_f8wltpmt20yz`
For a :term:`division assignment`, ``:term:`core::ops::DivAssign::div_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_jujx8p5hn9mh`
For a :term:`multiplication assignment`,
``:term:`core::ops::MulAssign::mul_assign`\ (&mut assigned_operand,
modifying_operand)`` is invoked.

:def_p:`fls_2a56j9ceb0n1`
For a :term:`remainder assignment`, ``:term:`core::ops::RemAssign::rem_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_z7i0zv1ivrxc`
For a :term:`shift left assignment`, ``:term:`core::ops::ShlAssign::shl_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_9rria6oqhc5r`
For a :term:`shift right assignment`, ``:term:`core::ops::ShrAssign::shr_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

:def_p:`fls_vfsva3pxg21f`
For a :term:`subtraction assignment`, ``:term:`core::ops::SubAssign::sub_assign`\
(&mut assigned_operand, modifying_operand)`` is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_2nogzbi9gelz`
It is undefined behavior for an :term:`addition assignment`, a :term:`division
assignment`, a :term:`multiplication assignment`, a :term:`remainder
assignment`, or a :term:`subtraction assignment` to cause overflow
with :term:`value`\ s of :term:`numeric type`\ s.

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

:def_p:`fls_9kanjj93iphx`
An :term:`underscore expression` is an :term:`expression` that acts as a
placeholder in a :term:`destructuring assignment`.

:def_p:`fls_njt35aryctoj`
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

:def_p:`fls_fsu1266pqkg7`
A :term:`parenthesized expression` is an :term:`expression` that groups
other :term:`expression`\ s.

:def_p:`fls_eay7tvph38pc`
A :term:`parenthesized expression` is a :term:`place expression` when
its :term:`operand` is a :term:`place expression`.

:def_p:`fls_8seytyb6lgnq`
A :term:`parenthesized expression` is a :term:`value expression` when
its :term:`operand` is a :term:`value expression`.

:def_p:`fls_29euvjdymc48`
The :term:`type` of a :term:`parenthesized expression` is the :term:`type` of
its :term:`operand`.

:def_p:`fls_r2ftj0cf2urm`
The :term:`value` of a :term:`parenthesized expression` is the :term:`value` of
its :term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_l7q2nvgqmn1i`
The :term:`evaluation` of a :term:`parenthesized expression` evaluates
its :term:`operand`.

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

:def_p:`fls_k53nyu3gn3ph`
An :term:`array expression` is an :term:`expression` that constructs
an :term:`array`.

:def_p:`fls_rjjb7xtdib2a`
An :term:`array element constructor` is an :term:`array expression` that lists
all elements of the :term:`array` being constructed.

:def_p:`fls_2jha31unry5i`
An :term:`array repetition constructor` is an :term:`array expression` that
specifies how many times an element is repeated in the :term:`array` being
constructed.

:def_p:`fls_cmbmxw7ljsb4`
A :term:`repeat operand` is an :term:`operand` that specifies the element being
repeated in an :term:`array repetition constructor`.

:def_p:`fls_g16g17tcqia9`
A :term:`size operand` is an :term:`operand` that specifies the size of
an :term:`array` or an :term:`array type`.

:def_p:`fls_pnghsz7ccatp`
An :term:`array expression` is a :term:`value expression`.

:def_p:`fls_3x7qw1xzo6ja`
The :term:`type`\ s of the :term:`operand`\ s of an :term:`array element
constructor` shall be :term:`unifiable`.

:def_p:`fls_gxg4nu1ybnom`
If the :term:`size operand` is greater than one, then
the :term:`type` of the :term:`repeat operand` shall implement
the :codeterm:`core::copy::Copy` :term:`trait` or the :term:`repeat operand`
shall be a :term:`path expression` resolving to a :term:`constant`.

:def_p:`fls_7i4248j4gkpl`
The :term:`type` of the :term:`size operand` shall
be :term:`type` :codeterm:`usize`.

:def_p:`fls_rusgbu49g6gf`
The :term:`value` of the :term:`size operand` shall be a :term:`constant
expression`.

:def_p:`fls_f38utz750ri5`
The :term:`type` of an :term:`array expression` is ``[T; N]``, where ``T``
is the element type and ``N`` is the size of the array. The :term:`size` of
an :term:`array` is determined as follows:

* :def_p:`fls_77a3zkpijuco`
  If the :term:`array expression` appears with an :term:`array element
  constructor`, then the :term:`size` is the number of :term:`operand`\ s in
  the :term:`array element constructor`.

* :def_p:`fls_yu9s3rq0p9k0`
  Otherwise the :term:`size` is the :term:`value` of :term:`size operand`.

:def_p:`fls_ovalisy39s3v`
The :term:`value` of an :term:`array expression` is the
constructed :term:`array`.

.. rubric:: Dynamic Semantics

:def_p:`fls_vexxobv7swju`
The :term:`evaluation` of an :term:`array expression` with an :term:`array
element constructor` evaluates its :term:`operand`\ s in left-to-right order.

:def_p:`fls_mj3li7itah33`
The :term:`evaluation` of an :term:`array expression` with an :term:`array
repetition constructor` proceeds as follows:

#. :def_p:`fls_j54kgcv3n2cm`
   If the :term:`value` of the :term:`size operand` is greater than zero, then:

#.    #. :def_p:`fls_dh4hx78d51nv`
         If the :term:`repeat operand` denotes a :term:`constant`,
         the :term:`repeat operand` is evaluated once and its :term:`value`
         is :term:`copied` :term:`size operand`'s :term:`value` times.

#.    #. :def_p:`fls_ah0mab7ozuv7`
         Otherwise the :term:`repeat operand` is evaluated :term:`size
         operand`'s :term:`value` times.

.. rubric:: Examples

.. code-block:: text

   [1, 2, 3]
   ["one", "two", "three",]

:def_p:`fls_5kd2c5fwj18d`
Two dimensional array.

.. syntax::


   [[0, 0], [0, 1], [1, 0], [1, 1]]


:def_p:`fls_9asbjul5lfwt`
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

:def_p:`fls_zh1uqvezj9wi`
An :term:`array index expression` is an :term:`expression` that indexes into
an :term:`array` or a :term:`slice`.

:def_p:`fls_dq2ca7t7dh7u`
An :term:`indexed array operand` is an :term:`operand` which indicates
the :term:`array` or :term:`slice` being indexed into by an :term:`array index
expression`.

:def_p:`fls_okrxneov445z`
An :term:`indexing operand` is an :term:`operand` which specifies the index of
the :term:`array` or :term:`slice` being indexed into by an :term:`array index
expression`.

:def_p:`fls_fl6gcgret8rf`
An :term:`array index expression` is a :term:`constant expression` if
the :term:`indexing operand` is a :term:`constant expression`.

:def_p:`fls_ouyszdbjbb5q`
The :term:`type` of the :term:`indexing operand` is the :term:`generic
parameter` of the :codeterm:`core::ops::Index` implementation of
the :term:`type` of the :term:`indexed array operand`.

:def_p:`fls_wp405mn8blz9`
If the :term:`indexed array operand` is evaluated in a :term:`value context`,
then

* :def_p:`fls_llae08mi90py`
  The :term:`array index expression` is a :term:`value expression`.

* :def_p:`fls_6ug7ujqbawmh`
  The :term:`type` of the :term:`indexed array operand` shall implement
  the :codeterm:`core::ops::Index` :term:`trait`.

* :def_p:`fls_8a8s1qc5y3dh`
  The :term:`type` of the :term:`array index expression` is ``&T``, where ``T``
  is :term:`associated type` :codeterm:`core::ops::Index::Output`.

:def_p:`fls_3z1oof42f9ov`
If the :term:`indexed array operand` is :term:`mutable` and the :term:`array
index expression` is evaluated in a :term:`mutable place context`, then

* :def_p:`fls_jgte1psut9kp`
  The :term:`array index expression` is a :term:`mutable place expression`.

* :def_p:`fls_h5cdd3tuo4jw`
  The :term:`type` of the :term:`indexed array operand` shall implement
  the :codeterm:`core::ops::IndexMut` :term:`trait`.

* :def_p:`fls_g26tren7x08`
  The :term:`type` of the :term:`array index expression` is ``&mut T``, where
  ``T`` is the element type of the :term:`indexed array operand`'s :term:`type`.

:def_p:`fls_jzvsqljnfnck`
The :term:`value` of an :term:`array index expression` is the indexed memory
location.

.. rubric:: Dynamic Semantics

:def_p:`fls_iq46o6f22krn`
The :term:`evaluation` of an :term:`array index expression` proceeds as follows:

#. :def_p:`fls_lluydryutf1j`
   The :term:`indexed array operand` is evaluated.

#. :def_p:`fls_ijcmjd8tc3sr`
   The :term:`indexing operand` is evaluated.

#. :def_p:`fls_r0gppus03dh9`
   If the :term:`array index expression` is evaluated as
   a :term:`mutable place expression`, then :term:`expression`
   ``*:term:`core::ops::IndexMut::index_mut`\ (&mut indexed_array_operand,
   inexing_operand)`` is evaluated.

#. :def_p:`fls_5ncoioskr3x2`
   Otherwise :term:`expression` ``*:term:`core::ops::Index::index`\
   (&indexed_array_operand, inexing_operand)`` is evaluated.

.. rubric:: Examples

.. code-block:: text

   let a = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
   a[1][2]

:def_p:`fls_jq8jv1igp2r2`
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

:def_p:`fls_onjlfn6p4cec`
A :term:`tuple expression` is an :term:`expression` that constructs
a :term:`tuple`.

:def_p:`fls_3uiooi7jptco`
A :term:`tuple initializer` is an :term:`operand` that provides
the :term:`value` of a :term:`tuple field` in a :term:`tuple expression`.

:def_p:`fls_dswq58q6w2y0`
A :term:`tuple expression` is a :term:`value expression`.

:def_p:`fls_qy8i05ctp1cp`
The :term:`type` of a :term:`tuple expression` is ``(T1, T2, ..., TN)``, where
``T1`` is the :term:`type` of the first :term:`tuple initializer`, ``T2``
is the :term:`type` of the second :term:`tuple initializer`, and ``TN`` is
the :term:`type` of the ``N``-th :term:`tuple initializer`.

:def_p:`fls_p4pl4hke492b`
The :term:`value` of a :term:`tuple expression` is ``(V1, V2, ..., VN)``, where
``V1`` is the :term:`value` of the first :term:`tuple initializer`, ``V2``
is the :term:`value` of the second :term:`tuple initializer`, and ``VN`` is
the :term:`value` of the ``N``-th :term:`tuple initializer`.

:def_p:`fls_83zqclfa3djv`
The :term:`value` of a :term:`tuple expression` without any :term:`tuple
initializer`\ s is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_6bt9hc4599x7`
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

:def_p:`fls_3hynbwaaki2x`
A :term:`struct expression` is an :term:`expression` that constructs
a :term:`struct` or a :term:`union`.

:def_p:`fls_hztf9la8s1al`
A :term:`record struct constructor` is a :term:`struct expression` that
constructs a :term:`record struct`.

:def_p:`fls_t8u71kuvdose`
A :term:`tuple struct constructor` is a :term:`struct expression` that
constructs a :term:`tuple struct`.

:def_p:`fls_1is9e67v2k36`
A :term:`union constructor` is a :term:`struct expression` that constructs
a :term:`union`.

:def_p:`fls_n0k42yjnwwf9`
A :term:`unit struct constructor` is a :term:`struct expression` that constructs
a :term:`unit struct`.

:def_p:`fls_5fe2y0aukf86`
A :term:`base initializer` is a :term:`construct` that specifies
a :term:`struct` to be used as a base for construction in a :term:`struct
expression`.

:def_p:`fls_b20ve5r5kc4s`
An :term:`indexed initializer` is a :term:`construct` that specifies the index
and initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_pitmyp8poliw`
A :term:`named initializer` is a :term:`construct` that specifies the name and
initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_yabwrj7cl0e1`
A :term:`positional initializer` is a :term:`construct` that specifies the
initial :term:`value` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_i5k7fzhijqb5`
A :term:`shorthand initializer` is a :term:`construct` that specifies
the :term:`name` of a :term:`field` in a :term:`struct expression`.

:def_p:`fls_jkt3qtupj3je`
The :term:`construction type` indicates the :term:`type` of the :term:`struct`
being constructed by a :term:`struct expression`.

:def_p:`fls_cwaurrczrml6`
A :syntax:`RecordStructConstructor` without
a :syntax:`RecordStructIndexedFieldInitializerList` is a :term:`record struct
constructor`.

:def_p:`fls_mj7qd4tw14zh`
A :syntax:`UnitStructConstructor` and a :syntax:`RecordStructConstructor`
without a :syntax:`RecordStructInitializer` are :term:`unit struct constructor`\
s.

:def_p:`fls_qj7rxs6t396g`
A :syntax:`TupleStructConstructor` and a :syntax:`RecordStructConstructor`
without a :syntax:`RecordStructFieldInitializerList` are :term:`tuple struct
constructor`\ s.

:def_p:`fls_rm8w6x5dj7ju`
A :syntax:`RecordStructConstructor` with
a :syntax:`RecordStructFieldInitializerList` is a :term:`union constructor`.

:def_p:`fls_dm6f0wiejq9w`
A :term:`struct expression` is a :term:`value expression`.

:def_p:`fls_rybvibmtunm6`
The :term:`type` of a :term:`struct expression` is the :term:`construction
type`.

:def_p:`fls_pjety0kdnlbh`
The :term:`value` of a :term:`struct expression` is the :term:`struct`
or :term:`union` in construction.

:def_p:`fls_l11bcpq5f135`
The :term:`type` of a :term:`base initializer` is the :term:`type` of
its :term:`operand`. The :term:`type` of a :term:`base initializer` shall be the
same as the :term:`construction type`.

.. rubric:: Dynamic Semantics

:def_p:`fls_35lh0kj9vqyz`
The :term:`evaluation` of a :term:`struct expression` evaluates
its :term:`operand`\ s in a left-to-right order.

Record Struct Construction
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_8f6e0jt9dyoa`
The :term:`construction type` of a :term:`struct constructor` shall resolve to
a :term:`struct type`.

:def_p:`fls_g3euz7s8kx9s`
A :term:`named initializer` matches a :term:`field` of the :term:`construction
type` when its :term:`identifier` and the :term:`name` of the :term:`field` are
the same.

:def_p:`fls_9kafbabmk7tq`
The :term:`type` of a :term:`named initializer` and the :term:`type` of the
matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_r81ckmdrqd39`
The :term:`value` of a :term:`named initializer` is the :term:`value` of
its :term:`expression`.

:def_p:`fls_8wt6h3a85lmc`
A :term:`named initializer` that matches a :term:`field` is referred to as
a :def_term:`matched named initializer`.

:def_p:`fls_9n3bwipex35d`
A :term:`shorthand initializer` matches a :term:`field` of
the :term:`construction type` when its :term:`identifier` and the :term:`name`
of the :term:`field` are the same.

:def_p:`fls_rir5drtfddzm`
The :term:`type` of a :term:`shorthand initializer` and the :term:`type` of the
matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_6fk4tcxxa6g7`
The :term:`value` of a :term:`shorthand initializer` is the :term:`value` of
its :term:`identifier`.

:def_p:`fls_ou5wtnxe84n7`
A :term:`shorthand initializer` that matches a :term:`field` is referred to as
a :def_term:`matched shorthand initializer`.

:def_p:`fls_m5eb32aup6nz`
A :term:`shorthand initializer` is equivalent to a :term:`named initializer`
where both the :term:`identifier` and the :term:`expression` of the :term:`named
initializer` denote the :term:`identifier` of the :term:`shorthand initializer`.

:def_p:`fls_m9pe5iqjdd9h`
For each :term:`field` of the :term:`construction type`, the :term:`record
struct constructor` shall either:

* :def_p:`fls_qcuz4cuvcyk5`
  Contain a :term:`matched named initializer`, or

* :def_p:`fls_al974u2353t0`
  Contain a :term:`matched shorthand initializer`, or

* :def_p:`fls_me0nakqzuueh`
  Have a :syntax:`RecordStructInitializer` with a :term:`base initializer` or
  a :syntax:`RecordStructFieldInitializerList` with a :term:`base initializer`.

:def_p:`fls_wokc6s72enfz`
The :term:`value` of a :term:`field` of a :term:`struct` in construction shall
be either:

* :def_p:`fls_f8ydq2iu04lp`
  The :term:`value` of a :term:`matched named initializer`, or

* :def_p:`fls_evh74giehr3v`
  The :term:`value` of a :term:`matched shorthand initializer`, or

* :def_p:`fls_wm25skt7uvui`
  The :term:`value` of the corresponding :term:`field` of the :term:`struct`
  indicated by the :term:`base initializer`, where the :term:`value` is
  either :term:`copied` or :term:`moved`.

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


:def_p:`fls_eagvzkvsgifv`
Matched named initializer.

.. syntax::


       name: "Bob".to_string(),

:def_p:`fls_nrgs75ye45kl`
Matched shorthand initializer.

.. code-block:: text

       age,

:def_p:`fls_bis858tssyrh`
Base initializer, equivalent to ``alice.occupation`` and ``alice.compensation``.

.. code-block:: text

       .. alice
   };

Tuple Struct Construction
~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_qwn4ei4r7sui`
The :term:`construction type` of a :term:`tuple struct constructor` shall
resolve to a :term:`tuple struct type`.

:def_p:`fls_p0tuxpxntb3`
A :term:`tuple struct constructor` shall contain one :term:`positional
initializer` for each :term:`field` of the :term:`construction type`.

:def_p:`fls_t8jcn2rxvfhl`
A :term:`positional initializer` matches a :term:`field` of
the :term:`construction type` when the position of the :term:`positional
initializer` and the position of the :term:`field` in the :term:`construction
type` are the same.

:def_p:`fls_rcrzqmqf17a`
The :term:`type` of the :term:`operand` of a :term:`positional initializer` and
the :term:`type` of the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_kw26qcylzkar`
The :term:`value` of a :term:`positional initializer` is the :term:`value` of
its :term:`operand`.

:def_p:`fls_jclhofi15w6l`
A :term:`positional initializer` that matches a :term:`field` is referred to as
a :def_term:`matched positional initializer`.

:def_p:`fls_v2j53t89rolc`
The :syntax:`RecordStructIndexedFieldInitializerList` of a :term:`record struct
constructor` shall:

* :def_p:`fls_yijaglmuo51m`
  Contain an :term:`indexed initializer` for each :term:`field` of
  the :term:`construction type`, covering all indices of the :term:`construction
  type`, or

* :def_p:`fls_oc1wvw8ouciz`
  Have a :syntax:`RecordStructInitializer` with a :term:`base initializer`
  or a :syntax:`RecordStructIndexedFieldInitializerList` with a :term:`base
  initializer`\ ``.``

:def_p:`fls_4weezk7uaquh`
An :term:`indexed initializer` matches a :term:`field` of
the :term:`construction type` when the :term:`tuple index` of the :term:`indexed
initializer` resolves to a valid position of a :term:`field` in
the :term:`construction type`.

:def_p:`fls_2b6d0ki5l6u8`
The :term:`type` of the :term:`operand` of an :term:`indexed initializer` and
the :term:`type` of the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_9dln71z9jpt3`
The :term:`value` of an :term:`indexed initializer` is the :term:`value` of
its :term:`operand`.

:def_p:`fls_l9cvb4padp80`
An :term:`indexed initializer` that matches a :term:`field` is referred to as
a :def_term:`matched indexed initializer`.

:def_p:`fls_1d3fwpu1y39c`
The :term:`value` of a :term:`field` of a :term:`tuple in construction` is
either:

* :def_p:`fls_az5f1x8hxz4y`
  The :term:`value` of a :term:`matched indexed initializer`, or

* :def_p:`fls_ad08wqst0x01`
  The :term:`value` of a :term:`matched positional initializer`, or

* :def_p:`fls_9h5dvsm75un8`
  The :term:`value` of the corresponding :term:`field` of the :term:`tuple
  indicated` by the :term:`base initializer`, where the :term:`value` is
  either :term:`copied` or :term:`moved`.

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


:def_p:`fls_5yy7kukgwho7`
Matched indexed initializer.

.. code-block:: text


       1: 1.1,

:def_p:`fls_e3m30dq1c3u3`
Base initializer, equivalent to ``origin.0`` and ``origin.2``.

.. code-block:: text

       .. origin
   };

Union Construction
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_r4py5j3w0vo0`
The :term:`construction type` of a :term:`union constructor` shall resolve to
a :term:`union type`.

:def_p:`fls_8lxwo9gr30wb`
The :syntax:`RecordStructFieldInitializerList` of a :term:`union constructor`
shall contain exactly one :syntax:`RecordStructFieldInitializer` and
no :term:`base initializer`.

:def_p:`fls_cpfps2wvc0xo`
For the single :term:`field` of the :term:`construction type`, a :term:`unit
constructor` shall either:

* :def_p:`fls_z4m8bik34hu2`
  Contain a :term:`matched named initializer`, or

* :def_p:`fls_1cdvh2wr5zcc`
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

:def_p:`fls_wqi3xeezzgdi`
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

:def_p:`fls_sk6s5jsrnhde`
A :term:`call expression` is an :term:`expression` that invokes
a :term:`function`.

:def_p:`fls_wz3464l9x24g`
An :term:`argument operand` is an :term:`operand` which is used as a parameter
in a :term:`call expression` or a :term:`method call expression`.

:def_p:`fls_nixbtmndu8bp`
The :term:`type` of a :term:`call operand` shall be a :term:`type` that
implements any of the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`,
or :codeterm:`core::ops::FnOnce` :term:`trait`\ s.

:def_p:`fls_604nrreckqaz`
The :term:`type` of a :term:`call expression` is the :term:`return
type` of the :term:`invoked function` or :term:`associated
type` :codeterm:`core::ops::FnOnce::Output`.

:def_p:`fls_gmriikvsdh8y`
The :term:`value` of a :term:`call expression` is determined as follows:

* :def_p:`fls_vy6d3jnsp85d`
  If the :term:`type` of the :term:`call operand` implements
  the :codeterm:`core::ops::Fn` :term:`trait`, then the :term:`value` is
  the result of invoking ``:term:`core::ops::Fn::call`\ (call_operand,
  argument_operand_tuple)``, where ``call_operand`` is :term:`borrowed` if it
  is not a :term:`reference`, and ``argument_operand_tuple`` is a :term:`tuple`
  that wraps the :term:`argument operand`\ s.

* :def_p:`fls_drjbh9etn293`
  If the :term:`type` of the :term:`call operand` implements
  the :codeterm:`core::ops::FnMut` :term:`trait`, then the :term:`value` is the
  result of invoking ``:term:`core::ops::FnMut::call_mut`\ (&mut call_operand,
  argument_operand_tuple)`` where ``call_operand`` is :term:`mutably borrowed`
  if it is not a :term:`mutable reference`, and ``argument_operand_tuple`` is
  a :term:`tuple` that wraps the :term:`argument operand`\ s.

* :def_p:`fls_tcvumkr74u3k`
  If the :term:`type` of the :term:`call operand` implements
  the :codeterm:`core::ops::FnOnce` :term:`trait`, then the :term:`value` is
  the result of invoking ``:term:`core::ops::FnOnce::call_once`\ (call_operand,
  argument_operand_tuple)`` where ``argument_operand_tuple`` is a :term:`tuple`
  that wraps the :term:`argument operand`\ s.

.. rubric:: Dynamic Semantics

:def_p:`fls_7tlxhjrcrmp3`
The :term:`evaluation` of a :term:`call expression` proceeds as follows:

#. :def_p:`fls_sx8tp5xo0dmh`
   The :term:`call operand` is evaluated.

#. :def_p:`fls_ea3b6tpco5ll`
   The :term:`argument operand`\ s are evaluated in left-to-right order.

#. :def_p:`fls_r29styttncmp`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::Fn` :term:`trait`, then
   ``:term:`core::ops::Fn::call`\ (&call_operand, argument_operands)`` is
   invoked.

#. :def_p:`fls_chzndcdkjfr`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::FnMut` :term:`trait`, then
   ``:term:`core::ops::FnMut::call_mut`\ (&mut call_operand,
   argument_operands)`` is invoked.

#. :def_p:`fls_v4rc844ouy9a`
   If the :term:`type` of the :term:`call operand` implements
   the :codeterm:`core::ops::FnOnce` :term:`trait`, then
   ``:term:`core::ops::FnOnce::call_once`\ (call_operand, argument_operands)``
   is invoked.

.. rubric:: Undefined Behavior

:def_p:`fls_isylx75c37vf`
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

:def_p:`fls_td4li5ncnn3s`
A :term:`method call expression` is an :term:`expression` that invokes
a :term:`method` of an :term:`object`.

:def_p:`fls_fsaaw1bp7vul`
A :term:`receiver operand` is an :term:`operand` that denotes the :term:`value`
whose :term:`method` is being invoked by a :term:`method call expression`.

:def_p:`fls_xgtw0and19h7`
A :term:`method call expression` is subject to :term:`method resolution`
(**should this be stated explicitly?**).

:def_p:`fls_vy62rmvj2lct`
The :term:`type` of a :term:`method call expression` is the :term:`return type`
of the invoked :term:`method`.

:def_p:`fls_5ml751f0020s`
The :term:`value` of a :term:`method call expression` is the :term:`value`
returned by the invoked :term:`method`.

.. rubric:: Dynamic Semantics

:def_p:`fls_kgj5nqs2bnaq`
The :term:`evaluation` of a :term:`method call expression` proceeds as follows:

#. :def_p:`fls_cm4yzli6sodn`
   The :term:`receiver operand` is evaluated.

#. :def_p:`fls_j9csvgbadvez`
   The :term:`argument operand`\ s are evaluated in left-to-right order.

#. :def_p:`fls_w558l2hd5mbr`
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

:def_p:`fls_hblz1n1es4tz`
A :term:`field access expression` is an :term:`expression` that accesses
a :term:`field` of an :term:`object`.

:def_p:`fls_9shw05xhwvm7`
A :term:`container operand` is an :term:`operand` that indicates
the :term:`object` whose :term:`field` is selected in a :term:`field access
expression`.

:def_p:`fls_erkmvrizy456`
A :term:`field selector` is a :term:`construct` that selects the :term:`field`
to be accessed in a :term:`field access expression`.

:def_p:`fls_oby9qirxqiwi`
A :term:`selected field` is a :term:`field` that is selected by a :term:`field
access expression`.

:def_p:`fls_dsgn6gufvzhh`
A :term:`field access expression` with an :syntax:`IndexedFieldSelector` is
referred to as an :def_term:`indexed field access`.

:def_p:`fls_afm7xj5j3jfn`
A :term:`field access expression` with a :syntax:`NamedFieldSelector` is
referred to as a :def_term:`named field access`.

:def_p:`fls_vmnzvla4he1n`
A :term:`field access expression` is a :term:`place expression`.

:def_p:`fls_9xwjvebk5gcc`
A :term:`field access expression` is a :term:`mutable place expression` when
its :term:`container operand` is :term:`mutable`.

:def_p:`fls_3lsgmbb2iwwu`
The :term:`type` of a :term:`field access expression` is the :term:`type` of
the :term:`selected field`.

:def_p:`fls_b0iyvhwkinbg`
The :term:`value` of a :term:`field access expression` is the :term:`value` of
the :term:`selected field`.

:def_p:`fls_g6ju61bs6zwt`
**TODO: Autoderef, revisit after names chapter. It might go there or here.
Therefor we still need to talk about the container type here and the validity of
the accessed field **

:def_p:`fls_nvr2xxglbwzk`
**Moves, borrows, copy, drop -> `borrowing <https://doc.rust-lang.org/stable/
reference/expressions/field-expr.html#borrowing>`_ where does this go.**

.. rubric:: Dynamic Semantics

:def_p:`fls_dvw5w6xokmgn`
The :term:`evaluation` of a :term:`field access expression` evaluates
its :term:`container operand`.

Named Field Access
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_s204len01q0t`
Reading the :term:`selected field` of a :term:`union` shall
require :term:`unsafe context`.

:def_p:`fls_o1rurfc7jjqt`
Writing to the :term:`selected field` of a :term:`union`
where the :term:`type` of the :term:`selected field`
implements the :codeterm:`core::marker::Copy` :term:`trait` or
the :codeterm:`core::mem::ManuallyDrop` :term:`trait` shall require :term:`safe
context`.

:def_p:`fls_kkggymvboads`
Writing to and then reading from the :term:`selected field` of
a :term:`union` subject to :term:`attribute` :codeterm:`repr` is equivalent
to invoking :term:`function` ``:term:`core::mem::transmute`\ <write_type,
read_type>(field_bits)`` where ``write_type`` is the :term:`type` used at the
time of writing the :term:`selected field`, ``read_type`` is the :term:`type`
used at the time of reading the :term:`selected field`, and ``field_bits`` is
the bit representation of the :term:`selected field`.

.. rubric:: Undefined Behavior

:def_p:`fls_8qqag4ee5vd7`
It is undefined behavior when the :term:`type` of the :term:`container operand`
is a :term:`union type` and the :term:`selected field` contains invalid data.

.. rubric:: Examples

:def_p:`fls_wn5mdxs2t4cl`
See :p:`6.8.1. <fls_hv4grs2tcuiw>` for the declaration of ``alice``.

.. code-block:: text

   alice.name

Indexed Field Access
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_njizrszfoq6l`
The :term:`decimal literal` of an :term:`indexed field access` shall denote a
valid index of a :term:`field` of the :term:`container operand`'s :term:`type`.

.. rubric:: Examples

:def_p:`fls_e9nkqh434lqd`
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

:def_p:`fls_h1i1m5dx6cl2`
A :term:`closure expression` is an :term:`expression` that defines
a :term:`closure type`.

:def_p:`fls_ug1cx3svhu65`
A :term:`closure body` is a :term:`construct` that represents the executable
portion of a :term:`closure expression`.

:def_p:`fls_o95klu3nggfq`
A :term:`closure body` denotes a new :term:`control flow boundary`.

:def_p:`fls_khvp3ze1nmd2`
A :term:`closure body` is subject to :term:`capturing`.

:def_p:`fls_j5rd2up4seda`
The :term:`type` of a :term:`closure expression` is the anonymous
unique :term:`closure type` defined by it.

:def_p:`fls_5in52eyutbyj`
The :term:`value` of a :term:`closure expression` is the :term:`value`
of the anonymous unique :term:`closure type` instantiated with the
selected :term:`capture`\ s.

.. rubric:: Dynamic Semantics

:def_p:`fls_arqeltuxbblg`
The :term:`evaluation` of a :term:`closure expression` proceeds as follows:

#. :def_p:`fls_ganwjavzkto5`
   An anonymous :term:`value` of an anonymous unique :term:`closure type` is
   created.

#. :def_p:`fls_zgduxx8uwbkb`
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

:def_p:`fls_ppw4u3baxehu`
A :term:`loop expression` is an :term:`expression` that evaluates a :term:`block
expression` continuously as long as some criterion holds true.

:def_p:`fls_24slax8tm897`
An :term:`anonymous loop` is a :term:`loop expression` without a :term:`label`.

:def_p:`fls_z9st5w51l7pg`
A :term:`named loop` is a :term:`loop expression` with a :term:`label`.

:def_p:`fls_yoadq4wumj35`
The :term:`type` of a :term:`loop expression` is determined as follows:

* :def_p:`fls_4zmqrpdse6d8`
  If the :term:`loop expression` does not contain a :term:`break expression`,
  then the :term:`type` is the :term:`never type`.

* :def_p:`fls_k0modxozurn`
  If the :term:`loop expression` contains at least one :term:`break expression`,
  then the :term:`type` is the :term:`unified type` of the :term:`break type`\ s
  of all :term:`break expression`\ s.

:def_p:`fls_yhh81oc88hem`
The :term:`value` of a :term:`loop expression` is determined as follows:

* :def_p:`fls_97dqqrii3ayj`
  If the :term:`loop expression` does not contain a :term:`break expression`,
  then the :term:`value` is the :term:`unit value`.

* :def_p:`fls_w1qshymefrub`
  If the :term:`loop expression` contains at least one :term:`break expression`,
  then the :term:`value` is the :term:`break value` of the :term:`break
  expression` that broke out of the :term:`loop expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_t23fiqi1ibs`
A :term:`loop expression` is :term:`terminated` when its :term:`block
expression` is no longer evaluated.

For Loops
~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ForLoopExpression ::=
       $$for$$ Pattern $$in$$ SubjectExpression BlockExpression

.. rubric:: Legality Rules

:def_p:`fls_v6g5bt60zrmw`
A :term:`for loop expression` is a :term:`loop expression` that continues to
evaluate its :term:`block expression` as long as its :term:`subject expression`
yields a :term:`value`.

:def_p:`fls_f03k0qp5y15l`
The :term:`type` of a :term:`subject expression` shall implement
the :codeterm:`core::iter::IntoIterator` :term:`trait`.

.. rubric:: Dynamic Semantics

:def_p:`fls_tj8qo28s1bxi`
The :term:`evaluation` of a :term:`for loop expression` of the form

.. code-block:: text

   'label: for pattern in subject_expression {
       /* loop body */
   }

:def_p:`fls_3k73t3ofbz9p`
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

:def_p:`fls_e9u5l4bdx4ey`
An :term:`infinite loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` indefinitely unless :term:`terminated`
with a :term:`break expression` or a :term:`return expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_62z78afqz5h`
The :term:`evaluation` of an :term:`infinite loop expression` proceeds as
follows:

#. :def_p:`fls_nrit7jm24jdz`
   The :term:`block expression` is evaluated.

#. :def_p:`fls_f4lyj61pv6wk`
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

:def_p:`fls_u3k6hxze75dx`
A :term:`while loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` as long as its :term:`iteration
expression` holds true.

:def_p:`fls_ja23c0isgaq`
An :term:`iteration expression` is an :term:`expression` that provides the
criterion of a :term:`while loop expression`.

:def_p:`fls_ayfzc6pnq5rh`
The :term:`type` of an :term:`iteration expression` shall
be :term:`type` :codeterm:`bool`.

.. rubric:: Dynamic Semantics

:def_p:`fls_bow5qlcn3qoz`
The :term:`evaluation` of a :term:`while loop expression` proceeds as follows:

#. :def_p:`fls_avjvg3td8etk`
   The :term:`iteration expression` is evaluated.

#. :def_p:`fls_4633qltmar22`
   If the :term:`iteration expression` evaluated to ``true``, then:

#.    #. :def_p:`fls_8l7i5frodsva`
         The :term:`block expression` is evaluated.

#.    #. :def_p:`fls_7q3g92emkuca`
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

:def_p:`fls_1mrfjd111mo9`
A :term:`while let loop expression` is a :term:`loop expression` that continues
to evaluate its :term:`block expression` as long as its :term:`subject
let expression` yields a :term:`value` that can be matched against
its :term:`pattern`.

.. rubric:: Dynamic Semantics

:def_p:`fls_8kndatvww75d`
The :term:`evaluation` of a :term:`while let loop expression` of the form

.. code-block:: text

   'label: let pattern = subject_let_expression {
       /* loop body */
   }

:def_p:`fls_i2s7c538624x`
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

:def_p:`fls_hyn01hia0km4`
A :term:`label indication` is a :term:`construct` that indicates
a :term:`label`.

:def_p:`fls_fh7jzags4fnu`
A :term:`label indication` shall indicate a :term:`label` of an
enclosing :term:`named loop` that does not pass a :term:`control flow boundary`
in order to reach the enclosing :term:`named loop`.

Loop Control Flow
~~~~~~~~~~~~~~~~~

Break Expressions
^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   BreakExpression ::=
       $$break$$ LabelIndication? Operand?

.. rubric:: Legality Rules

:def_p:`fls_u5t9x37mx48g`
A :term:`break expression` is an :term:`expression` that terminates
a :term:`loop expression`.

:def_p:`fls_yvl9goh02mpl`
A :term:`break expression` shall appear within a :term:`loop expression`.

:def_p:`fls_8dxud6la0v2h`
The :term:`label indication` of a :term:`break expression` shall resolve to
the :term:`label` of an enclosing :term:`named loop`.

:def_p:`fls_sgw7w94ihp5w`
A :term:`break expression` without a :term:`label indication` is associated with
the innermost enclosing :term:`loop expression`.

:def_p:`fls_3xh5g6g0osqs`
A :term:`break expression` with a :term:`label indication` is associated with
a :term:`named loop` whose :term:`label` is indicated by the :term:`label
indication`.

:def_p:`fls_bpipoo36cki4`
A :term:`break expression` shall have an :term:`operand` only when it is
associated with an :term:`infinite loop`.

:def_p:`fls_nd0s4528erde`
The :term:`type` of a :term:`break expression` is the :term:`never type`.

:def_p:`fls_554zrnxrjb5q`
The :def_term:`break type` of a :term:`break expression` is determined as
follows:

* :def_p:`fls_bd2h1bi4wxky`
  If the :term:`break expression` lacks an :term:`operand`, then
  the :term:`break type` is the :term:`unit type`.

* :def_p:`fls_m8cjd6u2a0m`
  If the :term:`break expression` has an :term:`operand`, then the :term:`break
  type` is the :term:`type` of its :term:`operand`.

:def_p:`fls_8oeg0tqvycgw`
The :def_term:`break value` of a :term:`break expression` is determined as
follows:

* :def_p:`fls_clmufzz9m58d`
  If the :term:`break expression` lacks an :term:`operand`, then
  the :term:`break value` is the :term:`unit value`.

* :def_p:`fls_uy7w773ub6l4`
  If the :term:`break expression` has an :term:`operand`, then the :term:`break
  value` is the :term:`value` of its :term:`operand`.

.. rubric:: Dynamic Semantics

:def_p:`fls_ndfffn3pj3o9`
The :term:`evaluation` of a :term:`break expression` proceeds as follows:

#. :def_p:`fls_ywbl1p6wtyl9`
   The :term:`operand` is evaluated.

#. :def_p:`fls_nmlv8ybiq23h`
   All enclosing :term:`loop expression`\ s upto and including the
   associated :term:`loop expression` are :term:`terminated`.

.. rubric:: Examples

:def_p:`fls_dd1y8uxsat0y`
The following break expression terminates both the inner and the outer loop.

.. code-block:: text

   'outer: loop {
       'inner: loop {
           break 'outer;
       }
   }

Continue Expressions
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ContinueExpression ::=
       $$continue$$ LabelIndication?

.. rubric:: Legality Rules

:def_p:`fls_d9ukpg66u20u`
A :term:`continue expression` shall appear within a :term:`loop expression`.

:def_p:`fls_n504p3it8snk`
A :term:`continue expression` without a :term:`label indication` is associated
with the innermost enclosing :term:`loop expression`.

:def_p:`fls_97sloklf9yby`
A :term:`continue expression` with a :term:`label indication` is associated
with a :term:`named loop` whose :term:`label` is indicated by the :term:`label
indication`.

:def_p:`fls_iclevs643f47`
The :term:`type` of a :term:`continue expression` is the :term:`never type`.

:def_p:`fls_ped8n2mkj9q5`
The :term:`value` of a :term:`continue expression` is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_ulpgds13ody1`
The :term:`evaluation` of a :term:`continue expression` proceeds as follows:

#. :def_p:`fls_83qrqueytt96`
   If the :term:`continue expression` appears with a :term:`label indication`,
   then all enclosing :term:`loop expression`\ s upto and including the
   associated :term:`loop expression` are :term:`terminated`.

#. :def_p:`fls_8x7vmdne8kfm`
   The :term:`evaluation` of the associated :term:`loop expression` is
   restarted.

.. rubric:: Examples

:def_p:`fls_rzh2wu4yliiv`
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

:def_p:`fls_s8pp534vpbj7`
A :def_syntax:`RangeExpression` shall denote either a ``RangeFromExpression``,
a ``RangeFromToExpression``, a ``RangeFullExpression``, a
``RangeInclusiveExpression``, a ``RangeToExpression``, or a
``RangeToInclusiveExpression``.

:def_p:`fls_b2va7mquxj7z`
A :def_syntax:`RangeFromExpression` shall start with a
``RangeExpressionLowBound``, followed by character sequence 0x2E 0x2E (full
stop, full stop).

:def_p:`fls_4kyba48kau76`
A :def_syntax:`RangeFromToExpression` shall start with a
``RangeExpressionLowBound``, followed by character sequence 0x2E 0x2E (full
stop, full stop), followed by a ``RangeExpressionHighBound``.

:def_p:`fls_w1et7qjhf0iy`
A :def_syntax:`RangeFullExpression` shall denote character sequence 0x2E 0x2E
(full stop, full stop).

:def_p:`fls_ajp7n4f75pmu`
A :def_syntax:`RangeInclusiveExpression` shall start with a
``RangeExpressionLowBound``, followed by character sequence 0x2E 0x2E 0x3D (full
stop, full stop, equals sign), followed by a ``RangeExpressionHighBound``.

:def_p:`fls_pfzia4k2plfu`
A :def_syntax:`RangeToExpression` shall start with character sequence 0x2E 0x2E
(full stop, full stop), followed by an ``Expression``.

:def_p:`fls_ca4nsk8ys0ld`
A :def_syntax:`RangeToInclusiveExpression` shall start with character
sequence 0x2E 0x2E 0x3D (full stop, full stop, equals sign), followed by a
``RangeExpressionHighBound``.

:def_p:`fls_o6b7n1sk6mtm`
A :def_syntax:`RangeExpressionLowBound` shall denote an :syntax:`Operand`.

:def_p:`fls_eix9mfurkgu7`
A :def_syntax:`RangeExpressionHighBound` shall denote an ``Operand``.

:def_p:`fls_ox9182ac8tyo`
A :term:`range expression` is an :term:`expression` that constructs a range.

:def_p:`fls_wyt6gaq2d5v5`
A :term:`range expression low bound` is an :term:`operand` that specifies the
start of a range.

:def_p:`fls_bc6ijpnhcay0`
A :term:`range expression high bound` is an :term:`operand` that specifies the
end of a range.

:def_p:`fls_xvxafg5feve1`
If a :term:`range expression` has two :term:`operand`\ s, then the :term:`type`\
s of the :term:`operand`\ s shall be :term:`unifiable`.

:def_p:`fls_jpc0yfp3nxa2`
A :term:`range-from expression` is a :term:`range expression` that specifies an
included :term:`range expression low bound`.

:def_p:`fls_bbqwi0lq914x`
The :term:`type` of a :term:`range-from expression`
is :codeterm:`core::ops::RangeFrom`.

:def_p:`fls_odglma6qkpwe`
The :term:`value` of a :term:`range-from expression` is
``:term:`core::ops::RangeFrom` { start: range_expression_low_bound }``.

:def_p:`fls_sw2r5lad5ex`
A :term:`range-from-to expression` is a :term:`range expression` that specifies
an included :term:`range expression low bound` and an excluded :term:`range
expression high bound`.

:def_p:`fls_po0hv7fyauji`
The :term:`type` of a :term:`range-from-to expression`
is :codeterm:`core::ops::Range`.

:def_p:`fls_70coz76ttetg`
The :term:`value` of a :term:`range-from-to expression` is
``:term:`core::ops::Range` { start: range_expression_low_bound, end:
range_expression_high_bound }``.

:def_p:`fls_rewfdvbai6hj`
A :term:`range-full expression` is a :term:`range expression` that covers the
whole range of a :term:`type`.

:def_p:`fls_q5c34nmoagea`
The :term:`type` of a :term:`range-full expression`
is :codeterm:`core::ops::RangeFull`.

:def_p:`fls_21suize4fih`
The :term:`value` of a :term:`range-full expression` is
``:term:`core::ops::RangeFull` {}``.

:def_p:`fls_1qlka7uj7icp`
A :term:`range-inclusive expression` is a :term:`range expression`
that specifies an included :term:`range expression low bound` and an
included :term:`range expression high bound`.

:def_p:`fls_zc4ww66tlyld`
The :term:`type` of a :term:`range-inclusive expression`
is :codeterm:`core::ops::RangeInclusive`.

:def_p:`fls_fhkkddcrhqj`
The :term:`value` of a :term:`range-inclusive expression` is
``:term:`core::ops::RangeInclusive::new`\ (range_expression_low_bound,
range_expression_high_bound)``.

:def_p:`fls_j6mfgh585gdt`
A :term:`range-to expression` is a :term:`range expression` that specifies an
excluded :term:`range expression high bound`.

:def_p:`fls_4a8ewg8wu0s0`
The :term:`type` of a :term:`range-to expression`
is :codeterm:`core::ops::RangeTo`.

:def_p:`fls_l49c1z8m51s8`
The :term:`value` of a :term:`range-to expression` is
``:term:`core::ops::RangeTo` { end: range_expression_high_bound }``.

:def_p:`fls_cyqffxzhakdp`
A :term:`range-to-inclusive expression` is a :term:`range expression` that
specifies an included :term:`range expression high bound`.

:def_p:`fls_nlrq8bcwqccb`
The :term:`type` of a :term:`range-to-inclusive expression`
is :codeterm:`core::ops::RangeToInclusive`.

:def_p:`fls_1zlhb0j3bn2a`
The :term:`value` of a :term:`range-to-inclusive expression` is
``:term:`core::ops::RangeToInclusive` { end: range_expression_high_bound }``.

.. rubric:: Dynamic Semantics

:def_p:`fls_esehzs6tb856`
The :term:`evaluation` of a :term:`range expression` evaluates
its :term:`operand`\ s in left-to-right order.

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

:def_p:`fls_xe4dcvywr46y`
An :term:`if expression` is an :term:`expression` that evaluates either
a :term:`block expression` or an :term:`else expression` depending on
the :term:`value` of its :term:`subject expression`.

:def_p:`fls_9bw6az2k27qm`
An :term:`else expression` is an :term:`expression` that represents either
a :term:`block expression`, an :term:`if expression`, or an :term:`if let
expression`.

:def_p:`fls_6gr2q6bhtx4s`
The :term:`type` of the :term:`subject expression` of an :term:`if expression`
shall be :term:`type` :codeterm:`bool`.

:def_p:`fls_nbd52gi405zd`
The :term:`type` of an :term:`if expression` is the :term:`type` of
its :term:`block expression`.

:def_p:`fls_bggxq3b3sdeb`
The :term:`value` of an :term:`if expression` is the :term:`value` of
its :term:`block expression`.

:def_p:`fls_51kj4r4j5r2m`
The :term:`type` of an :term:`else expression` is the :term:`type` of
its :term:`block expression`, :term:`if expression`, or :term:`if let
expression`.

:def_p:`fls_s940ol7ok1td`
The :term:`value` of an :term:`else expression` is the :term:`value` of
its :term:`block expression`, :term:`if expression`, or :term:`if let
expression`.

:def_p:`fls_s5lj2mdvsvii`
The :term:`type` of an :term:`if expression` and the :term:`type` of
an :term:`else expression` shall be :term:`unifiable`.

.. rubric:: Dynamic Semantics

:def_p:`fls_h9ghghh5i00x`
The :term:`evaluation` of an :term:`if expression` proceeds as follows:

#. :def_p:`fls_x337m7ht9uah`
   The :term:`subject expression` is evaluated.

#. :def_p:`fls_x7u9oo83b9s`
   If the :term:`subject expression` evaluated to ``true``, then
   the :term:`block expression` is evaluated.

#. :def_p:`fls_xc2lf6jbtnax`
   If the :term:`subject expression` evaluated to ``false`` and the :term:`if
   expression` has an :term:`else expression`, then the :term:`else expressio`\
   n is evaluated.

:def_p:`fls_tdji2z6s335f`
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

:def_p:`fls_foktg3bhfisg`
An :term:`if let expression` is an :term:`expression` that evaluates either
a :term:`block expression` or an :term:`else expression` depending on whether
its :term:`pattern` can be matched against its :term:`subject let expression`.

:def_p:`fls_jxv67qz4ium9`
The :term:`type` of an :term:`if let expression` is the :term:`type` of
its :term:`block expression`.

:def_p:`fls_p5jiysxbhxeg`
The :term:`value` of an :term:`if let expression` is the :term:`value` of
its :term:`block expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_q3iorry7aian`
The :term:`evaluation` of an :term:`if let expression` of the form

.. code-block:: text

   if let pattern = subject_let_expression {
       /* body */
   }

:def_p:`fls_gzvb8n5m98tb`
is equivalent to the :term:`evaluation` of the following :term:`match
expression`:

.. code-block:: text

   match subject_let_expression {
       pattern => { /* body */ },
       _ => ()
   }

:def_p:`fls_nb2sdeg2t9cs`
The :term:`evaluation` of an :term:`if let expression` of the form

.. code-block:: text

   if let pattern = subject_let_expression {
       /* body */
   } else {
       /* else */
   }

:def_p:`fls_b2pgfxw89h07`
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

:def_p:`fls_mh3xhvxba1ay`
A :term:`match expression` is an :term:`expression` that tries to match one of
its multiple :term:`pattern`\ s against its :term:`subject expression` and if it
succeeds, evaluates an :term:`operand`.

:def_p:`fls_r53pyuc8s6l6`
A :term:`match arm` is a :term:`construct` that consists of a :term:`match arm
matcher` and a :term:`match arm body`.

:def_p:`fls_lt86i1g4i9j7`
An :term:`intermediate match arm` is any non-:term:`final match arm` of
a :term:`match expression`.

:def_p:`fls_mx8snur13ydz`
A :term:`final match arm` is the last :term:`match arm` of a :term:`match
expression`.

:def_p:`fls_1hdbhk44ueeo`
A :term:`match arm matcher` is a :term:`construct` that consists of
a :term:`pattern` and a :term:`match arm guard`.

:def_p:`fls_wrgz2m3wu5oo`
A :term:`match arm body` is the :term:`operand` of a :term:`match arm`.

:def_p:`fls_blsbrkipc1rt`
A :term:`match arm guard` is a :term:`construct` that provides additional
filtering to a :term:`match arm matcher`.

:def_p:`fls_b2jiramqiwkj`
A :term:`match expression` is a :term:`place expression` when its :term:`subject
expression` is a :term:`place expression`. When a :term:`match expression`
is a :term:`place expression`, the :term:`value` produced by evaluating
its :term:`subject expression` is :term:`copied` or :term:`moved`.

:def_p:`fls_ea026ki38gm0`
A :term:`match expression` is a :term:`value expression` when its :term:`subject
expression` is a :term:`value expression`. When the :term:`match expression`
is a :term:`value expression`, the :term:`value` produced by evaluating
its :term:`subject expression` is captured in a :term:`temporary`.

:def_p:`fls_46o6ntuxikji`
The :term:`type` of the :term:`subject expression` and the :term:`type`\
s of all :term:`pattern`\ s of all :term:`match arm matcher`\ s shall
be :term:`unifiable`.

:def_p:`fls_sa5rlq304vhz`
The :term:`type` of the :term:`operand` of a :term:`match arm guard` shall
be :term:`type` :codeterm:`bool`.

:def_p:`fls_l1rqwu30rmnl`
The :term:`type`\ s of all :term:`match arm bodies` shall be :term:`unifiable`.

:def_p:`fls_m4g33xiaxmnk`
The :term:`type` of a :term:`match expression` is the :term:`unified type` of
the :term:`type`\ s of the :term:`operand`\ s of all :term:`match arm`\ s.

:def_p:`fls_z5cxscesmmg0`
A :term:`match arm` is selected when its :term:`pattern` matches
the :term:`subject expression` and its :term:`match arm guard` (if any)
evaluates to ``true``.

:def_p:`fls_ogua6lu9sgi7`
:term:`Match arm` selection happens in declarative order.

:def_p:`fls_oujfw74kl9fv`
The :term:`pattern`\ s of all :term:`match arm`\ s taken together shall
exhaustively match the :term:`subject expression`'s :term:`type`.

:def_p:`fls_mi4m1b4uxvhl`
The :term:`value` of a :term:`match expression` is the :term:`value` of
the :term:`operand` of the selected :term:`match arm`.

.. rubric:: Dynamic Semantics

:def_p:`fls_6tzux85cufmp`
The :term:`evaluation` of a :term:`match expression` proceeds as follows:

#. :def_p:`fls_qjjan8qu353x`
   The :term:`subject expression` is evaluated.

#. :def_p:`fls_qlixsnga607h`
   Each :term:`match arm` is evaluated in declarative order as follows:

#.    #. :def_p:`fls_ehlsypn4ih9s`
         The :term:`match arm marcher` of the :term:`match arm` is evaluated.

#.    #. :def_p:`fls_sqeevu41opyc`
         If the :term:`match arm marcher` succeeds, then

#.    #.    #. :def_p:`fls_cea7veivmiuq`
               The :term:`operand` of the :term:`match arm` is evaluated.

#.    #.    #. :def_p:`fls_m5oy7nh2rpva`
               Control stops the :term:`evaluation` of the :term:`match
               expression`.

#.    #. :def_p:`fls_95x37q2r1trb`
         Otherwise control proceeds with the :term:`evaluation` of the
         next :term:`match arm`.

:def_p:`fls_ay2hb2qz9h0c`
The :term:`evaluation` of a :term:`match arm marcher` proceeds as follows:

#. :def_p:`fls_489zwp50iih7`
   The :term:`pattern` of the :term:`match arm marcher` is evaluated.

#. :def_p:`fls_wbsno91f68m5`
   If the :term:`pattern` succeeds, then

#.    #. :def_p:`fls_a3uatbtqh9gw`
         If the :term:`match arm marcher` has a :term:`match arm guard`, then

#.    #.    #. :def_p:`fls_pmput7545awr`
               The :term:`match arm guard` is evaluated.

#.    #.    #. :def_p:`fls_iw9q43w89275`
               If the :term:`match arm guard` evaluates to ``true``, then
               the :term:`match arm marcher` succeeds.

#.    #. :def_p:`fls_bbeso4kdlyux`
         Otherwise the :term:`match arm marcher` fails.

#. :def_p:`fls_52cxcejv1crd`
   Otherwise the :term:`match arm marcher` fails.

:def_p:`fls_45gakineuny4`
The :term:`evaluation` of a :term:`match arm guard` evaluates
its :term:`operand`. A :term:`match arm guard` evaluates to ``true`` when
its :term:`operand` evaluates to ``true``, otherwise it evaluates to ``false``.

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

:def_p:`fls_fvjn2v2uzax4`
A :term:`return expression` is an :term:`expression` that optionally yields
a :term:`value` and causes control flow to return to the caller.

:def_p:`fls_orzplpfeuza0`
A :term:`return expression` shall appear within a :term:`control flow boundary`.

:def_p:`fls_ixnpmoy18elv`
The :term:`type` of a :term:`return expression` is determined as follows:

* :def_p:`fls_2grzllkvq7xm`
  If the :term:`return expression` has an :term:`operand`, then the :term:`type`
  is the :term:`type` of the :term:`operand`.

* :def_p:`fls_ewl2f5dagayi`
  If the :term:`return expression` does not have an :term:`operand`, then
  the :term:`type` is the :term:`never type`.

:def_p:`fls_vabvf0qb2aft`
The :term:`value` of a :term:`return expression` is determined as follows:

* :def_p:`fls_nej52ebsbwti`
  If the :term:`return expression` has an :term:`operand`, then
  the :term:`value` is the :term:`value` of the :term:`operand`.

* :def_p:`fls_irb50vwmzmik`
  If the :term:`return expression` does not have an :term:`operand`, then
  the :term:`value` is the :term:`unit value`.

.. rubric:: Dynamic Semantics

:def_p:`fls_fjim7bbltu1o`
The :term:`evaluation` of a :term:`return expression` proceeds as follows:

#. :def_p:`fls_bhifl49ywkyr`
   If the :term:`return expression` has an :term:`operand`, then

#.    #. :def_p:`fls_199zuz1swbhm`
         The :term:`operand` is evaluated.

#.    #. :def_p:`fls_l3vp0fjd1yf8`
         The :term:`value` of the :term:`operand` is :term:`moved` into the
         designated output location of the enclosing control flow boundary.

#. :def_p:`fls_m3h44jf8sb4c`
   Control destroys the current activation frame.

#. :def_p:`fls_mwlvqh2kiplx`
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

:def_p:`fls_rzkf1ehhxy5z`
An :term:`await expression` is an :term:`expression` that polls
a :term:`future`, suspending the :term:`execution` of the :term:`future` until
the :term:`future` is ready.

:def_p:`fls_emgiynh0h6u2`
A :term:`future operand` is an :term:`operand` whose :term:`future` is being
awaited by an :term:`await expression`.

:def_p:`fls_fss6zw5ox3o1`
An :term:`await expression` shall appear within

* :def_p:`fls_kgpz6vsobn7v`
  An :term:`async block expression`.

* :def_p:`fls_qweuxdrdcs14`
  An :term:`async function`.

:def_p:`fls_wk4kporkagmb`
The :term:`type` of a :term:`future operand` shall implement
the :codeterm:`core::future::Future` :term:`trait`.

:def_p:`fls_yf2pm3bh12cw`
The :term:`type` of an :term:`await expression` is ``<_
as :term:`core::future::Future`>::Output``.

:def_p:`fls_qowkghkrw262`
The :term:`value` of an :term:`await expression` is the :term:`value` held
by :codeterm:`core::task::Poll::Ready`.

.. rubric:: Dynamic Semantics

:def_p:`fls_ysk8r42bjc3x`
The :term:`evaluation` of an :term:`await expression` proceeds as follows:

#. :def_p:`fls_v49hgqrtns0v`
   The :term:`future operand` is evaluated to a :term:`temporary`.

#. :def_p:`fls_ibz01w7myakm`
   The :term:`temporary` is pinned
   using :codeterm:`core::pin::Pin::new_unchecked`.

#. :def_p:`fls_w0qubq33moic`
   The pinned :term:`temporary` is polled
   using :codeterm:`core::future::Future::poll`, passing in
   the :codeterm:`core::task::Context` of the current task.

#. :def_p:`fls_28xypi5bicxc`
   If :codeterm:`core::future::Future::poll`
   returns :codeterm:`core::task::Poll::Pending`, then **explain what the
   reference meant**.

#. :def_p:`fls_6q71dxww8d4`
   If :codeterm:`core::future::Future::poll`
   returns :codeterm:`core::task::Poll::Ready`, then

#.    #. :def_p:`fls_xcssatn848ux`
         The :term:`value` held within is unwrapped.

#.    #. :def_p:`fls_dc3pmjjo832z`
         Control stops the evaluation of the :term:`await expression`.

.. rubric:: Examples

:def_p:`fls_t4zscbejlf5r`
**provide an example**

Expression Precedence
---------------------

.. rubric:: Legality Rules

:def_p:`fls_mutctrhq3fdp`
Certain :term:`expression`\ s are subject to :term:`precedence`
and :term:`associativity`.

:def_p:`fls_hbags0kkqyiv`
:term:`Precedence` is the order by which :term:`expression`\ s are evaluated in
the presence of other :term:`expression`\ s.

:def_p:`fls_sz6jifo20knm`
:term:`Associativity` is the order by which :term:`operand`\ s are evaluated
within a single :term:`expression`.

:def_p:`fls_dyh0crif76ml`
The :term:`precedence` and :term:`associativity` of
qualifying :term:`expression`\ s are as follows:

.. list-table::

   * - .. rubric:: Expression
     - .. rubric:: Precedence
     - .. rubric:: Associativity
   * - :def_p:`fls_xr01q6518ilr`
       :term:`Array expression`

       :def_p:`fls_cs15qmwhv0yx`
       :term:`Block expression`

       :def_p:`fls_9afwcjcjqiyh`
       :term:`Continue expression`

       :def_p:`fls_pe77tequhjd5`
       :term:`If expression`

       :def_p:`fls_heiia6bh3xx9`
       :term:`If let expression`

       :def_p:`fls_7gefvcddt0v8`
       :term:`Literal expression`

       :def_p:`fls_j28qtekzhhxw`
       :term:`Loop expression`

       :def_p:`fls_sf5y4x9j2ga3`
       :term:`Match expression`

       :def_p:`fls_v80tf8m2vn26`
       :term:`Parenthesized expression`

       :def_p:`fls_649g3wd23d1t`
       :term:`Path expression`

       :def_p:`fls_ko90dj1txdij`
       :term:`Struct expression`

       :def_p:`fls_dkf16ldn0gm9`
       :term:`Tuple expression`

       :def_p:`fls_fvas2dcld97w`
       :term:`Underscore expression`
     - :def_p:`fls_3ep0vncivo8q`
       highest
     - :def_p:`fls_39modvjv87i1`
       none
   * - :def_p:`fls_fw0c84x6p5ue`
       :term:`Method call expression`
     -- :def_p:`fls_c82bgrqftoc`
       none
   * - :def_p:`fls_sx70mgovv3fw`
       \ | :term:`Await expression`
       | :term:`Field access expression`
     -- :def_p:`fls_g3ir9ffq41x4`
       left-to-right
   * - :def_p:`fls_yk9gnmmm4xd4`
       \ | :term:`Array index expression`
       | :term:`Call expression`
     -- :def_p:`fls_tn7gzo6wgyzb`
       none
   * - :def_p:`fls_4vwjupkfjiqj`
       :term:`Error propagation expression`
     -- :def_p:`fls_7kxgqka1u8iy`
       none
   * - :def_p:`fls_21enpbrtrdj3`
       \ | :term:`Borrow expression`
       | :term:`Dereference expression`
       | :term:`Negation expression`
     -- :def_p:`fls_9krp4pmn42mz`
       none
   * - :def_p:`fls_ve9tszjrd194`
       :term:`Type cast expression`
     -- :def_p:`fls_wu4khtlefu2z`
       left-to-right
   * - :def_p:`fls_tu88an2iam3r`
       \ | :term:`Division expression`
       | :term:`Multiplication expression`

       :def_p:`fls_9wx0kv4b3bn9`
       :term:`Remainder expression`
     -- :def_p:`fls_gre8wzv4zrak`
       left-to-right
   * - :def_p:`fls_uf80aoz2bzqv`
       :term:`Addition expression`

       :def_p:`fls_edqq6i6k1tms`
       :term:`Subtraction expression`
     -- :def_p:`fls_cs46ssrc11oo`
       left-to-right
   * - :def_p:`fls_2ueo80iwh90v`
       :term:`Shift left expression`

       :def_p:`fls_a315ed5ddyqs`
       :term:`Shift right expression`
     -- :def_p:`fls_vzksh2ifjsog`
       left-to-right
   * - :def_p:`fls_qz3766pfjkft`
       :term:`Bit and expression`
     -- :def_p:`fls_kvuuse13nc0y`
       left-to-right
   * - :def_p:`fls_xmhumvns04uy`
       :term:`Bit xor expression`
     -- :def_p:`fls_b44rxos1ri0y`
       left-to-right
   * - :def_p:`fls_579h0bkqkofl`
       :term:`Bit or expression`
     -- :def_p:`fls_cbbsni42uuh2`
       left-to-right
   * - :def_p:`fls_jkegdbtpyus`
       :term:`Comparison expression`
     -- :def_p:`fls_jv415ii0wl60`
       requires parentheses
   * - :def_p:`fls_t4621z2unjjy`
       :term:`Lazy and expression`
     -- :def_p:`fls_ew9bbhu8vb3t`
       left-to-right
   * - :def_p:`fls_tnqtm8i798aw`
       :term:`Lazy or expression`
     -- :def_p:`fls_z108kfe7jvuy`
       left-to-right
   * - :def_p:`fls_goldw47a2ced`
       :term:`Range expression`
     -- :def_p:`fls_nec9lvjpse20`
       requires parentheses
   * - :def_p:`fls_dosf4t12hcfm`
       :term:`Assignment expression`

       :def_p:`fls_puxoc3o51rru`
       :term:`Compound assignment expression`
     -- :def_p:`fls_7sp14sc6cfly`
       right-to-left
   * - :def_p:`fls_g4gmwcmwb6ep`
       :term:`Break expression`

       :def_p:`fls_xviqxytui65a`
       :term:`Closure expression`

       :def_p:`fls_eb0koyncm9p7`
       :term:`Return expression`
     - :def_p:`fls_dmzrwecl0snl`
       lowest
     - :def_p:`fls_h3jiufsphlio`
       none

Capturing
---------

.. rubric:: Legality Rules

:def_p:`fls_nlt8r8ap0c1s`
A :term:`capturing expression` is either an :term:`await block expression` or
a :term:`closure expression`.

:def_p:`fls_l7jq4c4kvqtx`
A :term:`capture target` is either a :term:`binding` or a :term:`field` of
a :term:`binding`.

:def_p:`fls_yg48ghp3hst4`
The :term:`capturing environment` of a :term:`capturing expression` consists
of all :term:`capture target`\ s that are defined outside the :term:`capture
expression`.

:def_p:`fls_kxwcv3skq1k`
:term:`Capturing` is the process of saving the :term:`capture target`\ s of
a :term:`capturing expression`'s :term:`capturing environment`.

:def_p:`fls_ataehanqpv8y`
A :term:`capturing target` requires :term:`capturing` when it is part of
the :term:`capturing expression`'s :term:`capture environment` and it is
referenced by the :term:`capturing expression`. Such a :term:`capturing target`
is said to be :def_term:`captured`.

:def_p:`fls_4ikhfe8cpqv7`
:term:`Capture mode` is the mechanism by which a :term:`capture target` is
captured.

:def_p:`fls_81avzoc84h6n`
The :term:`capture mode` is determined based on the usage of the :term:`capture
target`, as follows:

#. :def_p:`fls_t52xno7t7df`
   If the :term:`capturing expression` is subject to :term:`keyword`
   **``move``**, then

#.    #. :def_p:`fls_ya15aw0qfi`
         If the :term:`type` of the :term:`capture target` implements
         the :codeterm:`core::marker::Copy` :term:`trait`, then
         the :term:`capture mode` is *by copy*.

#.    #. :def_p:`fls_wy30sw6cgr11`
         Otherwise the :term:`capture mode` is *by move*.

#. :def_p:`fls_70kjhtt3i3b6`
   Otherwise the :term:`capture mode` is determined based on the following
   precedence:

#.    #. :def_p:`fls_xa074q6956im`
         *By immutable borrow*, if the :term:`capture target` is used *by
         reference*.

#.    #. :def_p:`fls_pzljx08acd2j`
         *By unique immutable borrow*, if the :term:`capture target` is
         a :term:`mutable reference` that is being modified.

#.    #. :def_p:`fls_v4bi4sg3g7vu`
         *By mutable borrow*, if the :term:`capture target` is used *by mutable
         reference*.

#.    #. :def_p:`fls_wfr53jz5si2x`
         *By move*.

:def_p:`fls_p6et0rqkneau`
A tool selects the first :term:`capture mode` that is compatible with the use of
the :term:`capture target`.

