.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_gdeyap4or1db:

Entities and Resolution
=======================

.. _fls_151r19d7xbgz:

Entities
--------

.. rubric:: Syntax

.. syntax::

   Name ::=
       Identifier

.. rubric:: Legality Rules

:dp:`fls_x7j6wcigqt7u`
An :t:`entity` is a :t:`construct` that can be referred to within program text,
usually via a :t:`field access expression` or a :t:`path`.

:dp:`fls_40d2g0hvq2il`
A :t:`name` is an :t:`identifier` that refers to an :t:`entity`.

:dp:`fls_lcca91wjwnpx`
A :t:`declaration` is a :t:`construct` that introduces a :t:`name` for an
:t:`entity`.

:dp:`fls_94l2d7ti0hjw`
An :t:`explicitly declared entity` is an :t:`entity` that has a
:t:`declaration`. The following :t:`entities` are
:t:`explicitly declared entities`:

* :dp:`fls_kvdqmo8gmdxi`
  :t:`[Associated item]s`,

* :dp:`fls_b3cdg74utyvo`
  :t:`[Binding]s`,

* :dp:`fls_njcmeqxzvfsa`
  :t:`[Constant]s`,

* :dp:`fls_63ul8sgf6dgr`
  :t:`[Declarative macro]s`,

* :dp:`fls_6hxf0rn9j1sr`
  :t:`[Enum type]s`,

* :dp:`fls_2qitjk5ssaau`
  :t:`[Enum variant]s`,

* :dp:`fls_4li2c5qc31c7`
  :t:`[Field]s`,

* :dp:`fls_nq8n7w2s3bja`
  :t:`[Function]s`,

* :dp:`fls_jv7qi34flit0`
  :t:`[Generic parameter]s`,

* :dp:`fls_rutlgmzh3tnz`
  :t:`[Implementation]s`,

* :dp:`fls_1owx5ch7sidm`
  :t:`[Label]s`,

* :dp:`fls_8ldy7lec9bcd`
  :t:`[Module]s`,

* :dp:`fls_3mt2p4ssqt0a`
  :t:`[Static]s`,

* :dp:`fls_qqwu3e98lktb`
  :t:`[Struct type]s`,

* :dp:`fls_fup6984lxdfy`
  :t:`[Trait]s`,

* :dp:`fls_ji9iem1c7ekq`
  :t:`[Type alias]es`,

* :dp:`fls_v7w8ptbyxv9w`
  :t:`[Union type]s`.

:dp:`fls_ig1l38gpy5gy`
An :t:`implicitly declared entity` is an :t:`entity` that lacks an explicit
:t:`declaration`. The following :t:`entities` are
:t:`implicitly declared entities`:

* :dp:`fls_ed0t6u7fo3fi`
  :t:`[Built-in attribute]s`.

* :dp:`fls_gjps01c8l6aa`
  :t:`Language prelude` :t:`entities`.

.. _fls_jdknpu3kf865:

Visibility
----------

.. rubric:: Syntax

.. syntax::

   VisibilityModifier ::=
       CratePublicModifier
     | SelfPublicModifier
     | SimplePathPublicModifier
     | SimplePublicModifier
     | SuperPublicModifier

   CratePublicModifier ::=
       $$pub$$ $$($$ $$crate$$ $$)$$

   SelfPublicModifier ::=
       $$pub$$ $$($$ $$self$$ $$)$$

   SimplePathPublicModifier ::=
       $$pub$$ $$($$ $$in$$ SimplePath $$)$$

   SimplePublicModifier ::=
       $$pub$$

   SuperPublicModifier ::=
       $$pub$$ $$($$ $$super$$ $$)$$

.. rubric:: Legality Rules

:dp:`fls_7kpepal8ghuj`
:t:`Visibility` is a property of :t:`[field]s` and :t:`[item]s` that determines
which :t:`[module]s` can refer to the :t:`name` of the :t:`field` or :t:`item`.

:dp:`fls_qo0itr5il1kk`
:t:`Public visibility` is a kind of :t:`visibility` that allows for a :t:`name`
to be referred to from arbitrary :t:`module` ``M`` as long as the ancestor
:t:`[module]s` of the related :t:`entity` can be referred to from ``M``.

:dp:`fls_knjruq5wppv`
:t:`Private visibility` is a kind of :t:`visibility` that allows a :t:`name`
to be referred to only by the current :t:`module` of the :t:`entity`, and its
descendant :t:`[module]s`.

:dp:`fls_t7i4n19qdgn4`
A :t:`visibility modifier` sets the :t:`visibility` of a :t:`name`.

:dp:`fls_aa4f3rvir9lm`
A :t:`crate public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the current :t:`crate` only.

:dp:`fls_tnh7o3pb4e22`
A :t:`self public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`private visibility`. A :t:`self public modifier` is equivalent
to a :t:`simple path public modifier` where the :t:`simple path` denotes
:t:`keyword` ``self``.

:dp:`fls_yymgpyi67dty`
A :t:`simple path public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the provided :t:`simple path` only.

:dp:`fls_hc121mxknq03`
The :t:`simple path` of a :t:`simple path public modifier` shall start
with a :t:`simple path segment` expressed by either :t:`keyword` ``crate``,
:t:`keyword` ``self``, or :t:`keyword` ``super``.

:dp:`fls_icztzxjpm1du`
The :t:`simple path` of a :t:`simple path public modifier` shall resolve to
an ancestor :t:`module` of the current :t:`module` or the current :t:`module`
itself.

:dp:`fls_np8aghofjqhm`
A :t:`simple public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility`.

:dp:`fls_quzvhzpr0124`
A :t:`super public modifier` is a :t:`visibility modifier` that grants a
:t:`name` :t:`public visibility` within the parent :t:`module` only. A
:t:`super public modifier` is equivalent to a :t:`simple path public modifier`
where the :t:`simple path` denotes :t:`keyword` ``super``.

:dp:`fls_utgjx6l5zwfl`
An :t:`external item`, a :t:`field`, or an :t:`item` that appears without a
:t:`visibility modifier` has :t:`private visibility` by default.

:dp:`fls_jifg2st5bfd6`
An :t:`associated item` of a :t:`trait` has the same :t:`visibility` as the
:t:`trait`.

:dp:`fls_dm0xr424ine1`
An :t:`enum variant` and its :t:`[field]s` have the same :t:`visibility` as the
containing :t:`enum type`.

.. rubric:: Examples

.. code-block:: rust

   pub mod outer_module {
       pub mod inner_module {
           pub(crate) fn crate_visible_function() {}

           pub(self) fn inner_module_visible_function() {}

           pub(super) fn outer_module_visible_function() {}

           pub fn visible_function() {}

           fn caller() {
               crate_visible_function();
               inner_module_visible_function();
               visible_function();
           }
       }

       fn caller() {
           inner_module::crate_visible_function();
           inner_module::outer_module_visible_function();
           inner_module::visible_function();
       }
   }

   fn caller() {
       outer_module::inner_module::crate_visible_function();
       outer_module::inner_module::visible_function();
   }

.. _fls_9i5msiuuyihf:

Paths
-----

.. rubric:: Syntax

.. syntax::

   SimplePath ::=
       $$::$$? SimplePathSegment ($$::$$ SimplePathSegment)*

   SimplePathSegment ::=
       Identifier
     | $$crate$$
     | $$$crate$$
     | $$self$$
     | $$super$$

   SimplePathList ::=
       SimplePath ($$,$$ SimplePath)* $$,$$?

   QualifiedType ::=
       $$<$$ TypeSpecification QualifyingTrait? $$>$$

   QualifyingTrait ::=
       $$as$$ TypePath

   PathExpression ::=
       $$::$$? PathExpressionSegment ($$::$$ PathExpressionSegment)*

   PathExpressionSegment ::=
       PathSegment ($$::$$ GenericArgumentList)?

   PathSegment ::=
       SimplePathSegment
     | $$Self$$

   QualifiedPathExpression ::=
       QualifiedType ($$::$$ PathExpressionSegment)+

   TypePath ::=
       $$::$$? TypePathSegment ($$::$$ TypePathSegment)*

   TypePathSegment ::=
       PathSegment $$::$$? (GenericArgumentList | QualifiedFnTrait)?

   QualifiedFnTrait ::=
       $$($$ TypeSpecificationList? $$)$$ ReturnType?

   QualifiedTypePath ::=
       QualifiedType ($$::$$ TypePathSegment)+

.. rubric:: Legality Rules

:dp:`fls_klcltwcwrw6i`
A :t:`path` is a sequence of :t:`[path segment]s` logically separated by
:t:`namespace qualifier` ``::`` that resolves to an :t:`entity`.

:dp:`fls_y1z7kougmahd`
A :t:`path segment` is an element of a :t:`path`.

:dp:`fls_opn5n5t2mo3m`
If a :t:`path segment` is expressed as either :t:`keyword` ``crate``,
:t:`keyword` ``$crate``, :t:`keyword` ``self``, or :t:`keyword` ``Self``, then
the :t:`path segment` shall be the first :t:`path segment` of a :t:`path`.

:dp:`fls_774uryecc2sx`
A :t:`path` that starts with a :t:`path segment` that is expressed as
:t:`keyword` ``$crate`` shall appear only within a :t:`macro transcriber`.

:dp:`fls_7k88ypcgaoff`
If a :t:`path segment` is expressed as :t:`keyword` ``super``, then the
:t:`path segment` shall either be the first :t:`path segment` of a :t:`path`,
or the previous :t:`path segment` of the :t:`path` shall also be expressed as
:t:`keyword` ``super``.

:dp:`fls_7kb6ltajgiou`
A :t:`global path` is a :t:`path` that starts with :t:`namespace qualifier`
``::``.

:dp:`fls_n77icl6idazp`
A :t:`simple path` is a :t:`path` whose :t:`[path segment]s` consist of either
:t:`[identifier]s` or certain :t:`[keyword]s` as defined in the syntax rules
above.

:dp:`fls_iuzvtr3oax1o`
If a :t:`simple path` appears in a :t:`use import` and starts with a
:t:`path segment` expressed as either :t:`keyword` ``crate``, :t:`keyword`
``$crate``, :t:`keyword` ``self``, or :t:`keyword` ``super``, then the
:t:`path` shall be the :t:`simple path prefix` of a :t:`glob import` or a
:t:`nesting import`, or the :t:`simple path` of a :t:`simple import`.

:dp:`fls_cw006jhlboa`
If a :t:`simple path` appears in a :t:`use import` and starts with a
:t:`path segment` expressed as :t:`keyword` ``self``, then the :t:`path` shall
be part of the :s:`UseImportContent` of a :t:`nesting import` as long as the
:t:`path` is a :t:`single segment path`.

:dp:`fls_kv5bpq8rf1j9`
A :t:`simple path` is subject to :t:`simple path resolution`.

:dp:`fls_chtj3hcfe3ap`
A :t:`single segment path` is a :t:`path` consisting of exactly one
:t:`path segment`.

:dp:`fls_wm61yeclairz`
A :t:`multi segment path` is a :t:`path` consisting of more than one
:t:`path segment`.

:dp:`fls_1hi5xjym7152`
A :t:`path expression` is a :t:`path` that acts as an :t:`expression`.

:dp:`fls_tvvycup09b51`
A :t:`path expression` is subject to :t:`path expression resolution`.

:dp:`fls_h2zikgmazoxx`
A :t:`type path` is a :t:`path` that acts as a :t:`type specification`.

:dp:`fls_nj7s6xmzx55f`
A :t:`type path` is subject to :t:`type path resolution`.

:dp:`fls_e65q3iz50j6a`
A :t:`qualifying trait` is a :t:`trait` that imposes a restriction on a
:t:`qualified type`.

:dp:`fls_ybv0tdu7dnj5`
A :t:`qualified type` is a :t:`type` that is restricted to a set of
:t:`[implementation]s` that exhibit :t:`implementation conformance` to a
:t:`qualifying trait`.

:dp:`fls_7sm3206va03c`
A :t:`qualified path expression` is a :t:`path expression` that resolves
through a :t:`qualified type`.

:dp:`fls_huynsyx13gsz`
A :t:`qualified type path` is a :t:`type path` that resolves through a
:t:`qualified type`.

:dp:`fls_f1ciozzetj5a`
A :dt:`qualified fn trait` is a :t:`construct` that refers to the
:std:`core::ops::Fn`, :std:`core::ops::FnMut`, or :std:`core::ops::FnOnce`
:t:`trait`.

:dp:`fls_cy7vza3flqi9`
If a :t:`path` contains a :t:`path segment` with a :t:`qualified fn trait`,
then the :t:`path segment` shall be the last :t:`path segment` of the
:t:`path`.

.. rubric:: Examples

:dp:`fls_cul31g1kkz5c`
The following is a simple path. See :p:`14.2. <fls_q13sty1g9jtn>` for the
declaration of ``crate_visible_function``.

.. code-block:: rust

   crate::outer_module::inner_module::crate_visible_function();

:dp:`fls_no853u27p4f3`
The following is a path expression with a generic argument.

.. code-block:: rust

   Vec::<u8>::with_capacity(42);

:dp:`fls_28c21rzc6rsp`
The following is a type path with a generic argument.

.. code-block:: rust

   std::boxed::Box<dyn std::ops::FnOnce(isize) -> size>;

   struct S;
   impl S {
       fn f() { println!("f of S"); }
   }
   trait T {
       fn f() { println!("f of T"); }
   }
   impl T for S {}

:dp:`fls_4s2n95h4rd1q`
The following is a qualified type path (**isn't it a qualified path
expression?**). The call expression invokes T's function.

.. code-block:: rust

   <S as T>::f();

:dp:`fls_ojdntg5i79pb`
**Add an example for qualified path expression.**

.. _fls_izl8iuhoz9e0:

Scopes
------

.. rubric:: Legality Rules

:dp:`fls_5x5xykocwyiy`
A :t:`scope` is a region of program text where an :t:`entity` can be referred
to. An :t:`entity` is :t:`in scope` when it can be referred to.

.. _fls_6ozthochxz1i:

Binding Scopes
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_ncg9etb3x7k0`
A :t:`binding scope` is a :t:`scope` for :t:`[binding]s`.

:dp:`fls_u52mx4xw8zod`
The :t:`binding` of a :t:`closure parameter` is :t:`in scope` within the
related :t:`closure body`.

:dp:`fls_t9mk8kasobea`
The :t:`binding` of a :t:`function parameter` is :t:`in scope` within the
related :t:`function body`.

:dp:`fls_h9cvs854ae34`
The :t:`binding` of a :t:`for loop` or a :t:`while let loop` is :t:`in scope`
within the related :t:`loop body`.

:dp:`fls_vl1qk0odouyb`
The :t:`binding` of an :t:`if let expression` is :t:`in scope` within the
related :t:`block expression`.

:dp:`fls_74nk389rk075`
The :t:`binding` of a :t:`let statement` is :t:`in scope` after the related
:t:`let statement`, until the end of the :t:`block expression` where the
related :t:`let statement` appears.

:dp:`fls_xbnki64un70v`
The :t:`binding` of a :t:`match arm` is :t:`in scope` within its related
:t:`[expression]s` and related :t:`match arm guard`.

.. _fls_do6x6xe0rlwz:

Generic Parameter Scope
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_amoh8r4gghyj`
A :t:`generic parameter scope` is a :t:`scope` for :t:`[generic parameter]s`.

:dp:`fls_6o38qhbna46z`
A :t:`generic parameter` is :t:`in scope` of a :s:`GenericParameterList`.

:dp:`fls_jqevvpndxzdz`
A :t:`generic parameter` of an :t:`enum type` is :t:`in scope` within the
related :t:`[enum variant]s` and :t:`where clause`.

:dp:`fls_t9ztg017itkp`
A :t:`generic parameter` of a :t:`function pointer type` is :t:`in scope`
within the related :t:`type specification`.

:dp:`fls_pmo939jw9m1m`
A :t:`generic parameter` of an :t:`implementation` is :t:`in scope` within the
related :t:`implementation body` and :t:`where clause`.

:dp:`fls_67dtv1z3arbl`
A :t:`generic parameter` of a :t:`struct type` is :t:`in scope` within the
related :t:`[field]s` and :t:`where clause`.

:dp:`fls_y8j4isk9libl`
A :t:`generic parameter` of a :t:`trait` is :t:`in scope` within the related
:t:`trait body` and :t:`where clause`.

:dp:`fls_ow5ih7q3xxfx`
A :t:`generic parameter` of a :t:`trait bound` is :t:`in scope` within the
related :t:`[generic parameter]s` or the related :t:`type path`.

:dp:`fls_h9rpwxpz72v0`
A :t:`generic parameter` of a :t:`type alias` is :t:`in scope` within the
related :t:`initialization type` and :t:`where clause`.

:dp:`fls_3qm3vh97bvpb`
A :t:`generic parameter` of a :t:`type bound predicate` is :t:`in scope` within
the related :s:`TypeBoundList`.

:dp:`fls_xuxbpv5b2ym9`
A :t:`generic parameter` of a :t:`union type` is :t:`in scope` within the
related :t:`[field]s` and :t:`where clause`.

:dp:`fls_95z5mytvfjia`
A :t:`generic parameter` is not :t:`in scope` within nested :t:`[item]s`,
except within :t:`[associated item]s`.

.. _fls_m0z7omni9hp0:

Item Scope
~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_p5o243hhe1y3`
An :t:`item scope` is a :t:`scope` for :t:`[item]s`.

:dp:`fls_huvo0mp2i6fb`
An :t:`item` declared within the :t:`block expression` of an
:t:`expression-with-block` is :t:`in scope` within the related
:t:`block expression`.

:dp:`fls_x8r0oppuc1t6`
An :t:`item` declared within a :t:`module` is :t:`in scope` within the
related :t:`module`. Such an :t:`item` is not :t:`in scope` within nested
:t:`[module]s`.

.. _fls_769b4p8v3cwu:

Label Scope
~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_96kczd4zhpco`
A :t:`label scope` is a :t:`scope` for :t:`[label]s`.

:dp:`fls_8sevg1sa82h4`
A :t:`label` is :t:`in scope` within the :t:`block expression` of the related
:t:`loop expression`.

:dp:`fls_ep5smja1rxdv`
A :t:`label` is not :t:`in scope` within nested :t:`[async block]s`,
:t:`[closure expression]s`, :t:`[constant context]s`, and :t:`[item]s`.

.. _fls_5v3an4n7x3:

Self Scope
~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_kgt81m4f72ne`
A :t:`Self scope` is a :t:`scope` for :c:`Self`.

:dp:`fls_kxdwq4b136tl`
:c:`Self` of an :t:`enum type` is :t:`in scope` within the related
:t:`[enum variant]s`, :t:`[generic parameter]s`, and :t:`where clause`.

:dp:`fls_nf4g82gi12ij`
:c:`Self` of an :t:`implementation` is :t:`in scope` within the related
:t:`[generic parameter]s`, :t:`implementation body`, and :t:`where clause`.

:dp:`fls_dy4gyepebe7b`
:c:`Self` of a :t:`struct type` is :t:`in scope` within the related
:t:`[field]s`, :t:`[generic parameter]s`, and :t:`where clause`.

:dp:`fls_cha4ddwfqwvj`
:c:`Self` of a :t:`trait` is :t:`in scope` within the related
:t:`[generic parameter]s`, :t:`trait body`, and :t:`where clause`.

:dp:`fls_ql4i021ut2n8`
:c:`Self` of a :t:`union type` is :t:`in scope` within the related
:t:`[field]s`, :t:`[generic parameter]s`, and :t:`where clause`.

:dp:`fls_mj9vlxnf44oi`
:c:`Self` is not :t:`in scope` within :t:`[attribute]s`.

.. _fls_octf6sf7yso:

Textual Macro Scope
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_xkh8cqubhxad`
A :t:`textual macro scope` is a :t:`scope` for :t:`[declarative macro]s`.

:dp:`fls_iec3otx863yp`
A :t:`declarative macro` is :t:`in scope` after the related :t:`macro rules`
declaration, until the end of the :t:`block expression` or the enclosing
:t:`module` where the :t:`macro rules` declaration appears.

:dp:`fls_cbfuh9y87y6i`
If the :t:`textual macro scope` is introduced by a :t:`module` and the
:t:`module` is subject to :t:`attribute` :c:`macro_use`, then the
:t:`textual macro scope` extends until the end of the :t:`scope` introduced by
the enclosing :t:`block` or :t:`module`.

.. _fls_lnpyb285qdiy:

Scope Hierarchy
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_4o7vfo6v39l7`
The :t:`scope hierarchy` reflects the nesting of :t:`[scope]s` as introduced
by :t:`[scoping construct]s`. An inner :t:`scope` introduced by a nested
:t:`scoping construct` is the child of an outer :t:`scope` introduced by an
enclosing :t:`scoping construct`.

:dp:`fls_ns4eog3od4kw`
A :dt:`scoping construct` is a :t:`construct` that introduces :t:`[scope]s`
into the :t:`scope hierarchy`. The following :t:`[construct]s` are
:t:`[scoping construct]s`:

* :dp:`fls_kqmykyzdb1k6`
  :t:`[Block expression]s`,

* :dp:`fls_g86d5v14sxxv`
  :t:`[Closure expression]s`,

* :dp:`fls_ldwencd8zp9a`
  :t:`[Declarative macro]s`,

* :dp:`fls_jz7hgkvocc9r`
  :t:`Enum type` :t:`[declaration]s`,

* :dp:`fls_p4g8sxba7at9`
  :t:`Function` :t:`[declaration]s`,

* :dp:`fls_d1cp5pt5wn0z`
  :t:`Function pointer type` :t:`[specification]s`,

* :dp:`fls_ibmm8y748z4`
  :t:`[If let expression]s`,

* :dp:`fls_39011vsy2vxx`
  :t:`Implementation` :t:`[declaration]s`,

* :dp:`fls_m81hyd154yun`
  :t:`[Let statement]s`,

* :dp:`fls_fvgzmsaox4z3`
  :t:`[Loop expression]s`,

* :dp:`fls_rj8uld11o1br`
  :t:`[Match arm]s`,

* :dp:`fls_hyp4dnpqe620`
  :t:`Module` :t:`[declaration]s`,

* :dp:`fls_zgied4qysk2a`
  :t:`Struct type` :t:`[declaration]s`,

* :dp:`fls_cn6dzmrxdp1w`
  :t:`[Trait bound]s`,

* :dp:`fls_9n7m0tv7w2np`
  :t:`Trait` :t:`[declaration]s`,

* :dp:`fls_sj2ivbf2l2dp`
  :t:`Type alias` :t:`[declaration]s`,

* :dp:`fls_cejfio3ddy0j`
  :t:`[Type bound predicate]s`,

* :dp:`fls_j3rot386teec`
  :t:`Union type` :t:`[declaration]s`.

:dp:`fls_nuobrpnymym1`
A :t:`closure expression` introduces a :t:`binding scope` into the
:t:`scoping hierarchy`.

:dp:`fls_r0x9sw7dwnww`
A :t:`declarative macro` introduces a :t:`textual macro scope` into the
:t:`scoping hierarchy`.

:dp:`fls_ve7svuy7xvh0`
The :t:`declaration` of an :t:`enum type` introduces a
:t:`generic parameter scope` and a :t:`Self scope` into the
:t:`scoping hierarchy`.

:dp:`fls_pvfqhtts3qsa`
The :t:`declaration` of a :t:`function` introduces a :t:`binding scope` and a
:t:`generic parameter scope` into the :t:`scoping hierarchy`.

:dp:`fls_9k9hourczbv7`
The :t:`specification` of a :t:`function pointer type` introduces a
:t:`generic parameter scope` into the :t:`scoping hierarchy`.

:dp:`fls_p6wiuhkeypzs`
An :t:`if let expression` introduces a :t:`binding scope` into the
:t:`scoping hierarchy`.

:dp:`fls_34usianesmf6`
The :t:`declaration` of an :t:`implementation` introduces a
:t:`generic parameter scope` and a :t:`Self scope` into the
:t:`scoping hierarchy`.

:dp:`fls_n1a41d8i0rot`
A :t:`let statement` introduces a :t:`binding scope` into the
:t:`scoping hierarchy`.

:dp:`fls_amhi3d9dd3i3`
A :t:`for loop expression` or a :t:`while let loop expression` introduces a
:t:`binding scope` and a :t:`label scope` into the :t:`scoping hierarchy`.

:dp:`fls_nu8xj3vza55j`
An :t:`infinite loop expression` or a :t:`while loop expression` introduces a
:t:`label scope` into the :t:`scoping hierarchy`.

:dp:`fls_fiyj50u6cg2n`
A :t:`match arm` introduces a :t:`binding scope` into the
:t:`scoping hierarchy`.

:dp:`fls_azjx3y5yezoi`
The :t:`declaration` of a :t:`module` introduces an :t:`item scope` into the
:t:`scoping hierarchy`.

:dp:`fls_puly43s4x360`
The :t:`declaration` of a :t:`struct type` introduces a
:t:`generic parameter scope` and a :t:`Self scope` into the
:t:`scoping hierarchy`.

:dp:`fls_pxtlu7ud6w2h`
The :t:`declaration` of a :t:`trait` introduces a :t:`generic parameter scope`
and a :t:`Self scope` into the :t:`scoping hierarchy`.

:dp:`fls_ddxxt11u0yal`
A :t:`trait bound` introduces a :t:`generic parameter scope` into the
:t:`scoping hierarchy`.

:dp:`fls_qofr9vme46wp`
The :t:`declaration` of a :t:`type alias` introduces a
:t:`generic parameter scope`.

:dp:`fls_gjvfty9m84a9`
A :t:`type bound predicate` introduces a :t:`generic parameter scope` into the
:t:`scoping hierarchy`.

:dp:`fls_xr9wors6oa7w`
The :t:`declaration` of a :t:`union type` introduces a
:t:`generic parameter scope` and a :t:`Self scope` into the
:t:`scoping hierarchy`.

.. _fls_4df1tsti1693:

Namespaces
----------

.. rubric:: Legality Rules

:dp:`fls_1d4jm61qnt4l`
A :t:`namespace` is a logical grouping of :t:`[name]s` such that the occurrence
of a :t:`name` in one :t:`namespace` does not conflict with an occurrence of
the same :t:`name` in another :t:`namespace`.

:dp:`fls_avsua7bho205`
:t:`[Name]s` are segregated into one of five :t:`[namespace]s` based on the
kind of :t:`entity` the :t:`name` refers to.

:dp:`fls_9e3xeza853wx`
A :dt:`label namespace` contains :t:`[label]s`.

:dp:`fls_w625tk3ogdui`
A :dt:`lifetime namespace` contains the :t:`[name]s` of
:t:`[lifetime parameter]s`.

:dp:`fls_crwfafrmydr7`
A :dt:`macro namespace` contains the :t:`[name]s` of the following kinds of
:t:`entities`:

* :dp:`fls_t8fcpm8ldv1y`
  :t:`[Attribute macro]s`,

* :dp:`fls_7pkex797rkeu`
  :t:`[Built-in attribute]s`,

* :dp:`fls_v32f2evgqt5q`
  :t:`[Declarative macro]s`,

* :dp:`fls_f6yrzwu6yi30`
  :t:`[Derive macro]s`,

* :dp:`fls_nk0swexy2ztm`
  :t:`[Function-like macro]s`.

:dp:`fls_ckptn88o6lla`
A :dt:`type namespace` contains the :t:`[name]s` of the following kinds of
:t:`entities`:

* :dp:`fls_3ma5v1fop98p`
  :t:`[Associated type]s`,

* :dp:`fls_nj7sep7ht7lg`
  :t:`[Boolean type]s`,

* :dp:`fls_g8h6t5x6yprm`
  :t:`[Enum type]s`,

* :dp:`fls_2l1o7vqfr4m7`
  :t:`[Enum variant]s`,

* :dp:`fls_6q8rjv1jmu84`
  :t:`[Module]s`,

* :dp:`fls_lx2tx1jt8t3a`
  :t:`[Numeric type]s`,

* :dp:`fls_mo00df28t7c1`
  :c:`Self`,

* :dp:`fls_8o3izim4zf8t`
  :t:`[Struct type]s`,

* :dp:`fls_fweohszgbuj4`
  :t:`[Textual type]s`,

* :dp:`fls_ry02dzisxz3h`
  :t:`[Trait]s`,

* :dp:`fls_dcz1bxjjfsq`
  :t:`[Type aliase]s`,

* :dp:`fls_wt9kgsi6n6ep`
  :t:`[Type parameter]s`,

* :dp:`fls_w29t5njbe46s`
  :t:`[Union type]s`.

:dp:`fls_u1533bngb0yv`
A :dt:`value namespace` contains the :t:`[name]s` of the following kinds of
:t:`entities`:

* :dp:`fls_e8v4g45v5ry2`
  :t:`[Associated constant]s`,

* :dp:`fls_pq8bzav84q3z`
  :t:`[Associated function]s`,

* :dp:`fls_ttr6v8ca4av0`
  :t:`[Binding]s`,

* :dp:`fls_aivi97hhfxy2`
  :t:`[Constant]s`,

* :dp:`fls_pie4ltdtzkl3`
  :t:`[Constant parameter]s`,

* :dp:`fls_qmf7lk6h96sv`
  :t:`[Enum variant constructor]s`,

* :dp:`fls_ufp3btk8pet5`
  :t:`[Function]s`,

* :dp:`fls_t3bnpkfazw4z`
  :t:`[Self constructor]s`,

* :dp:`fls_y0shlli54n5y`
  :t:`[Static]s`,

* :dp:`fls_tghgxcju5u2t`
  :t:`[Struct constructor]s`.

:dp:`fls_yesesxynpq6s`
The :t:`[name]s` of the following kinds of :t:`entities` are not part of any
:t:`namespace`:

* :dp:`fls_40o8y6exr3df`
  :t:`[Enum field]s`,

* :dp:`fls_y76o5ug7dtv`
  :t:`[Struct field]s`,

* :dp:`fls_3np518s1su4w`
  :t:`[Union field]s`.

.. _fls_ld0ize96cm6m:

Preludes
--------

.. rubric:: Legality Rules

:dp:`fls_po4gw6t2ptwu`
A :t:`prelude` is a collection of :t:`entities` that are automatically brought
:t:`in scope` of every :t:`module` in a :t:`crate`. Such :t:`entities` are
referred to as :t:`prelude entities`. The :t:`name` of a :t:`prelude entity`
is referred to as a :t:`prelude name`.

:dp:`fls_n4102qskkmz2`
The :dt:`core prelude` is a :t:`prelude` that brings :t:`in scope` of every
:t:`module` all re-exported :t:`entities` from the
:std:`core::prelude::rust_2021` :t:`module`.

:dp:`fls_atvnwly4w8g2`
An :dt:`external prelude` is a :t:`prelude` that brings :t:`in scope` of the
:t:`root module` the :t:`entities` of the :t:`[crate]s` imported using
:t:`[external crate import]s`. If the :t:`external crate import` uses a
:t:`renaming`, then the :t:`renaming` is instead added to the
:t:`external prelude`. The :t:`core crate` is always added to the
:t:`external prelude` unless the :t:`crate root` is subject to :t:`attribute`
:c:`no_core`.

:dp:`fls_pbc7ktlu0pl`
The :dt:`language prelude` is a :t:`prelude` that brings :t:`in scope` of every
:t:`module` the following :t:`entities`:

* :dp:`fls_frjv68kqqxfh`
  :t:`Boolean type` :c:`bool`.

* :dp:`fls_rf6a2ae3y7vu`
  :t:`[Built-in attribute]s`.

* :dp:`fls_sxnnkzmuvexa`
  :t:`[Floating-point type]s` :c:`f32` and :c:`f64`.

* :dp:`fls_qsyorqjkdh2t`
  :t:`[Integer type]s` :c:`i8`, :c:`i16`, :c:`i32`, :c:`i64`, :c:`i128`,
  :c:`isize`, :c:`u8`, :c:`u16`, :c:`u32`, :c:`u64`, :c:`u128`, and :c:`usize`.

* :dp:`fls_aolj6abvp9sa`
  :t:`[Textual type]s` :c:`char` and :c:`str`.

:dp:`fls_of4n3vv15l5z`
The :dt:`macro_use prelude` is a :t:`prelude` that brings :t:`in scope` of the
:t:`root module` the :t:`entities` of :t:`[macro]s` from :t:`[external crate]s`
that were imported using an :t:`external crate import`.

.. _fls_623n65ppmm4z:

Use Imports
-----------

.. rubric:: Syntax

.. syntax::

   UseImport ::=
       $$use$$ UseImportContent $$;$$

   UseImportContent ::=
       GlobImport
     | NestingImport
     | SimpleImport

   GlobImport ::=
       SimplePathPrefix? $$*$$

   NestingImport ::=
       SimplePathPrefix? $${$$ UseImportContentList? $$}$$

   SimpleImport ::=
       SimplePath Renaming?

   SimplePathPrefix ::=
       SimplePath? $$::$$

   UseImportContentList ::=
       UseImportContent ($$,$$ UseImportContent)* $$,$$?

.. rubric:: Legality Rules

:dp:`fls_lyw4t098sxrj`
A :t:`use import` brings :t:`[entitie]s` :t:`in scope` within the
:t:`block expression` of an :t:`expression-with-block` or :t:`module` where the
:t:`use import` resides.

:dp:`fls_sxo1jb25pl8a`
A :t:`simple path prefix` is the leading :t:`simple path` of a :t:`glob import`
or a :t:`nesting import`.

:dp:`fls_v3a6y2ze44v2`
A :t:`glob import` is a :t:`use import` that brings all :t:`[entitie]s` with
:t:`public visibility` prefixed by its :t:`simple path prefix` into :t:`scope`.

:dp:`fls_ldr7tsuqw34s`
A :t:`nesting import` is a :t:`use import` that provides a common
:t:`simple path prefix` for its nested :t:`[use import]s`.

:dp:`fls_2bkcn83smy2y`
A :t:`simple import` is a :t:`use import` that binds a :t:`simple path` to a
local :t:`name` by using an optional :t:`renaming`.

:dp:`fls_60pldfz61amr`
use self as foo -> imports the current module under the name "foo"

:dp:`fls_hipvjvigycwq`
use blah::{self} -> imports "blah"

:dp:`fls_h5fftft9i0vo`
use blah::{self as foo} -> imports blah under the name "foo"

:dp:`fls_do95zsjb7opx`
use blah::gah::{self} -> imports "gah"

:dp:`fls_husf96ez1wao`
use blah::{gah::{self as foo}} -> imports gah under the name "foo"

:dp:`fls_39sywf5n3qfg`
**The above imports the names in the type namespace only**

.. rubric:: Examples

:dp:`fls_5dlnffim6fso`
The following is a glob import. See :p:`14.2. <fls_q13sty1g9jtn>`
for the declaration of modules and functions. The imported functions
are ``create_visible_function``, ``outer_module_visible_function``,
``visible_function``.

.. code-block:: rust

   use outer_module::inner_module::*;

:dp:`fls_9rhflreuubhq`
The following is a renaming import. The imported function is
``visible_function`` under the name ``f``.

.. code-block:: rust

   use outer_module::inner_module::visible_function as f;

:dp:`fls_s86dgrdpl1w4`
The following is a selective import. The imported functions are
``crate_visible_function`` and ``visible_function``.

.. code-block:: rust

   use outer_module::inner_module
       {crate_visible_function, visible_function}

.. rubric:: Legality Rules

.. _fls_23xcf5jsjr0v:

Shadowing
---------

.. rubric:: Legality Rules

:dp:`fls_ob0riinmitkl`
:t:`Shadowing` is a property of :t:`[name]s`. A :t:`name` is said to be
:t:`shadowed` when another :t:`name` with the same characters is introduced
in the same :t:`scope` within the same :t:`namespace`, effectively hiding it.
A :t:`name` cannot be referred to by any means once it is :t:`shadowed`.

:dp:`fls_fslg89a70e3n`
No :t:`name` shall be :t:`shadowed` except for

* :dp:`fls_hp3f4r3399kt`
  :t:`Prelude names`,

* :dp:`fls_z8qjpskt13yq`
  The :t:`[name]s` of :t:`[macro]s` within :t:`textual macro scope`,

* :dp:`fls_i0gp1y38lr73`
  The :t:`[name]s` of :t:`[variable]s`.

:dp:`fls_saf1meo443fq`
(**this needs to mention about builtin type and module collisions, builtin
attribute and macro collisions**)

:dp:`fls_7pif12rt4s4s`
A :t:`prelude name` shadows other :t:`[prelude name]s` depending on which
:t:`[prelude]s` are included in a :t:`module`. The order of shadowing is as
follows, where a later :t:`prelude name` shadows earlier :t:`prelude name`:

#. :dp:`fls_are9qz67p7b6`
   :t:`Language prelude` :t:`[name]s`.

#. :dp:`fls_4tis5syofyg0`
   :t:`Standard library prelude` :t:`[name]s`.

#. :dp:`fls_u0tsnkhacr06`
   :t:`macro_use prelude` :t:`[name]s`.

#. :dp:`fls_iaklf84guczc`
   :t:`Tool prelude` :t:`[name]s`.

#. :dp:`fls_a0zovslu2v4u`
   :t:`External prelude` :t:`[name]s`.

.. _fls_40xoego2thsp:

Resolution
----------

.. rubric:: Legality Rules

:dp:`fls_ho4kem1slcxg`
:t:`Resolution` is the process of finding a unique interpretation for a
:t:`construct` in a program.

:dp:`fls_7le2vcdbtxbq`
A :t:`construct` that is being resolved is said to be :t:`under resolution`.

:dp:`fls_x3alg07yd7hx`
A :t:`dereference type` is either a :t:`reference type` or a :t:`type` that
implements the :std:`core::ops::Deref` :t:`trait`.

:dp:`fls_4hulwazdu20i`
A :t:`dereference type chain` is a sequence of :t:`[dereference type]s`. A
:t:`dereference type chain` with an initial :t:`dereference type`. From then
on, the :t:`dereference type chain` continues as follows:

* :dp:`fls_ptocwx5p25lj`
  If the previous :t:`dereference type` is a :t:`reference type`, then the
  :t:`dereference type chain` continues with the inner :t:`type` of the
  previous :t:`dereference type`.

* :dp:`fls_ygam5nisv98c`
  Otherwise the :t:`dereference type chain` continues with :t:`type`
  :std:`core::ops::Deref::Target` of the previous :t:`dereference type`.

.. _fls_xcwfotmq2e5d:

Field Resolution
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_1nxknwjdp0am`
:t:`Field resolution` is a form of :t:`resolution` that applies to the
:t:`field selector` of a :t:`field access expression`.

:dp:`fls_j1bip4w30q8`
A :dt:`candidate container type` is the :t:`type` of the :t:`container operand`
of a :t:`field access expression` :t:`under resolution`.

:dp:`fls_jrk3gzqvqr8e`
A :t:`candidate container type chain` is a sequence of
:t:`[candidate container type]s`. The :t:`candidate container type chain`
starts with the :t:`type` of the :t:`container operand` of the
:t:`field access expression` :t:`under resolution`. From then on, the
:t:`candidate container type chain` is treated as a
:t:`dereference type chain`.

:dp:`fls_asn20qx16sr6`
A :dt:`candidate field` is a :t:`field` of a :t:`candidate container type`
that is visible from the location of the :t:`field access expression`
:t:`under resolution`.

:dp:`fls_jzoon4x89zp7`
A :dt:`candidate indexed field` is a :t:`candidate field` whose position in the
:t:`candidate operand type` matches the index of an
:t:`indexed field selector`.

:dp:`fls_r80pixfoe5hk`
A :dt:`candidate named field` is a :t:`candidate field` whose :t:`name` matches
the characters of a :t:`named field selector`.

:dp:`fls_40oa0j6aiop3`
:t:`Field resolution` of an :t:`indexed field access` proceeds as follows:

#. :dp:`fls_2bp1zs7qaz7o`
   For each :t:`candidate container type` of the
   :t:`candidate container type chain`

   #. :dp:`fls_s14fegwhwnc8`
      Try to locate a :t:`candidate indexed field` of the
      :t:`candidate container type`.

   #. :dp:`fls_tfjm27ydiake`
      If such a :t:`candidate indexed field` exists, the
      :t:`indexed field access` resolves to that :t:`candidate indexed field`
      and :t:`field resolution` stops.

:dp:`fls_p6hgoqo0kcx`
:t:`Field resolution` of a :t:`named field access` proceeds as follows:

#. :dp:`fls_e7sj392ohvbd`
   For each :t:`candidate container type` of the
   :t:`candidate container type chain`

   #. :dp:`fls_z6qt9obbhhcg`
      Try to locate a :t:`candidate named field` of the
      :t:`candidate operand type`.

   #. :dp:`fls_ljnjxex3u5o`
      If such a :t:`candidate named field` exists, the :t:`named field access`
      resolves to that :t:`candidate named field` and :t:`field resolution`
      stops.

:dp:`fls_nm06mru40tyg`
A :t:`field access expression` shall resolve to exactly one :t:`field`.

.. _fls_wqazkzle0ix9:

Method Resolution
~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_e5a5z5yht26l`
:t:`Method resolution` is a kind of :t:`resolution` that applies to the
:t:`method operand` of a :t:`method call expression`.

:dp:`fls_z80ylmlu1f3q`
A :dt:`candidate receiver type` is the :t:`type` of the :t:`receiver operand`
of a :t:`method call expression` :t:`under resolution`.

:dp:`fls_e1029pvq706h`
A :dt:`candidate receiver type chain` is a sequence of
:t:`[candidate receiver type]s`. The :t:`candidate receiver type chain` starts
with the :t:`type` of the :t:`receiver operand` of the
:t:`method call expression` :t:`under resolution`. From then on, the
:t:`candidate receiver type chain` is treated as a :t:`dereference type chain`.

:dp:`fls_w3ik83d43fr1`
A :dt:`candidate method` is a method of a :t:`candidate receiver type` that
is visible from the location of the :t:`method call expression`
:t:`under resolution`.

:dp:`fls_pybv4krsvktv`
:t:`Method resolution` proceeds as follows:

#. :dp:`fls_m2njj6no0p1i`
   For each :t:`candidate receiver type` of the
   :t:`candidate receiver type chain`

   #. :dp:`fls_16l2q1wpcnbp`
      Perform :t:`method resolution receiver candidate lookup` for the
      :t:`candidate receiver type`.

   #. :dp:`fls_fcnahkqxomuo`
      If the last :t:`candidate receiver type` is an :t:`array type`, then
      perform :t:`method resolution receiver candidate lookup` for a
      :t:`slice type` where the :t:`slice type` has the same :t:`element type`
      as the :t:`array type`.

:dp:`fls_ii0fdpekn1qt`
:dt:`Method resolution receiver candidate lookup` for a :t:`receiver type`
proceeds as follows:

#. :dp:`fls_ohjmxhbw3nx3`
   Perform :t:`method resolution implementation candidate lookup` for the
   :t:`receiver type`.

#. :dp:`fls_lgpdicxxwq13`
   Perform :t:`method resolution implementation candidate lookup` for the
   :t:`immutable borrow` of the :t:`receiver type`.

#. :dp:`fls_ugl3x4y3lli2`
   Perform :t:`method resolution implementation candidate lookup` for the
   :t:`mutable borrow` of the :t:`receiver type`.

:dp:`fls_bb4cbmvui8fk`
:dt:`Method resolution implementation candidate lookup` for a
:t:`receiver type` proceeds as follows:

#. :dp:`fls_5wny1yxbyuz0`
   Perform :t:`method resolution inherent implementation candidate lookup` for
   the :t:`receiver type`.

#. :dp:`fls_gsc8pt4tlsqv`
   Perform :t:`method resolution trait implementation candidate lookup` for the
   :t:`receiver type`.

:dp:`fls_tfglce1wuq5q`
:dt:`Method resolution inherent implementation candidate lookup` for a
:t:`receiver type` proceeds as follows:

#. :dp:`fls_64bfcn9okeve`
   Construct the :t:`dereference type chain` for the :t:`receiver type`.

#. :dp:`fls_om90v9re8b2l`
   For each :t:`dereference type` in the :t:`dereference type chain`

   #. :dp:`fls_bsf4hy9x7c2e`
      For each :t:`inherent implementation` in the set of
      :t:`[inherent implementation]s` of the :t:`dereference type` where the
      :t:`implementing type` unifies with the :t:`dereference type`

      #. :dp:`fls_cnn5hkf1z5q4`
         Try to locate a :t:`candidate method` in the :t:`inherent
         implementation`, where the :t:`type` of the :t:`self parameter`
         unifies with the :t:`receiver type`.

      #. :dp:`fls_j9ho6xc2fj0w`
         If such a :t:`candidate method` exists, then the
         :t:`method call expression` resolves to that :t:`candidate method` and
         :t:`method resolution` stops.

:dp:`fls_1y94elgpg0uk`
:dt:`Method resolution trait implementation candidate lookup` for a
:t:`receiver type` proceeds as follows:

#. :dp:`fls_npsdxrtcslcf`
   Construct the :t:`dereference type chain` for the :t:`receiver type`.

#. :dp:`fls_yv5l823lwdsv`
   For each :t:`dereference type` in the :t:`dereference type chain`

   #. :dp:`fls_ckdoyvbaybe0`
      For each :t:`trait implementation` of the :t:`dereference type` where the
      :t:`implemented trait` is :t:`in scope`

      #. :dp:`fls_1azkiu20r0e4`
         Try to locate a :t:`candidate method` in the
         :t:`trait implementation`, where the :t:`type` of the
         :t:`self parameter` unifies with the :t:`receiver type`.

      #. :dp:`fls_ose5m4bhkg57`
         If such a :t:`candidate method` exists, then the
         :t:`method call expression` resolves to that :t:`candidate method` and
         :t:`method resolution` stops.

:dp:`fls_jw2yv23cduu4`
A :t:`method call expression` shall resolve to exactly one :t:`method`.

.. _fls_i6qzga6dyaee:

Path Resolution
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_8slvisr3jfja`
:t:`Path resolution` is a form of :t:`resolution` that applies to :t:`[path]s`.

:dp:`fls_nmev0tnzgw35`
:t:`Path resolution` resolves a :t:`path` by resolving individual
:t:`[path segment]s` in sequence, starting from the leftmost :t:`path segment`.

:dp:`fls_p23q1ob2qitz`
A :t:`path segment` shall resolve to exactly one :t:`entity`.

:dp:`fls_e9rv8dfa0arl`
A :dt:`candidate direct entity` is an :t:`entity` that is visible from the
location of a :t:`path` :t:`under resolution` and is located by first examining
:t:`[textual macro scope]s`, followed by examining the :t:`scope hierarchy`
from the innermost :t:`scope` enclosing the :t:`path` to the outermost
:t:`scope`, followed by examining :t:`[prelude]s`.

:dp:`fls_yule33qm1ok`
A :dt:`candidate external prelude entity` is an :t:`entity` that is visible
from the location of a :t:`path` :t:`under resolution` and is located by
examining the :t:`external prelude`.

:dp:`fls_7xmhm2lf2h8f`
A :dt:`candidate selected entity` is an :t:`entity` that is visible from
the location of a :t:`path` :t:`under resolution` and is located within a
:t:`resolution context`.

:dp:`fls_ec4wo8odusqp`
A :dt:`namespace context` is a set of :t:`[namespace]s` where the :t:`[name]s`
of :t:`candidate selected entities` reside.

:dp:`fls_9tedg9lpewqa`
A :dt:`resolution context` is a set of :t:`entities` that informs
:t:`path resolution` by restricting the number of
:t:`candidate selected entities`.

:dp:`fls_mvymlhp7192e`
The resolution of the leftmost :t:`path segment` of a :t:`path` proceeds as
follows:

* :dp:`fls_cs485plo4z49`
  If the leftmost :t:`path segment` is expressed as :t:`keyword` ``crate``,
  then what the leftmost :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the current :t:`crate`.

* :dp:`fls_yrpem8vhxpr5`
  If the leftmost :t:`path segment` is expressed as :t:`keyword` ``$crate``,
  then what the leftmost :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the :t:`crate` that declares
  the :t:`macro` that is being expanded.

* :dp:`fls_ri50nc2dg7c4`
  If the leftmost :t:`path segment` is expressed as :t:`keyword` ``self``,
  then what the leftmost :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the current :t:`module`.

* :dp:`fls_to52oma1bvx3`
  If the leftmost :t:`path segment` is expressed as :t:`keyword` ``Self``,
  then what the leftmost :t:`path segment` resolves to and its
  :t:`resolution context` is

  * :dp:`fls_kpn2y7xb3s8q`
    The :t:`entity` of the :t:`abstract data type`, if the :t:`path` appears
    within an :t:`abstract data type`, or

  * :dp:`fls_z71op1vdnazq`
    The :t:`entity` of the :t:`implementing type`, if the :t:`path` appears
    within an :t:`implementation`, or

  * :dp:`fls_2km29ekj9464`
    The :t:`entity` of the :t:`trait`, if the :t:`path` appears within a
    :t:`trait`.

* :dp:`fls_l2y464skbuta`
  If the leftmost :t:`path segment` is expressed as :t:`keyword` ``super``,
  then what the leftmost :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the parent :t:`module` of the
  current :t:`module`.

* :dp:`fls_n2x13sg5szbl`
  If the leftmost :t:`path segment` is an :t:`identifier`, then

  * :dp:`fls_53kd7eb1qzuz`
    If the :t:`path` is a :t:`global path`, then try to find a
    :t:`candidate external prelude entity` whose :t:`name` matches the
    characters of the leftmost :t:`path segment`. What the leftmost
    :t:`path segment` resolves to and its :t:`resolution context` is that
    :t:`candidate external prelude entity`.

  * :dp:`fls_3spnlz9tqnhj`
    Otherwise try to find a :t:`candidate direct entity` whose :t:`name`
    matches the characters of the leftmost :t:`path segment`. What the leftmost
    :t:`path segment` resolves to and its :t:`resolution context` is that
    :t:`candidate direct entity`.

* :dp:`fls_lxa7uhmdoy9d`
  If the leftmost :t:`path segment` starts with a :t:`qualified type`, then

  * :dp:`fls_xujlscsir05f`
    If the :t:`qualified type` is subject to a :t:`qualifying trait`, then the
    :t:`resolution context` of the leftmost :t:`path segment` consists of the
    :t:`entities` of all :t:`[implementation]s` of the :t:`qualified type` that
    implement the :t:`qualifying trait`. **What does the path segment resolve
    to?**

  * :dp:`fls_wypnvfklnmc1`
    Otherwise the :t:`resolution context` of the leftmost :t:`path segment`
    consists of the :t:`entity` of the :t:`qualified type`, the :t:`entities`
    of all its :t:`[inherent implementation]s`, and the :t:`entities` of all
    its :t:`[trait implementation]s` of :t:`[trait]s` that are :t:`in scope`.
    **What does the path segment resolve to?**

:dp:`fls_zi46lmwsn4rg`
The resolution of the rightmost :t:`path segment` is determined based on the
:t:`path resolution` kind, where the :t:`name` of the
:t:`candidate selected entity` is restricted by the :t:`namespace context`.

.. _fls_bbso3c45kr9z:

Simple Path Resolution
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_uml24jw5jo7a`
:t:`Simple path resolution` is a kind of :t:`path resolution` that applies to
:t:`[simple path]s`.

:dp:`fls_59wd7loxst43`
The :t:`namespace context` of :t:`simple path resolution` is determined as
follows:

* :dp:`fls_mk0ufkeggot6`
  If the :t:`simple path` is part of an :t:`attribute` or a
  :t:`macro invocation`, then the :t:`namespace context` is the
  :t:`macro namespace`.

* :dp:`fls_ayv8okec9fwb`
  If the :t:`simple path` is part of a :t:`use import`, then the
  :t:`namespace context` consists of the :t:`macro namespace`, the
  :t:`type namespace`, and the :t:`value namespace`.

* :dp:`fls_ppoc6wcplab6`
  If the :t:`simple path` is part of a :t:`visibility modifier`, then the
  :t:`namespace context` consists of the :t:`type namespace`.

:dp:`fls_dc0yv4306p82`
The leftmost :t:`path segment` of a :t:`simple path` is resolved using general
:t:`path resolution`. The remaining :t:`[path segment]s` are resolved in
left-to-right order, as follows:

* :dp:`fls_jhivcca0xcqj`
  If the current :t:`path segment` is expressed as :t:`keyword` ``super``, then
  what the current :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the parent :t:`module` of the
  previous :t:`[path segment]'s` :t:`resolution context`.

* :dp:`fls_tfsgutcpube2`
  Otherwise try to find a :t:`candidate selected entity` whose :t:`name`
  matches the characters of the current :t:`path segment` within the previous
  :t:`[path segment]'s` :t:`resolution context`, where if the current
  :t:`path segment` is not the rightmost :t:`path segment`, the
  :t:`resolution context` is restricted to the :t:`entities` of :t:`modules`.
  What the current :t:`path segment` resolves to and its
  :t:`resolution context` is that :t:`candidate selected entity`.

.. _fls_o9u2h5m17kpz:

Path Expression Resolution
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_akjlqm3a2lb1`
:t:`Path expression resolution` is a form of :t:`path resolution` that applies
to :t:`path expressions`.

:dp:`fls_xyzdajtf4u2t`
The :t:`namespace context` of :t:`path expression resolution` is the
:t:`value namespace`.

:dp:`fls_d45vu3iazi3`
The leftmost :t:`path segment` of a :t:`path expression` is resolved using
general :t:`path resolution`. The remaining :t:`[path segment]s` are resolved
in left-to-right order, as follows:

* :dp:`fls_9pjhok9rctty`
  If the current :t:`path segment` is expressed as :t:`keyword` ``super``, then
  what the current :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the parent :t:`module` of the
  previous :t:`[path segment]'s` :t:`resolution context`.

* :dp:`fls_2wbpr9lvz5yq`
  If the current :t:`path segment` is the rightmost :t:`path segment`, then

  * :dp:`fls_u5nyv6ii1g2l`
    If the previous :t:`[path segment]'s` :t:`resolution context` is an
    :t:`enum type`, then try to find a :t:`candidate selected entity` whose
    :t:`name` matches the characters of the current :t:`path segment`, where
    the :t:`resolution context` is restricted to the :t:`entities` of the
    :t:`[enum variant]s` of that :t:`enum type`. What the current
    :t:`path segment` resolves to and its :t:`resolution context` is that
    :t:`candidate selected entity`.

  * :dp:`fls_bscg48os5otx`
    If the previous :t:`[path segment]'s` :t:`[resolution contex]t` is a
    :t:`module`, then try to find a :t:`candidate selected entity` whose
    :t:`name` matches the characters of the current :t:`path segment` within
    that :t:`module`. What the current :t:`path segment` resolves to and its
    :t:`resolution context` is that :t:`candidate selected entity`.

  * :dp:`fls_qbmp0blpoxx9`
    If the previous :t:`[path segment]'s` :t:`resolution context` is a
    :t:`trait` or a :t:`type`, then perform
    :t:`path expression resolution implementation candidate lookup` for the
    current :t:`path segment` and that :t:`trait` or :t:`type`.

* :dp:`fls_ydni5laqv6gp`
  Otherwise

  * :dp:`fls_lsxbl6ep3150`
    If the previous :t:`[path segment]'s` :t:`resolution context` is a
    :t:`module`, then try to find a :t:`candidate selected entity` whose
    :t:`name` matches the characters of the current :t:`path segment` within
    that :t:`module`, where the :t:`resolution context` is restricted to the
    :t:`entities` whose :t:`[name]s` reside in the :t:`type namespace`. What
    the current :t:`path segment` resolves to and its :t:`resolution context`
    is that :t:`candidate selected entity`.

  * :dp:`fls_x1n7w8w6lwm`
    If the previous :t:`[path segment]'s` :t:`resolution context` is a
    :t:`trait`, then try to find a :t:`candidate selected entity` whose
    :t:`name` matches the characters of the current :t:`path segment` within
    that :t:`trait`, where the :t:`resolution context` is restricted to the
    :t:`entities` of all :t:`[associated item]s` of that :t:`trait`. What the
    current :t:`path segment` resolves to and its :t:`resolution context` is
    that :t:`candidate selected entity`.

  * :dp:`fls_v1h4frnbqruu`
    If the previous :t:`[path segment]'s` :t:`resolution context` is a
    :t:`type`, then try to find a :t:`candidate selected entity` whose
    :t:`name` matches the characters of the current :t:`path segment` within
    that :t:`type`, where the :t:`resolution context` is restricted to the
    :t:`entities` of all :t:`[associated item]s` from its
    :t:`[inherent implementation]s`, and the :t:`entities` of all its
    :t:`[trait implementation]s` of :t:`[trait]s` that are :t:`in scope`. What
    the current :t:`path segment` resolves to and its :t:`resolution context`
    is that :t:`candidate selected entity`.

  * :dp:`fls_j6px1hxcsqer`
    If the current :t:`path segment` has :t:`[generic argument]s`, then the
    :t:`[generic argument]s` are passed (**better term?**) to the
    :t:`resolution context` of the current :t:`path segment`.

:dp:`fls_utfpnwlo0v99`
:dt:`Path expression resolution implementation candidate lookup` for a
:t:`path segment` and a :t:`trait` or :t:`type` proceeds as follows:

#. :dp:`fls_1p8ocf1w5bp4`
   Perform
   :t:`path expression resolution inherent implementation candidate lookup` for
   the :t:`path segment` and the :t:`trait` or :t:`type`.

#. :dp:`fls_qb5yo7j5gnvf`
   Perform
   :t:`path expression resolution trait implementation candidate lookup` for
   the :t:`path segment` and the :t:`trait` or :t:`type`.

:dp:`fls_o1g0forw6xw`
:dt:`Path expression resolution inherent implementation candidate lookup` for a
:t:`path segment` and a :t:`trait` or :t:`type` proceeds as follows:

#. :dp:`fls_bcqe13q696zg`
   For each :t:`inherent implementation` in the set of :t:`[inherent
   implementation]s` of the :t:`trait` or :t:`type` where the
   :t:`implementing type` :t:`unifies` with the :t:`trait` or :t:`type`

   #. :dp:`fls_3sceutaqpqha`
      Try to locate a visible :t:`function` in the :t:`inherent implementation`
      whose :t:`name` matches the characters of the :t:`path segment`.

   #. :dp:`fls_6q9cwqlvxmd1`
      If such a :t:`function` exists, then the :t:`path segment` resolves to
      that :t:`function` and :t:`path expression resolution` stops.

:dp:`fls_qeym3vbi36iv`
:dt:`Path expression resolution trait implementation candidate lookup` for a
:t:`path segment` and a :t:`trait` or :t:`type` proceeds as follows:

#. :dp:`fls_8x0pqwpm80sj`
   For each :t:`trait implementation` of the :t:`trait` or :t:`type` where the
   :t:`implemented trait` is :t:`in scope`

   #. :dp:`fls_pp09gmrnasjp`
      Try to locate a visible :t:`function` in the :t:`trait implementation`
      whose :t:`name` matches the characters of the :t:`path segment`.

   #. :dp:`fls_q0jt6n2j1hsx`
      If such a :t:`function` exists, then the :t:`path segment` resolves to
      that :t:`function` and :t:`path expression resolution` stops.

.. _fls_1h0olpc7vbui:

Type Path Resolution
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_2zuncql8ir5k`
:t:`Type path resolution` is a form of :t:`path resolution` that applies to
:t:`[type path]s`.

:dp:`fls_bv5cj918dqqe`
The :t:`namespace context` of :t:`type path resolution` is the
:t:`type namespace`.

:dp:`fls_bsakzuteuh5s`
The leftmost :t:`path segment` of a :t:`type path` is resolved using general
:t:`path resolution`. The remaining :t:`[path segment]s` are resolved in
left-to-right order, as follows:

* :dp:`fls_j1ewjisx0mc2`
  If the current :t:`path segment` is expressed as :t:`keyword` ``super``, then
  what the current :t:`path segment` resolves to and its
  :t:`resolution context` is the :t:`entity` of the parent :t:`module` of the
  previous :t:`[path segment]'s` :t:`resolution context`.

* :dp:`fls_o4snu1him277`
  If the previous :t:`[path segment]'s` :t:`resolution context` is a
  :t:`module`, then try to find a :t:`candidate selected entity` whose
  :t:`name` matches the characters of the current :t:`path segment` within
  that :t:`module`. What the current :t:`path segment` resolves to and its
  :t:`resolution context` is that :t:`candidate selected entity`.

* :dp:`fls_goe8q52toik2`
  If the previous :t:`[path segment]'s` :t:`resolution context` is a
  :t:`trait`, then try to find a :t:`candidate selected entity` whose :t:`name`
  matches the characters of the current :t:`path segment` within that
  :t:`trait`, where the :t:`resolution context` is restricted to the
  :t:`entities` of all :t:`[associated item]s` of that :t:`trait`. What the
  current :t:`path segment` resolves to and its :t:`resolution context` is that
  :t:`candidate selected entity`.

* :dp:`fls_4rs35f6ydckj`
  If the previous :t:`[path segment]'s` :t:`resolution context` is a :t:`type`,
  then try to find a :t:`candidate selected entity` whose :t:`name` matches
  the characters of the current :t:`path segment` within that :t:`type`,
  where the :t:`resolution context` is restricted to the :t:`entities` of all
  :t:`[associated item]s` from its :t:`[inherent implementation]s`, and the
  :t:`entities` of all its :t:`[trait implementation]s` of :t:`[trait]s` that
  are :t:`in scope`. What the current :t:`path segment` resolves to and its
  :t:`resolution context` is that :t:`candidate selected entity`.

* :dp:`fls_wm3sglgg29h6`
  If the current :t:`path segment` has :t:`[generic argument]s`, then the
  :t:`[generic argument]s` are passed (**better term?**) to the
  :t:`resolution context` of the current :t:`path segment`.

* :dp:`fls_jh4db1p7or0x`
  If the current :t:`path segment` has a :t:`qualified fn trait`, then the
  current :t:`path segment` shall resolve to either the :std:`core::ops::Fn`,
  :std:`core::ops::FnMut`, or :std:`core::ops::FnOnce` :t:`trait`.
