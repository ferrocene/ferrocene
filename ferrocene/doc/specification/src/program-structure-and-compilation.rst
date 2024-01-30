.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_hdwwrsyunir:

Program Structure and Compilation
=================================

.. _fls_s35hob3i7lr:

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
       $$#!$$ ~[NewLine]*

   NewLine ::=
       $$\n$$

.. rubric:: Legality Rules

:dp:`fls_4vicosdeaqmp`
A :t:`source file` contains the program text consisting of :t:`[inner
attribute]s`, :t:`[inner doc comment]s`, and :t:`[item]s`. The location of a
:t:`source file` is tool defined.

:dp:`fls_ann3cha1xpek`
A :s:`Shebang` does not have an effect on the compilation.

.. _fls_e9hwvqsib5d5:

Modules
-------

.. rubric:: Syntax

.. syntax::

   ModuleDeclaration ::=
       $$unsafe$$? $$mod$$ Name ModuleSpecification

   ModuleSpecification ::=
       InlineModuleSpecification
     | OutlineModuleSpecification

   InlineModuleSpecification ::=
       $${$$
         InnerAttributeOrDoc*
         Item*
       $$}$$

   OutlineModuleSpecification ::=
       $$;$$

.. rubric:: Legality Rules

:dp:`fls_odd1hj3y1mgu`
A :t:`module` is a container for zero or more :t:`[item]s`.

:dp:`fls_whgv72emrm47`
The ``unsafe`` :t:`keyword` of an :t:`module` is rejected, but may still
be consumed by :t:`[macro]s`.

:dp:`fls_qypjjpcf8uwq`
An :t:`inline module` is a :t:`module` with an :s:`InlineModuleSpecification`.

:dp:`fls_cavwpr1ybk37`
An :t:`outline module` is a :t:`module` with an :s:`OutlineModuleSpecification`.

:dp:`fls_plepew2319g4`
An :t:`outline module` loads a :t:`source file` and considers the text of the
:t:`source file` to be inlined within the context of the :t:`outline module`.

:dp:`fls_1aruwps62c4p`
The location of a :t:`module` :t:`source file` can be specified using
:t:`attribute` :c:`path`.

.. rubric:: Examples

.. code-block:: rust

   #[path = "path/to/module"]
   pub mod module {
   	#![allow(dead_code)]

        struct Struct;
        pub mod other;
   }

.. _fls_maw4u1o8q37u:

Crates
------

.. rubric:: Legality Rules

:dp:`fls_qwghk79ok5h0`
A :t:`crate` is a unit of compilation and linking that contains a tree of
nested :t:`[module]s`.

:dp:`fls_9ub6ks8qrang`
A :t:`binary crate` is a :t:`crate` that contains a :t:`main function`. A tool
can compile a :t:`binary crate` to an executable.

:dp:`fls_d9nn4yuiw1ja`
A :t:`library crate` is either a :t:`crate` without a :t:`main function` or a
:t:`crate` subject to :t:`attribute` :c:`no_main`. A tool is free to compile a
:t:`library crate` to a shared library.

:dp:`fls_Mf62VqAhoZ3c`
A :t:`proc-macro crate` is a :t:`crate` that contains :t:`[procedural macro]s`.
A tool is free to compile a :t:`proc-macro crate` to a shared library.

:dp:`fls_cXLyCjs9T3Mj`
A :t:`proc-macro crate` shall not declare :t:`[item]s` with
:t:`public visibility` unless the :t:`item` is a :t:`procedural macro`.

:dp:`fls_h93C3wfbAoz1`
Only a :t:`proc-macro crate` shall declare :t:`[procedural macro]s`.

.. _fls_gklst7joeo33:

Crate Imports
-------------

.. rubric:: Syntax

.. syntax::

   ExternalCrateImport ::=
       $$extern$$ $$crate$$ CrateIndication Renaming? $$;$$

   CrateIndication ::=
       Identifier
     | $$self$$

.. rubric:: Legality Rules

:dp:`fls_d0pa807s5d5h`
A :t:`crate import` specifies a required dependency on an external :t:`crate`.

:dp:`fls_vfam3wzeAiah`
A :t:`crate indication` is a :t:`construct` that indicates a :t:`crate`.

:dp:`fls_ft860vkz0lkc`
A :t:`crate import` binds an external :t:`crate` to its :t:`crate indication`.

:dp:`fls_k90qtnf8kgu1`
:t:`Crate indication` ``self`` shall require a :t:`renaming`.

:dp:`fls_siv8bl6s2ndu`
A :t:`crate import` with a :t:`renaming` with an :t:`identifier` binds the
external :t:`crate` to a local :t:`name` and introduces the local :t:`name` into
the enclosing :t:`scope`.

:dp:`fls_7vz5n3x6jo1s`
If a :t:`crate import` appears at the :t:`crate root module`, then the
:t:`crate indication` is added to the :t:`external prelude`.

:dp:`fls_3bgpc8m8yk4p`
A :t:`crate indication` shall resolve to an external :t:`crate`. The process of
resolving a :t:`crate indication` to an external :t:`crate` is tool-defined.

.. _fls_5w50kf83oo1u:

Compilation Roots
-----------------

.. rubric:: Legality Rules

:dp:`fls_fhiqvgdamq5`
A :t:`crate root module` is the root of the nested :t:`module` tree of a
:t:`crate`.

:dp:`fls_tk8tl2e0a34`
A tool can define a :t:`crate root module` for a single :t:`crate`.

:dp:`fls_bsyfxdk3ap1t`
A :t:`compilation root` is an input to a compilation performed by a tool. A
:t:`crate root module` is a :t:`compilation root`.

.. _fls_u1afezy1ye99:

Conditional Compilation
-----------------------

.. rubric:: Legality Rules

:dp:`fls_9stc6nul6vq9`
:t:`Conditionally-compiled source code` is source code that may or may
not be considered a part of a Rust program depending on
:t:`[configuration predicate]s`.

:dp:`fls_a0u9nnaf6drz`
:t:`Conditional compilation` is the process of compiling
:t:`conditionally-compiled source code`.

:dp:`fls_pf1v89h7pjhh`
A :t:`construct` subject to :t:`attribute` :c:`cfg` where the related
:t:`configuration predicate` evaluates to ``false`` is not considered part of a
Rust program.

:dp:`fls_y56RGw3cbFex`
A :t:`crate root module` subject to :t:`attribute` :c:`cfg` where the related
:t:`configuration predicate` evaluates to ``false`` is considered empty except
for all :t:`[attribute]s` up to the invoked :t:`attribute` :c:`cfg`.

:dp:`fls_h6b1fuw4nvi1`
An :t:`attribute` :c:`cfg_attr` where the related :t:`configuration predicate`
evaluates to ``false`` is not considered part of a Rust program.

