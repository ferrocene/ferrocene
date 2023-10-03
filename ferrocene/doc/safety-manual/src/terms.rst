.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Terms, Definitions, and Abbreviations
=====================================

For the purpose of this qualification, the terms, definitions and abbreviated
terms from the ISO 26262 and IEC 61508 standards apply. Additional terms
specific to the current qualification are listed below.

Definition of Terms
-------------------

Lexer
    Processes a stream of characters that represent the text of a Rust program
    in order to produce lexical elements.

Parser
    Uses the lexical elements produced by the Lexer to construct the AST.

Macro Expander
    Replaces macro invocations with the implementations of the invoked macros.

Name Resolver
    Uses the expanded AST produced by the Macro Expander to determine which
    paths refer to which names, which methods are being called by method call
    expressions, and which fields are selected by field access expressions.

HazOp
    Hazard and Operability study. It is a process for analyzing hazards in a
    systematic way by applying guide words to each node of studied item.

HIR Lowering
    Checks various correctness properties of the AST, and then translates the
    AST to HIR.

Trait Analyzer
    Pairs an implementation with a trait or a type by satisfying the various
    implicit and explicit constraints imposed by the trait or type.

Type Analyzer
    Infers and verifies the types of expressions and values based on the type
    system.

THIR Lowering
    Translates the HIR into THIR.

Exhaustiveness Analyzer
    Determines whether the alternatives of a match expression fully cover the
    range of a type.

Pattern Analyzer
    Uses type information to determine whether a single pattern or a tree of
    patterns can fully capture a value. 

MIR Lowering
    Translates the THIR into MIR.

Borrow Checker
    Uses the control flow graph to verify the ownership and borrowing system of
    the programming language.

Monomorphizer
    Uses the control flow graph to instantiate a generic by copying its template
    and replicating its generic parameters with generic arguments at appropriate
    instantiation sites.

Optimizer
    Performs inlining, machine-code layout, register allocation, and similar
    optimizations.

LLVM IR Generation
    Translates the MIR into LLVM IR.



Abbreviated Terms
-----------------

ASIL
    Automotive Safety Integrity Level

AST
    Abstract Syntax Tree

CI
    Continuous Integration

CLI
    Command-Line Interface

HIR
    High-Level Intermediate Representation    

IEC
    International Electrotechnical Commission

ISO
    International Standard Organization

KP(s)
    Known Problem(s)

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
