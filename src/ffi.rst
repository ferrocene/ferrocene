.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_osd6c4utyjb3:

FFI
===

.. rubric:: Legality Rules

:dp:`fls_djlglv2eaihl`
:t:`Foreign Function Interface` or :t:`FFI` employs :t:`ABI`, :t:`[attribute]s`,
:t:`external blocks`, :t:`[external function]s`, linkage, and :t:`type`
:t:`layout` to interface a Rust program with foreign code.

:dp:`fls_k1hiwghzxtfa`
The following :t:`[attribute]s` affect :t:`FFI`:

* :dp:`fls_3cgtdk4698hm`
  :t:`Attribute` :c:`export_name`.

* :dp:`fls_shzmgci4f7o5`
  :t:`Attribute` :c:`link`.

* :dp:`fls_m7x5odt4nb23`
  :t:`Attribute` :c:`link_section`

* :dp:`fls_4akfvpq1yg4g`
  :t:`Attribute` :c:`no_mangle`.

* :dp:`fls_9d8v0xeyi0f`
  :t:`Attribute` :c:`used`.

.. _fls_usgd0xlijoxv:

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

:dp:`fls_xangrq3tfze0`
:t:`Application Binary Interface` or :t:`ABI` is a set of conventions that
dictate how data and computation cross language boundaries.

:dp:`fls_2w0xi6rxw3uz`
The :t:`ABI kind` indicates the :t:`ABI` of a :t:`construct`.

:dp:`fls_9zitf1fvvfk8`
The following :t:`[ABI]s` are supported:

* :dp:`fls_x7ct9k82fpgn`
  ``extern "C"`` - The default :t:`ABI` of :t:`C` code, referred to as :dt:`extern
  C ABI`.

* :dp:`fls_a2d8ltpgtvn6`
  ``extern "Rust"`` - The default :t:`ABI` of a Rust program, referred to as
  :dt:`Rust ABI`.

* :dp:`fls_8m7pc3riokst`
  ``extern "system"`` - The operating system-dependent :t:`ABI`, referred to as
  :dt:`external system ABI`.

:dp:`fls_r2drzo3dixe4`
A :t:`function` without an explicit :t:`ABI` has implicit :t:`Rust ABI`, unless
it appears within an :t:`external block`.

:dp:`fls_z2kzyin8dyr7`
A :t:`function` with an :t:`ABI` but without a specified :t:`ABI kind` has
implicit :t:`external C ABI`.

.. rubric:: Implementation Permissions

:dp:`fls_j6pqchx27ast`
A tool is allowed to specify additional :t:`[ABI]s`. These :t:`[ABI]s` may
include, but may not be limited to, the following:

* :dp:`fls_dbbfqaqa80r8`
  ``extern "aapcs"`` - The ARM :t:`ABI`.

* :dp:`fls_36qrs2fxxvi7`
  ``extern "cdecl"`` - The x86_32 :t:`ABI` of :t:`C` code.

* :dp:`fls_6rtj6rwqxojh`
  ``extern "fastcall"`` - The ``fastcall`` :t:`ABI` that corresponds to MSVC's
  ``__fastcall`` and GCC and clang's ``__attribute__((fastcall))``.

* :dp:`fls_d3nmpc5mtg27`
  ``extern "stdcall"`` - The x86_32 :t:`ABI` of the Win32 API.

* :dp:`fls_7t7yxh94wnbl`
  ``extern "sysv64"`` - The x86_64 non-Windows :t:`ABI` of :t:`C` code.

* :dp:`fls_sxj4vy39sj4g`
  ``extern "vectorcall"`` - The ``vectorcall`` :t:`ABI` that corresponds to
  MSVC's ``__vectorcall`` and clang's ``__attribute__((vectorcall))``.

* :dp:`fls_tyjs1x4j8ovp`
  ``extern "win64"`` - The x86_64 Windows :t:`ABI` of :t:`C` code.

.. rubric:: Examples

.. code-block:: rust

   extern
   extern "C"

.. _fls_tmoh3y9oyqsy:

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

:dp:`fls_4dje9t5y2dia`
An :t:`external block` is a :t:`construct` that provides the declarations of
foreign :t:`[function]s` as unchecked imports.

:dp:`fls_iaimuqcclstl`
The ``unsafe`` :t:`keyword` of an :t:`external block` is rejected, but may still
be consumed by :t:`[macro]s`.

.. rubric:: Examples

.. code-block:: rust

   extern "C" {
       static MAX_LENGTH: size_t;

       fn compress
           (input: *const u8,
            input_length: size_t,
            compressed: *mut u8,
            compressed_length: *mut size_t) -> c_int;
       fn log(msg: *const c_char, ...);
   }

.. _fls_yztwtek0y34v:

External Functions
------------------

.. rubric:: Legality Rules

:dp:`fls_w00qi1gx204e`
:t:`[External function]s` and :t:`[external static]s` inherit the :t:`ABI` of
their enclosing :t:`external block`.

:dp:`fls_v24ino4hix3m`
An :t:`external function` is an unchecked import of a foreign :t:`function`.

:dp:`fls_l88r9fj82650`
An :t:`external function` shall be invoked from an :t:`unsafe context`.

:dp:`fls_qwchgvvnp0qe`
An :t:`external function` shall not specify a :s:`FunctionQualifierList`.

:dp:`fls_m7tu4w4lk8v`
An :t:`external function` shall not specify a :s:`GenericParameterList`
containing :t:`[constant parameter]s` or :t:`[type parameter]s`.

:dp:`fls_rdu4723vp0oo`
An :t:`external function` shall not specify a :s:`FunctionBody`.

:dp:`fls_9div9yusw64h`
An :t:`external function` shall not specify patterns other than identifier
patterns and underscore patterns.

:dp:`fls_juob30rst11r`
Only the last parameter :s:`FunctionParameter` of an :t:`external function` may
specify a :s:`FunctionParameterVariadicPart`.

.. _fls_s4yt19sptl7d:

External Statics
----------------

.. rubric:: Legality Rules

:dp:`fls_8ddsytjr4il6`
An :t:`external static` is an import of a foreign :t:`variable`.

:dp:`fls_fo9with6xumo`
An :t:`external static` shall be referenced from an :t:`unsafe context`.

:dp:`fls_tr7purzcldn0`
An :t:`external static` shall not specify a :t:`StaticInitializer`.

.. rubric:: Dynamic Semantics

:dp:`fls_en2h09ehj0j3`
An :t:`immutable` :t:`external static` shall be initialized before Rust code
is executed.

