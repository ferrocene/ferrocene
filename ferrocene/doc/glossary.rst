.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Terms, Definitions, and Abbreviations
=====================================

For the purpose of this qualification, the terms, definitions and abbreviated
terms from the ISO 26262 and IEC 61508 standards apply. Additional terms
specific to the current qualification are listed below.

Definition of Terms
-------------------

.. glossary::

   Borrow Checker
       Uses the control flow graph to verify the ownership and borrowing system of
       the programming language.

   CircleCI
       CI platform used by the Ferrocene repository to run its tests and build
       its releases.

   Exhaustiveness Analyzer
       Determines whether the alternatives of a match expression fully cover the
       range of a type.

   Git
       A version control system that tracks changes in a set of computer files.

   GitHub
       Code hosting and collaboration platform.

   HazOp
       Hazard and Operability study. It is a process for analyzing hazards in a
       systematic way by applying guide words to each node of studied item.

   HIR Lowering
       Checks various correctness properties of the AST, and then translates the
       AST to HIR.

   Known Problem
   KP
       Ferrocene Known Problems contain the description and scope of the unwanted
       behavior, suggest workarounds, detection and mitigation strategies, and
       list the affected releases.

   Lexer
       A stream of characters that represent the text of a Rust program in order to
       produce lexical elements.

   Linker driver

       An external linker or system C compiler used to locate system-specific
       C libraries needed to link executables and shared libraries, before
       deferring control to the actual linker.

   LLVM IR Generation
       Translates the MIR into LLVM IR.

   Macro Expander
       Replaces macro invocations with the implementations of the invoked macros.

   MIR Lowering
       Translates the THIR into MIR.

   Monomorphizer
       Uses the control flow graph to instantiate a generic by copying its template
       and replicating its generic parameters with generic arguments at appropriate
       instantiation sites.

   Name Resolver
       Uses the expanded AST produced by the Macro Expander to determine which
       paths refer to which names, which methods are being called by method call
       expressions, and which fields are selected by field access expressions.

   Optimizer
       Performs inlining, machine-code layout, register allocation, and similar
       optimizations.

   Parser
       Uses the lexical elements produced by the Lexer to construct the AST.

   Pattern Analyzer
       Uses type information to determine whether a single pattern or a tree of
       patterns can fully capture a value.

   SSH Key
       An access credential in the SSH protocol. Its function is similar to that of
       user names and passwords, but the keys are primarily used for automated
       processes.

   SSO
       An authentication scheme that allows a user to login to multiple independent
       services with a single ID.

   THIR Lowering
       Translates the HIR into THIR.

   Trait Analyzer
       Pairs an implementation with a trait or a type by satisfying the various
       implicit and explicit constraints imposed by the trait or type.

   Type Analyzer
       Infers and verifies the types of expressions and values based on the type
       system.

   Upstream
       The Rust project, also referred to as "upstream", is an umbrella project
       that encompasses contributions from individuals and dedicated teams, and is
       sponsored by interested companies through the Rust Foundation.

   LLVM IR
       A portable high-level assembly language that employs a RISC instruction set.

Abbreviated Terms
-----------------

.. glossary::

   ASIL
       Automotive Safety Integrity Level

   AST
       Abstract Syntax Tree

   CI
       Continuous Integraiton

   HIR
       High-Level Intermediate Representation

   IT
       Information Technology

   IEC
       International Electrotechnical Commission

   ISO
       International Standard Organization

   MIR
       Mid-Level Intermediate Representation

   PE
       Product Engineering

   PR
       Pull Request

   QMS
       Quality Management System

   RTOS
       Real-Time Operating System

   SSH
       Secure SHell [protocol]

   TCL
       Tool Confidence Level

   THIR
       Typed High-Level Intermediate Representation

   TD
       Tool error Detection

   TI
       Tool Impact

   TQL
       Tool Qualification Level
