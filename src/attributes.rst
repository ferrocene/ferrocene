.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

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

:def_p:`fls_rnzxj1t0hehl`
An :term:`attribute` is a general, free-form metadatum that is interpreted based
on its :term:`name`, convention, language, and tool.

:def_p:`fls_yd0ehw5csaur`
An :term:`inner attribute` is an :term:`attribute` that applies to an enclosing
:term:`item`.

:def_p:`fls_8o6vmzbw1b1j`
An :term:`outer attribute` is an :term:`attribute` that applies to a subsequent
:term:`item`.

.. rubric:: Examples

.. code-block:: text

   #[cfg[target_os = "linux"]
   mod linux_only_module {
       #![allow(unused_variables)]

       let unused = ();
   }

Attribute Properties
--------------------

.. rubric:: Legality Rules

:def_p:`fls_p4potvq7x532`
An :term:`active attribute` is an :term:`attribute` that is removed from the
:term:`item` it decorates.

:def_p:`fls_xk7lb2g02sy7`
An :term:`inert attribute` is an :term:`attribute` that remains with the
:term:`item` it decorates.

:def_p:`fls_q8wl7pidx2za`
The following :term:`attribute`\ s are :term:`active attribute`\ s:

* :def_p:`fls_jottio69o9e7`
  :term:`Attribute macro`\ s.

* :def_p:`fls_gzyx9lfi5pvd`
  :term:`Attribute` :codeterm:`cfg`.

* :def_p:`fls_elsfqsiqor1y`
  :term:`Attribute` :codeterm:`cfg_attr`.

:def_p:`fls_4xu1rwecd9au`
:term:`Attribute` :codeterm:`test` is an :term:`inert attribute` when compiling
for testing purposes, otherwise it is an :term:`active attribute`.

:def_p:`fls_n3737i320qum`
All remaining :term:`attribute`\ s are :term:`inert attribute`\ s.

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

:def_p:`fls_92tqo8uas8kd`
A :term:`built-in attribute` is a language-defined :term:`attribute`.

:def_p:`fls_bxucstrfcco8`
The following :term:`built-in attribute`\ s are :def_term:`code generation
attribute`\ s:

* :def_p:`fls_wle815gb9ai2`
  :term:`Attribute` :codeterm:`cold`.

* :def_p:`fls_tvn08dtuilue`
  :term:`Attribute` :codeterm:`inline`.

* :def_p:`fls_q4c023zdsfgn`
  :term:`Attribute` :codeterm:`no_builtins`.

* :def_p:`fls_xtu3p0kzwn7b`
  :term:`Attribute` :codeterm:`target_feature`.

* :def_p:`fls_gxxbf6eag3et`
  :term:`Attribute` :codeterm:`track_caller`.

:def_p:`fls_87o6n9et9jio`
The following :term:`built-in attribute`\ s are :def_term:`conditional
compilation attribute`\ s:

* :def_p:`fls_ui0i3rpt5v5u`
  :term:`Attribute` :codeterm:`cfg`.

* :def_p:`fls_6utorag4adlv`
  :term:`Attribute` :codeterm:`cfg_attr`.

:def_p:`fls_d8spdkjzp496`
The following :term:`built-in attribute`\ s are :def_term:`derivation
attribute`\ s:

* :def_p:`fls_vidbcv25dyud`
  :term:`Attribute` :codeterm:`automatically_derived`.

* :def_p:`fls_d0298bmlyuu4`
  :term:`Attribute` :codeterm:`derive`.

:def_p:`fls_dtb3t5ht5ngf`
The following :term:`built-in attribute`\ s are :def_term:`diagnostics
attribute`\ s:

* :def_p:`fls_c5n4gzgs79vv`
  :term:`Attribute` :codeterm:`allow`.

* :def_p:`fls_xheohvupr8kb`
  :term:`Attribute` :codeterm:`deny`.

* :def_p:`fls_s5z2q5pl14p4`
  :term:`Attribute` :codeterm:`deprecated`.

* :def_p:`fls_5ko0q9jnxv5a`
  :term:`Attribute` :codeterm:`forbid`.

* :def_p:`fls_rgjf5ibhurda`
  :term:`Attribute` :codeterm:`must_use`.

* :def_p:`fls_29y8icoou1gx`
  :term:`Attribute` :codeterm:`warn`.

:def_p:`fls_3fxhz0olhbcy`
The following :term:`built-in attribute`\ s are :def_term:`documentation
attribute`\ s:

* :def_p:`fls_oexj0952o05u`
  :term:`Attribute` :codeterm:`doc`.

:def_p:`fls_q579e97n1m8j`
The following :term:`built-in attribute`\ s are :def_term:`foreign function
interface attribute`\ s:

* :def_p:`fls_sn43rofpq6ld`
  :term:`Attribute` :codeterm:`crate_name`.

* :def_p:`fls_56d70gkmin4p`
  :term:`Attribute` :codeterm:`crate_type`.

* :def_p:`fls_mgb1xipm0qwo`
  :term:`Attribute` :codeterm:`export_name`.

* :def_p:`fls_rmhlssasdtkj`
  :term:`Attribute` :codeterm:`link`.

* :def_p:`fls_josaywt6g3rq`
  :term:`Attribute` :codeterm:`link_name`.

* :def_p:`fls_qk4vkn42c2jh`
  :term:`Attribute` :codeterm:`link_section`.

* :def_p:`fls_f21azsygoovw`
  :term:`Attribute` :codeterm:`no_link`.

* :def_p:`fls_4d31lwzblg91`
  :term:`Attribute` :codeterm:`no_main`.

* :def_p:`fls_muucfla1s8yn`
  :term:`Attribute` :codeterm:`no_mangle`.

* :def_p:`fls_wbdtpntjr95w`
  :term:`Attribute` :codeterm:`repr`.

* :def_p:`fls_lglwcbsvi9yj`
  :term:`Attribute` :codeterm:`used`.

:def_p:`fls_1gyg8hfb13n7`
The following :term:`built-in attribute`\ s are :def_term:`limits attribute`\ s:

* :def_p:`fls_6005g57evfbp`
  :term:`Attribute` :codeterm:`recursion_limit`.

* :def_p:`fls_3y4o8kq58dt8`
  :term:`Attribute` :codeterm:`type_length_limit`.

:def_p:`fls_vsix3pqf519x`
The following :term:`built-in attribute`\ s are :def_term:`macro attribute`\ s:

* :def_p:`fls_c8uqw8p0qrh5`
  :term:`Attribute` :codeterm:`macro_export`.

* :def_p:`fls_b3jobjxmqppy`
  :term:`Attribute` :codeterm:`macro_use`.

* :def_p:`fls_xyhoxm30i7wn`
  :term:`Attribute` :codeterm:`proc_macro`.

* :def_p:`fls_nowfw1ffhupd`
  :term:`Attribute` :codeterm:`proc_macro_attribute`.

* :def_p:`fls_5i27houut1mu`
  :term:`Attribute` :codeterm:`proc_macro_derive`.

:def_p:`fls_1v9p4vr1nszn`
The following :term:`built-in attribute`\ s are :def_term:`modules attribute`\
s:

* :def_p:`fls_jvkgtnulrqgh`
  :term:`Attribute` :codeterm:`path`.

:def_p:`fls_k9p2xrs3dotn`
The following :term:`built-in attribute`\ s are :def_term:`prelude attribute`\
s:

* :def_p:`fls_73n30xdcx8e`
  :term:`Attribute` :codeterm:`no_implicit_prelude`.

* :def_p:`fls_e7zusnfka5dt`
  :term:`Attribute` :codeterm:`no_std`.

:def_p:`fls_85ul6x76ew9`
The following :term:`built-in attribute`\ s are :def_term:`runtime attribute`\
s:

* :def_p:`fls_xkhm1sht2ju5`
  :term:`Attribute` :codeterm:`global_allocator`.

* :def_p:`fls_w9za4moh6gb3`
  :term:`Attribute` :codeterm:`panic_handler`.

* :def_p:`fls_3vubhygy9jje`
  :term:`Attribute` :codeterm:`windows_subsystem`.

:def_p:`fls_mhaplbf40j02`
The following :term:`built-in attribute`\ s are :def_term:`testing attribute`\
s:

* :def_p:`fls_23huzf3c4arx`
  :term:`Attribute` :codeterm:`ignore`.

* :def_p:`fls_i63y9xnnwq2z`
  :term:`Attribute` :codeterm:`should_panic`.

* :def_p:`fls_yic8ksed28no`
  :term:`Attribute` :codeterm:`test`.

:def_p:`fls_p1ugiol1e5v5`
The following :term:`built-in attribute`\ s are :def_term:`type attribute`\ s:

* :def_p:`fls_7xh2iphiteam`
  :term:`Attribute` :codeterm:`non_exhaustive`.

Code Generation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~

Attribute ``cold``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ColdContent ::=
       $$cold$$

.. rubric:: Legality Rules

:def_p:`fls_x860jl4103p`
:term:`Attribute` :codeterm:`cold` shall apply to :term:`function`\ s.

:def_p:`fls_8zdexi5lgm2f`
:term:`Attribute` :def_codeterm:`cold` indicates that its related
:term:`function` is unlikely to be called.

.. rubric:: Examples

.. code-block:: text

   #[cold]
   fn rarely_called_function () {}

Attribute ``inline``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   InlineContent ::=
       $$inline$$ InlineHint?

   InlineHint ::=
       $$($$ ($$always$$ | $$never$$) $$)$$

.. rubric:: Legality Rules

:def_p:`fls_jwyhky49ssup`
:term:`Attribute` :codeterm:`inline` shall apply to :term:`function`\ s.

:def_p:`fls_s7bf7tf9206d`
:term:`Attribute` :def_codeterm:`inline` marks its related :term:`function` as
:def_term:`inlined`. The process of replacing a :term:`call expression` to an
:term:`inlined` :term:`function` with the :term:`function body` is referred to
as :def_term:`inlining`.

:def_p:`fls_930o6urn669w`
:term:`Attribute` :codeterm:`inline` without an :syntax:`InlineHint` suggests to
a tool that :term:`inlining` should be performed.

:def_p:`fls_z7ufiqqujgdh`
:term:`Attribute` :codeterm:`inline` with :syntax:`InlineHint` ``always``
suggests to a tool that :term:`inlining` should always be performed.

:def_p:`fls_f0n4g5uky9tp`
:term:`Attribute` :codeterm:`inline` with :syntax:`InlineHint` ``never``
suggests to a tool that :term:`inlining` should never be performed.

:def_p:`fls_r3p4din7rjz8`
A tool is not obliged to perform :term:`inlining`.

.. rubric:: Examples

.. code-block:: text

   #[inline]
   fn suggests_inlining() {}

   #[inline(always)]
   fn requests_consistent_inlining() {}

   #[inline(never)]
   fn requests_suppressed_inlining() {}

Attribute ``no_builtins``
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoBinutilsContent ::=
       $$no_builtins$$

.. rubric:: Legality Rules

:def_p:`fls_x36c6j1ivbvp`
:term:`Attribute` :codeterm:`no_builtins` shall apply to :term:`crate`\ s.

:def_p:`fls_k2k10qtn6f0g`
:term:`Attribute` :def_codeterm:`no_builtins` prevents the tool from replacing
certain code patterns with calls to intrinsic functions.

.. rubric:: Examples

.. syntax::

   $$#![no_builtins]$$

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

:def_p:`fls_3qj3jvmtxvx6`
:term:`Attribute` :codeterm:`target_feature` shall apply to :term:`unsafe
function`\ s.

:def_p:`fls_agpkz1v3c281`
:term:`Attribute` :def_codeterm:`target_feature` enables target architecture
features for its related :term:`function`.

:def_p:`fls_91b7nd6qslsb`
The target architecture features are as follows:

.. list-table::

   * - .. rubric:: Feature
     - .. rubric:: implicitly enables
     - .. rubric:: Description
   * - :def_p:`fls_pdyotoq8uqi2`
       adx
     -
     - :def_p:`fls_xvt9lkunjrw3`
       Intel Multi-Precision Add-Cary Instruction Extensions
   * - :def_p:`fls_vdbjoy6gbk7l`
       aes
     - :def_p:`fls_z3o1rym18tc`
       sse2
     - :def_p:`fls_fz0qdrxf2b8x`
       Advanced Encryption Standard
   * - :def_p:`fls_k3szii6nviza`
       avx
     - :def_p:`fls_xu7n574yhvjy`
       sse4.2
     - :def_p:`fls_kohbxvhxjofj`
       Advanced Vector Extensions
   * - :def_p:`fls_xsdkkfgv0pz6`
       avx2
     - :def_p:`fls_a50km9mt56it`
       avx
     - :def_p:`fls_lzss4v5m9dr2`
       Advanced Vector Extensions 2
   * - :def_p:`fls_eglaup5zyfsk`
       bmi1
     -
     - :def_p:`fls_21okiye0af0a`
       Bit Manipulation Instruction Sets
   * - :def_p:`fls_tkayqh7li81o`
       bmi2
     -
     - :def_p:`fls_8nx4bgmopzn`
       Bit Manipulation Instruction Sets 2
   * - :def_p:`fls_vv8o31s0zmli`
       fma
     - :def_p:`fls_zederqobnh28`
       avx
     - :def_p:`fls_bquwsgvc6izr`
       Three-operand fused multiply-add
   * - :def_p:`fls_g0762ekvl2ah`
       fxsr
     -
     - :def_p:`fls_h9bwinhtew3q`
       Save and restore x87 FPU, MMX technology, and SSE state
   * - :def_p:`fls_406ltkru11tk`
       lzcnt
     -
     - :def_p:`fls_8090mly3ohqc`
       Leading zeros count
   * - :def_p:`fls_4mhyvwj8pbpk`
       pclmulqdq
     - :def_p:`fls_6o32ezkodp35`
       sse2
     - :def_p:`fls_banskie1ogan`
       Packed carry-less multiplication quadword
   * - :def_p:`fls_ijsmqk7pgyiz`
       popcnt
     -
     - :def_p:`fls_qwzx3pf2075`
       Count of bits set to 1
   * - :def_p:`fls_a1jggfetycxh`
       rdrand
     -
     - :def_p:`fls_ryzbgf8esqey`
       Read random number
   * - :def_p:`fls_rmbadmoeaoxu`
       rdseed
     -
     - :def_p:`fls_ejwoescy1jfe`
       Read random seed
   * - :def_p:`fls_f9ydn049isbv`
       sha
     - :def_p:`fls_kkmfoiesmqxv`
       sse2
     - :def_p:`fls_ccd6hrx8xf3g`
       Secure Hash Algorithm
   * - :def_p:`fls_7c31e21g6bdj`
       sse
     -
     - :def_p:`fls_6betbnzf8ycc`
       Streaming SIMD Extensions
   * - :def_p:`fls_6d5a375j2775`
       sse2
     - :def_p:`fls_xc6owtvwre9p`
       sse
     - :def_p:`fls_2qddgeh2s1e6`
       Streaming SIMD Extensions 2
   * - :def_p:`fls_xthidh2nyrno`
       sse3
     - :def_p:`fls_hm2nubqjce7e`
       sse2
     - :def_p:`fls_68kp86qghcv1`
       Streaming SIMD Extensions 3
   * - :def_p:`fls_w02pk6kf9w9e`
       sse4.1
     - :def_p:`fls_umd8dyjptnc3`
       sse3
     - :def_p:`fls_uieos7d2fk6a`
       Streaming SIMD Extensions 4.1
   * - :def_p:`fls_lzl1gpco3osx`
       sse4.2.
     - :def_p:`fls_77lnhn2bi3h0`
       sse4.1
     - :def_p:`fls_qvwb3wpv7sgn`
       Streaming SIMD Extensions 4.2
   * - :def_p:`fls_9x2on8w44k4f`
       ssse3
     - :def_p:`fls_ioa5uo8ybad7`
       sse3
     - :def_p:`fls_s5fq1r31yi5u`
       Supplemental Streaming SIMD Extensions 3
   * - :def_p:`fls_rilqwazchfpp`
       xsave
     -
     - :def_p:`fls_j0v7eqn0eohk`
       Save processor extended status
   * - :def_p:`fls_f0of1395z9pn`
       xsavec
     -
     - :def_p:`fls_gi67qivyplxp`
       Save processor extended status with compaction
   * - :def_p:`fls_v9gf6selc17l`
       xsaveopt
     -
     - :def_p:`fls_ht97aq7xd5hd`
       Save processor extended states optimized
   * - :def_p:`fls_jyb5s2r8w1po`
       xsaves
     -
     - :def_p:`fls_m7ejnghh2n51`
       Save processor extended states supervizor

.. rubric:: Undefined Behavior

:def_p:`fls_xx51fjkbgg5g`
It is undefined behavior to execute a program compiled with target architecture
features that are not supported.

.. rubric:: Examples

.. code-block:: text

   #[target_feature(enable="bmi1,sse4.1")]
   fn requires_target_architecture_features () {}

Attribute ``track_caller``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TrackCallerContent ::=
       $$track_caller$$

.. rubric:: Legality Rules

:def_p:`fls_h8yepgchjxv9`
:term:`Attribute` :codeterm:`track_caller` shall apply to non-:term:`main
function`\ s with :term:`ABI` "Rust".

:def_p:`fls_w1pxtzp7acty`
:term:`Attribute` :def_codeterm:`track_caller` allows the :term:`function body`
of its related :term:`function` to obtain a :codeterm:`core::panic::Location`
which indicates the topmost untracked caller that ultimately led to the
invocation of the :term:`function`.

:def_p:`fls_zch43jpetmdu`
A tool is not required to implement this indication in an effective manner.

:def_p:`fls_y1e258p4rby5`
When applied to an :term:`associated trait function`, :term:`attribute`
:codeterm:`track_caller` applies to all :term:`implementing function`\ s. If the
:term:`associated trait function` has a :term:`default implementation`, then the
:term:`attribute` applies to all :term:`overriding function`\ s.

.. rubric:: Undefined Behavior

:def_p:`fls_vkz8t751gfhk`
It is undefined behavior when :term:`attribute` :codeterm:`track_caller`
applies to an :term:`external function` but does not apply to all linked
:term:`implementation`\ s.

:def_p:`fls_ddg0u5lej74x`
It is undefined behavior when :term:`attribute` :codeterm:`track_caller` is
applied to an :term:`exported function` but the :term:`external function` it
links to is missing the :term:`attribute`.

.. rubric:: Examples

.. code-block:: text

   #[track_caller]
   fn who_called_me () {}

Conditional Compilation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

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

:def_p:`fls_xrjp7xw9jutz`
:term:`Attribute` :def_codeterm:`cfg` enables :term:`conditional compilation`.

:def_p:`fls_l96kyix5xsof`
A :term:`configuration predicate` evaluates statically to either ``true`` or
``false``.

:def_p:`fls_tncxxsyutppf`
An :term:`all configuration predicate` evaluates statically to ``true`` when
either all nested configuration predicates evaluate to ``true``, or there are no
nested configuration predicates.

:def_p:`fls_m0zxktz168e0`
An :term:`any configuration predicate` evaluates statically to ``true`` when any
nested configuration predicate evaluates to ``true``.

:def_p:`fls_tvsadfy9uibu`
A :term:`not configuration predicate `\ evaluates statically to ``true`` when
its nested configuration predicate evaluates to ``false``.

:def_p:`fls_jbl9xyynjo0g`
The :term:`evaluation` of a configuration option is tool-defined.

.. rubric:: Examples

.. code-block:: text

   #[cfg(all(unix, target_pointer_width = "32"))]
   fn on_32bit_unix() {}

Attribute ``cfg_attr``
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CfgAttrContent ::=
       $$cfg_attr$$ $$($$ CondigurationPredicate $$,$$ AttributeContentList $$)$$

.. rubric:: Legality Rules

:def_p:`fls_r66jhict6rlq`
:term:`Attribute` :def_codeterm:`cfg_attr` enables :term:`conditional
compilation`.

:def_p:`fls_rzw12sagm585`
An :term:`attribute` :codeterm:`cfg_attr` where the related :term:`configuration
predicate` evaluates to ``true`` is replaced with a new :term:`attribute`
for each :syntax:`AttributeContent` enumerated in the :term:`attribute`'s
:syntax:`AttributeContentList`.

.. rubric:: Examples

.. code-block:: text

   #[cfg_attr(windows, path="windows.rs")]
   mod os;

Derivation Attributes
~~~~~~~~~~~~~~~~~~~~~

Attribute ``automatically_derived``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   AutomaticallyDerivedContent ::=
       $$automatically_derived$$

.. rubric:: Legality Rules

:def_p:`fls_5u1e0lkt0ab1`
:term:`Attribute` :def_codeterm:`automatically_derived` is automatically
added to :term:`implementation`\ s that are created by :term:`attribute`
:codeterm:`derive` for :term:`built-in trait`\ s.

Attribute ``derive``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   DeriveContent ::=
       $$derive$$ $$($$ SimplePathList? $$)$$

.. rubric:: Legality Rules

:def_p:`fls_4btm6zwf445a`
:term:`Attribute` :codeterm:`derive` shall apply to an :term:`abstract data
type`.

:def_p:`fls_pjmbmj2b35y8`
:term:`Attribute` :def_codeterm:`derive` lists :term:`derive macro`\ s for
automatic implementation by a tool.

.. rubric:: Examples

.. code-block:: text

   #[derive(PartialEq)]
   struct S<T> {
       field: T
   }

:def_p:`fls_xtty2ino4vwc`
Attribute ``derive`` causes trait :std:`core::cmp::PartialEq` to be
automatically implemented for struct ``S<T>`` as follows:

.. code-block:: text

   impl<T: core::cmp::PartialEq> core::cmp::PartialEq for S<T> {
       fn eq(&self, other: &S<T>) -> bool {
          self.field == other.field
       }

       fn ne(&self, other: &S<T>) -> bool {
          self.field != other.field
       }
   }

Diagnostics Attributes
~~~~~~~~~~~~~~~~~~~~~~

:def_p:`fls_ghumzt9ybtit`
Diagnostic :term:`attribute`\ s are related to linting, and are not defined in
this document.

Documentation Attributes
~~~~~~~~~~~~~~~~~~~~~~~~

Attribute ``doc``
^^^^^^^^^^^^^^^^^

:def_p:`fls_5cqi7is713f1`
`Rust
<https://doc.rust-lang.org/stable/rustdoc/the-doc-attribute.html#the-doc-attribu
te>`_

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
       $$attr( $$AttributeContentList $$)$$

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

:def_p:`fls_1ee9qjcgbwme`
The :term:`inner attribute` version and the :term:`outer attribute` version
of :term:`attribute` :def_codeterm:`doc` associate documentation with a
:term:`construct`.

.. rubric:: Examples

:def_p:`fls_necp8a7v255c`
**???**

Foreign Function Interface Attributes
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

Attribute ``crate_name``
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   CrateNameContent ::=
       $$crate_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_tsdk8jyajcg`
:term:`Attribute` :codeterm:`crate_name` shall appear at the :term:`crate
level`.

:def_p:`fls_6riphqysh0gd`
:term:`Attribute` :def_codeterm:`crate_name` shall specify the name of the
related :term:`crate`.

.. rubric:: Examples

.. code-block:: text

   #![crate_name = "factories"]

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

:def_p:`fls_2i2g55nqqpc1`
:term:`Attribute` :codeterm:`crate_type` shall appear at the :term:`crate
level`.

:def_p:`fls_1zziddjuzjeq`
:term:`Attribute` :def_codeterm:`crate_type` shall specify the linkage
:term:`type` of the :term:`crate` it appears in.

.. rubric:: Examples

.. code-block:: text

   #![crate_type = "cdylib"]

Attribute ``export_name``
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ExportNameContent ::=
       $$export_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_r3fwpuuaoeie`
:term:`Attribute` :codeterm:`export_name` shall apply to :term:`function`\ s and
:term:`static`\ s.

:def_p:`fls_spwr6gf7kpds`
:term:`Attribute` :def_codeterm:`export_name` shall specify the exported symbol
of the related :term:`function` or :term:`static`.

.. rubric:: Examples

.. code-block:: text

   #[export_name = "exported_symbol"]
   pub fn rust_name() {}

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

:def_p:`fls_yslpkdngo8hj`
:term:`Attribute` :codeterm:`link` shall apply to :term:`external block`\ s.

:def_p:`fls_6rohnk4swj6c`
:term:`Attribute` :def_codeterm:`link` shall specify the name of a native
library that a tool should link with.

:def_p:`fls_o83pf3bcrzma`
The following native library kinds are available:

.. list-table::

   * - .. rubric:: Native Library Kind
     - .. rubric:: Description
   * - :def_p:`fls_wpqawdpevkj6`
       dylib
     - :def_p:`fls_vvrzw1ki1gzm`
       Dynamic library
   * - :def_p:`fls_h9dfs6kzmobp`
       framework
     - :def_p:`fls_c9s192hstdy`
       macOS framework
   * - :def_p:`fls_3a3r4jf7hzqr`
       static
     - :def_p:`fls_u8sta6llysr`
       Static library

:def_p:`fls_1tdheukgm6ai`
When :term:`attribute` :codeterm:`link` appears without a native library kind,
its native library kind defaults to ``dylib``. Native library kind framework is
only valid on macOS targets.

:def_p:`fls_3i9ijypnh8nx`
If :term:`attribute` :codeterm:`link` appears without a WebAssembly module name,
then the WebAssembly module name defaults to ``env``.

.. rubric:: Examples

.. code-block:: text

   #[link(name = "CoreFoundation", kind = "framework")]
   extern {}

Attribute ``link_name``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   LinkNameContent ::=
       $$link_name$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_g09jhukl0ez2`
:term:`Attribute` :codeterm:`link_name` shall apply to :term:`external
function`\ s and :term:`external static`\ s.

:def_p:`fls_d00wni4edi8f`
:term:`Attribute` :def_codeterm:`link_name` shall specify the linking symbol of
the related :term:`external function` or :term:`external static`.

.. rubric:: Examples

.. code-block:: text

   extern {
       #[link_name = "linking_symbol"]
       pub fn rust_name() {}
   }

Attribute ``link_section``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   LinkSectionContent ::=
       $$link_section$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_5loqzajiz34m`
:term:`Attribute` :codeterm:`link_section` shall apply to :term:`function`\ s
and :term:`static`\ s.

:def_p:`fls_cyxk12wuicml`
:term:`Attribute` :def_codeterm:`link_section` specifies the object file section
where the symbol of the related :term:`function` or :term:`static` will be
placed.

.. rubric:: Examples

.. code-block:: text

   #[link_section = ".example_section"]
   pub static THE_ANSWER: u32 = 42;

Attribute ``no_link``
^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoLinkContent ::=
       $$no_link$$

.. rubric:: Legality Rules

:def_p:`fls_ayhn6g6sgt3h`
:term:`Attribute` :codeterm:`no_link` shall apply to :term:`external crate
import`\ s.

:def_p:`fls_76ox8n3eef5`
:term:`Attribute` :def_codeterm:`no_link` indicates that the imported
:term:`external crate` will not be linked into the resulting binary or library.

.. rubric:: Examples

.. code-block:: text

   #[no_link]
   extern crate do_not_link;

Attribute ``no_main``
^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoMainContent ::=
       $$no_main$$

.. rubric:: Legality Rules

:def_p:`fls_84a9k0fzmnfk`
:term:`Attribute` :codeterm:`no_main` shall appear at the :term:`crate level`.

:def_p:`fls_6qig3s3qpj0i`
:term:`Attribute` :def_codeterm:`no_main` indicates that the symbols of the
:term:`main function` will not be present in a binary.

.. rubric:: Examples

.. code-block:: text

   #![no_main]

Attribute ``no_mangle``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoMangleContent ::=
       $$no_mangle$$

.. rubric:: Legality Rules

:def_p:`fls_q5swm5meafmx`
:term:`Attribute` :codeterm:`no_mangle` shall apply to an :term:`entity`.

:def_p:`fls_esaew4fqk8mm`
:term:`Attribute` :def_codeterm:`no_mangle` indicates that the :term:`name` of
the related :term:`entity` will be used as the symbol for that :term:`entity`.

:def_p:`fls_lvnclpxbye9u`
:term:`Attribute` :codeterm:`no_mangle` causes the related :term:`entity` to be
publicly exported from the produced library or object file.

.. rubric:: Examples

.. code-block:: text

   #[no_mangle]
   pub fn symbol_name() {}

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

:def_p:`fls_vetjq9sw84qc`
:term:`Attribute` :codeterm:`repr` shall apply to an :term:`abstract data type`.

:def_p:`fls_is2esjz1sy36`
:term:`Attribute` :def_codeterm:`repr` shall indicate the :term:`type
representation` of the related :term:`type`.

.. rubric:: Examples

.. code-block:: text

   #[repr(C, align(8))]
   struct c_struct {
       first_field: i16,
       second_field: i8
   }

Attribute ``used``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   UsedContent ::=
       $$used$$

.. rubric:: Legality Rules

:def_p:`fls_s4ii078wgpk`
:term:`Attribute` :codeterm:`used` shall apply to :term:`static`\ s.

:def_p:`fls_k293nzcffks4`
:term:`Attribute` :def_codeterm:`used` forces a tool to keep the related
:term:`static` in the output object file even if the :term:`static` is not used
or referenced by other :term:`item`\ s in the :term:`crate`.

.. rubric:: Examples

.. code-block:: text

   #[used]
   pub static THE_ANSWER: u32 = 42;

Limits Attributes
~~~~~~~~~~~~~~~~~

Attribute ``recursion_limit``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   RecursionLimitContent ::=
       $$recursion_limit$$ $$=$$ $$"$$ DecimalLiteral $$"$$

.. rubric:: Legality Rules

:def_p:`fls_o55cxc67sya7`
:term:`Attribute` :codeterm:`recursion_limit` shall appear at the :term:`crate
level`.

:def_p:`fls_o9p8fa8zhe15`
:term:`Attribute` :def_codeterm:`recursion_limit` sets the maximum depth of
:term:`macro expansion` and :term:`auto-dereferencing`.

.. rubric:: Examples

.. code-block:: text

   #![recursion_limit = "42"]

Attribute ``type_length_limit``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TypeLengthLimitContent ::=
       $$type_length_limit$$ $$=$$ $$"$$ DecimalLiteral $$"$$

.. rubric:: Legality Rules

:def_p:`fls_dfnkzj8ob3uq`
:term:`Attribute` :codeterm:`type_length_limit` shall appear at the :term:`crate
level`.

:def_p:`fls_61vt1r8g51nh`
:term:`Attribute` :def_codeterm:`type_length_limit` sets the maximum number of
:term:`type substitution`\ s when constructing a :term:`concrete type`.

.. rubric:: Examples

.. code-block:: text

   #![type_length_limit = "42"]

Macros Attributes
~~~~~~~~~~~~~~~~~

Attribute ``macro_export``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   MacroExportContent ::=
       $$macro_export$$

.. rubric:: Legality Rules

:def_p:`fls_3ma7zkk6john`
:term:`Attribute` :codeterm:`macro_export` shall apply to :term:`declarative
macro`\ s.

:def_p:`fls_h26iw5wh4lla`
:term:`Attribute` :def_codeterm:`macro_export` changes the :term:`visibility` of
the related :term:`declarative macro` to :term:`public` and brings the name of
the :term:`declarative macro` into :term:`path scope`.

.. rubric:: Examples

.. code-block:: text

   #[macro_export]
   macro_rules! m {
       () => {};
   }

Attribute ``macro_use``
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   MacroUseContent ::=
       $$macro_use$$ ImportedMacroList?

   ImportedMacroList ::=
       $$($$ IdentifierList $$)$$

.. rubric:: Legality Rules

:def_p:`fls_uua0nthq9id`
:term:`Attribute` :codeterm:`macro_use` shall apply to :term:`external crate
import`\ s and :term:`module`\ s.

:def_p:`fls_oq4kyo5z5tj5`
An :syntax:`ImportedMacroList` enumerates macros-to-import. A macro-to-import
shall be subject to attribute :codeterm:`macro_export`.

:def_p:`fls_skexvtpbjknn`
When applied to an :term:`external crate import`, :term:`attribute`
:def_codeterm:`macro_use` imports from the related :term:`crate` either:

* :def_p:`fls_v03924dr0u0z`
  The enumerated macros-to-import, or

* :def_p:`fls_eha2hoey857x`
  If no macros-to-import have been specified, all :term:`macro`\ s subject to
  :term:`attribute` :codeterm:`macro_export`.

:def_p:`fls_p6jlgmn2sg7j`
When applied to a :term:`module`, :term:`attribute` :codeterm:`macro_use`
extends the :term:`scope` of the related :term:`macro`.

.. rubric:: Examples

.. code-block:: text

   #[macro_use(first_macro, second_macro)]
   extern crate macros;
   #[macro_use]
   mod module {}

Attribute ``proc_macro``
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ProcMacroContent ::=
       $$proc_macro$$

.. rubric:: Legality Rules

:def_p:`fls_u48dtmh97g`
:term:`Attribute` :codeterm:`proc_macro` shall apply to public :term:`function`\
s in the crate root.

:def_p:`fls_t4ez0zg1m569`
:term:`Attribute` :def_codeterm:`proc_macro` turns the related :term:`function`
into a :term:`function-like macro`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro]
   pub fn make_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

Attribute ``proc_macro_attribute``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. syntax::

   ProcMacroAttributeContent ::=
       $$proc_macro_attribute$$

.. rubric:: Legality Rules

:def_p:`fls_huznzmkuhdky`
:term:`Attribute` :codeterm:`proc_macro_attribute` shall apply to
:term:`function`\ s.

:def_p:`fls_gc3ly8fsodf1`
:term:`Attribute` :def_codeterm:`proc_macro_attribute` turns the related
:term:`function` into an :term:`attribute macro`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro_attribute]
   pub fn output_and_return_item
       (attr: TokenStream, item: TokenStream) -> TokenStream
   {
       println!("attr: \"{}\"", attr.to_string());
       println!("item: \"{}\"", item.to_string());
       item
   }

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

:def_p:`fls_l82yswg0k3px`
:term:`Attribute` :codeterm:`proc_macro_derive` shall apply to :term:`function`\
s.

:def_p:`fls_ir9i4i2x5gyx`
:term:`Attribute` :def_codeterm:`proc_macro_derive` turns the related
:term:`function` into a :term:`derive macro`, where :syntax:`DeriveName` defines
the :term:`name` of the :term:`derive macro` available to :term:`attribute`
:codeterm:`derive`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro_derive(Answer, attributes(marker))]
   pub fn derive_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

   #[derive(Answer)]
   struct S {
       #[marker] field: ()
   }

Modules Attributes
~~~~~~~~~~~~~~~~~~

Attribute ``path``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   PathContent ::=
       $$path$$ $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_lht4pcezmbxw`
:term:`Attribute` :codeterm:`path` shall apply to :term:`module`\ s.

:def_p:`fls_qb6anohvc03k`
:term:`Attribute` :def_codeterm:`path` specifies the :def_term:`module path` of
the respective :term:`module` as a :term:`string literal`.

:def_p:`fls_18tcecx4p2wp`
A tool is free to define the format of a :term:`module path`.

.. rubric:: Examples

.. code-block:: text

   #[path = "path/to/inline_module"]
   mod inline_module {
       #[path = "path/to/outline_module"]
       mod outline_module;
   }

Prelude Attributes
~~~~~~~~~~~~~~~~~~

Attribute ``no_implicit_prelude``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoImplicitPreludeContent ::=
       $$no_implicit_prelude$$

.. rubric:: Legality Rules

:def_p:`fls_tki5k5uo74gw`
The :term:`inner attribute` version of :term:`attribute`
:codeterm:`no_implicit_prelude` shall apply at the :term:`crate level` or to
:term:`module`\ s.

:def_p:`fls_cmrqxc5oax4r`
The :term:`outer attribute` version of :term:`attribute`
:codeterm:`no_implicit_prelude` shall apply to :term:`module`\ s.

:def_p:`fls_c7v2hbdb7g2d`
:term:`Attribute` :def_codeterm:`no_implicit_prelude` prevents the import
of the :term:`extern prelude`, the :term:`standard library prelude`, and the
:term:`tool prelude`.

.. rubric:: Examples

.. code-block:: text

   #[no_implicit_prelude]
   mod module {}

Attribute ``no_std``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NoStdContent ::=
       $$no_std$$

.. rubric:: Legality Rules

:def_p:`fls_qnxihxsvkyf6`
:term:`Attribute` :codeterm:`no_std` shall apply at the :term:`crate level`.

:def_p:`fls_kxav9vw59ts4`
:term:`Attribute` :def_codeterm:`no_std` has the following effects:

* :def_p:`fls_ve1shwjq09pl`
  Prevents the import of the :term:`standard library prelude`.

* :def_p:`fls_wgwsn7laoju7`
  Imports the :term:`core prelude`.

* :def_p:`fls_lxkd6hdboav4`
  Imports all :term:`exported macro`\ s of the :term:`core crate` into the
  :term:`macro_use prelude`.

.. rubric:: Examples

.. code-block:: text

   #![no_std]

Runtime Attributes
~~~~~~~~~~~~~~~~~~

Attribute ``global_allocator``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   GlobalAllocatorContent ::=
       $$global_allocator$$

.. rubric:: Legality Rules

:def_p:`fls_5b8aewlgeon8`
:term:`Attribute` :codeterm:`global_allocator` shall apply to :term:`static`\
s whose :term:`type`\ s implement the :codeterm:`core::alloc::GlobalAlloc`
:term:`trait`.

:def_p:`fls_homoidh8mu1r`
:term:`Attribute` :def_codeterm:`global_allocator` sets the global allocator to
the related :term:`static`.

.. rubric:: Examples

.. code-block:: text

   #[global_allocator]
   pub static THE_ANSWER: u32 = 42;

Attribute ``panic_handler``
^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   PanicHandlerContent ::=
       $$panic_handler$$

.. rubric:: Legality Rules

:def_p:`fls_ryz8qy1wdnma`
:term:`Attribute` :codeterm:`panic_handler` shall apply to :term:`function`\ s
with :term:`function signature` ``fn(&core::panic::PanicInfo) -> !``.

:def_p:`fls_8gqun8lma9wz`
:term:`Attribute` :def_codeterm:`panic_handler` indicates that its related
:term:`function` defines the behavior of :term:`panic`\ s.

:def_p:`fls_ka66jcu8gir7`
A :term:`crate` graph shall contain exactly one :term:`function` subject to
:term:`attribute` :codeterm:`panic_handler`.

.. rubric:: Examples

.. code-block:: text

   #[panic_handler]
   fn panic(info: &core::panic::PanicInfo) -> ! {}

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

:def_p:`fls_7mzjahvdzpy5`
:term:`Attribute` :codeterm:`windows_subsystem` shall appear at the :term:`crate
level` of a :term:`binary crate`.

:def_p:`fls_t3c0t3lcnebk`
:term:`Attribute` :def_codeterm:`windows_subsystem` specifies the subsystem
on Windows.

:def_p:`fls_go7pfkgpjk2t`
If :term:`attribute` :codeterm:`windows_subsystem` is missing, the subsystem of
the related :term:`binary crate` defaults to ``console``.

.. rubric:: Examples

.. code-block:: text

   #![windows_subsystem = "console"]

Testing Attributes
~~~~~~~~~~~~~~~~~~

Attribute ``ignore``
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   IgnoreContent ::=
       $$ignore$$ IgnoreReason?

   IgnoreReason ::=
       $$=$$ StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_qmdylxse9yhu`
:term:`Attribute` :codeterm:`ignore` shall apply to :term:`testing function`\ s.

:def_p:`fls_9m8e59fc1tyh`
:term:`Attribute` :def_codeterm:`ignore` prevents the execution of its related
:term:`testing function`.

.. rubric:: Examples

.. code-block:: text

   #[test]
   #[ignore = "not implemented yet"]
   fn unit_testing_function() {}

Attribute ``should_panic``
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ShouldPanicContent ::=
       $$should_panic$$ ExpectedPanicMessage?

   ExpectedPanicMessage ::=
       $$($$ $$expected$$ $$=$$ StringLiteral $$)$$

.. rubric:: Legality Rules

:def_p:`fls_w7dq8gnzel36`
:term:`Attribute` :codeterm:`should_panic` shall apply to :term:`testing
function`\ s.

:def_p:`fls_bm5x846zfnb8`
:term:`Attribute` :def_codeterm:`should_panic` indicates that for the related
:term:`testing function` to pass, it should :term:`panic`.

:def_p:`fls_bcoq5aus8nkr`
If :syntax:`ExpectedPanicMessage` is specified, then the related :term:`testing
function` passes only when the :term:`panic` message contains the
:syntax:`ExpectedPanicMessage`.

.. rubric:: Examples

.. code-block:: text

   #[test]
   #[should_panic(expected = "did not get meaning of life")]
   fn test_meaning_of_life() {
       assert_eq!(meaning_of_life(), 42, "did not get meaning of life");
   }

Attribute ``test``
^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   TestContent ::=
       $$test$$

.. rubric:: Legality Rules

:def_p:`fls_o2elhg5w1rj9`
:term:`Attribute` :codeterm:`test` shall apply to a :term:`function` that:

* :def_p:`fls_ert22u8rvkxt`
  Is not an :term:`async function`, and

* :def_p:`fls_c9ckjrq6emdj`
  Is not an :term:`unsafe function`, and

* :def_p:`fls_4uykzqpq6svl`
  Lacks :term:`generic parameter`\ s, and

* :def_p:`fls_aqzd30s267pt`
  Lacks :term:`function parameter`\ s, and

* :def_p:`fls_n3hjhh3d7tyx`
  Its :term:`return type` is the :term:`unit type`.

:def_p:`fls_ze6cs75y9aft`
:term:`Attribute` :def_codeterm:`test` indicates that the respective
:term:`function` is a :def_term:`testing function`.

:def_p:`fls_pcs0prrh23y3`
A :term:`testing function` that returns the :term:`unit type` passes when it
terminates and does not :term:`panic`.

:def_p:`fls_niky8lbkvej9`
A :term:`testing function` that returns ``core::result::Result<(), E>`` passes
when it returns ``core::result::Result::OK(())``.

:def_p:`fls_qfuntdm2g184`
A :term:`testing function` that does not terminate shall pass and fail according
to the tool.

.. rubric:: Examples

.. code-block:: text

   #[test]
   fn test_the_thing() -> core::result::Result<()> {
       let input = create_input()?;
       do_the_thing(&input)?;
       core::result::Result::Ok(());
   }

Type Attributes
~~~~~~~~~~~~~~~

Attribute ``non_exhaustive``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   NonExhaustiveContent ::=
       $$non_exhaustive$$

.. rubric:: Legality Rules

:def_p:`fls_szvrd79cgzsg`
:term:`Attribute` :codeterm:`non_exhaustive` shall apply to :term:`enum type`\
s, :term:`enum variant`\ s, and :term:`struct type`\ s.

:def_p:`fls_1of56vl2ewq0`
:term:`Attribute` :def_codeterm:`non_exhaustive` indicates that the
related :term:`type` or :term:`enum variant` may have more :term:`field`\
s or :term:`enum variant`\ s added in the future. A :term:`type` subject
to :term:`attribute` :codeterm:`non_exhaustive` is referred to as a
:def_term:`non-exhaustive type`.

:def_p:`fls_hkyzdmmdyoin`
A :term:`non-exhaustive type` shall not be constructed outside of its defining
:term:`crate`.

:def_p:`fls_7b0fvwrmz0mh`
An :term:`enum variant` subject to :term:`attribute` :codeterm:`non_exhaustive`
is referred to as a :def_term:`non-exhaustive variant`.

:def_p:`fls_oqfrg9tqgaj8`
A :term:`non-exhaustive variant` shall not be constructed outside of its
defining :term:`crate`.

:def_p:`fls_aql3c89840ix`
:term:`Pattern matching` a :term:`non-exhaustive variant` shall require
a :term:`struct pattern` with a :term:`rest pattern` outside its defining
:term:`crate`.

:def_p:`fls_cez7yxfc376c`
:term:`Pattern matching` a :term:`non-exhaustive variant` does not contribute
towards the exhaustiveness of :term:`match arm`\ s.

.. rubric:: Examples

.. code-block:: text

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

