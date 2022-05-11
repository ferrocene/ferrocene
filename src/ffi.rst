FFI
===

:spec:p:`fls_blx1anmfs2go`
The Foreign Function Interface of Rust or :spec:term:`FFI`
employs :spec:ref:`ABI`, specialized attributes, :spec:ref:`extern block`\
s, :spec:ref:`extern function`\ s, linkage, and type :spec:ref:`layout` to
interface with foreign code.

ABI
---

.. rubric:: Syntax

.. spec:syntax::

   AbiSpecification ::=
       $$extern$$ AbiString?

   AbiString ::=
       RawStringLiteral
     | StringLiteral

.. rubric:: Legality Rules

:spec:p:`fls_9yau29magdz6`
An :spec:syntax:`AbiSpecification` shall start with keyword **``extern``**,
followed by an optional ``AbiString``.

:spec:p:`fls_cguaq8jhsrpj`
An :spec:syntax:`AbiString` shall denote either a :spec:ref:`RawStringLiteral`
or a :spec:ref:`StringLiteral`.

:spec:p:`fls_ikwo0a7xt229`
An ``AbiSpecification`` specifies the Abstract Binary Interface
or :spec:term:`ABI` of a :spec:ref:`function` or a :spec:ref:`static`. An
``AbiString`` describes the :spec:term:`ABI kind`.

:spec:p:`fls_mvr0zqkn2asv`
The following ABIs kinds shall be supported:

* :spec:p:`fls_xys43y73pahl`
  ``**extern** "C"`` - The default ABI of C code, referred to
  as :spec:term:`extern C`.

* :spec:p:`fls_9o0jwevkafe6`
  ``**extern** "Rust"`` - The default ABI of a Rust program, referred to
  as :spec:term:`Rust` ABI.

* :spec:p:`fls_lk52vr9efcn`
  ``**extern** "system"`` - The operating system-dependent ABI, referred to
  as :spec:term:`extern system`.

:spec:p:`fls_h3l7gzmtj99w`
A function without an explicit ABI shall have implicit ABI Rust, unless it
appears within an :spec:ref:`extern block`.

:spec:p:`fls_z37ucsjxtlev`
A function with an ABI but without a specified ABI kind shall have implicit ABI
extern C.

.. rubric:: Implementation Permissions

:spec:p:`fls_8mrzh3cz8z5`
An implementation is allowed to specify additional ABIs. These ABIs may include,
but may not be limited to, the following:

* :spec:p:`fls_2ryjdxch52pw`
  ``**extern** "aapcs"`` - The ARM ABI.

* :spec:p:`fls_35r6yxklg08k`
  ``**extern** "cdecl"`` - The x86_32 ABI of C code.

* :spec:p:`fls_s7sby5dhb92t`
  ``**extern** "fastcall"`` - The ``fastcall`` ABI that corresponds to MSVC's
  ``__fastcall`` and GCC and clang's ``__attribute__((fastcall))``.

* :spec:p:`fls_4qfcv3dzcj46`
  ``**extern** "stdcall"`` - The x86_32 ABI of the Win32 API.

* :spec:p:`fls_nz48dff0i4vf`
  ``**extern** "sysv64"`` - The x86_64 non-Windows ABI of C code.

* :spec:p:`fls_bfuhqagdv3dt`
  ``**extern** "vectorcall"`` - The ``vectorcall`` ABI that corresponds to
  MSVC's ``__vectorcall`` and clang's ``__attribute__((vectorcall))``.

* :spec:p:`fls_8yrsmjpr19l`
  ``**extern** "win64"`` - The x86_64 Windows ABI of C code.

.. rubric:: Examples

.. code-block:: text

   extern
   extern "C"

External Blocks
---------------

.. rubric:: Syntax

.. spec:syntax::

   ExternBlock ::=
       $$unsafe$$? $$extern$$ AbiSpecification? $${$$
         InnerAttributeOrDoc*
         ExternItem*
       $$}$$

   ExternItem ::=
       OuterAttributeOrDoc* ExternalItemContent

   ExternItemContent ::=
       ExternItemWithVisibility
     | TerminatedMacroInvocation

   ExternItemWithVisibility ::=
       VisibilityModifier? (
           ExternFunctionDeclaration
         | ExternStaticDeclaration
       )

.. rubric:: Legality Rules

:spec:p:`fls_7hd2nuqd8704`
An :spec:syntax:`ExternBlock` shall start with optional keyword
**``unsafe``**, followed by keyword **``extern``**, followed by an
optional :spec:ref:`AbiSpecification`, followed by the character whose
code point is 0x7B (left curly bracket), followed by a sequence of zero or
more :spec:ref:`InnerAttributeOrDoc`\ s, followed by a sequence of zero or more
``ExternItem``\ s, followed by the character whose code point is 0x7D (right
curly bracket).

:spec:p:`fls_bzidhtyfuf7v`
An :spec:syntax:`ExternItem` shall start with a sequence of zero or
more :spec:ref:`OuterAttributeOrDoc`\ s, followed by an ``ExternItemContent``.

:spec:p:`fls_gae62c1iozu4`
An :spec:syntax:`ExternItemContent` shall denote
either an :spec:ref:`ExternItemWithVisibility` or
a :spec:ref:`TerminatedMacroInvocation`.

:spec:p:`fls_kovku6k5uunm`
An :spec:syntax:`ExternItemWithVisibility` shall start
with an optional :spec:ref:`VisibilityModifier`, followed
by either an :spec:ref:`ExternFunctionDeclaration` or
an :spec:ref:`ExternStaticDeclaration`.

:spec:p:`fls_905wi27vpfm4`
An ``ExternBlock`` describes an extern block. An :spec:term:`extern block`
provides the declarations of foreign functions as unchecked imports.

:spec:p:`fls_563lah7f2y5t`
The **``unsafe``** keyword of an extern block shall be rejected, but may still
be consumed by :spec:ref:`macro`\ s.

:spec:p:`fls_x8ik93qowavi`
:spec:ref:`Extern function`\ s and :spec:ref:`extern static`\ s shall inherit
the :spec:ref:`ABI` of their enclosing extern block.

.. rubric:: Examples

.. code-block:: text

   extern "C" {
       static MAX_LENGTH: size_t;

       fn compress
           (input: *const u8,
            input_length: size_t,
            compressed: *mut u8,
            compressed_length: *mut size_t) -> c_int;
   }

Extern Functions
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. spec:syntax::

   ExternFunctionDeclaration ::=
       $$fn$$ DefiningIdentifier $$($$ ExternFunctionParameterList? $$)$$ ReturnType? $$;$$

   ExternFunctionParameterList ::=
       ExternFunctionParameter ($$,$$ ExternFunctionParameter)* (, VariadicPart)? $$,$$?

   ExternFunctionParameter ::=
       OuterAttributeOrDoc* IdentifierOrUnderscore $$:$$ TypeSpecification

   ExternFunctionVariadicPart ::=
       OuterAttributeOrDoc* (IdentifierOrUnderscore $$:$$)? $$...$$

.. rubric:: Legality Rules

:spec:p:`fls_cohb4ckxhg5i`
An :spec:syntax:`ExternFunctionDeclaration` shall start with keyword
**``fn``**, followed by a :spec:ref:`DefiningIdentifier`, followed by the
character whose code point is 0x28 (left parenthesis), followed by an optional
``ExternFunctionParameterList``, followed by the character whose code point
is 0x29 (right parenthesis), followed by an optional :spec:ref:`ReturnType`,
followed by the character whose code point is 0x3B (semicolon).

:spec:p:`fls_uk168r14vcug`
An :spec:syntax:`ExternFunctionParameterList` shall start with a
``ExternFunctionParameter``, followed by a sequence of zero or more of the
character whose code point is 0x2C (comma) and an ``ExternFunctionParameter``,
followed by a sequence of the character whose code point is 0x2C (comma) and
an ``ExternFunctionVariadicPart``, followed by an optional character whose code
point is 0x2C (comma).

:spec:p:`fls_a8yf4mtfm3ag`
An :spec:syntax:`ExternFunctionParameter` shall start with a sequence of zero or
more :spec:ref:`OuterAttributeOrDoc`\ s, followed by an :spec:ref:`Identifier`,
followed by the character whose code point is 0x3A (colon), followed by
a :spec:ref:`TypeSpecification`.

:spec:p:`fls_8k199gk868gl`
An :spec:syntax:`ExternFunctionVariadicPart` shall start with a sequence of zero
or more :spec:ref:`OuterAttributeOrDoc`\ s, followed by an optional sequence of
an Identifier and the character whose code point is 0x3A (colon), followed by
character sequence 0x2E 0x2E 0x2E (full stop, full stop, full stop).

:spec:p:`fls_8b6ktmx2it1t`
The ``ExternFunctionDeclaration`` declares an extern function.
An :spec:term:`extern function` is an unchecked import of a foreign function.

:spec:p:`fls_4un6qeqh5ti0`
An extern function shall be invoked only from an :spec:ref:`unsafe context`.

.. rubric:: Examples

.. code-block:: text

   extern C fn log(msg: *const c_char, ...);

External Statics
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. spec:syntax::

   ExternStaticDeclaration ::=
       $$static$$ $$mut$$? DefiningIdentifier $$:$$ TypeSpecification $$;$$

.. rubric:: Legality Rules

:spec:p:`fls_jqqrkk1zoy28`
A :spec:syntax:`ExternStaticDeclaration` shall start with keyword
**``static``**, followed by optional keyword **``mut``**, followed by
a :spec:ref:`DefiningIdentifier`, followed by character 0x3A (colon), followed
by a :spec:ref:`TypeSpecification`, followed by character 0x3B (semicolon).

:spec:p:`fls_glp995o3tu89`
A ``ExternStaticDeclaration`` declares an extern static. An :spec:term:`extern
static` is an import of a foreign variable.

:spec:p:`fls_m5oxediesf43`
An extern static shall be referenced only from an :spec:ref:`unsafe context`.

.. rubric:: Dynamic Semantics

:spec:p:`fls_6v7crfbf71do`
An immutable extern static shall be initialized before Rust code is executed.

FFI-related Attributes
----------------------

.. rubric:: Legality Rules

:spec:p:`fls_xbr55njuelp`
The following attributes affect FFI:

* :spec:p:`fls_2yrl6yw3wuvk`
  Attribute :spec:ref:`export_name`.

* :spec:p:`fls_6awpd77e1j9g`
  Attribute :spec:ref:`link`.

* :spec:p:`fls_s2ddyiwvqn0o`
  Attribute :spec:ref:`link_section`

* :spec:p:`fls_f5eq7cxsidjd`
  Attribute :spec:ref:`no_mangle`.

* :spec:p:`fls_q3ew6w3rzk3x`
  Attribute :spec:ref:`used`.

