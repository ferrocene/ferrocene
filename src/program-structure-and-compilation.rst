.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Program Structure and Compilation
=================================

Source Files
------------

.. rubric:: Syntax

.. syntax::


   SourceFile ::=
       ZeroWidthNoBreakSpace?
       Shebang?
       InnerAttributeOrDoc*
       Item*

   ZeroWidthNoBreakSpace ::=
       $$\u{FEFF}$$
   Shebang ::=
       $$#!$$ ~[NewLine]+

   NewLine ::=
       $$\n$$


.. rubric:: Legality Rules

:def_p:`fls_4vicosdeaqmp`
A :term:`source file` contains the program text consisting of :term:`inner
attribute`\ s, :term:`inner doc comment`\ s and :term:`item`\ s. The location of
a :term:`source file` is tool defined.

:def_p:`fls_ann3cha1xpek`
A :syntax:`Shebang` does not have an effect on the compilation.

Modules
-------

.. rubric:: Syntax

.. syntax::

   ModuleDeclaration ::=
       $$unsafe$$? $$mod$$ Name ModuleSpecification

   ModuleSpecification =
       InlineModuleSpecification
     | OutlineModuleSpecification

   InlineModuleSpecification ::=
       $${$$
         InnerAttributeOrDoc*
         Item*
       $$}$$

   OutlineModuleSpecification = $$;$$


.. rubric:: Legality Rules

:def_p:`fls_odd1hj3y1mgu`
A :term:`module` is a container for zero or more :term:`item`\ s.

:def_p:`fls_l4ne3w8dgghv`
An :term:`unsafe module` is a :term:`module` subject to :term:`keyword`
``unsafe``.

:def_p:`fls_whgv72emrm47`
The ``unsafe`` :term:`keyword` of an :term:`unsafe module` is rejected, but may
still be consumed by :term:`macro`\ s.

:def_p:`fls_qypjjpcf8uwq`
An :term:`inline module` is a :term:`module` with an
:syntax:`InlineModuleSpecification`.

:def_p:`fls_cavwpr1ybk37`
An :term:`outline module` is a :term:`module` with an
:syntax:`OutlineModuleSpecification`.

:def_p:`fls_plepew2319g4`
An :term:`outline module` loads a :term:`source file` that considers the text of
the :term:`source file` to be inlined within the context of the :term:`outline
module`.

:def_p:`fls_1aruwps62c4p`
The location of a :term:`module` :term:`source file` can be specified using
:term:`attribute` :codeterm:`path`.

:def_p:`fls_xtfx03s99egy`
The following :term:`attribute`\ s apply to :term:`module`\ s:

* :def_p:`fls_na6trldagfzb`
  :term:`Attribute` :codeterm:`cfg`.

* :def_p:`fls_renjf86vcve0`
  :term:`Attribute` :codeterm:`deprecated`.

* :def_p:`fls_3dcjkx4c1ui1`
  :term:`Attribute` :codeterm:`doc`.

* :def_p:`fls_ihtj1e11hm88`
  :term:`Attribute` :codeterm:`no_implicit_prelude`.

* :def_p:`fls_3glucm48a5fy`
  :term:`Attribute` :codeterm:`path`.

.. rubric:: Examples

.. code-block:: text


   #[path = "path/to/module"]
   pub mod module {
   	#![allow(dead_code)]
   struct Struct;
   pub mod other;
   }

Crates
------

.. rubric:: Legality Rules

:def_p:`fls_qwghk79ok5h0`
A :term:`crate` is a unit of compilation and linking. A :term:`crate` contains a
tree of nested :term:`module` :term:`scope`\ s.

:def_p:`fls_9ub6ks8qrang`
A :term:`binary crate` is a :term:`crate` that contains a :term:`main function`.
A tool can compile a :term:`binary crate` to an executable.

:def_p:`fls_d9nn4yuiw1ja`
A :term:`library crate` is either a :term:`crate` without a :term:`main
function` or a :term:`crate` subject to :term:`attribute` :codeterm:`no_main`. A
tool is free to compile a :term:`library crate` to a shared library.

External Crates
---------------

.. rubric:: Syntax

.. syntax::

   ExternCrateImport ::=
       $$extern$$ $$crate$$ CrateIndication Renaming? $$;$$

   CrateIndication ::=
       Identifier
     | $$self$$

.. rubric:: Legality Rules

:def_p:`fls_d0pa807s5d5h`
A :term:`crate import` specifies a required dependency on an external
:term:`crate`.

:def_p:`fls_ft860vkz0lkc`
A :term:`crate import` binds an external :term:`crate` to its
:syntax:`CrateIndication`.

:def_p:`fls_k90qtnf8kgu1`
:syntax:`CrateIndication` ``self`` shall require a :term:`renaming`.

:def_p:`fls_siv8bl6s2ndu`
A :term:`crate import` with a :term:`renaming` binds the external :term:`crate`
to a local :term:`name` and introduces the local :term:`name` into the enclosing
:term:`scope`.

:def_p:`fls_7vz5n3x6jo1s`
If a :term:`crate import` appears at the :term:`crate root module`, then the
:term:`crate indication` is added to the :term:`external prelude`.

:def_p:`fls_3bgpc8m8yk4p`
A :syntax:`CrateIndication` shall resolve to an external :term:`crate`. The
process of resolving a :syntax:`CrateIndication` to an external :term:`crate`
is tool-defined.

:def_p:`fls_bfxkmm9px6k8`
The following :term:`attribute`\ s apply to :term:`crate import`\ s:

* :def_p:`fls_d6eu6xiczpxk`
  :term:`Attribute` :codeterm:`no_link`.

Compilation Roots
-----------------

.. rubric:: Legality Rules

:def_p:`fls_fhiqvgdamq5`
A :term:`crate root module` is the root of the nested :term:`module`
:term:`scope`\ s tree of a :term:`crate`.

:def_p:`fls_tk8tl2e0a34`
A tool can define a :term:`crate root module` for a single :term:`crate`.

:def_p:`fls_bsyfxdk3ap1t`
A :term:`compilation root` is an input to a compilation performed by a tool. A
:term:`crate root module` is a :term:`compilation root`.

Conditional Compilation
-----------------------

.. rubric:: Legality Rules

:def_p:`fls_9stc6nul6vq9`
:term:`Conditionally-compiled source code` is source code that may or may
not be considered a part of a Rust program depending on :term:`configuration
predicate`\ s.

:def_p:`fls_a0u9nnaf6drz`
:term:`Conditional compilation` is the process of compiling
:term:`conditionally-compiled source code`.

:def_p:`fls_pf1v89h7pjhh`
A :term:`construct` subject to :term:`attribute` :codeterm:`cfg` where the
related :term:`configuration predicate` evaluates to ``false`` is not considered
part of a Rust program.

:def_p:`fls_h6b1fuw4nvi1`
An :term:`attribute` :codeterm:`cfg_attr` where the related :term:`configuration
predicate` evaluates to ``false`` is not considered part of a Rust program.

:def_p:`fls_212qrhdifs5e`
An :term:`attribute` :codeterm:`cfg_attr` where the related
:term:`configuration predicate` evaluates to ``true`` is replaced with a
new :term:`attribute` for each :syntax:`AttributeContent` in the attribute's
:syntax:`AttributeContentList`.

