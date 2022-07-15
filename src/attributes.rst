.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_gvwd0kf72jt:

Attributes
==========

.. rubric:: Syntax

.. syntax::

   InnerAttributeOrDoc ::=
       InnerAttribute
     | InnerBuiltinAttribute
     | InnerDocBlock
     | InnerLineDoc

   InnerAttribute ::=
       $$#![$$ AttributeContent $$]$$

   OuterAttributeOrDoc ::=
       OuterAttribute
     | OuterBuiltinAttribute
     | OuterDocBlock
     | OuterLineDoc

   OuterAttribute ::=
       $$#[$$ AttributeContent $$]$$

   AttributeContent ::=
       SimplePath AttributeInput?

   AttributeInput ::=
       DelimitedTokenTree
     | $$=$$ Expression

   AttributeContentList ::=
       AttributeContent (, AttributeContent)* ,?

.. rubric:: Legality Rules

:dp:`fls_rnzxj1t0hehl`
An :t:`attribute` is a general, free-form metadatum that is interpreted based on
its :t:`name`, convention, language, and tool.

:dp:`fls_yd0ehw5csaur`
An :t:`inner attribute` is an :t:`attribute` that applies to an enclosing
:t:`item`.

:dp:`fls_8o6vmzbw1b1j`
An :t:`outer attribute` is an :t:`attribute` that applies to a subsequent
:t:`item`.

.. rubric:: Examples

.. code-block:: rust

   #[cfg[target_os = "linux"]
   mod linux_only_module {
       #![allow(unused_variables)]

       let unused = ();
   }

.. _fls_i52cujixq9qs:

Attribute Properties
--------------------

.. rubric:: Legality Rules

:dp:`fls_p4potvq7x532`
An :t:`active attribute` is an :t:`attribute` that is removed from the :t:`item`
it decorates.

:dp:`fls_xk7lb2g02sy7`
An :t:`inert attribute` is an :t:`attribute` that remains with the :t:`item`
it decorates.

:dp:`fls_q8wl7pidx2za`
The following :t:`[attribute]s` are :t:`[active attribute]s`:

* :dp:`fls_jottio69o9e7`
  :t:`[Attribute macro]s`.

* :dp:`fls_gzyx9lfi5pvd`
  :t:`Attribute` :c:`cfg`.

* :dp:`fls_elsfqsiqor1y`
  :t:`Attribute` :c:`cfg_attr`.

:dp:`fls_4xu1rwecd9au`
:t:`Attribute` :c:`test` is an :t:`inert attribute` when compiling for testing
purposes, otherwise it is an :t:`active attribute`.

:dp:`fls_n3737i320qum`
All remaining :t:`[attribute]s` are :t:`[inert attribute]s`.

.. _fls_ahmnqhm8anlb:

Built-in Attributes
-------------------

.. rubric:: Syntax

.. syntax::

   InnerBuiltinAttribute ::=
       $$#![$$ BuiltinAttributeContent $$]$$

   OuterBuiltinAttribute ::=
       $$#[$$ BuiltinAttributeContent $$]$$

   BuiltinAttributeContent ::=
       AutomaticallyDerivedContent
     | CfgAttrContent
     | CfgContent
     | ColdContent
     | CrateNameContent
     | CrateTypeContent
     | DeriveContent
     | DocContent
     | ExportNameContent
     | GlobalAllocatorContent
     | InlineContent
     | IgnoreContent
     | LinkContent
     | LinkNameContent
     | LinkSectionContent
     | MacroExportContent
     | MacroUseContent
     | NoBinutilsContent
     | NoImplicitPreludeContent
     | NoLinkContent
     | NoMainContent
     | NoMangleContent
     | NonExhaustiveContent
     | NoStdContent
     | PanicHandlerContent
     | PathContent
     | ProcMacroAttributeContent
     | ProcMacroContent
     | ProcMacroDeriveContent
     | RecursionLimitContent
     | ReprContent
     | ShouldPanicContent
     | TargetFeatureContent
     | TestContent
     | TrackCallerContent
     | TypeLengthLimitContent
     | UsedContent
     | WindowsSubsystemContent

.. rubric:: Legality Rules

:dp:`fls_92tqo8uas8kd`
A :t:`built-in attribute` is a language-defined :t:`attribute`.

:dp:`fls_bxucstrfcco8`
The following :t:`[built-in attribute]s` are :dt:`[code generation attribute]s`:

* :dp:`fls_wle815gb9ai2`
  :t:`Attribute` :c:`cold`.

* :dp:`fls_tvn08dtuilue`
  :t:`Attribute` :c:`inline`.

* :dp:`fls_q4c023zdsfgn`
  :t:`Attribute` :c:`no_builtins`.

* :dp:`fls_xtu3p0kzwn7b`
  :t:`Attribute` :c:`target_feature`.

* :dp:`fls_gxxbf6eag3et`
  :t:`Attribute` :c:`track_caller`.

:dp:`fls_87o6n9et9jio`
The following :t:`[built-in attribute]s` are :dt:`[conditional compilation
attribute]s`:

* :dp:`fls_ui0i3rpt5v5u`
  :t:`Attribute` :c:`cfg`.

* :dp:`fls_6utorag4adlv`
  :t:`Attribute` :c:`cfg_attr`.

:dp:`fls_d8spdkjzp496`
The following :t:`[built-in attribute]s` are :dt:`[derivation attribute]s`:

* :dp:`fls_vidbcv25dyud`
  :t:`Attribute` :c:`automatically_derived`.

* :dp:`fls_d0298bmlyuu4`
  :t:`Attribute` :c:`derive`.

:dp:`fls_dtb3t5ht5ngf`
The following :t:`[built-in attribute]s` are :dt:`[diagnostics attribute]s`:

* :dp:`fls_c5n4gzgs79vv`
  :t:`Attribute` :c:`allow`.

* :dp:`fls_xheohvupr8kb`
  :t:`Attribute` :c:`deny`.

* :dp:`fls_s5z2q5pl14p4`
  :t:`Attribute` :c:`deprecated`.

* :dp:`fls_5ko0q9jnxv5a`
  :t:`Attribute` :c:`forbid`.

* :dp:`fls_rgjf5ibhurda`
  :t:`Attribute` :c:`must_use`.

* :dp:`fls_29y8icoou1gx`
  :t:`Attribute` :c:`warn`.

:dp:`fls_3fxhz0olhbcy`
The following :t:`[built-in attribute]s` are :dt:`[documentation attribute]s`:

* :dp:`fls_oexj0952o05u`
  :t:`Attribute` :c:`doc`.

:dp:`fls_q579e97n1m8j`
The following :t:`[built-in attribute]s` are :dt:`[foreign function interface
attribute]s`:

* :dp:`fls_sn43rofpq6ld`
  :t:`Attribute` :c:`crate_name`.

* :dp:`fls_56d70gkmin4p`
  :t:`Attribute` :c:`crate_type`.

* :dp:`fls_mgb1xipm0qwo`
  :t:`Attribute` :c:`export_name`.

* :dp:`fls_rmhlssasdtkj`
  :t:`Attribute` :c:`link`.

* :dp:`fls_josaywt6g3rq`
  :t:`Attribute` :c:`link_name`.

* :dp:`fls_qk4vkn42c2jh`
  :t:`Attribute` :c:`link_section`.

* :dp:`fls_f21azsygoovw`
  :t:`Attribute` :c:`no_link`.

* :dp:`fls_4d31lwzblg91`
  :t:`Attribute` :c:`no_main`.

* :dp:`fls_muucfla1s8yn`
  :t:`Attribute` :c:`no_mangle`.

* :dp:`fls_wbdtpntjr95w`
  :t:`Attribute` :c:`repr`.

* :dp:`fls_lglwcbsvi9yj`
  :t:`Attribute` :c:`used`.

:dp:`fls_1gyg8hfb13n7`
The following :t:`[built-in attribute]s` are :dt:`[limits attribute]s`:

* :dp:`fls_6005g57evfbp`
  :t:`Attribute` :c:`recursion_limit`.

* :dp:`fls_3y4o8kq58dt8`
  :t:`Attribute` :c:`type_length_limit`.

:dp:`fls_vsix3pqf519x`
The following :t:`[built-in attribute]s` are :dt:`[macro attribute]s`:

* :dp:`fls_c8uqw8p0qrh5`
  :t:`Attribute` :c:`macro_export`.

* :dp:`fls_b3jobjxmqppy`
  :t:`Attribute` :c:`macro_use`.

* :dp:`fls_xyhoxm30i7wn`
  :t:`Attribute` :c:`proc_macro`.

* :dp:`fls_nowfw1ffhupd`
  :t:`Attribute` :c:`proc_macro_attribute`.

* :dp:`fls_5i27houut1mu`
  :t:`Attribute` :c:`proc_macro_derive`.

:dp:`fls_1v9p4vr1nszn`
The following :t:`[built-in attribute]s` are :dt:`[modules attribute]s`:

* :dp:`fls_jvkgtnulrqgh`
  :t:`Attribute` :c:`path`.

:dp:`fls_k9p2xrs3dotn`
The following :t:`[built-in attribute]s` are :dt:`[prelude attribute]s`:

* :dp:`fls_73n30xdcx8e`
  :t:`Attribute` :c:`no_implicit_prelude`.

* :dp:`fls_e7zusnfka5dt`
  :t:`Attribute` :c:`no_std`.

:dp:`fls_85ul6x76ew9`
The following :t:`[built-in attribute]s` are :dt:`[runtime attribute]s`:

* :dp:`fls_xkhm1sht2ju5`
  :t:`Attribute` :c:`global_allocator`.

* :dp:`fls_w9za4moh6gb3`
  :t:`Attribute` :c:`panic_handler`.

* :dp:`fls_3vubhygy9jje`
  :t:`Attribute` :c:`windows_subsystem`.

:dp:`fls_mhaplbf40j02`
The following :t:`[built-in attribute]s` are :dt:`[testing attribute]s`:

* :dp:`fls_23huzf3c4arx`
  :t:`Attribute` :c:`ignore`.

* :dp:`fls_i63y9xnnwq2z`
  :t:`Attribute` :c:`should_panic`.

* :dp:`fls_yic8ksed28no`
  :t:`Attribute` :c:`test`.

:dp:`fls_p1ugiol1e5v5`
The following :t:`[built-in attribute]s` are :dt:`[type attribute]s`:

* :dp:`fls_7xh2iphiteam`
  :t:`Attribute` :c:`non_exhaustive`.

.. _fls_h4k49eadninz:

Code Generation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. _fls_kpwbpp5hc00s:

Attribute ``cold``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ColdContent ::=
       $$cold$$

.. rubric:: Legality Rules

:dp:`fls_x860jl4103p`
:t:`Attribute` :c:`cold` shall apply to :t:`[function]s`.

:dp:`fls_8zdexi5lgm2f`
:t:`Attribute` :dc:`cold` indicates that its related :t:`function` is unlikely
to be called.

.. rubric:: Examples

.. code-block:: rust

   #[cold]
   fn rarely_called_function () {}

.. _fls_ypio6boj3pwf:

Attribute ``inline``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   InlineContent ::=
       $$inline$$ InlineHint?

   InlineHint ::=
       $$($$ ($$always$$ | $$never$$) $$)$$

.. rubric:: Legality Rules

:dp:`fls_jwyhky49ssup`
:t:`Attribute` :c:`inline` shall apply to :t:`[function]s`.

:dp:`fls_s7bf7tf9206d`
:t:`Attribute` :dc:`inline` marks its related :t:`function` as :dt:`inlined`.
The process of replacing a :t:`call expression` to an :t:`inlined` :t:`function`
with the :t:`function body` is referred to as :dt:`inlining`.

:dp:`fls_930o6urn669w`
:t:`Attribute` :c:`inline` without an :s:`InlineHint` suggests to a tool that
:t:`inlining` should be performed.

:dp:`fls_z7ufiqqujgdh`
:t:`Attribute` :c:`inline` with :s:`InlineHint` ``always`` suggests to a tool
that :t:`inlining` should always be performed.

:dp:`fls_f0n4g5uky9tp`
:t:`Attribute` :c:`inline` with :s:`InlineHint` ``never`` suggests to a tool
that :t:`inlining` should never be performed.

:dp:`fls_r3p4din7rjz8`
A tool is not obliged to perform :t:`inlining`.

.. rubric:: Examples

.. code-block:: rust

   #[inline]
   fn suggests_inlining() {}

   #[inline(always)]
   fn requests_consistent_inlining() {}

   #[inline(never)]
   fn requests_suppressed_inlining() {}

.. _fls_zakwockktml8:

Attribute ``no_builtins``
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoBinutilsContent ::=
       $$no_builtins$$

.. rubric:: Legality Rules

:dp:`fls_x36c6j1ivbvp`
:t:`Attribute` :c:`no_builtins` shall apply to :t:`[crate]s`.

:dp:`fls_k2k10qtn6f0g`
:t:`Attribute` :dc:`no_builtins` prevents the tool from replacing certain code
patterns with calls to intrinsic functions.

.. rubric:: Examples

.. syntax::

   $$#![no_builtins]$$

.. _fls_spdmit5fy7el:

Attribute ``target_feature``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TargetFeatureContent ::=
       $$target_feature$$ $$($$ $$enable$$ $$=$$ $$"$$ FeatureList $$"$$ $$)$$

   FeatureList ::=
       Feature (, Feature)*

   Feature ::=
       $$adx$$
     | $$aes$$
     | $$avx$$
     | $$avx2$$
     | $$bmi1$$
     | $$bmi2$$
     | $$fma$$
     | $$fxsr$$
     | $$lzcnt$$
     | $$pclmulqdq$$
     | $$popcnt$$
     | $$rdrand$$
     | $$rdseed$$
     | $$sha$$
     | $$sse$$
     | $$sse2$$
     | $$sse3$$
     | $$sse4.1$$
     | $$sse4.2$$
     | $$ssse3$$
     | $$xsave$$
     | $$xsavec$$
     | $$xsaveopt$$
     | $$xsaves$$

.. rubric:: Legality Rules

:dp:`fls_3qj3jvmtxvx6`
:t:`Attribute` :c:`target_feature` shall apply to :t:`[unsafe function]s`.

:dp:`fls_agpkz1v3c281`
:t:`Attribute` :dc:`target_feature` enables target architecture features for its
related :t:`function`.

:dp:`fls_91b7nd6qslsb`
The target architecture features are as follows:

.. list-table::

   * - :dp:`fls_yz4itbk700ot`
     - **Feature**
     - **implicitly enables**
     - **Description**
   * - :dp:`fls_pdyotoq8uqi2`
     - adx
     -
     - Intel Multi-Precision Add-Cary Instruction Extensions
   * - :dp:`fls_vdbjoy6gbk7l`
     - aes
     - sse2
     - Advanced Encryption Standard
   * - :dp:`fls_k3szii6nviza`
     - avx
     - sse4.2
     - Advanced Vector Extensions
   * - :dp:`fls_xsdkkfgv0pz6`
     - avx2
     - avx
     - Advanced Vector Extensions 2
   * - :dp:`fls_eglaup5zyfsk`
     - bmi1
     -
     - Bit Manipulation Instruction Sets
   * - :dp:`fls_tkayqh7li81o`
     - bmi2
     -
     - Bit Manipulation Instruction Sets 2
   * - :dp:`fls_vv8o31s0zmli`
     - fma
     - avx
     - Three-operand fused multiply-add
   * - :dp:`fls_g0762ekvl2ah`
     - fxsr
     -
     - Save and restore x87 FPU, MMX technology, and SSE state
   * - :dp:`fls_406ltkru11tk`
     - lzcnt
     -
     - Leading zeros count
   * - :dp:`fls_4mhyvwj8pbpk`
     - pclmulqdq
     - sse2
     - Packed carry-less multiplication quadword
   * - :dp:`fls_ijsmqk7pgyiz`
     - popcnt
     -
     - Count of bits set to 1
   * - :dp:`fls_a1jggfetycxh`
     - rdrand
     -
     - Read random number
   * - :dp:`fls_rmbadmoeaoxu`
     - rdseed
     -
     - Read random seed
   * - :dp:`fls_f9ydn049isbv`
     - sha
     - sse2
     - Secure Hash Algorithm
   * - :dp:`fls_7c31e21g6bdj`
     - sse
     -
     - Streaming SIMD Extensions
   * - :dp:`fls_6d5a375j2775`
     - sse2
     - sse
     - Streaming SIMD Extensions 2
   * - :dp:`fls_xthidh2nyrno`
     - sse3
     - sse2
     - Streaming SIMD Extensions 3
   * - :dp:`fls_w02pk6kf9w9e`
     - sse4.1
     - sse3
     - Streaming SIMD Extensions 4.1
   * - :dp:`fls_lzl1gpco3osx`
     - sse4.2.
     - sse4.1
     - Streaming SIMD Extensions 4.2
   * - :dp:`fls_9x2on8w44k4f`
     - ssse3
     - sse3
     - Supplemental Streaming SIMD Extensions 3
   * - :dp:`fls_rilqwazchfpp`
     - xsave
     -
     - Save processor extended status
   * - :dp:`fls_f0of1395z9pn`
     - xsavec
     -
     - Save processor extended status with compaction
   * - :dp:`fls_v9gf6selc17l`
     - xsaveopt
     -
     - Save processor extended states optimized
   * - :dp:`fls_jyb5s2r8w1po`
     - xsaves
     -
     - Save processor extended states supervizor

.. rubric:: Undefined Behavior

:dp:`fls_xx51fjkbgg5g`
It is undefined behavior to execute a program compiled with target architecture
features that are not supported.

.. rubric:: Examples

.. code-block:: rust

   #[target_feature(enable="bmi1,sse4.1")]
   fn requires_target_architecture_features () {}

.. _fls_6qj249hphj1s:

Attribute ``track_caller``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TrackCallerContent ::=
       $$track_caller$$

.. rubric:: Legality Rules

:dp:`fls_h8yepgchjxv9`
:t:`Attribute` :c:`track_caller` shall apply to :t:`non-[main function]s` with
:t:`ABI` "Rust".

:dp:`fls_w1pxtzp7acty`
:t:`Attribute` :dc:`track_caller` allows the :t:`function body` of its
related :t:`function` to obtain a :std:`core::panic::Location` which indicates
the topmost untracked caller that ultimately led to the invocation of the
:t:`function`.

:dp:`fls_zch43jpetmdu`
A tool is not required to implement this indication in an effective manner.

:dp:`fls_y1e258p4rby5`
When applied to an :t:`associated trait function`, :t:`attribute`
:c:`track_caller` applies to all :t:`[implementing function]s`. If the
:t:`associated trait function` has a :t:`default implementation`, then the
:t:`attribute` applies to all :t:`[overriding function]s`.

.. rubric:: Undefined Behavior

:dp:`fls_vkz8t751gfhk`
It is undefined behavior when :t:`attribute` :c:`track_caller` applies to an
:t:`external function` but does not apply to all linked :t:`[implementation]s`.

:dp:`fls_ddg0u5lej74x`
It is undefined behavior when :t:`attribute` :c:`track_caller` is applied to
an :t:`exported function` but the :t:`external function` it links to is missing
the :t:`attribute`.

.. rubric:: Examples

.. code-block:: rust

   #[track_caller]
   fn who_called_me () {}

.. _fls_cdx9zb1yxcc8:

Conditional Compilation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. _fls_fymvsy6ig99a:

Attribute ``cfg``
^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CfgContent ::=
       $$cfg$$ $$($$ CondigurationPredicate $$)$$

   ConfigurationPredicate ::=
       ConfigurationOption
     | ConfigurationPredicateAll
     | ConfigurationPredicateAny
     | ConfigurationPredicateNot

   ConfigurationOption ::=
       ConfigurationOptionName ConfigurationOptionValue?

   ConfigurationOptionName ::=
       Identifier

   ConfigurationOptionValue ::=
       $$=$$ StringLiteral

   ConfigurationPredicateAll ::=
       $$all$$ $$($$ ConfigurationPredicateList? $$)$$

   ConfigurationPredicateAny ::=
       $$any$$ $$($$ ConfigurationPredicateList? $$)$$

   ConfigurationPredicateNot ::=
       $$not$$ $$($$ ConfigurationPredicate $$)$$

   ConfigurationPredicateList ::=
       ConfigurationPredicate ($$,$$ ConfigurationPredicate)* $$,$$?

.. rubric:: Legality Rules

:dp:`fls_xrjp7xw9jutz`
:t:`Attribute` :dc:`cfg` enables :t:`conditional compilation`.

:dp:`fls_l96kyix5xsof`
A :t:`configuration predicate` evaluates statically to either ``true`` or
``false``.

:dp:`fls_tncxxsyutppf`
An :t:`all configuration predicate` evaluates statically to ``true`` when either
all nested configuration predicates evaluate to ``true``, or there are no nested
configuration predicates.

:dp:`fls_m0zxktz168e0`
An :t:`any configuration predicate` evaluates statically to ``true`` when any
nested configuration predicate evaluates to ``true``.

:dp:`fls_tvsadfy9uibu`
A :t:`not configuration predicate` evaluates statically to ``true`` when its
nested configuration predicate evaluates to ``false``.

:dp:`fls_jbl9xyynjo0g`
The :t:`evaluation` of a configuration option is tool-defined.

.. rubric:: Examples

.. code-block:: rust

   #[cfg(all(unix, target_pointer_width = "32"))]
   fn on_32bit_unix() {}

.. _fls_dd9xh3wdjudo:

Attribute ``cfg_attr``
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CfgAttrContent ::=
       $$cfg_attr$$ $$($$ CondigurationPredicate $$,$$ AttributeContentList $$)$$

.. rubric:: Legality Rules

:dp:`fls_r66jhict6rlq`
:t:`Attribute` :dc:`cfg_attr` enables :t:`conditional compilation`.

:dp:`fls_rzw12sagm585`
An :t:`attribute` :c:`cfg_attr` where the related :t:`configuration
predicate` evaluates to ``true`` is replaced with a new :t:`attribute`
for each :s:`AttributeContent` enumerated in the :t:`[attribute]'s`
:s:`AttributeContentList`.

.. rubric:: Examples

.. code-block:: rust

   #[cfg_attr(windows, path="windows.rs")]
   mod os;

.. _fls_wednba84zi3y:

Derivation Attributes
~~~~~~~~~~~~~~~~~~~~~

.. _fls_bqw87nz4qbrb:

Attribute ``automatically_derived``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   AutomaticallyDerivedContent ::=
       $$automatically_derived$$

.. rubric:: Legality Rules

:dp:`fls_5u1e0lkt0ab1`
:t:`Attribute` :dc:`automatically_derived` is automatically added to
:t:`[implementation]s` that are created by :t:`attribute` :c:`derive` for
:t:`[built-in trait]s`.

.. _fls_r6gj1p4gajnq:

Attribute ``derive``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   DeriveContent ::=
       $$derive$$ $$($$ SimplePathList? $$)$$

.. rubric:: Legality Rules

:dp:`fls_4btm6zwf445a`
:t:`Attribute` :c:`derive` shall apply to an :t:`abstract data type`.

:dp:`fls_pjmbmj2b35y8`
:t:`Attribute` :dc:`derive` lists :t:`[derive macro]s` for automatic
implementation by a tool.

.. rubric:: Examples

.. code-block:: rust

   #[derive(PartialEq)]
   struct S<T> {
       field: T
   }

:dp:`fls_xtty2ino4vwc`
Attribute ``derive`` causes trait :std:`core::cmp::PartialEq` to be
automatically implemented for struct ``S<T>`` as follows:

.. code-block:: rust

   impl<T: core::cmp::PartialEq> core::cmp::PartialEq for S<T> {
       fn eq(&self, other: &S<T>) -> bool {
          self.field == other.field
       }

       fn ne(&self, other: &S<T>) -> bool {
          self.field != other.field
       }
   }

.. _fls_t2590yyvclgb:

Diagnostics Attributes
~~~~~~~~~~~~~~~~~~~~~~

:dp:`fls_ghumzt9ybtit`
Diagnostic :t:`[attribute]s` are related to linting, and are not defined in
this document.

.. _fls_8wcliky2svcs:

Documentation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~

.. _fls_63v1fqedzwfd:

Attribute ``doc``
^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   DocContent ::=
       $$This needs to be discussed$$

   AttributeDocInner ::=
       $$#![doc$$ DocInnerOption? $$]$$

   DocInnerOption ::=
       $$($$ SourceOption | TestOption | UrlOption $$)$$

   SourceOption ::=
       $$html_no_source$$

   TestOption ::=
   $$test($$
   AttrOption
     		| NoCreateInjectOption
   $$)$$

   AttrOption ::=
       $$attr($$ AttributeContentList $$)$$

   NoCreateInjectOption ::=
       $$no_crate_inject$$

   UrlOption ::=
       UrlKind $$=$$ StringLiteral

   UrlKind ::=
       $$html_favicon_url$$
     | $$html_logo_url$$
     | $$html_playground_url$$
     | $$html_root_url$$
     | $$issue_tracker_base_url$$

   AttributeDocOuter ::=
       $$#[doc$$ $$($$ DocOuterOption? $$)$$ $$]$$

   DocOuterOption ::=
       AliasOption
     | $$hidden$$
     | $$inline$$
     | $$no_inline$$

   AliasOption ::=
       $$alias$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_1ee9qjcgbwme`
The :t:`inner attribute` version and the :t:`outer attribute` version of
:t:`attribute` :dc:`doc` associate documentation with a :t:`construct`.

.. rubric:: Examples

:dp:`fls_necp8a7v255c`
**???**

.. _fls_pgp7ezcc9lh8:

Foreign Function Interface Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. _fls_sun645voqex6:

Attribute ``crate_name``
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CrateNameContent ::=
       $$crate_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_tsdk8jyajcg`
:t:`Attribute` :c:`crate_name` shall appear at the :t:`crate level`.

:dp:`fls_6riphqysh0gd`
:t:`Attribute` :dc:`crate_name` shall specify the name of the related
:t:`crate`.

.. rubric:: Examples

.. code-block:: rust

   #![crate_name = "factories"]

.. _fls_ujig607lmwbm:

Attribute ``crate_type``
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CrateTypeContent ::=
       $$crate_type$$ $$=$$ $$"$$ CrateType $$"$$

   CrateType ::=
       $$bin$$
     | $$cdylib$$
     | $$dylib$$
     | $$lib$$
     | $$proc-macro$$
     | $$rlib$$
     | $$staticlib$$

.. rubric:: Legality Rules

:dp:`fls_2i2g55nqqpc1`
:t:`Attribute` :c:`crate_type` shall appear at the :t:`crate level`.

:dp:`fls_1zziddjuzjeq`
:t:`Attribute` :dc:`crate_type` shall specify the linkage :t:`type` of the
:t:`crate` it appears in.

.. rubric:: Examples

.. code-block:: rust

   #![crate_type = "cdylib"]

.. _fls_olzilmy8n0nl:

Attribute ``export_name``
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ExportNameContent ::=
       $$export_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_r3fwpuuaoeie`
:t:`Attribute` :c:`export_name` shall apply to :t:`[function]s` and
:t:`[static]s`.

:dp:`fls_spwr6gf7kpds`
:t:`Attribute` :dc:`export_name` shall specify the exported symbol of the
related :t:`function` or :t:`static`.

.. rubric:: Examples

.. code-block:: rust

   #[export_name = "exported_symbol"]
   pub fn rust_name() {}

.. _fls_o0f9ae22ug1x:

Attribute ``link``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   LinkContent ::=
       $$link$$ $$($$ LinkOption $$)$$

   LinkOption ::=
       NativeLibraryName
     | NativeLibraryNameWithKind
     | WebAssemblyModuleName

   NativeLibraryName ::=
       $$name$$ $$=$$ StringLiteral

   NativeLibraryNameWithKind ::=
       NativeLibraryName , NativeLibrayKind

   WebAssemblyModuleName ::=
       $$wasm_import_module$$ $$=$$ StringLiteral

   NativeLibrayKind ::=
       $$kind$$ $$=$$ " NativeLibrayKindType "

   NativeLibrayKindType ::=
       $$dylib$$
     | $$framework$$
     | $$static$$

.. rubric:: Legality Rules

:dp:`fls_yslpkdngo8hj`
:t:`Attribute` :c:`link` shall apply to :t:`[external block]s`.

:dp:`fls_6rohnk4swj6c`
:t:`Attribute` :dc:`link` shall specify the name of a native library that a tool
should link with.

:dp:`fls_o83pf3bcrzma`
The following native library kinds are available:

.. list-table::

   * - :dp:`fls_5541q1qoxdpf`
     - **Native Library Kind**
     - **Description**
   * - :dp:`fls_wpqawdpevkj6`
     - dylib
     - Dynamic library
   * - :dp:`fls_h9dfs6kzmobp`
     - framework
     - macOS framework
   * - :dp:`fls_3a3r4jf7hzqr`
     - static
     - Static library

:dp:`fls_1tdheukgm6ai`
When :t:`attribute` :c:`link` appears without a native library kind, its native
library kind defaults to ``dylib``. Native library kind framework is only valid
on macOS targets.

:dp:`fls_3i9ijypnh8nx`
If :t:`attribute` :c:`link` appears without a WebAssembly module name, then the
WebAssembly module name defaults to ``env``.

.. rubric:: Examples

.. code-block:: rust

   #[link(name = "CoreFoundation", kind = "framework")]
   extern {}

.. _fls_p44fky7fifc:

Attribute ``link_name``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   LinkNameContent ::=
       $$link_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_g09jhukl0ez2`
:t:`Attribute` :c:`link_name` shall apply to :t:`[external function]s` and
:t:`[external static]s`.

:dp:`fls_d00wni4edi8f`
:t:`Attribute` :dc:`link_name` shall specify the linking symbol of the related
:t:`external function` or :t:`external static`.

.. rubric:: Examples

.. code-block:: rust

   extern {
       #[link_name = "linking_symbol"]
       pub fn rust_name() {}
   }

.. _fls_hffpo88r61rh:

Attribute ``link_section``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   LinkSectionContent ::=
       $$link_section$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_5loqzajiz34m`
:t:`Attribute` :c:`link_section` shall apply to :t:`[function]s` and
:t:`[static]s`.

:dp:`fls_cyxk12wuicml`
:t:`Attribute` :dc:`link_section` specifies the object file section where the
symbol of the related :t:`function` or :t:`static` will be placed.

.. rubric:: Examples

.. code-block:: rust

   #[link_section = ".example_section"]
   pub static THE_ANSWER: u32 = 42;

.. _fls_ch9nkxkloozv:

Attribute ``no_link``
^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoLinkContent ::=
       $$no_link$$

.. rubric:: Legality Rules

:dp:`fls_ayhn6g6sgt3h`
:t:`Attribute` :c:`no_link` shall apply to :t:`[external crate import]s`.

:dp:`fls_76ox8n3eef5`
:t:`Attribute` :dc:`no_link` indicates that the imported :t:`external crate`
will not be linked into the resulting binary or library.

.. rubric:: Examples

.. code-block:: rust

   #[no_link]
   extern crate do_not_link;

.. _fls_fh27ljezn3qz:

Attribute ``no_main``
^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoMainContent ::=
       $$no_main$$

.. rubric:: Legality Rules

:dp:`fls_84a9k0fzmnfk`
:t:`Attribute` :c:`no_main` shall appear at the :t:`crate level`.

:dp:`fls_6qig3s3qpj0i`
:t:`Attribute` :dc:`no_main` indicates that the symbols of the :t:`main
function` will not be present in a binary.

.. rubric:: Examples

.. code-block:: rust

   #![no_main]

.. _fls_mvd7nz8k3wcy:

Attribute ``no_mangle``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoMangleContent ::=
       $$no_mangle$$

.. rubric:: Legality Rules

:dp:`fls_q5swm5meafmx`
:t:`Attribute` :c:`no_mangle` shall apply to an :t:`entity`.

:dp:`fls_esaew4fqk8mm`
:t:`Attribute` :dc:`no_mangle` indicates that the :t:`name` of the related
:t:`entity` will be used as the symbol for that :t:`entity`.

:dp:`fls_lvnclpxbye9u`
:t:`Attribute` :c:`no_mangle` causes the related :t:`entity` to be publicly
exported from the produced library or object file.

.. rubric:: Examples

.. code-block:: rust

   #[no_mangle]
   pub fn symbol_name() {}

.. _fls_aibb2quva4mn:

Attribute ``repr``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ReprContent ::=
       $$repr$$ $$($$ Representation $$)$$

   Representation ::=
       RepresentationKind Alignment?

   RepresentationKind ::=
       PrimitiveRepresentation
     | $$C$$
     | $$transparent$$
   Alignment ::=
       AlignmentDecrease
     | AlignmentIncrease
   PrimitiveRepresentation ::=
       $$i8$$
     | $$i16$$
     | $$i32$$
     | $$i64$$
     | $$i128$$
     | $$isize$$
     | $$u8$$
     | $$u16$$
     | $$u32$$
     | $$u64$$
     | $$u128$$
     | $$usize$$
   AlignmentDecrease ::=
       $$packed$$ $$($$ DecimalLiteral $$)$$
   AlignmentIncrease ::=
       $$align$$ $$($$ DecimalLiteral $$)$$

.. rubric:: Legality Rules

:dp:`fls_vetjq9sw84qc`
:t:`Attribute` :c:`repr` shall apply to an :t:`abstract data type`.

:dp:`fls_is2esjz1sy36`
:t:`Attribute` :dc:`repr` shall indicate the :t:`type representation` of the
related :t:`type`.

.. rubric:: Examples

.. code-block:: rust

   #[repr(C, align(8))]
   struct c_struct {
       first_field: i16,
       second_field: i8
   }

.. _fls_7skf24auayqy:

Attribute ``used``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   UsedContent ::=
       $$used$$

.. rubric:: Legality Rules

:dp:`fls_s4ii078wgpk`
:t:`Attribute` :c:`used` shall apply to :t:`[static]s`.

:dp:`fls_k293nzcffks4`
:t:`Attribute` :dc:`used` forces a tool to keep the related :t:`static` in the
output object file even if the :t:`static` is not used or referenced by other
:t:`[item]s` in the :t:`crate`.

.. rubric:: Examples

.. code-block:: rust

   #[used]
   pub static THE_ANSWER: u32 = 42;

.. _fls_cjq792yj6vft:

Limits Attributes
~~~~~~~~~~~~~~~~~

.. _fls_u2hzlzpzh7yy:

Attribute ``recursion_limit``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   RecursionLimitContent ::=
       $$recursion_limit$$ $$=$$ $$"$$ DecimalLiteral $$"$$

.. rubric:: Legality Rules

:dp:`fls_o55cxc67sya7`
:t:`Attribute` :c:`recursion_limit` shall appear at the :t:`crate level`.

:dp:`fls_o9p8fa8zhe15`
:t:`Attribute` :dc:`recursion_limit` sets the maximum depth of :t:`macro
expansion` and :t:`auto-dereferencing`.

.. rubric:: Examples

.. code-block:: rust

   #![recursion_limit = "42"]

.. _fls_tdjjuwbr7mkg:

Attribute ``type_length_limit``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TypeLengthLimitContent ::=
       $$type_length_limit$$ $$=$$ $$"$$ DecimalLiteral $$"$$

.. rubric:: Legality Rules

:dp:`fls_dfnkzj8ob3uq`
:t:`Attribute` :c:`type_length_limit` shall appear at the :t:`crate level`.

:dp:`fls_61vt1r8g51nh`
:t:`Attribute` :dc:`type_length_limit` sets the maximum number of :t:`[type
substitution]s` when constructing a :t:`concrete type`.

.. rubric:: Examples

.. code-block:: rust

   #![type_length_limit = "42"]

.. _fls_2084b06dr0wz:

Macros Attributes
~~~~~~~~~~~~~~~~~

.. _fls_e0a96eb6ux3y:

Attribute ``macro_export``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   MacroExportContent ::=
       $$macro_export$$

.. rubric:: Legality Rules

:dp:`fls_3ma7zkk6john`
:t:`Attribute` :c:`macro_export` shall apply to :t:`[declarative macro]s`.

:dp:`fls_h26iw5wh4lla`
:t:`Attribute` :dc:`macro_export` changes the :t:`visibility` of the related
:t:`declarative macro` to :t:`public` and brings the name of the :t:`declarative
macro` into :t:`path scope`.

.. rubric:: Examples

.. code-block:: rust

   #[macro_export]
   macro_rules! m {
       () => {};
   }

.. _fls_qxjy0f758x5s:

Attribute ``macro_use``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   MacroUseContent ::=
       $$macro_use$$ ImportedMacroList?

   ImportedMacroList ::=
       $$($$ IdentifierList $$)$$

.. rubric:: Legality Rules

:dp:`fls_uua0nthq9id`
:t:`Attribute` :c:`macro_use` shall apply to :t:`[external crate import]s` and
:t:`[module]s`.

:dp:`fls_oq4kyo5z5tj5`
An :s:`ImportedMacroList` enumerates macros-to-import. A macro-to-import shall
be subject to attribute :c:`macro_export`.

:dp:`fls_skexvtpbjknn`
When applied to an :t:`external crate import`, :t:`attribute` :dc:`macro_use`
imports from the related :t:`crate` either:

* :dp:`fls_v03924dr0u0z`
  The enumerated macros-to-import, or

* :dp:`fls_eha2hoey857x`
  If no macros-to-import have been specified, all :t:`[macro]s` subject to
  :t:`attribute` :c:`macro_export`.

:dp:`fls_p6jlgmn2sg7j`
When applied to a :t:`module`, :t:`attribute` :c:`macro_use` extends the
:t:`scope` of the related :t:`macro`.

.. rubric:: Examples

.. code-block:: rust

   #[macro_use(first_macro, second_macro)]
   extern crate macros;
   #[macro_use]
   mod module {}

.. _fls_qkmkev85o5jf:

Attribute ``proc_macro``
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ProcMacroContent ::=
       $$proc_macro$$

.. rubric:: Legality Rules

:dp:`fls_u48dtmh97g`
:t:`Attribute` :c:`proc_macro` shall apply to public :t:`[function]s` in the
crate root.

:dp:`fls_t4ez0zg1m569`
:t:`Attribute` :dc:`proc_macro` turns the related :t:`function` into a
:t:`function-like macro`.

.. rubric:: Examples

.. code-block:: rust

   #[proc_macro]
   pub fn make_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

.. _fls_ejhlylrcajo:

Attribute ``proc_macro_attribute``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. syntax::

   ProcMacroAttributeContent ::=
       $$proc_macro_attribute$$

.. rubric:: Legality Rules

:dp:`fls_huznzmkuhdky`
:t:`Attribute` :c:`proc_macro_attribute` shall apply to :t:`[function]s`.

:dp:`fls_gc3ly8fsodf1`
:t:`Attribute` :dc:`proc_macro_attribute` turns the related :t:`function` into
an :t:`attribute macro`.

.. rubric:: Examples

.. code-block:: rust

   #[proc_macro_attribute]
   pub fn output_and_return_item
       (attr: TokenStream, item: TokenStream) -> TokenStream
   {
       println!("attr: \"{}\"", attr.to_string());
       println!("item: \"{}\"", item.to_string());
       item
   }

.. _fls_q6qecp6e413:

Attribute ``proc_macro_derive``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ProcMacroDeriveContent ::=
       $$proc_macro_derive$$ $$($$ DeriveName ($$,$$ HelperAttributeList)? $$)$$

   DeriveName ::=
       Name

   HelperAttributeList ::=
       $$attributes$$ $$($$ IdentifierList $$)$$

.. rubric:: Legality Rules

:dp:`fls_l82yswg0k3px`
:t:`Attribute` :c:`proc_macro_derive` shall apply to :t:`[function]s`.

:dp:`fls_ir9i4i2x5gyx`
:t:`Attribute` :dc:`proc_macro_derive` turns the related :t:`function` into a
:t:`derive macro`, where :s:`DeriveName` defines the :t:`name` of the :t:`derive
macro` available to :t:`attribute` :c:`derive`.

.. rubric:: Examples

.. code-block:: rust

   #[proc_macro_derive(Answer, attributes(marker))]
   pub fn derive_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

   #[derive(Answer)]
   struct S {
       #[marker] field: ()
   }

.. _fls_7bb5ua4g06k8:

Modules Attributes
~~~~~~~~~~~~~~~~~~

.. _fls_1zbaajz5prpn:

Attribute ``path``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   PathContent ::=
       $$path$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_lht4pcezmbxw`
:t:`Attribute` :c:`path` shall apply to :t:`[module]s`.

:dp:`fls_qb6anohvc03k`
:t:`Attribute` :dc:`path` specifies the :dt:`module path` of the respective
:t:`module` as a :t:`string literal`.

:dp:`fls_18tcecx4p2wp`
A tool is free to define the format of a :t:`module path`.

.. rubric:: Examples

.. code-block:: rust

   #[path = "path/to/inline_module"]
   mod inline_module {
       #[path = "path/to/outline_module"]
       mod outline_module;
   }

.. _fls_go457hpaf7ov:

Prelude Attributes
~~~~~~~~~~~~~~~~~~

.. _fls_iikmhqsp1r5a:

Attribute ``no_implicit_prelude``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoImplicitPreludeContent ::=
       $$no_implicit_prelude$$

.. rubric:: Legality Rules

:dp:`fls_tki5k5uo74gw`
The :t:`inner attribute` version of :t:`attribute` :c:`no_implicit_prelude`
shall apply at the :t:`crate level` or to :t:`[module]s`.

:dp:`fls_cmrqxc5oax4r`
The :t:`outer attribute` version of :t:`attribute` :c:`no_implicit_prelude`
shall apply to :t:`[module]s`.

:dp:`fls_c7v2hbdb7g2d`
:t:`Attribute` :dc:`no_implicit_prelude` prevents the import of the :t:`extern
prelude`, the :t:`standard library prelude`, and the :t:`tool prelude`.

.. rubric:: Examples

.. code-block:: rust

   #[no_implicit_prelude]
   mod module {}

.. _fls_9xnaxd7qbakp:

Attribute ``no_std``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoStdContent ::=
       $$no_std$$

.. rubric:: Legality Rules

:dp:`fls_qnxihxsvkyf6`
:t:`Attribute` :c:`no_std` shall apply at the :t:`crate level`.

:dp:`fls_kxav9vw59ts4`
:t:`Attribute` :dc:`no_std` has the following effects:

* :dp:`fls_ve1shwjq09pl`
  Prevents the import of the :t:`standard library prelude`.

* :dp:`fls_wgwsn7laoju7`
  Imports the :t:`core prelude`.

* :dp:`fls_lxkd6hdboav4`
  Imports all :t:`[exported macro]s` of the :t:`core crate` into the
  :t:`macro_use prelude`.

.. rubric:: Examples

.. code-block:: rust

   #![no_std]

.. _fls_nbbvukrdngev:

Runtime Attributes
~~~~~~~~~~~~~~~~~~

.. _fls_fs0lcfllamj:

Attribute ``global_allocator``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   GlobalAllocatorContent ::=
       $$global_allocator$$

.. rubric:: Legality Rules

:dp:`fls_5b8aewlgeon8`
:t:`Attribute` :c:`global_allocator` shall apply to :t:`[static]s` whose
:t:`[type]s` implement the :std:`core::alloc::GlobalAlloc` :t:`trait`.

:dp:`fls_homoidh8mu1r`
:t:`Attribute` :dc:`global_allocator` sets the global allocator to the related
:t:`static`.

.. rubric:: Examples

.. code-block:: rust

   #[global_allocator]
   pub static THE_ANSWER: u32 = 42;

.. _fls_ls5eryuoxlp9:

Attribute ``panic_handler``
^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   PanicHandlerContent ::=
       $$panic_handler$$

.. rubric:: Legality Rules

:dp:`fls_ryz8qy1wdnma`
:t:`Attribute` :c:`panic_handler` shall apply to :t:`[function]s` with
:t:`function signature` ``fn(&core::panic::PanicInfo) -> !``.

:dp:`fls_8gqun8lma9wz`
:t:`Attribute` :dc:`panic_handler` indicates that its related :t:`function`
defines the behavior of :t:`[panic]s`.

:dp:`fls_ka66jcu8gir7`
A :t:`crate` graph shall contain exactly one :t:`function` subject to
:t:`attribute` :c:`panic_handler`.

.. rubric:: Examples

.. code-block:: rust

   #[panic_handler]
   fn panic(info: &core::panic::PanicInfo) -> ! {}

.. _fls_1l4mnlfk5rr2:

Attribute ``windows_subsystem``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   WindowsSubsystemContent ::=
       $$windows_subsystem$$ $$=$$ $$"$$ SubsystemKind $$"$$
   SubsystemKind ::=
       $$console$$
     | $$windows$$

.. rubric:: Legality Rules

:dp:`fls_7mzjahvdzpy5`
:t:`Attribute` :c:`windows_subsystem` shall appear at the :t:`crate level` of a
:t:`binary crate`.

:dp:`fls_t3c0t3lcnebk`
:t:`Attribute` :dc:`windows_subsystem` specifies the subsystem on Windows.

:dp:`fls_go7pfkgpjk2t`
If :t:`attribute` :c:`windows_subsystem` is missing, the subsystem of the
related :t:`binary crate` defaults to ``console``.

.. rubric:: Examples

.. code-block:: rust

   #![windows_subsystem = "console"]

.. _fls_riyi0gy48fxw:

Testing Attributes
~~~~~~~~~~~~~~~~~~

.. _fls_x849a4u7h82j:

Attribute ``ignore``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   IgnoreContent ::=
       $$ignore$$ IgnoreReason?

   IgnoreReason ::=
       $$=$$ StringLiteral

.. rubric:: Legality Rules

:dp:`fls_qmdylxse9yhu`
:t:`Attribute` :c:`ignore` shall apply to :t:`[testing function]s`.

:dp:`fls_9m8e59fc1tyh`
:t:`Attribute` :dc:`ignore` prevents the execution of its related :t:`testing
function`.

.. rubric:: Examples

.. code-block:: rust

   #[test]
   #[ignore = "not implemented yet"]
   fn unit_testing_function() {}

.. _fls_aes2d94g12b9:

Attribute ``should_panic``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ShouldPanicContent ::=
       $$should_panic$$ ExpectedPanicMessage?

   ExpectedPanicMessage ::=
       $$($$ $$expected$$ $$=$$ StringLiteral $$)$$

.. rubric:: Legality Rules

:dp:`fls_w7dq8gnzel36`
:t:`Attribute` :c:`should_panic` shall apply to :t:`[testing function]s`.

:dp:`fls_bm5x846zfnb8`
:t:`Attribute` :dc:`should_panic` indicates that for the related :t:`testing
function` to pass, it should :t:`panic`.

:dp:`fls_bcoq5aus8nkr`
If :s:`ExpectedPanicMessage` is specified, then the related :t:`testing
function` passes only when the :t:`panic` message contains the
:s:`ExpectedPanicMessage`.

.. rubric:: Examples

.. code-block:: rust

   #[test]
   #[should_panic(expected = "did not get meaning of life")]
   fn test_meaning_of_life() {
       assert_eq!(meaning_of_life(), 42, "did not get meaning of life");
   }

.. _fls_dv2j1fvvnk1t:

Attribute ``test``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TestContent ::=
       $$test$$

.. rubric:: Legality Rules

:dp:`fls_o2elhg5w1rj9`
:t:`Attribute` :c:`test` shall apply to a :t:`function` that:

* :dp:`fls_ert22u8rvkxt`
  Is not an :t:`async function`, and

* :dp:`fls_c9ckjrq6emdj`
  Is not an :t:`unsafe function`, and

* :dp:`fls_4uykzqpq6svl`
  Lacks :t:`[generic parameter]s`, and

* :dp:`fls_aqzd30s267pt`
  Lacks :t:`[function parameter]s`, and

* :dp:`fls_n3hjhh3d7tyx`
  Its :t:`return type` is the :t:`unit type`.

:dp:`fls_ze6cs75y9aft`
:t:`Attribute` :dc:`test` indicates that the respective :t:`function` is a
:dt:`testing function`.

:dp:`fls_pcs0prrh23y3`
A :t:`testing function` that returns the :t:`unit type` passes when it
terminates and does not :t:`panic`.

:dp:`fls_niky8lbkvej9`
A :t:`testing function` that returns ``core::result::Result<(), E>`` passes when
it returns ``core::result::Result::OK(())``.

:dp:`fls_qfuntdm2g184`
A :t:`testing function` that does not terminate shall pass and fail according to
the tool.

.. rubric:: Examples

.. code-block:: rust

   #[test]
   fn test_the_thing() -> core::result::Result<()> {
       let input = create_input()?;
       do_the_thing(&input)?;
       core::result::Result::Ok(());
   }

.. _fls_r3zwgf9sg1xp:

Type Attributes
~~~~~~~~~~~~~~~

.. _fls_9tmvuqrmk3ug:

Attribute ``non_exhaustive``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NonExhaustiveContent ::=
       $$non_exhaustive$$

.. rubric:: Legality Rules

:dp:`fls_szvrd79cgzsg`
:t:`Attribute` :c:`non_exhaustive` shall apply to :t:`[enum type]s`, :t:`[enum
variant]s`, and :t:`[struct type]s`.

:dp:`fls_1of56vl2ewq0`
:t:`Attribute` :dc:`non_exhaustive` indicates that the related :t:`type` or
:t:`enum variant` may have more :t:`[field]s` or :t:`[enum variant]s` added
in the future. A :t:`type` subject to :t:`attribute` :c:`non_exhaustive` is
referred to as a :dt:`non-exhaustive type`.

:dp:`fls_hkyzdmmdyoin`
A :t:`non-exhaustive type` shall not be constructed outside of its defining
:t:`crate`.

:dp:`fls_7b0fvwrmz0mh`
An :t:`enum variant` subject to :t:`attribute` :c:`non_exhaustive` is referred
to as a :dt:`non-exhaustive variant`.

:dp:`fls_oqfrg9tqgaj8`
A :t:`non-exhaustive variant` shall not be constructed outside of its defining
:t:`crate`.

:dp:`fls_aql3c89840ix`
:t:`Pattern matching` a :t:`non-exhaustive variant` shall require a :t:`struct
pattern` with a :t:`rest pattern` outside its defining :t:`crate`.

:dp:`fls_cez7yxfc376c`
:t:`Pattern matching` a :t:`non-exhaustive variant` does not contribute towards
the exhaustiveness of :t:`[match arm]s`.

.. rubric:: Examples

.. code-block:: rust

   #[non_exhaustive]
   enum enum_with_future_variants {
       Variant
   }

   enum enum_variants_with_future_fields {
       #[non_exhaustive] Send { from: u32, to: u32 },
       #[non_exhaustive] Quit
   }

   #[non_exhaustive]
   struct struct_with_future_fields {
       field: u32
   }

