.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

=========================
ISO 26262 6-Method Tables
=========================

Table 1	Topics to be covered by modelling and coding guidelines
---------------------------------------------------------------

The Rust project has extensive measures (lints and tests) in place to assure quality and consistency of the codebase. The certified core library uses the same implicit standards as are ensured in the upstream codebase, to minimize divergence. Increased divergence from upstream leads to a higher maintenance burden and is a source of potential bugs.

As such, the certified core library does not have a coding standard.

Table 2	Notations for software architectural design
----------------------------------------------------

Not applicable, as there is no architecture

Table 3	Principles for software architectural design
----------------------------------------------------

Not applicable, as there is no architecture

Table 4	Methods for the verification of the software architectural design
-------------------------------------------------------------------------

Not applicable, as there is no architecture

Table 5 Notations for software unit design
-------------------------------------------

.. csv-table::
    :delim: ;

    No.;Notations;ASIL B;ASIL D;Justification;
    1a;Natural language;++;++;Applied, see core/safety-plan/requirements.html#architecture-and-software-design;
    1b;Informal notations ;++;\+;Applied, see core/safety-plan/requirements.html#architecture-and-software-design;
    1c;Semi-formal notations;\+;++;Not applied, functions in scope are of limited functionality, no dependencies to other components etc.;
    1d;Formal notations;\+;\+;No formal methods applied;

Table 6	Design principles for software unit design and implementation
---------------------------------------------------------------------

.. csv-table::
    :delim: ;

    No.;Principle;ASIL B;ASIL D;Justification;Todo
    1a;One entry and one exit point in subprograms and functions;++;++;Applied, multiple returns only for specific error handling cases;
    1b;No dynamic objects or variables, or else online test during their creation;++;++;Applied, no dynamic memory in library functions allocated;
    1c;Initialization of variables ;++;++;Applied, compiler forbids use of uninitialized variable bindings;
    1d;No multiple use of variable names;++;++;Applied, even though shadowing is possible, library functions are very short, no shadowing taking place as per community guidelines;
    1e;Avoid global variables or else justify their usage;\+;++;Applied, core lib is not stateful, no global variables inside;
    1f;Restricted use of pointers;++;++;Applied, for certain low level operations it might be necessary;
    1g;No implicit type conversions;++;++;Applied, rust does not allow implicit type conversions ;
    1h;No hidden data flow or control flow;++;++;Applied, no dependencies outside of the library, no hidden flows;
    1i;No unconditional jumps;++;++;Applied, no unconditional jumps in rust;
    1j;No recursions ;\+;++;Applied, no recursion used in core library;Is this true? Were there checks done? Some evidence for that?

Table 7 Methods for software unit verification
----------------------------------------------

.. csv-table::
    :delim: ;

    No.;Methods;ASIL B;ASIL D;Justification;
    1a;Walk-through;\+;o;Not applied, 1c applied;
    1b;Pair-programming;\+;\+;Not applied, 1c applied;
    1c;Inspection;++;++;Applied, via community process on contribution to upstream;
    1d;Semi-formal verification ;\+;++;Not applied, as there is no semi-formal notations/specification;
    1e;Formal verification;o;\+;Not applied, as there is no formal specification;
    1f;Control flow analysis;\+;++;Applied, symbol review via compiler outpout (subsetting) ;
    1g;Data flow analysis;\+;++;Not applied, no internal data flow to analyse, no global states, all only via the clear interfaces;
    1h;Static code analysis;++;++;Applied, see core/safety-plan/testing-plan.html#rustc-lints;
    1i;Static analyses based on abstract interpretation;\+;\+;Not applied, 1h applied instead;
    1j;Requirements-based test;++;++;Applied, all tests are based on the specification;
    1k;Interface test;++;++;Applied, all tests are also interface tests, as this is the only type of testing possible;
    1l;Fault injection test;\+;++;Applied, tests cases also assume faults etc., but HW faults are out of scope;
    1m;Resource usage evaluation;\+;++;Not applied, integrator responsibility, as only a library;
    1n;Back-to-back comparison test between model and code, if applicable;\+;++;Not applied, as no model based development;


Table 8;Methods for deriving test cases for software unit testing
------------------------------------------------------------------

.. csv-table::
    :delim: ;

    No.;Methods;ASIL B;ASIL D;Justification;
    1a;Analysis of requirements ;++;++;Applied, requirements/specification is the source of the testing;
    1b;Generation and analysis of equivalence classes;++;++;Partially applied, the test cases in coretests are crafted with a lot of care. Ferrous Systems did not do a full review to ensure boundary and extreme values are always tested. But achieving 100% line coverage will ensure all code paths have been executed and no untested code exists due to no test with a specific input.;
    1c;Analysis of boundary values;++;++;Partially applied, the test cases in coretests are crafted with a lot of care. Ferrous Systems did not do a full review to ensure boundary and extreme values are always tested. But achieving 100% line coverage will ensure all code paths have been executed and no untested code exists due to no test with a specific input.;
    1d;Error guessing based on knowledge or experience;\+;\+;Partially applied, the test cases in coretests are crafted with a lot of care, but not all functions include also error guessing;

Table 9 Structural coverage metrics at the software unit level
--------------------------------------------------------------

.. csv-table::
    :delim: ;

    No.;Methods;ASIL B;ASIL D;Justification;
    1a;Statement coverage ;++;\+;Applied, 100% line coverage with explanations for any coverage gaps, see core/safety-plan/testing-plan.html#code-coverage-tests;
    1b;Branch coverage;++;++;Not applied, 1a applied instead;
    1c;MC/DC (Modified Condition/Decision Coverage) ;\+;++;Not applied, 1a applied instead;

Table 10 Methods for verification of software integration
----------------------------------------------------------

Not applicable, as there is no integration, left for integrators of the library.

Table 11 Methods for deriving test cases for software integration testing
--------------------------------------------------------------------------

Not applicable, as there is no integration, left for integrators of the library.

Table 12 Structural coverage at the software architectural level
-----------------------------------------------------------------

Not applicable, as there is no integration or architectural level, left for integrators of the library

Table 13 Test environments for conducting the software testing
--------------------------------------------------------------

Not applicable, as there is no integration, left for integrators of the library.

Table 14 Methods for tests of the embedded software
----------------------------------------------------

Not applicable, as there is no integration, left for integrators of the library.

Table 15 Methods for deriving test cases for the test of the embedded software
-------------------------------------------------------------------------------

Not applicable, as there is no integration, left for integrators of the library.
