.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_wb86edg02t6a:

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
         | ExternalBlock
         | ExternalCrateImport
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

:dp:`fls_s3b1cba9lfj5`
The :t:`macro expansion` of a :t:`terminated macro invocation` is treated as
zero or more :t:`[item]s` if the :t:`terminated macro invocation` appears as
an :t:`item`.

.. rubric:: Dynamic Semantics

:dp:`fls_hil5f7y4xdhe`
:t:`Elaboration` is the process by which a :t:`declaration` achieves its runtime
effects.

