.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

FFI
===

.. rubric:: Legality Rules

:def_p:`fls_djlglv2eaihl`
:term:`Foreign Function Interface` or :term:`FFI` employs :term:`ABI`,
:term:`attribute`\ s, :term:`external blocks`, :term:`external function`\
s, linkage, and :term:`type` :term:`layout` to interface a Rust program with
foreign code.

:def_p:`fls_k1hiwghzxtfa`
The following :term:`attribute`\ s affect :term:`FFI`:

* :def_p:`fls_3cgtdk4698hm`
  :term:`Attribute` :codeterm:`export_name`.

* :def_p:`fls_shzmgci4f7o5`
  :term:`Attribute` :codeterm:`link`.

* :def_p:`fls_m7x5odt4nb23`
  :term:`Attribute` :codeterm:`link_section`

* :def_p:`fls_4akfvpq1yg4g`
  :term:`Attribute` :codeterm:`no_mangle`.

* :def_p:`fls_9d8v0xeyi0f`
  :term:`Attribute` :codeterm:`used`.

ABI
---

.. rubric:: Syntax

.. syntax::

   AbiSpecification ::=
       $$extern$$ AbiKind?

   AbiKind ::=
       RawStringLiteral
     | StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_xangrq3tfze0`
:term:`Application Binary Interface` or :term:`ABI` is a set of conventions that
dictate how data and computation cross language boundaries.

:def_p:`fls_2w0xi6rxw3uz`
The :term:`ABI kind` indicates the :term:`ABI` of a :term:`construct`.

:def_p:`fls_9zitf1fvvfk8`
The following :term:`ABI`\ s are supported:

* :def_p:`fls_x7ct9k82fpgn`
  ``extern "C"`` - The default :term:`ABI` of C code, referred to as
  :def_term:`extern C ABI`.

* :def_p:`fls_a2d8ltpgtvn6`
  ``extern "Rust"`` - The default :term:`ABI` of a Rust program, referred to as
  :def_term:`Rust ABI`.

* :def_p:`fls_8m7pc3riokst`
  ``extern "system"`` - The operating system-dependent :term:`ABI`, referred to
  as :def_term:`external system ABI`.

:def_p:`fls_r2drzo3dixe4`
A :term:`function` without an explicit :term:`ABI` has implicit :term:`Rust
ABI`, unless it appears within an :term:`external block`.

:def_p:`fls_z2kzyin8dyr7`
A :term:`function` with an :term:`ABI` but without a specified :term:`ABI kind`
has implicit :term:`external C ABI`.

.. rubric:: Implementation Permissions

:def_p:`fls_j6pqchx27ast`
A tool is allowed to specify additional :term:`ABI`\ s. These :term:`ABI`\ s may
include, but may not be limited to, the following:

* :def_p:`fls_dbbfqaqa80r8`
  ``extern "aapcs"`` - The ARM :term:`ABI`.

* :def_p:`fls_36qrs2fxxvi7`
  ``extern "cdecl"`` - The x86_32 :term:`ABI` of C code.

* :def_p:`fls_6rtj6rwqxojh`
  ``extern "fastcall"`` - The ``fastcall`` :term:`ABI` that corresponds to
  MSVC's ``__fastcall`` and GCC and clang's ``__attribute__((fastcall))``.

* :def_p:`fls_d3nmpc5mtg27`
  ``extern "stdcall"`` - The x86_32 :term:`ABI` of the Win32 API.

* :def_p:`fls_7t7yxh94wnbl`
  ``extern "sysv64"`` - The x86_64 non-Windows :term:`ABI` of C code.

* :def_p:`fls_sxj4vy39sj4g`
  ``extern "vectorcall"`` - The ``vectorcall`` :term:`ABI` that corresponds to
  MSVC's ``__vectorcall`` and clang's ``__attribute__((vectorcall))``.

* :def_p:`fls_tyjs1x4j8ovp`
  ``extern "win64"`` - The x86_64 Windows :term:`ABI` of C code.

.. rubric:: Examples

.. code-block:: text

   extern
   extern "C"

External Blocks
---------------

.. rubric:: Syntax

.. syntax::

   ExternalBlock ::=
       $$unsafe$$? $$extern$$ AbiSpecification? $${$$
         InnerAttributeOrDoc*
         ExternItem*
       $$}$$

   ExternItem ::=
       OuterAttributeOrDoc* (ExternalItemWithVisibility | TerminatedMacroInvocation)

   ExternalItemWithVisibility ::=
       VisibilityModifier? (
           FunctionDeclaration
         | StaticDeclaration
       )

.. rubric:: Legality Rules

:def_p:`fls_4dje9t5y2dia`
An :term:`external block` is a :term:`construct` that provides the declarations
of foreign :term:`function`\ s as unchecked imports.

:def_p:`fls_iaimuqcclstl`
The ``unsafe`` :term:`keyword` of an :term:`external block` is rejected, but may
still be consumed by :term:`macro`\ s.

.. rubric:: Examples

.. code-block:: text

   extern "C" {
       static MAX_LENGTH: size_t;

       fn compress
           (input: *const u8,
            input_length: size_t,
            compressed: *mut u8,
            compressed_length: *mut size_t) -> c_int;
       fn log(msg: *const c_char, ...);
   }

External Functions
------------------

.. rubric:: Legality Rules

:term:`External function`\ s and :term:`external static`\ s inherit the
:term:`ABI` of their enclosing :term:`external block`.

An :term:`external function` is an unchecked import of a foreign
:term:`function`.

An :term:`external function` shall be invoked from an :term:`unsafe context`.

An :term:`external function` shall not specify a
:syntax:`FunctionQualifierList`.

An :term:`external function` shall not specify a :syntax:`GenericParameterList`
containing :term:`constant parameter`\ s or :term:`type parameter`\ s.

An :term:`external function` shall not specify a :syntax:`FunctionBody`.

An :term:`external function` shall not specify patterns other than identifier
patterns and underscore patterns.

Only the last parameter :syntax:`FunctionParameter` of an :term:`external
function` may specify a :syntax:`FunctionParameterVariadicPart`.

External Statics
----------------

.. rubric:: Legality Rules

An :term:`external static` is an import of a foreign :term:`variable`.

An :term:`external static` shall be referenced from an :term:`unsafe context`.

An :term:`external static` shall not specify a :term:`StaticInitializer`.

.. rubric:: Dynamic Semantics

An :term:`immutable` :term:`external static` shall be initialized before Rust
code is executed.

