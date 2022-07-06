.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Names and Resolution
====================

.. rubric:: Syntax

.. syntax::

   Name ::=
       Identifier

.. rubric:: Legality Rules

:dp:`fls_yf5r11o1kevr`
A :dt:`name` identifies an entity within the program text.

:dp:`fls_w5snt8shnkbn`
An :dt:`entity` is a construct that can be referred to within a program by using
a :t:`path`.

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

:dp:`fls_59qhsxbly73f`
A :s:`VisibilityModifier` shall denote either a ``CratePublicModifier``,
a ``SelfPublicModifier``, a ``SimplePathPublicModifier``, a
``SimplePublicModifier``, or a ``SuperPublicModifier``.

:dp:`fls_2tp6zj9b79ma`
A :s:`CratePublicModifier` shall start with :t:`keyword` ``pub``, followed by
character 0x28 (left parenthesis), followed by keyword ``crate``, followed by
character 0x29 (right parenthesis).

:dp:`fls_1nz0w1yaqwka`
A :s:`SelfPublicModifier` shall start with keyword ``pub``, followed by
character 0x28 (left parenthesis), followed by keyword ``self``, followed by
character 0x29 (right parenthesis).

:dp:`fls_rji599qcit8g`
A :s:`SelfPublicModifier` shall start with keyword ``pub``, followed by
character 0x28 (left parenthesis), followed by keyword ``in``, followed by a
:s:`SimplePath`, followed by character 0x29 (right parenthesis).

:dp:`fls_3zbkfw1yf2ko`
A :s:`SimplePublicModifier` shall denote keyword ``pub``.

:dp:`fls_as9yn24nwug0`
A :s:`SuperPublicModifier` shall start with keyword ``pub``, followed by
character 0x28 (left parenthesis), followed by keyword ``super``, followed by
character 0x29 (right parenthesis).

:dp:`fls_siva1u877xi`
A ``VisibilityModifier`` describes the visibility of a :t:`name`.
:dt:`Visibility` is a property that determines which modules can refer to the
name.

:dp:`fls_7e4u7eo1ifm5`
A name with :dt:`public visibility` can be referred to from arbitrary module
``M`` as long as the ancestor modules of the related entity can be referred to
from ``M``. Such a name is said to be :dt:`public`.

:dp:`fls_h1mhi47vae1q`
A name with :dt:`private visibility` can be referred to only by the current
module of the entity, and its descendant modules. Such a name is said to be
:dt:`private`.

:dp:`fls_aa4f3rvir9lm`
A ``CratePublicModifier`` makes a name public within the current :t:`crate`
only.

:dp:`fls_tnh7o3pb4e22`
A ``SelfPublicModifier`` makes a name public within the current module
and any descendant modules. A ``SelfPublicModifier`` is equivalent to a
``SimplePathPublicModifier`` where the ``SimplePath`` denotes keyword ``self``.

:dp:`fls_yymgpyi67dty`
A ``SimplePathPublicModifier`` makes a name public within the provided
``SimplePath`` only. The ``SimplePath`` of a ``SimplePathPublicModifier``
shall start with a ``SimplePathSegment`` expressed by either keyword ``crate``,
keyword ``self``, or keyword ``super``. The ``SimplePath`` shall resolve to an
ancestor module of the current module or the current module itself.

:dp:`fls_np8aghofjqhm`
A ``SimplePublicModifier`` makes a name public.

:dp:`fls_quzvhzpr0124`
A ``SuperPublicModifier`` makes a name public within the parent module only. A
``SuperPublicModifier`` is equivalent to a ``SimplePathPublicModifier`` where
the ``SimplePath`` denotes keyword ``super``.

:dp:`fls_utgjx6l5zwfl`
An :t:`external item`, a :t:`field`, or an :t:`item` that appears without a
``VisibilityModifier`` is private by default.

:dp:`fls_jifg2st5bfd6`
An :t:`associated item` of a public :t:`trait` is public by default.

:dp:`fls_dm0xr424ine1`
An :t:`enum variant and its fields` have the same visibility as the containing
:t:`enum type`.

.. rubric:: Examples

.. code-block:: text

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

:dp:`fls_pur93emodd9f`
A :s:`SimplePath` shall start with optional character sequence 0x3A 0x3A (colon,
colon), followed by a ``SimplePathSegment``, followed by a sequence of zero or
more of character sequence 0x3A 0x3A (colon, colon) and ``SimplePathSegment``.

:dp:`fls_xmvr3jrcp2ym`
A :s:`SimplePathList` shall start with a ``SimplePath``, followed by a sequence
of zero or more of character 0x2C (comma) and ``SimplePath``, followed by
optional character 0x2C (comma).

:dp:`fls_43a9n1al8udf`
A :s:`SimplePathSegment` shall denote either an :s:`Identifier`, :t:`keyword`
``crate``, keyword ``$crate``, keyword ``self``, or keyword ``super``.

:dp:`fls_92phcct5f0ln`
A :s:`PathInExpression` shall start with optional character sequence 0x3A
0x3A (colon, colon), followed by a ``PathInExpressionSegment``, followed by
a sequence of zero or more of character sequence 0x3A 0x3A (colon, colon) and
``PathInExpressionSegment``.

:dp:`fls_o9te6bmngc8i`
A :s:`PathInExpressionSegment` shall start with a ``PathSegment``,
followed by optional character sequence 0x3A 0x3A (colon, colon) and
:s:`GenericArgumentList`.

:dp:`fls_za8qfqx116qf`
A :s:`PathSegment` shall denote either an ``Identifier``, keyword ``crate``,
keyword ``$crate``, keyword ``self``, keyword ``Self``, or keyword ``super``.

:dp:`fls_akqey3or732n`
A :s:`TypePath` shall start with optional character sequence 0x3A 0x3A (colon,
colon), followed by a ``TypePathSegment``, followed by a sequence of zero or
more of character sequence 0x3A 0x3A (colon, colon) and ``TypePathSegment``.

:dp:`fls_2fl9gkl19k0k`
A :s:`TypePathSegment` shall start with a ``PathSegment``, followed by
optional character sequence 0x3A 0x3A (colon, colon), followed by optional
``GenericArgumentList`` or ``TypePathFn``.

:dp:`fls_8od85h7qez4`
A :s:`TypePathFn` shall start with character 0x28 (left parenthesis), followed
by an optional :s:`TypeSpecificationList`, followed by character 0x29 (right
parenthesis), followed by optional :s:`ReturnType`.

:dp:`fls_ucjm04s4609b`
A :s:`QualifiedPathInExpression` shall start with a ``QualifiedPathType``,
followed by a sequence of one or more of character sequence 0x3A 0x3A (colon,
colon) and ``PathInExpressionSegment``.

:dp:`fls_81o41wytt2hd`
A :s:`QualifiedPathType` shall start with character 0x3C (less-than
sign), followed by a :s:`TypeSpecification`, followed by an optional
``TypePathRenaming``, followed by character 0x3E (greater-than sign).

:dp:`fls_6it6jhgeqc1z`
A :s:`TypePathRenaming` shall start with keyword ``as``, followed by a
``TypePath``.

:dp:`fls_1c3ge9nimif0`
A :s:`QualifiedPathInType` shall start with a ``QualifiedPathType``, followed
by a sequence of one or more of character sequence 0x3A 0x3A (colon, colon) and
``TypePathSegment``.

:dp:`fls_uo6mu0tx3pk8`
A ``SimplePath`` describes a :dt:`simple path`. A ``PathInExpression`` describes
a :dt:`path-in-expression`. A ``TypePath`` describes a :dt:`type path`. A
``QualifiedPathInExpression`` describes a :dt:`qualified path-in-expression`. A
``QualifiedPathInType`` describes a :dt:`qualified path-in-type`. Collectively,
simple paths, paths-in-expressions, type paths, qualified paths-in-expressions,
and qualified paths-in-types are referred to as :dt:`path`\ s.

:dp:`fls_cnk0t4oj3yg4`
A ``PathSegment``, ``SimplePathSegment``, and ``TypePathSegment`` describe a
:dt:`path segment`.

:dp:`fls_75s5ivc2cesf`
A path is a sequence of one or more path segments logically separated by
:dt:`namespace qualifier` ``::`` that ultimately resolves to an :t:`entity`. A
path that starts with namespace qualifier ``::`` is referred to as a :dt:`global
path`.

:dp:`fls_23lpzxs14wnk`
A path that starts with qualifier ``$crate`` shall appear only within a
:t:`macro transcriber`.

:dp:`fls_uheh0jvg6yie`
**Should talk about the effects of a QualifiedPathType with respect to
generics.**

:dp:`fls_uldqdgn520vs`
The :dt:`canonical path` of an :t:`entity` is the fully qualified path to that
entity's :t:`name` starting from the current :t:`crate`.

:dp:`fls_tjwehd9ofzrj`
The following :t:`[construct]s` do not have a canonical path:

* :dp:`fls_j5m34x689twd`
  :t:`[Associated item]s`.

* :dp:`fls_1s77w8eorw65`
  :t:`[Implementation]s`.

* :dp:`fls_lt91iobkn6b4`
  :t:`[Use import]s`.

* :dp:`fls_6k0esdfp4nqs`
  :t:`[Item]s` declared in :t:`[block expression]s`.

* :dp:`fls_7hr4d9gmnd2u`
  Items declared in a :t:`module` where the module does not have a canonical
  path.

.. rubric:: Examples

:dp:`fls_cul31g1kkz5c`
The following is a simple path. See :p:`14.2. <fls_q13sty1g9jtn>` for the
declaration of ``crate_visible_function``.

.. code-block:: text

   crate::outer_module::inner_module::crate_visible_function();

:dp:`fls_no853u27p4f3`
The following is a path-in-expression.

.. code-block:: text

   Vec::<u8>::with_capacity(42);

:dp:`fls_28c21rzc6rsp`
The following is a path-in-type.

.. code-block:: text

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

.. code-block:: text

   <S as T>::f();

:dp:`fls_ojdntg5i79pb`
**Add an example for qualified path-in-expression.**

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

:dp:`fls_e4gdr1on8fqr`
A :s:`UseImport` shall start with :t:`keyword` ``use``, followed by a
``UseImportContent``, followed by character 0x3B (semicolon).

:dp:`fls_po9tfbhn7sgs`
A :s:`UseImportContent` shall denote either a ``GlobImport``, a
``SimpleImport``, or a ``NestingImport``.

:dp:`fls_uotjut4lw2y8`
A :s:`GlobImport` shall start with an optional ``SimplePathPrefix``, followed by
character 0x2A (asterisk).

:dp:`fls_gt0dq4n94ej`
A :s:`SimpleImport` shall start with a :s:`SimplePath`, followed by an optional
:s:`Renaming`.

:dp:`fls_o46ntzgb8hmy`
A :s:`NestingImport` shall start with an optional ``SimplePathPrefix``,
followed by character 0x7B (left curly bracket), followed by an optional
``UseImportContentList``, followed by character 0x7D (right curly bracket).

:dp:`fls_tc86789gq2v0`
A :s:`SimplePathPrefix` shall start with an optional ``SimplePath``, followed by
character sequence 0x3A 0x3A (colon, colon).

:dp:`fls_8h2irgmeqpea`
A :s:`UseImportContentList` shall start with a ``UseImportContent``, followed by
a sequence of zero or more of character 0x2C (comma) and ``UseImportContent``,
followed by optional character 0x2C (comma).

:dp:`fls_4qgflb8tsdo8`
A ``UseImport`` describes a :dt:`use import` or simply an import. An
:dt:`import` brings :t:`[name]s` into :t:`scope` within the :t:`module` or block
expression where the use import resides.

:dp:`fls_xpn0gi2euo01`
A ``GlobImport`` describes a glob import. A :dt:`glob import` brings all
:t:`public` names prefixed by its ``SimplePathPrefix`` into scope.

:dp:`fls_wln1ydrlaqy`
A glob import shall contain a ``SimplePath``.

:dp:`fls_lahpu2vk6aqu`
A ``NestingImport`` describes a nesting import. A :dt:`nesting import` provides
a common path prefix for its nested use imports.

:dp:`fls_t64jr6jd1723`
A ``SimpleImport`` describes a simple import. A :dt:`simple import` binds a
:t:`simple path` to a local name by using an optional :t:`renaming`.

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
An import with public visibility is said to :dt:`re-export` imported names.
**What does this do exactly? What are the effects?**

.. rubric:: Examples

:dp:`fls_ajvmj1g2aj3q`
The following is a glob import. See :p:`14.2. <fls_q13sty1g9jtn>`
for the declaration of modules and functions. The imported functions
are ``create_visible_function``, ``outer_module_visible_function``,
``visible_function``.

.. code-block:: text

   use outer_module::inner_module::*;

:dp:`fls_1eckpji7yo32`
The following is a renaming import. The imported function is
``visible_function`` under the name ``f``.

.. code-block:: text

   use outer_module::inner_module::visible_function as f;

:dp:`fls_k78go91qxdtt`
The following is a selective import. The imported functions are
``crate_visible_function`` and ``visible_function``.

.. code-block:: text

   use outer_module::inner_module
       {crate_visible_function, visible_function}

Scopes
------

:dp:`fls_k9fk1icjmxgs`
`Rust
<https://github.com/rust-lang/reference/pull/1040/commits/77ab06c34e50e9cce04acf
979a4402fa01ef48e9>`_

.. rubric:: Legality Rules

:dp:`fls_5x5xykocwyiy`
A :dt:`scope` is a region of program text where a :t:`name` can be referred to.
Such a name is said to be :dt:`in scope`.

:dp:`fls_ia3eeqrio5ur`
**How are hierarchies of scopes formed?**

Associated Item Scope
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_27x1a6byib4b`
The :t:`name` of an :t:`associated item` is never :t:`in scope`.

Binding Scopes
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_2gk77ytv88n7`
The :t:`binding` of a :t:`closure parameter` is :t:`in scope` within the related
:t:`closure body`.

:dp:`fls_8yyomyq38ony`
The binding of a :t:`function parameter` is in scope within the related
:t:`function body`.

:dp:`fls_abk12gxkjddd`
The binding of a :t:`for loop` or a :t:`while let loop` is in scope within the
related :t:`loop body`.

:dp:`fls_dm0b4sczajlj`
The binding of an :t:`if let expression` is in scope within the related
:t:`block expression`.

:dp:`fls_3l9vcbay6obm`
The binding of a :t:`let statement` is in scope after the related let statement
until the end of the :t:`block expression` where the related let statement
appears.

:dp:`fls_dirjv2g18zuu`
The binding of a :t:`match arm` is in scope within its related
:t:`[expression]s` and related :t:`match arm guard`.

Declarative Macro Scope
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_xbgri34o3zff`
The name of a declarative macro is in scope after the related macro rules
declaration until the end of the :t:`block expression` or the enclosing
:t:`module` where the macro rules declaration appears.

Generic Parameter Scope
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_pwuk2tyhrfpo`
The :t:`name` of a :t:`generic parameter` of a :t:`construct` is :t:`in scope`
within the related construct.

:dp:`fls_uocarmzavgsk`
The name of a generic parameter is not in scope within :t:`[item]s` declared
inside a :t:`function`.

Item Scope
~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_7aym7k1mtv5`
The :t:`name` of an :t:`item` declared within a :t:`module` is :t:`in scope`
within the related module. Such a name is not in scope within nested modules.

:dp:`fls_l26n2qc5lmti`
The name of an item declared within a :t:`block expression` is in scope within
the related block expression.

:dp:`fls_xjzmxub1bg8v`
It is a static error to declare an item within a block expression or a module
where the name of the item is already used by another item within the same block
expression or module.

Lifetime Parameter Scope
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_46jak1bsujl4`
The :t:`name` of a :t:`lifetime parameter` is :t:`in scope` within the related
:t:`implementation`, :t:`function`, or :t:`trait`. (**merge into Generic
Parameter Scopes?**)

:dp:`fls_uhxkmj9y8l4z`
The name of a lifetime parameter is not in scope in :t:`[constant]s` and
:t:`[static]s`. (**isn't this redundant?**)

Loop Label Scope
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_bb6zwflhg91t`
The :t:`label` of a :t:`loop expression` is :t:`in scope` from its declaration
until the end of the related loop expression.

:dp:`fls_i0c9ergp782r`
The label of a loop expression is not in scope in :t:`[async block]s`,
:t:`[closure]s`, :t:`[constant argument]s`, :t:`[constant context]s`,
:t:`[item]s`, and the iterator expression of the related :t:`for loop`.

Prelude Scopes
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_ri6pmto8jej3`
:t:`[Prelude name]s` are :t:`in scope` of every :t:`module`.

``Self`` Scope
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_2o0gn3xnjqel`
The :c:`Self` :t:`type` is :t:`in scope` within :t:`[abstract data type]s`,
:t:`[implementation]s`, and :t:`[trait]s`.

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

Shadowing
---------

.. rubric:: Legality Rules

:dp:`fls_w7d1sil7gh8z`
:dt:`Shadowing` is a property of :t:`[name]s`. A name is said to be
:dt:`shadowed` when another name with the same characters is introduced in the
same scope within the same namespace, effectively hiding it. A name cannot be
referred to by any means once it is shadowed.

:dp:`fls_ifrl9p79surf`
The name of :t:`[built-in attribute]s` shall not be shadowed.

:dp:`fls_jgs0q9seyx7g`
The name of a :t:`generic parameter` shall not be shadowed.

:dp:`fls_9kwknx9ycjcf`
The name of an :t:`item` declared within a :t:`module` may shadow a :t:`prelude
name`. (**is this rule needed?**)

:dp:`fls_3eekxbuih7wc`
A :t:`binding` shall not shadow the name of a :t:`constant parameter`, a
:t:`constant`, an :t:`enum constructor`, a :t:`static`, or a :t:`struct
constructor`.

:dp:`fls_5c7p3459gnn1`
A :t:`prelude name` shadows other prelude names depending on which
:t:`[prelude]s` are included in a module. The order of shadowing is as follows,
where a later prelude name shadows earlier prelude name:

#. :dp:`fls_c28gfhfk8gn4`
   :t:`Language prelude` names.

#. :dp:`fls_nyuutp3uvclb`
   :t:`Standard library prelude` names.

#. :dp:`fls_mohwxdi59ouq`
   :t:`macro_use prelude` names.

#. :dp:`fls_ogzke6s92qme`
   :t:`Tool prelude` names.

#. :dp:`fls_qdyf00tq52p6`
   :t:`External prelude` names.

Namespaces
----------

.. rubric:: Legality Rules

:dp:`fls_8xbtcifiyvwh`
A :dt:`namespace` is a logical grouping of :t:`[name]s`. Names are segregated
into separate namespaces based on the kind of :t:`entity` the name belongs to.
Within a namespace, names are organized into a hierarchy of :t:`[scope]s`.

:dp:`fls_mt7qi7yh8y0n`
A namespace is classified as either an anonymous namespace, a label namespace, a
lifetime namespace, a macro namespace, a type namespace, or a value namespace.

:dp:`fls_swgq8k4wybdw`
A :dt:`label namespace` contains the names of the following entities:

* :dp:`fls_2mj3v7nwt58s`
  :t:`[Label]s` of :t:`[loop expression]s`.

:dp:`fls_thplejq7vwaw`
A :dt:`lifetime namespace` contains the names of the following entities:

* :dp:`fls_32k27skptc8d`
  :t:`[Lifetime parameter]s` of :t:`[generic]s`.

:dp:`fls_h5gkq5ghq9uw`
A :dt:`macro namespace` contains the names of the following entities:

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
A :dt:`type namespace` contains the names of the following entities:

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
A :dt:`value namespace` contains the names of the following entities:

* :dp:`fls_cq5kqtyh6kbk`
  :t:`[Associated constant]s`.

* :dp:`fls_pttfh8f61bqn`
  :t:`[Associated function]s`.

* :dp:`fls_sxmju66ot5mo`
  Captured :t:`closure` variables.

* :dp:`fls_mhsfm533cwls`
  :t:`[Constant]s`.

* :dp:`fls_fu7jp3y2tu24`
  :t:`[Constant parameter]s` of generics.

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
The names of the following entities are not part of any namespace:

* :dp:`fls_dicnt0l6pite`
  :t:`[Enum field]s`.

* :dp:`fls_ocri7tm0duhg`
  :t:`[Struct field]s`.

* :dp:`fls_j4rmmu1opec8`
  :t:`[Union field]s`.

Preludes
--------

.. rubric:: Legality Rules

:dp:`fls_9bo3y5s1fxa1`
A :dt:`prelude` is a collection of :t:`[name]s` that are automatically brought
:t:`in scope` of every :t:`module` in a :t:`crate`. Such names are referred to
as :dt:`prelude name`\ s.

:dp:`fls_z33lxzlfb14t`
The :dt:`core prelude` is a prelude that **???**.

:dp:`fls_xqeud0y6tkds`
An :dt:`external prelude` is a prelude that brings in scope of the :t:`root
module` the names of the crates imported using :t:`[external crate import]s`.
If the external crate import uses a :t:`renaming`, then the renaming is instead
added to the external prelude. The core crate is always added to the external
prelude unless the crate root is subject to attribute ``no_core``.

:dp:`fls_s7vhr3ipu9y`
The :dt:`language prelude` is a prelude that brings in scope of every module the
following names:

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
The :dt:`macro_use prelude` is a prelude that brings in scope of the root module
the names of :t:`[macro]s` from :t:`[external crate]s` that were imported using
an external crate import.

.. rubric:: Legality Rules

Name Resolution
---------------

.. rubric:: Legality Rules

:dp:`fls_s8fa9sxj9dxe`
:dt:`Name resolution` is the process of relating a :t:`path` to a :t:`name` by
considering :t:`[namespace]s`, :t:`[scope]s`, and :t:`visibility`. A path that
is successfully related to a name is said to be :dt:`resolved`.

:dp:`fls_ch2hhrbzqz2k`
:dt:`Containment name resolution` is a kind of name resolution that relates the
:t:`identifier` of a :t:`path segment` to a name that is expected to be defined
in a given module, as follows:

#. :dp:`fls_auzozbud012u`
   Make the scope of the given module where the identifier resides be the
   current scope.

#. :dp:`fls_hcn2ac9rqzk`
   If the given module contains a name that matches the characters of the
   identifier, then relate the identifier to the matched name.

#. :dp:`fls_e2w6rf1eww6`
   Otherwise this is a static error.

:dp:`fls_dxnbwhqyjlev`
:dt:`Macro name resolution` is a kind of name resolution that relates the
identifier of a path segment to the name of a :t:`declarative macro` as follows:

#. :dp:`fls_o08avj1e2q5l`
   Make the scope where the identifier resides be the current scope.

#. :dp:`fls_y4d9yecnjpll`
   While there is a current scope

   #. :dp:`fls_4tc9yagzdxwl`
      If the current scope contains a name of a declarative macro that matches
      the characters of the identifier, then

      #. :dp:`fls_94fzpnkf579`
         Relate the identifier to the matched name.

      #. :dp:`fls_jv3v4dddhk63`
         Stop the macro name resolution.

   #. :dp:`fls_8vhwna9f7ub0`
      Otherwise make the current scope be the enclosing scope of the current
      scope.

#. :dp:`fls_kxjlf6t8tuwn`
   If the :t:`macro scope` contains a name of a declarative macro that matches
   the characters of the identifier, then relate the identifier to the matched
   name.

#. :dp:`fls_zh3nt3nwbblt`
   Otherwise this is a static error.

:dp:`fls_8kq4piz1w2gx`
:dt:`Nearest enclosing name resolution` is a kind of name resolution that
relates the identifier of a path segment to a name that is expected to be
declared in a given namespace, as follows:

#. :dp:`fls_yxrimll8as6d`
   Make the scope of the given namespace where the identifier resides be the
   current scope.

#. :dp:`fls_lnb9mlgyhz9f`
   While there is a current scope

   #. :dp:`fls_4g4375us47yb`
      If the current scope contains a name that matches the characters of the
      identifier, then

      #. :dp:`fls_c225wy3moan2`
         Relate the identifier to the matched name.

      #. :dp:`fls_722t9zdg5dih`
         Stop the nearest enclosing name resolution.

   #. :dp:`fls_2wag4633ktz1`
      Otherwise make the current scope be the enclosing scope of the current
      scope.

#. :dp:`fls_xsqflv4vzsdv`
   If the :t:`prelude scope` contains a name that matches the characters of the
   identifier, then relate the identifier to the matched name.

#. :dp:`fls_8ob8cwl8uqhc`
   Otherwise this is a static error.

:dp:`fls_mnlz6v5ut9q2`
:dt:`Type name resolution` is a kind of name resolution that relates the
identifier of a path segment to a name that is expected to be declared in an
implementation of a type, as follows:

* :dp:`fls_73rswlz0rkwk`
  **Explain**

* :dp:`fls_zh70yix8fbvp`
  **I can't figure out how to hook this into the algorithm below**

:dp:`fls_ydyjavnku9bg`
If a path consists of multiple path segments, then the path is resolved as
follows:

#. :dp:`fls_56e0pov11z3o`
   Make the first path segment be the current path segment.

#. :dp:`fls_yfarrr5cz8zg`
   Perform nearest enclosing name resolution, where the path segment is the
   current path segment and the namespace is the :t:`type namespace`.

#. :dp:`fls_4m7tv3tyizv0`
   If the current path segment did not resolve to a :t:`module`, then this is a
   static error.

#. :dp:`fls_oxjlvkxt08ey`
   Make the current path segment be the previous path segment.

#. :dp:`fls_nkv9qmwz32rj`
   Make the next path segment be the current path segment.

#. :dp:`fls_nzs8j9kivsw4`
   While the current path segment is not the last segment

   #. :dp:`fls_fz70hv7pzpzx`
      Perform containment name resolution, where the path segment is the current
      path segment and the module is the module that the previous path segment
      resolved to.

   #. :dp:`fls_6eya9q7jir3k`
      If the current path segment did not resolve to a module, then this is a
      static error.

   #. :dp:`fls_min9nnblcc7g`
      Make the current path segment be the previous path segment.

   #. :dp:`fls_4smxzvsncyp8`
      Make the next path segment be the current path segment.

#. :dp:`fls_pgepm7dqj9b4`
   Perform containment name resolution, where the path segment is the current
   path segment and the module is the module that the previous path segment
   resolved to. (**more?**)

:dp:`fls_gz9sxvudppg7`
It is a static error is a path that consists of multiple path segments cannot be
related to the name of an item.

:dp:`fls_py6hxo1edqvo`
A :t:`global path` is resolved starting from the :t:`external prelude`.

:dp:`fls_vj7ctoj65ft`
If a path starts with qualifier ``crate``, then the path is resolved relative to
the current :t:`crate`.

:dp:`fls_u70rq8qvont3`
If a path starts with qualifier ``$crate``, then the path is resolved relative
to the crate where the related :t:`macro` is declared.

:dp:`fls_sfd92kxfe3o`
If a path starts with qualifier ``self``, then the path is resolved relative to
the current module.

:dp:`fls_uqqqsrqyg9k8`
If a path starts with qualifier ``Self``, then the path is resolved relative to
the :t:`implementing type` within an :t:`implementation` or a :t:`trait`.

:dp:`fls_q76k2ln276hw`
If a path starts with qualifier ``super``, then the path is resolved relative to
the parent module.

:dp:`fls_yx78nuv60zdv`
If a path consists of a single path segment, then the path is resolved as
follows:

#. :dp:`fls_rxgorbvuujjn`
   Make the first path segment be the current path segment.

#. :dp:`fls_dr5zn2g13x9l`
   Perform nearest enclosing name resolution where the path segment is the
   current path segment, and the namespace is **which one???**.

:dp:`fls_4ldkkufsajm0`
It is a static error if a path that consists of a single path segment cannot be
related to the name of a locally declared item or a locally declared variable.

:dp:`fls_wqrttnn5py4y`
A macro invocation is resolved using macro name resolution.

