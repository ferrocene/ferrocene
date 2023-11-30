.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Terms, Definitions, and Abbreviations
=====================================

For the purpose of this qualification, the terms, definitions and abbreviated
terms from the ISO 26262 and IEC 61508 standards apply. Additional terms
specific to the current qualification are listed below.

Definition of Terms
-------------------

Borrow Checker
    Uses the control flow graph to verify the ownership and borrowing system of
    the programming language.

Exhaustiveness Analyzer
    Determines whether the alternatives of a match expression fully cover the
    range of a type.

HazOp
    Hazard and Operability study. It is a process for analyzing hazards in a
    systematic way by applying guide words to each node of studied item.

HIR Lowering
    Checks various correctness properties of the AST, and then translates the
    AST to HIR.

Lexer
    A stream of characters that represent the text of a Rust program in order to
    produce lexical elements.

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

THIR Lowering
    Translates the HIR into THIR.

Trait Analyzer
    Pairs an implementation with a trait or a type by satisfying the various
    implicit and explicit constraints imposed by the trait or type.

Type Analyzer
    Infers and verifies the types of expressions and values based on the type
    system.


Abbreviated Terms
-----------------

ASIL
    Automotive Safety Integrity Level

CI
    Continuous Integration

HIR
    High-Level Intermediate Representation

IEC
    International Electrotechnical Commission

ISO
    International Standard Organization

QMS
    Quality Management System

MIR
    Mid-Level Intermediate Representation

PR
    Pull Request

RTOS
    Real-Time Operating System

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
