.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_gdeyap4or1db:

Names and Resolution
====================

.. rubric:: Syntax

.. syntax::

   Name ::=
       Identifier

.. rubric:: Legality Rules

:dp:`fls_s79wluq3u5wv`
An :t:`entity` is a :t:`construct` that can be referred to within a program by
using a :t:`path`.

:dp:`fls_yf5r11o1kevr`
A :t:`name` identifies an :t:`entity` within the program text.

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
:t:`Visibility` is a property of :t:`[item]s` that determines which
:t:`[module]s` can refer to a :t:`name` of an :t:`item`.

:dp:`fls_qo0itr5il1kk`
:t:`Public visibility` is a kind of :t:`visibility` that allows for a :t:`name`
to be referred to from arbitrary :t:`module` ``M`` as long as the ancestor
:t:`[module]s` of the related :t:`entity` can be referred to from ``M``.

:dp:`fls_knjruq5wppv`
:t:`Private visibility` is a kind of :t:`visibility` that allows a :t:`name`
to be referred to only by the current :t:`module` of the :t:`entity`, and its
descendant :t:`[module]s`.

:dp:`fls_t7i4n19qdgn4`
A :t:`visibility modifier` sets the :t:`visibility` of the :t:`name` of an
:t:`item`.

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
:t:`name` :t:`public visibility` within the parent :t:`module` only. A :t:`super
public modifier` is equivalent to a :t:`simple path public modifier` where the
:t:`simple path` denotes :t:`keyword` ``super``.

:dp:`fls_utgjx6l5zwfl`
An :t:`external item`, a :t:`field`, or an :t:`item` that appears without a
:t:`visibility modifier` has :t:`private visibility` by default.

:dp:`fls_jifg2st5bfd6`
An :t:`associated item` of a :t:`trait` with :t:`public visibility` has
:t:`public visibility` by default.

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

   SimplePathList ::=
       SimplePath ($$,$$ SimplePath)* $$,$$?

   SimplePathSegment ::=
       Identifier
     | $$crate$$
     | $$$crate$$
     | $$self$$
     | $$super$$

   PathInExpression ::=
       $$::$$? PathInExpressionSegment ($$::$$ PathInExpressionSegment)*

   PathInExpressionSegment ::=
       PathSegment ($$::$$ GenericArgumentList)?

   PathSegment ::=
       Identifier
     | $$crate$$
     | $$$crate$$
     | $$self$$
     | $$Self$$
     | $$super$$

   TypePath ::=
       $$::$$? TypePathSegment ($$::$$ TypePathSegment)*

   TypePathSegment ::=
       PathSegment $$::$$? (GenericArgumentList | TypePathFn)?

   TypePathFn ::=
       $$($$ TypeSpecificationList? $$)$$ ReturnType?

   QualifiedPathInExpression ::=
       QualifiedPathType ($$::$$ PathInExpressionSegment)+

   QualifiedPathType ::=
       $$<$$ TypeSpecification TypePathRenaming? $$>$$

   TypePathRenaming ::=
       $$as$$ TypePath

   QualifiedPathInType ::=
       QualifiedPathType ($$::$$ TypePathSegment)+

.. rubric:: Legality Rules

:dp:`fls_klcltwcwrw6i`
A :t:`path` is a sequence of :t:`[path segment]s` logically separated by
:t:`namespace qualifier` ``::`` that resolves to a :t:`name`.

:dp:`fls_pu0qwlmndtwf`
A :t:`path` that starts with qualifier ``$crate`` shall appear only within a
:t:`macro transcriber`.

:dp:`fls_8e03ie9p08ib`
A :t:`simple path` is a :t:`path` whose :t:`[path segment]s` consist of either
:t:`[identifier]s` or certain :t:`[keyword]s`.

:dp:`fls_hwxjhdx10xlm`
A :t:`global path` is a :t:`path` that starts with :t:`namespace qualifier`
``::``.

:dp:`fls_ylkv4ut37v2m`
A :t:`path segment` is a constituent of a :t:`path`.

:dp:`fls_uheh0jvg6yie`
**Should talk about the effects of a QualifiedPathType with respect to
generics.**

:dp:`fls_wv9anw30msgq`
A :t:`canonical path` is a :t:`path` that fully qualifies a :t:`name` starting
from the current :t:`crate`.

:dp:`fls_tjwehd9ofzrj`
The following :t:`[construct]s` do not have a :t:`canonical path`:

* :dp:`fls_j5m34x689twd`
  :t:`[Associated item]s`.

* :dp:`fls_1s77w8eorw65`
  :t:`[Implementation]s`.

* :dp:`fls_lt91iobkn6b4`
  :t:`[Use import]s`.

* :dp:`fls_6k0esdfp4nqs`
  :t:`[Item]s` declared in :t:`[block expression]s`.

* :dp:`fls_7hr4d9gmnd2u`
  :t:`[Item]s` declared in a :t:`module` where the :t:`module` does not have a
  :t:`canonical path`.

.. rubric:: Examples

:dp:`fls_cul31g1kkz5c`
The following is a simple path. See :p:`14.2. <fls_q13sty1g9jtn>` for the
declaration of ``crate_visible_function``.

.. code-block:: rust

   crate::outer_module::inner_module::crate_visible_function();

:dp:`fls_no853u27p4f3`
The following is a path-in-expression.

.. code-block:: rust

   Vec::<u8>::with_capacity(42);

:dp:`fls_28c21rzc6rsp`
The following is a path-in-type.

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
The following is a path-in-type. The call expression invokes T's function.

.. code-block:: rust

   <S as T>::f();

:dp:`fls_ojdntg5i79pb`
**Add an example for qualified path-in-expression.**

.. _fls_9gprp17h6t1q:

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

:dp:`fls_4qgflb8tsdo8`
A :t:`use import` brings :t:`[name]s` into :t:`scope` within the :t:`module` or
:t:`block expression` where the :t:`use import` resides.

:dp:`fls_xpn0gi2euo01`
A :t:`glob import` is a :t:`use import` that brings all :t:`[name]s` with
:t:`public visibility` prefixed by its :t:`path` prefix into :t:`scope`.

:dp:`fls_wln1ydrlaqy`
A :t:`glob import` shall contain a :t:`simple path`.

:dp:`fls_lahpu2vk6aqu`
A :t:`nesting import` is a :t:`use import` that provides a common :t:`path`
prefix for its nested :t:`[use import]s`.

:dp:`fls_t64jr6jd1723`
A :t:`simple import` is a :t:`use import` that binds a :t:`simple path` to a
local :t:`name` by using an optional :t:`renaming`.

:dp:`fls_w4s3ed1btre3`
use self as foo -> imports the current module under the name "foo"

:dp:`fls_m233f57imu4a`
use blah::{self} -> imports "blah"

:dp:`fls_9c50s1ivmox0`
use blah::{self as foo} -> imports blah under the name "foo"

:dp:`fls_kk2jtk5ljm2t`
use blah::gah::{self} -> imports "gah"

:dp:`fls_bwy2smwycrhd`
use blah::{gah::{self as foo}} -> imports gah under the name "foo"

:dp:`fls_dd3ctnsrs1c9`
**The above imports the names in the type namespace only**

:dp:`fls_se0ffyendc6n`
When keyword ``self`` appears by itself in a use import, then the use import
shall be a simple import with a renaming.

:dp:`fls_3q7bw3hqnbo0`
When keyword ``crate``

:dp:`fls_htp8rum8mo5x`
A :t:`use import` with :t:`public visibility` is said to :dt:`re-export`
imported :t:`[name]s`. **What does this do exactly? What are the effects?**

.. rubric:: Examples

:dp:`fls_ajvmj1g2aj3q`
The following is a glob import. See :p:`14.2. <fls_q13sty1g9jtn>`
for the declaration of modules and functions. The imported functions
are ``create_visible_function``, ``outer_module_visible_function``,
``visible_function``.

.. code-block:: rust

   use outer_module::inner_module::*;

:dp:`fls_1eckpji7yo32`
The following is a renaming import. The imported function is
``visible_function`` under the name ``f``.

.. code-block:: rust

   use outer_module::inner_module::visible_function as f;

:dp:`fls_k78go91qxdtt`
The following is a selective import. The imported functions are
``crate_visible_function`` and ``visible_function``.

.. code-block:: rust

   use outer_module::inner_module
       {crate_visible_function, visible_function}

.. _fls_izl8iuhoz9e0:

Scopes
------

:dp:`fls_k9fk1icjmxgs`
`Rust
<https://github.com/rust-lang/reference/pull/1040/commits/77ab06c34e50e9cce04acf
979a4402fa01ef48e9>`_

.. rubric:: Legality Rules

:dp:`fls_5x5xykocwyiy`
A :t:`scope` is a region of program text where a :t:`name` can be referred to. A
:t:`name` is :t:`in scope` when it can be referred to.

:dp:`fls_ia3eeqrio5ur`
**How are hierarchies of scopes formed?**

.. _fls_jnknu1xzrgh4:

Associated Item Scope
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_27x1a6byib4b`
The :t:`name` of an :t:`associated item` is never :t:`in scope`.

.. _fls_4gh34bv0oii5:

Binding Scopes
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_2gk77ytv88n7`
The :t:`binding` of a :t:`closure parameter` is :t:`in scope` within the related
:t:`closure body`.

:dp:`fls_8yyomyq38ony`
The :t:`binding` of a :t:`function parameter` is :t:`in scope` within the
related :t:`function body`.

:dp:`fls_abk12gxkjddd`
The :t:`binding` of a :t:`for loop` or a :t:`while let loop` is :t:`in scope`
within the related :t:`loop body`.

:dp:`fls_dm0b4sczajlj`
The :t:`binding` of an :t:`if let expression` is :t:`in scope` within the
related :t:`block expression`.

:dp:`fls_3l9vcbay6obm`
The :t:`binding` of a :t:`let statement` is :t:`in scope` after the related
:t:`let statement` until the end of the :t:`block expression` where the related
:t:`let statement` appears.

:dp:`fls_dirjv2g18zuu`
The :t:`binding` of a :t:`match arm` is :t:`in scope` within its related
:t:`[expression]s` and related :t:`match arm guard`.

.. _fls_x6wok0k52um2:

Declarative Macro Scope
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_xbgri34o3zff`
The :t:`name` of a :t:`declarative macro` is :t:`in scope` after the related
:t:`macro rules` declaration until the end of the :t:`block expression` or the
enclosing :t:`module` where the :t:`macro rules` declaration appears.

.. _fls_ftphlagzd2te:

Generic Parameter Scope
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_pwuk2tyhrfpo`
The :t:`name` of a :t:`generic parameter` of a :t:`construct` is :t:`in scope`
within the related construct.

:dp:`fls_uocarmzavgsk`
The :t:`name` of a :t:`generic parameter` is not :t:`in scope` within
:t:`[item]s` declared inside a :t:`function`.

.. _fls_bsypfus3olxu:

Item Scope
~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_7aym7k1mtv5`
The :t:`name` of an :t:`item` declared within a :t:`module` is :t:`in scope`
within the related :t:`module`. Such a :t:`name` is not :t:`in scope` within
nested :t:`[module]s`.

:dp:`fls_l26n2qc5lmti`
The :t:`name` of an :t:`item` declared within a :t:`block expression` is :t:`in
scope` within the related :t:`block expression`.

:dp:`fls_xjzmxub1bg8v`
It is a static error to declare an :t:`item` within a :t:`block expression` or
a :t:`module` where the :t:`name` of the :t:`item` is already used by another
:t:`item` within the same :t:`block expression` or :t:`module`.

.. _fls_1oe31hyqcwmq:

Lifetime Parameter Scope
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_46jak1bsujl4`
The :t:`name` of a :t:`lifetime parameter` is :t:`in scope` within the related
:t:`implementation`, :t:`function`, or :t:`trait`. (**merge into Generic
Parameter Scopes?**)

:dp:`fls_uhxkmj9y8l4z`
The :t:`name` of a :t:`lifetime parameter` is not :t:`in scope` in
:t:`[constant]s` and :t:`[static]s`. (**isn't this redundant?**)

.. _fls_euiehnmeugd:

Loop Label Scope
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_bb6zwflhg91t`
The :t:`label` of a :t:`loop expression` is :t:`in scope` from its declaration
until the end of the related :t:`loop expression`.

:dp:`fls_i0c9ergp782r`
The :t:`label` of a :t:`loop expression` is not :t:`in scope` in :t:`[async
block]s`, :t:`[closure]s`, :t:`[constant argument]s`, :t:`[constant context]s`,
:t:`[item]s`, and the :t:`iterator expression` of the related :t:`for loop`.

.. _fls_n4enftia76e5:

Prelude Scopes
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_ri6pmto8jej3`
:t:`[Prelude name]s` are :t:`in scope` of every :t:`module`.

.. _fls_kgbi26212eof:

``Self`` Scope
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_2o0gn3xnjqel`
The :c:`Self` :t:`type` is :t:`in scope` within :t:`[abstract data type]s`,
:t:`[implementation]s`, and :t:`[trait]s`.

.. _fls_kegoxt2k6t8l:

Trait Bound Scopes
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_y86jxdq94wyg`
**Could you translate this (I never understood higher-ranked trait bounds)?**

:dp:`fls_yxor2mvixvb1`
The scope of a lifetime parameter declared as a [higher-ranked trait
bound][hrtb] depends on the scenario where it is used.

:dp:`fls_igo03vtzhzpu`
As a [_TypeBoundWhereClauseItem_] the declared lifetimes are in scope in the
type and the type bounds.

:dp:`fls_9htct0x9nw0t`
As a [_TraitBound_] the declared lifetimes are in scope within the bound type
path.

:dp:`fls_yhul6w8m9gnr`
As a [_BareFunctionType_] the declared lifetimes are in scope within the
function parameters and return type.

.. _fls_ydmnb7qnmzzq:

Shadowing
---------

.. rubric:: Legality Rules

:dp:`fls_w7d1sil7gh8z`
:dt:`Shadowing` is a property of :t:`[name]s`. A :t:`name` is said to be
:dt:`shadowed` when another :t:`name` with the same characters is introduced
in the same :t:`scope` within the same :t:`namespace`, effectively hiding it. A
:t:`name` cannot be referred to by any means once it is :t:`shadowed`.

:dp:`fls_ifrl9p79surf`
The :t:`name` of :t:`[built-in attribute]s` shall not be :t:`shadowed`.

:dp:`fls_jgs0q9seyx7g`
The :t:`name` of a :t:`generic parameter` shall not be :t:`shadowed`.

:dp:`fls_9kwknx9ycjcf`
The :t:`name` of an :t:`item` declared within a :t:`module` may shadow a
:t:`prelude name`. (**is this rule needed?**)

:dp:`fls_3eekxbuih7wc`
A :t:`binding` shall not shadow the :t:`name` of a :t:`constant parameter`,
a :t:`constant`, an :t:`enum constructor`, a :t:`static`, or a :t:`struct
constructor`.

:dp:`fls_5c7p3459gnn1`
A :t:`prelude name` shadows other :t:`[prelude name]s` depending on which
:t:`[prelude]s` are included in a :t:`module`. The order of shadowing is as
follows, where a later :t:`prelude name` shadows earlier :t:`prelude name`:

#. :dp:`fls_c28gfhfk8gn4`
   :t:`Language prelude` :t:`[name]s`.

#. :dp:`fls_nyuutp3uvclb`
   :t:`Standard library prelude` :t:`[name]s`.

#. :dp:`fls_mohwxdi59ouq`
   :t:`macro_use prelude` :t:`[name]s`.

#. :dp:`fls_ogzke6s92qme`
   :t:`Tool prelude` :t:`[name]s`.

#. :dp:`fls_qdyf00tq52p6`
   :t:`External prelude` :t:`[name]s`.

.. _fls_dq403wq5yrs:

Namespaces
----------

.. rubric:: Legality Rules

:dp:`fls_8xbtcifiyvwh`
A :dt:`namespace` is a logical grouping of :t:`[name]s`. :t:`[Name]s` are
segregated into separate :t:`[namespace]s` based on the kind of :t:`entity` the
:t:`name` belongs to. Within a :t:`namespace`, :t:`[name]s` are organized into a
hierarchy of :t:`[scope]s`.

:dp:`fls_mt7qi7yh8y0n`
A :t:`namespace` is classified as either an :t:`anonymous namespace`, a
:t:`label namespace`, a :t:`lifetime namespace`, a :t:`macro namespace`, a
:t:`type namespace`, or a :t:`value namespace`.

:dp:`fls_swgq8k4wybdw`
A :dt:`label namespace` contains the :t:`[name]s` of the following entities:

* :dp:`fls_2mj3v7nwt58s`
  :t:`[Label]s` of :t:`[loop expression]s`.

:dp:`fls_thplejq7vwaw`
A :dt:`lifetime namespace` contains the :t:`[name]s` of the following entities:

* :dp:`fls_32k27skptc8d`
  :t:`[Lifetime parameter]s` of :t:`[generic]s`.

:dp:`fls_h5gkq5ghq9uw`
A :dt:`macro namespace` contains the :t:`[name]s` of the following entities:

* :dp:`fls_riizlyw5thuy`
  :t:`[Attribute macro]s`.

* :dp:`fls_w8xdgosyjmp`
  :t:`[Built-in attribute]s`.

* :dp:`fls_e83ifgq3vacf`
  :t:`[Declarative macro]s`.

* :dp:`fls_50hu7mljy17d`
  :t:`[Derive macro]s`.

* :dp:`fls_268m2dicn4i1`
  :t:`[Function-like macro]s`.

:dp:`fls_3z1xgyaoq85f`
A :dt:`type namespace` contains the :t:`[name]s` of the following entities:

* :dp:`fls_vg2e7fmqqk3g`
  :t:`[Associated type]s`.

* :dp:`fls_rnlz8c1uxsio`
  :t:`[Boolean type]s`.

* :dp:`fls_lf5vqcip2qkp`
  :t:`[Enum type]s`.

* :dp:`fls_bum0cfgr0ov5`
  :t:`[Enum variant]s`.

* :dp:`fls_4p2irc4i13dj`
  :t:`[External crate import]s`.

* :dp:`fls_w4idm2l370gn`
  :t:`External crate prelude` :t:`[item]s`.

* :dp:`fls_sjm0azadnp1u`
  :t:`[Module]s`.

* :dp:`fls_d8vpnauthd3`
  :t:`[Numeric type]s`.

* :dp:`fls_b1dfiajmppfw`
  :c:`Self`.

* :dp:`fls_2r3x5vzckm31`
  :t:`[Struct type]s`.

* :dp:`fls_hbonb0a2h6p0`
  :t:`[Textual type]s`.

* :dp:`fls_et0rz4hgh9eg`
  :t:`[Trait]s`.

* :dp:`fls_tdpwgqehttlw`
  :t:`[Type aliase]s`.

* :dp:`fls_ige8t79v0b74`
  :t:`[Type parameter]s` of generics.

* :dp:`fls_drtv1utvmmyo`
  :t:`[Union type]s`.

:dp:`fls_19ul1yb8mvg9`
A :dt:`value namespace` contains the :t:`[name]s` of the following entities:

* :dp:`fls_cq5kqtyh6kbk`
  :t:`[Associated constant]s`.

* :dp:`fls_pttfh8f61bqn`
  :t:`[Associated function]s`.

* :dp:`fls_sxmju66ot5mo`
  Captured :t:`closure` :t:`[variable]s`.

* :dp:`fls_mhsfm533cwls`
  :t:`[Constant]s`.

* :dp:`fls_fu7jp3y2tu24`
  :t:`[Constant parameter]s` of :t:`[generic]s`.

* :dp:`fls_pi2tpb9376hk`
  :t:`[Enum variant constructor]s`.

* :dp:`fls_ji94d3rqxcj3`
  :t:`[Function]s`.

* :dp:`fls_dwajteg2puz5`
  :t:`[Pattern binding]s` of :t:`[closure parameter]s` for :t:`[loop
  expression]s`, :t:`[function parameter]s`, :t:`[if let expression]s`,
  :t:`[let statement]s`, :t:`[match arm matcher]s`, and :t:`[while let loop
  expression]s`.

* :dp:`fls_vgjsg69igyqw`
  :t:`[Self constructor]s`.

* :dp:`fls_bzmmqxhrsym`
  :t:`[Static]s`.

* :dp:`fls_hbthp73gfyyz`
  :t:`[Struct constructor]s`.

:dp:`fls_jhha0nr3vsgq`
The :t:`[name]s` of the following entities are not part of any :t:`namespace`:

* :dp:`fls_dicnt0l6pite`
  :t:`[Enum field]s`.

* :dp:`fls_ocri7tm0duhg`
  :t:`[Struct field]s`.

* :dp:`fls_j4rmmu1opec8`
  :t:`[Union field]s`.

.. _fls_c140hwzhup95:

Preludes
--------

.. rubric:: Legality Rules

:dp:`fls_9bo3y5s1fxa1`
A :dt:`prelude` is a collection of :t:`[name]s` that are automatically brought
:t:`in scope` of every :t:`module` in a :t:`crate`. Such :t:`[name]s` are
referred to as :dt:`[prelude name]s`.

:dp:`fls_z33lxzlfb14t`
The :dt:`core prelude` is a :t:`prelude` that **???**.

:dp:`fls_xqeud0y6tkds`
An :dt:`external prelude` is a :t:`prelude` that brings :t:`in scope` of
the :t:`root module` the :t:`[name]s` of the :t:`[crate]s` imported using
:t:`[external crate import]s`. If the :t:`external crate import` uses a
:t:`renaming`, then the :t:`renaming` is instead added to the :t:`external
prelude`. The :t:`core crate` is always added to the :t:`external prelude`
unless the :t:`crate root` is subject to :t:`attribute` :c:`no_core`.

:dp:`fls_s7vhr3ipu9y`
The :dt:`language prelude` is a :t:`prelude` that brings :t:`in scope` of every
:t:`module` the following :t:`[name]s`:

* :dp:`fls_up3442238u2u`
  :t:`Boolean type` :c:`bool`.

* :dp:`fls_5ivqg3milcsx`
  :t:`[Built-in attribute]s`.

* :dp:`fls_hcvdh0pbtckc`
  :t:`[Floating-point type]s` :c:`f32` and :c:`f64`.

* :dp:`fls_7fg3erf8lm38`
  :t:`[Integer type]s` :c:`i8`, :c:`i16`, :c:`i32`, :c:`i64`, :c:`i128`,
  :c:`isize`, :c:`u8`, :c:`u16`, :c:`u32`, :c:`u64`, :c:`u128`, and :c:`usize`.

* :dp:`fls_m1bb0dxgb0y0`
  :t:`[Textual type]s` :c:`char` and :c:`str`.

:dp:`fls_7csxtbclebly`
The :dt:`macro_use prelude` is a :t:`prelude` that brings :t:`in scope` of the
:t:`root module` the :t:`[name]s` of :t:`[macro]s` from :t:`[external crate]s`
that were imported using an :t:`external crate import`.

.. rubric:: Legality Rules

.. _fls_40xoego2thsp:

Name Resolution
---------------

.. rubric:: Legality Rules

:dp:`fls_s8fa9sxj9dxe`
:dt:`Name resolution` is the process of relating a :t:`path` to a :t:`name` by
considering :t:`[namespace]s`, :t:`[scope]s`, and :t:`visibility`. A :t:`path`
that is successfully related to a :t:`name` is said to be :dt:`resolved`.

:dp:`fls_ch2hhrbzqz2k`
:dt:`Containment name resolution` is a kind of :t:`name resolution` that relates
the :t:`identifier` of a :t:`path segment` to a :t:`name` that is expected to be
defined in a given :t:`module`, as follows:

#. :dp:`fls_auzozbud012u`
   Make the :t:`scope` of the given :t:`module` where the :t:`identifier`
   resides be the current :t:`scope`.

#. :dp:`fls_hcn2ac9rqzk`
   If the given :t:`module` contains a :t:`name` that matches the characters
   of the :t:`identifier`, then relate the :t:`identifier` to the matched
   :t:`name`.

#. :dp:`fls_e2w6rf1eww6`
   Otherwise this is a static error.

:dp:`fls_dxnbwhqyjlev`
:dt:`Macro name resolution` is a kind of :t:`name resolution` that relates the
:t:`identifier` of a :t:`path segment` to the :t:`name` of a :t:`declarative
macro` as follows:

#. :dp:`fls_o08avj1e2q5l`
   Make the :t:`scope` where the :t:`identifier` resides be the current
   :t:`scope`.

#. :dp:`fls_y4d9yecnjpll`
   While there is a current :t:`scope`

   #. :dp:`fls_4tc9yagzdxwl`
      If the current :t:`scope` contains a :t:`name` of a :t:`declarative macro`
      that matches the characters of the :t:`identifier`, then

      #. :dp:`fls_94fzpnkf579`
         Relate the :t:`identifier` to the matched :t:`name`.

      #. :dp:`fls_jv3v4dddhk63`
         Stop the :t:`macro name resolution`.

   #. :dp:`fls_8vhwna9f7ub0`
      Otherwise make the current :t:`scope` be the enclosing :t:`scope` of the
      current :t:`scope`.

#. :dp:`fls_kxjlf6t8tuwn`
   If the :t:`macro scope` contains a :t:`name` of a :t:`declarative macro`
   that matches the characters of the :t:`identifier`, then relate the
   :t:`identifier` to the matched :t:`name`.

#. :dp:`fls_zh3nt3nwbblt`
   Otherwise this is a static error.

:dp:`fls_8kq4piz1w2gx`
:dt:`Nearest enclosing name resolution` is a kind of :t:`name resolution`
that relates the :t:`identifier` of a :t:`path segment` to a :t:`name` that is
expected to be declared in a given :t:`namespace`, as follows:

#. :dp:`fls_yxrimll8as6d`
   Make the :t:`scope` of the given :t:`namespace` where the :t:`identifier`
   resides be the current :t:`scope`.

#. :dp:`fls_lnb9mlgyhz9f`
   While there is a current :t:`scope`

   #. :dp:`fls_4g4375us47yb`
      If the current :t:`scope` contains a :t:`name` that matches the characters
      of the :t:`identifier`, then

      #. :dp:`fls_c225wy3moan2`
         Relate the :t:`identifier` to the matched :t:`name`.

      #. :dp:`fls_722t9zdg5dih`
         Stop the :t:`nearest enclosing name resolution`.

   #. :dp:`fls_2wag4633ktz1`
      Otherwise make the current :t:`scope` be the enclosing :t:`scope` of the
      current :t:`scope`.

#. :dp:`fls_xsqflv4vzsdv`
   If the :t:`prelude scope` contains a :t:`name` that matches the characters
   of the :t:`identifier`, then relate the :t:`identifier` to the matched
   :t:`name`.

#. :dp:`fls_8ob8cwl8uqhc`
   Otherwise this is a static error.

:dp:`fls_mnlz6v5ut9q2`
:dt:`Type name resolution` is a kind of :t:`name resolution` that relates the
:t:`identifier` of a :t:`path segment` to a :t:`name` that is expected to be
declared in an :t:`implementation` of a :t:`type`, as follows:

* :dp:`fls_73rswlz0rkwk`
  **Explain**

* :dp:`fls_zh70yix8fbvp`
  **I can't figure out how to hook this into the algorithm below**

:dp:`fls_ydyjavnku9bg`
If a :t:`path` consists of multiple :t:`[path segment]s`, then the :t:`path` is
resolved as follows:

#. :dp:`fls_56e0pov11z3o`
   Make the first :t:`path segment` be the current :t:`path segment`.

#. :dp:`fls_yfarrr5cz8zg`
   Perform :t:`nearest enclosing name resolution`, where the :t:`path segment`
   is the current :t:`path segment` and the :t:`namespace` is the :t:`type
   namespace`.

#. :dp:`fls_4m7tv3tyizv0`
   If the current :t:`path segment` did not resolve to a :t:`module`, then this
   is a static error.

#. :dp:`fls_oxjlvkxt08ey`
   Make the current :t:`path segment` be the previous :t:`path segment`.

#. :dp:`fls_nkv9qmwz32rj`
   Make the next :t:`path segment` be the current :t:`path segment`.

#. :dp:`fls_nzs8j9kivsw4`
   While the current :t:`path segment` is not the last :t:`path segment`

   #. :dp:`fls_fz70hv7pzpzx`
      Perform :t:`containment name resolution`, where the :t:`path segment` is
      the current :t:`path segment` and the :t:`module` is the :t:`module` that
      the previous path segment resolved to.

   #. :dp:`fls_6eya9q7jir3k`
      If the current :t:`path segment` did not resolve to a :t:`module`, then
      this is a static error.

   #. :dp:`fls_min9nnblcc7g`
      Make the current :t:`path segment` be the previous :t:`path segment`.

   #. :dp:`fls_4smxzvsncyp8`
      Make the next :t:`path segment` be the current :t:`path segment`.

#. :dp:`fls_pgepm7dqj9b4`
   Perform :t:`containment name resolution`, where the :t:`path segment` is the
   current :t:`path segment` and the :t:`module` is the :t:`module` that the
   previous :t:`path segment` resolved to. (**more?**)

:dp:`fls_gz9sxvudppg7`
It is a static error if a :t:`path` that consists of multiple :t:`[path
segment]s` cannot be related to a :t:`name` of an :t:`item`.

:dp:`fls_py6hxo1edqvo`
A :t:`global path` is resolved starting from the :t:`external prelude`.

:dp:`fls_vj7ctoj65ft`
If a :t:`path` starts with qualifier ``crate``, then the :t:`path` is resolved
relative to the current :t:`crate`.

:dp:`fls_u70rq8qvont3`
If a :t:`path` starts with qualifier ``$crate``, then the :t:`path` is resolved
relative to the :t:`crate` where the related :t:`macro` is declared.

:dp:`fls_sfd92kxfe3o`
If a :t:`path` starts with qualifier ``self``, then the :t:`path` is resolved
relative to the current :t:`module`.

:dp:`fls_uqqqsrqyg9k8`
If a :t:`path` starts with qualifier ``Self``, then the :t:`path` is resolved
relative to the :t:`implementing type` within an :t:`implementation` or a
:t:`trait`.

:dp:`fls_q76k2ln276hw`
If a :t:`path` starts with qualifier ``super``, then the :t:`path` is resolved
relative to the parent :t:`module`.

:dp:`fls_yx78nuv60zdv`
If a :t:`path` consists of a single :t:`path segment`, then the :t:`path` is
resolved as follows:

#. :dp:`fls_rxgorbvuujjn`
   Make the first :t:`path segment` be the current :t:`path segment`.

#. :dp:`fls_dr5zn2g13x9l`
   Perform :t:`nearest enclosing name resolution` where the :t:`path segment` is
   the current :t:`path segment`, and the :t:`namespace` is **which one???**.

:dp:`fls_4ldkkufsajm0`
It is a static error if a :t:`path` that consists of a single :t:`path segment`
cannot be related to the :t:`name` of a locally declared :t:`item` or a locally
declared :t:`variable`.

:dp:`fls_wqrttnn5py4y`
A :t:`macro invocation` is resolved using :t:`macro name resolution`.

