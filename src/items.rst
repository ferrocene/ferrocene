.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Items
=====

.. rubric:: Syntax

.. syntax::

   Item ::=
       OuterAttributeOrDoc* (ItemWithVisibility | MacroItem)

   ItemWithVisibility ::=
       VisibilityModifier? (
           ConstantDeclaration
         | EnumDeclaration
         | ExternBlock
         | ExternCrateImport
         | FunctionDeclaration
         | Implementation
         | ModuleDeclaration
         | StaticDeclaration
         | StructDeclaration
         | TraitDeclaration
         | TypeAliasDeclaration
         | UnionDeclaration
         | UseImport
       )

   MacroItem ::=
       MacroRulesDeclaration
     | TerminatedMacroInvocation

.. rubric:: Legality Rules

:def_p:`fls_s3b1cba9lfj5`
The :term:`expansion` of a :term:`terminated macro invocation` is treated as
zero or more :term:`[item]s` if the :term:`terminated macro invocation` appears
as an :term:`item`.

.. rubric:: Dynamic Semantics

:def_p:`fls_hil5f7y4xdhe`
:term:`Elaboration` is the process by which a :term:`declaration` achieves its
runtime effects.

