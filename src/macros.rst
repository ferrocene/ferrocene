.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Macros
======

.. rubric:: Legality Rules

:def_p:`fls_j1jc83erljo0`
A :term:`macro` is a custom definition that extends Rust by defining callable
syntactic transformations. The effects of a :term:`macro` are realized through
:term:`[macro invocation]s` or :term:`attribute` use. :term:`[Macro]s` come in
two distinct forms:

* :def_p:`fls_23eapx3ckymf`
  :term:`[Declarative macro]s` define rules for recognizing syntactic patterns
  and generating direct syntax.

* :def_p:`fls_a5uemz2hnbi8`
  :term:`[Procedural macro]s` define augmented :term:`[function]s` that operate
  on and return a stream of :term:`[lexical element]s`.

:def_p:`fls_rnty1c8l5495`
:term:`[Token]s` are a subset of :term:`[lexical element]s` consumed by
:term:`[macro]s`.

Declarative Macros
------------------

.. rubric:: Syntax

.. syntax::

   MacroRulesDeclaration ::=
       $$macro_rules$$ $$!$$ Name MacroRulesDefinition

   MacroRulesDefinition ::=
       $$($$ MacroRuleList $$)$$ $$;$$
     | $$[$$ MacroRuleList $$]$$ $$;$$
     | $${$$ MacroRuleList $$}$$
   MacroRuleList ::=
       MacroRule ($$;$$ MacroRule)* $$;$$?

   MacroRule ::=
       MacroMatcher $$=>$$ MacroTranscriber

   MacroMatcher ::=
       $$($$ MacroMatch* $$)$$
     | $$[$$ MacroMatch* $$]$$
     | $${$$ MacroMatch* $$}$$

   MacroTranscriber ::=
       DelimitedTokenTree

   MacroMatch ::=
       MacroMatcher
     | MacroMatchToken
     | MacroMetavariableMatch
     | MacroRepetitionMatch


:def_p:`fls_ikzjsq8heyk6`
A :def_syntax:`MacroMatchToken` is any :term:`lexical element` in
category :syntax:`LexicalElement`, except punctuation ``$`` and category
:syntax:`Delimiter`.

.. rubric:: Legality Rules

:def_p:`fls_w44hav7mw3ao`
A :term:`declarative macro` is a :term:`macro` that associates a :term:`name`
with a set of syntactic transformation :term:`[macro rule]s`.

:def_p:`fls_dw1nq4r9ghhd`
A :term:`macro rule` is a :term:`construct` that consists of a :term:`macro
matcher` and a :term:`macro transcriber`.

:def_p:`fls_oq4xn8guos8f`
A :term:`macro matcher` is a :term:`construct` that describes a syntactic
pattern that a :term:`macro` must match.

:def_p:`fls_cdaf8viwmdfe`
A :term:`macro match` is the most basic form of a satisfied :term:`macro
matcher`.

:def_p:`fls_ljavs0w61z3j`
A :term:`macro transcriber` is a :term:`construct` that describes the
replacement syntax of a :term:`macro`.

:def_p:`fls_3jspk8obv7sd`
A :term:`declarative macro` is invoked using a :term:`macro invocation`.

.. rubric:: Examples

.. code-block:: text

   macro_rules! answer_to_life {
       () => { 42 };
   }

Metavariables
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   MacroMetavariableMatch ::=
       $$$$$ MacroMetavariable $$:$$ MacroFragmentSpecifier

   MacroMetavariable ::=
       Keyword
     | NonKeywordIdentifier

   MacroFragmentSpecifier ::=
       $$block$$
     | $$expr$$
     | $$ident$$
     | $$item$$
     | $$lifetime$$
     | $$literal$$
     | $$meta$$
     | $$pat$$
     | $$pat_param$$
     | $$path$$
     | $$stmt$$
     | $$tt$$
     | $$ty$$
     | $$vis$$

   MacroMetavariableIndication ::=
       $$$$$ MacroMetavariable

.. rubric:: Legality Rules

:def_p:`fls_g93r3teei8wo`
:term:`[Declarative macro]s` employ :term:`[metavariable]s` to match a
:term:`token` of a particular kind and bind its :term:`value` to a name for use
during :term:`macro transcription`.

:def_p:`fls_4zdait30exvn`
A :term:`metavariable` is a :term:`macro match` that describes a
:term:`variable`.

:def_p:`fls_8zypylq60zba`
A :term:`fragment specifier` is a :term:`construct` that indicates the
:term:`type` of a :term:`metavariable`.

.. rubric:: Examples

.. code-block:: text

   macro_rules! square {
       ($e:expr) => { $e * $e };
   }

Repetition
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   MacroRepetitionMatch ::=
       $$$$$ $$($$ MacroMatch* $$)$$ MacroRepetitionSeparator? MacroRepetitionOperator

   MacroRepetitionTranscriber ::=
       $$$$$ $$($$ TokenTree* $$)$$ MacroRepetitionSeparator? MacroRepetitionOperator

   MacroRepetitionOperator ::=
       $$+$$
     | $$*$$
     | $$?$$

:def_p:`fls_4ps4x4513xau`
A :def_syntax:`MacroRepetitionSeparator` is any :term:`lexical element` in
category :syntax:`LexicalElement`, except punctuation ``+``, ``*``, ``?``, and
category :syntax:`Delimiter`.

.. rubric:: Legality Rules

:def_p:`fls_8byjmlgum2f3`
A :term:`macro repetition in matching` allows for a syntactic pattern to be
matched zero or multiple times during :term:`macro matching`.

:def_p:`fls_ltdp3zs60dzr`
A :term:`macro repetition in transcription` allows for a syntactic pattern to be
transcribed zero or multiple times during :term:`macro transcription`.

:def_p:`fls_u86j0zm2jshf`
A :term:`repetition operator` is a :term:`construct` that indicates the number
of times a :term:`macro repetition in matching` or a :term:`macro repetition in
transcription` can be repeated.

:def_p:`fls_h5f8x4jdnvbu`
The effects of a :term:`repetition operator` are as follows:

* :def_p:`fls_hf4gj5pfl437`
  ``*`` - Zero or more repetitions.

* :def_p:`fls_tm0w0680wf4x`
  ``+`` - One or more repetitions.

* :def_p:`fls_10lsg5212ffb`
  ``?`` - Zero or one repetition.

.. rubric:: Examples

.. code-block:: text

   macro_rules! generate_pairs {
       ( $( $first:ident )* ; $( &second:ident )* )
           =>
       { $( $first, $second )* };
   }

Procedural Macros
-----------------

.. rubric:: Legality Rules

:def_p:`fls_ejbddhggstd2`
A :term:`procedural macro` is a :term:`macro` that encapsulates syntactic
transformations in a :term:`function`. :term:`[Procedural macro]s` consume one
or more streams of :term:`[token]s` and produce a stream of :term:`[token]s`.

:def_p:`fls_pcce9gmjpxba`
:term:`[Procedural macro]s` shall be defined in a :term:`crate` subject to
:term:`attribute` :codeterm:`crate_type` where the type is ``proc-macro``.

:def_p:`fls_vtzuplb1p3s`
A :term:`macro implementation function` is the :term:`function` that
encapsulates the syntactic transformations of a :term:`procedural macro`.

:def_p:`fls_mewfehvgm16r`
A :term:`macro implementation function` enters the :term:`name` of the
:term:`procedural macro` into the :term:`macro namespace`.

Function-like Macros
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_utd3zqczix`
A :term:`function-like macro` is a :term:`procedural macro` that consumes a
stream of :term:`[token]s` and produces a stream of :term:`[token]s`.

:def_p:`fls_ojr30lf6jfx0`
The :term:`macro implementation function` of a :term:`function-like macro` shall
be subject to the following restrictions:

* :def_p:`fls_ljkjmegynhiy`
  The :term:`macro implementation function` shall be subject to
  :term:`attribute` :codeterm:`proc_macro`,

* :def_p:`fls_8a8qhzjw5hax`
  The :term:`macro implementation function` shall be subject to visibility
  modifier ``pub``,

* :def_p:`fls_ofzql79i9if`
  The :term:`macro implementation function` shall lack :term:`[function
  qualifier]s`,

* :def_p:`fls_j1wsyzip2qb3`
  The :term:`macro implementation function` shall lack :term:`[generic
  parameter]s`,

* :def_p:`fls_etyo9bmzxby6`
  The :term:`macro implementation function` shall have a single :term:`function
  parameter` whose :term:`type specification` indicates :term:`type`
  :codeterm:`proc_macro::TokenStream`,

* :def_p:`fls_mkl9b38m0sf1`
  The :term:`macro implementation function` shall have a :term:`return
  type` whose :term:`type specification` indicates :term:`type`
  :codeterm:`proc_macro::TokenStream`.

:def_p:`fls_lfmb22bfnrye`
A :term:`function-like macro` is invoked using a :term:`macro invocation`.

:def_p:`fls_fbgal48cgj44`
The sole parameter of the :term:`macro implementation function` captures the
:term:`token` stream produced from the :syntax:`DelimitedTokenTree` of the
:term:`macro invocation`, excluding outer :syntax:`[Delimiter]s`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro]
   pub fn make_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

Derive Macros
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_e5x92q2rq8a0`
A :term:`derive macro` is a :term:`procedural macro` that consumes a stream
of :term:`[token]s` and produces a stream of :term:`[token]s`. :term:`[Derive
macro]s` are used to construct new syntax for :term:`[abstract data type]s`.

:def_p:`fls_ldw75sy5uj7p`
The :term:`macro implementation function` of a :term:`derive macro` shall be
subject to the following restrictions:

* :def_p:`fls_7gcnui9beky`
  The :term:`macro implementation function` shall be subject to
  :term:`attribute` :codeterm:`proc_macro_derive`,

* :def_p:`fls_ef30ropg7dhx`
  The :term:`macro implementation function` shall be subject to visibility
  modifier ``pub``,

* :def_p:`fls_mo00vqm9xfqc`
  The :term:`macro implementation function` shall lack :term:`[function
  qualifier]s`,

* :def_p:`fls_gr9wugeqyb3b`
  The :term:`macro implementation function` shall lack :term:`[generic
  parameter]s`,

* :def_p:`fls_npnze2cg8ae`
  The :term:`macro implementation function` shall have a single :term:`function
  parameter` whose :term:`type specification` indicates :term:`type`
  :codeterm:`proc_macro::TokenStream`,

* :def_p:`fls_w2h4lk6bmht`
  The :term:`macro implementation function` shall have a :term:`return
  type` whose :term:`type specification` indicates :term:`type`
  :codeterm:`proc_macro::TokenStream`.

:def_p:`fls_x96a0xzcyrko`
A :term:`derive macro` is invoked using :term:`attribute` :codeterm:`derive`.

:def_p:`fls_caa16usjxryg`
The sole parameter of the :term:`macro implementation function` captures the
:term:`token` stream produced from the related :syntax:`EnumDeclaration`,
:syntax:`StructDeclaration`, or :syntax:`UnionDeclaration`.

:def_p:`fls_mobky5ck1mi`
A :def_term:`helper attribute` is an :term:`inert` :term:`attribute` that acts
as a hint to :term:`attribute` :codeterm:`derive`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro_derive(Answer)]
   pub fn derive_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

Attribute Macros
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_l3epi1dqpi8o`
An :term:`attribute macro` is a :term:`procedural macro` that consumes two
streams of :term:`[token]s` to produce a single stream of :term:`[token]s`, and
defines a new :term:`outer attribute` that can be attached to :term:`[item]s`.
:term:`[Attribute macro]s` are used to replace :term:`[item]s` with other
:term:`[item]s`.

:def_p:`fls_3sublbi9bz7k`
The :term:`macro implementation function` of an :term:`attribute macro` shall be
subject to the following restrictions:

* :def_p:`fls_eb8jxl70wmeh`
  The :term:`macro implementation function` shall be subject to
  :term:`attribute` :codeterm:`proc_macro_attribute`,

* :def_p:`fls_7ugtmobgb2t9`
  The :term:`macro implementation function` shall be subject to visibility
  modifier ``pub``,

* :def_p:`fls_y700oif45wum`
  The :term:`macro implementation function` shall lack :term:`[function
  qualifier]s`,

* :def_p:`fls_hhsf1a9p6o55`
  The :term:`macro implementation function` shall lack :term:`[generic
  parameter]s`,

* :def_p:`fls_4g932k8ueyqp`
  The :term:`macro implementation function` shall have two :term:`[function
  parameter]s` whose :term:`[type specification]s` indicate :term:`type`
  :codeterm:`proc_macro::TokenStream`,

* :def_p:`fls_f5qy1pnlbpng`
  The :term:`macro implementation function` shall have a
  :term:`return type` whose :term:`type specification` indicates type
  :codeterm:`proc_macro::TokenStream`.

:def_p:`fls_rzn48xylk4yj`
An :term:`attribute macro` is invoked using an :term:`attribute` of the form

* :def_p:`fls_78400zh02sdq`
  ``#[SimplePath]``, or

* :def_p:`fls_eyesmvuwpjn1`
  ``#[SimplePath DelimitedTokenTree]``

:def_p:`fls_fku5beu3mr4c`
The first :term:`function parameter` of the :term:`macro implementation
function` captures the :term:`token` stream produced from the
:syntax:`DelimitedTokenTree` of the invoking :term:`attribute`, excluding outer
:syntax:`[Delimiter]s`. If no :syntax:`DelimitedTokenTree` is provided, then the
:term:`token` stream is considered empty.

:def_p:`fls_knjsslplv5ri`
The second :term:`function parameter` of the :term:`macro implementation
function` captures the :term:`token` stream produced from the related
:term:`item`, including all :term:`[outer attribute]s` that apply to that
:term:`item`.

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

Macro Invocation
----------------

.. rubric:: Syntax

.. syntax::

   MacroInvocation ::=
       SimplePath $$!$$ DelimitedTokenTree

   DelimitedTokenTree ::=
       $$($$ TokenTree* $$)$$
     | $$[$$ TokenTree* $$]$$
     | $${$$ TokenTree* $$}$$

   TokenTree ::=
       DelimitedTokenTree
     | NonDelimitedToken

   TerminatedMacroInvocation ::=
       SimplePath $$!$$ $$($$ TokenTree* $$)$$ $$;$$
     | SimplePath $$!$$ $$[$$ TokenTree* $$]$$ $$;$$
     | SimplePath $$!$$ $${$$ TokenTree* $$}$$

:def_p:`fls_wushtmw9qt3y`
A :def_syntax:`NonDelimitedToken` is any :term:`lexical element` in category
:syntax:`LexicalElement`, except delimiters ``(``, ``)``, ``[``, ``]``, ``{``,
and ``}``.

.. rubric:: Legality Rules

:def_p:`fls_snpxxcqhtjfv`
A :term:`macro invocation` is a call of a :term:`declarative macro` or
:term:`function-like macro` that is expanded statically and replaced with the
result of the :term:`macro`.

:def_p:`fls_6v06zvi1ctub`
A :term:`terminated macro invocation` is a :term:`macro invocation` that may be
used as a :term:`statement`.

.. rubric:: Examples

:def_p:`fls_338rmbazl67o`
See :p:`20.1. <fls_yrq1n547uzp>` for the declaration of ``answer_to_life``.

.. code-block:: text

   answer_to_life!();


:def_p:`fls_lrr7gg8tian`
See :p:`20.1.1. <fls_mej9pty172v4>` for the declaration of ``square``.

.. code-block:: text


   square!(5);


:def_p:`fls_8qxwwf4trnl`
See :p:`20.1.2. <fls_b45ng0j84lli>` for the declaration of ``generate_pairs``.

.. code-block:: text


   generate_pairs!(1, 2, 3; 9, 8, 7);


:def_p:`fls_8z1sgtvchhhw`
See :p:`20.2.1. <fls_33w6tcb743j0>` for the declaration of
``make_answer_to_life``.

.. code-block:: text


   make_answer_to_life!();


:def_p:`fls_d9w3dn2yn7mo`
See :p:`20.2.2. <fls_uqp2svg2kntl>` for the declaration of ``Answer``.

.. code-block:: text


   #[derive(Answer)]
   struct derive_macro_invoker;


:def_p:`fls_1tftbd91yfpd`
See :p:`20.2.3. <fls_r5isidirsy03>` for the declaration of
``output_and_return_item``.

.. code-block:: text


   #[output_and_return_item]
   fn attribute_macro_invoker() {}

Macro Expansion
---------------

.. rubric:: Legality Rules

:def_p:`fls_xscdaxvs4wx4`
:term:`Macro expansion` is the process of statically executing a :term:`macro
invocation` and replacing it with the produced output of the :term:`macro
invocation`.

:def_p:`fls_nz5stwcc41gk`
:term:`Macro expansion` of :term:`[declarative macro]s` proceeds as follows:

#. :def_p:`fls_76prdp6k1fga`
   The :syntax:`TokenTree` of the :term:`macro invocation` is matched against
   the :term:`[macro rule]s` of the resolved :term:`macro` by considering
   individual :term:`[macro matcher]s`. It is a static error if no :term:`macro
   matcher` is satisfied.

#. :def_p:`fls_76u274l4kew8`
   The :term:`macro transcriber` of the satisfied :term:`macro rule` produces
   its result, with all :term:`[metavariable indication]s` resolved. It is a
   static error if the :term:`macro transcriber` fails to produce its result.

#. :def_p:`fls_lakpily1zwfl`
   The :term:`macro invocation` is replaced with the result of the :term:`macro
   transcriber`. It is a static error if the result cannot be parsed according
   to the expected expansion syntax of the context where the :term:`macro
   invocation` resides. The expected expansion syntax is as follows:

   #. :def_p:`fls_3zn4dz19nyvq`
      If the :term:`macro invocation` appears as part of a :term:`statement`,
      the output is required to constitute zero or more :term:`[statement]s`.

   #. :def_p:`fls_nsh2vwx8oiw`
      If the :term:`macro invocation` appears as part of an
      :term:`expression-without-block`, the output is required to constitute an
      :term:`expression`.

   #. :def_p:`fls_tu6kmwm4v9nj`
      If the :term:`macro invocation` appears as part of a
      :term:`pattern-without-range`, the output is required to constitute zero
      or more :term:`[pattern]s`.

   #. :def_p:`fls_y20pmwo3v3uu`
      If the :term:`macro invocation` appears as part of an :term:`associated
      item`, an :term:`external item`, or a :term:`macro item`, the output is
      required to constitute zero or more :term:`[item]s`.

   #. :def_p:`fls_t89sw6az99z7`
      If the :term:`macro invocation` appears as part of a
      :term:`type-specification-without-bounds`, the output is required to
      constitute a :term:`type`.

:def_p:`fls_417hvhvj2554`
Expansion of :term:`[function-like macro]s` proceeds as follows:

#. :def_p:`fls_srtqkdceaz5t`
   The :codeterm:`proc_macro::TokenStream` of the :term:`macro invocation`
   is passed to the sole :term:`function parameter` of the :term:`macro
   implementation function`. The :codeterm:`proc_macro::TokenStream` captures
   the :syntax:`DelimitedTokenTree` without the outer :syntax:`[Delimiter]s`.

#. :def_p:`fls_mi92etjtpamu`
   The :term:`macro implementation function` produces its
   :term:`output proc_macro::TokenStream`. It is a static error if the
   :term:`macro implementation function` fails to produce its output
   :codeterm:`proc_macro::TokenStream`.

#. :def_p:`fls_n8beqlt54rhy`
   The :term:`macro invocation` is replaced with the result of the :term:`macro
   transcriber`. It is a static error if the result can not be parsed according
   to the expected expansion syntax of the context where the :term:`macro
   invocation` resides. The expected expansion syntax is as follows:

   #. :def_p:`fls_stseor6tln22`
      If the :term:`macro invocation` appears as part of a :term:`statement`,
      the output is required to constitute zero or more :term:`[statement]s`.

   #. :def_p:`fls_l8j2jiuuao4f`
      If the :term:`macro invocation` appears as part of an
      :term:`expression-without-block`, the output is required to constitute an
      :term:`expression`.

   #. :def_p:`fls_xvemyqj5gc6g`
      If the :term:`macro invocation` appears as part of a
      :term:`pattern-without-range`, the output is required to constitute zero
      or more :term:`[pattern]s`.

   #. :def_p:`fls_vd3dzvr6re19`
      If the :term:`macro invocation` appears as part of an :term:`associated
      item`, an :term:`external item`, or a :term:`macro item`, the output is
      required to constitute zero or more :term:`[item]s`.

   #. :def_p:`fls_u11o90szy68s`
      If the :term:`macro invocation` appears as part of a
      :term:`type-specification-without-bounds`, the output is required to
      constitute a :term:`type`.

:def_p:`fls_qi5kyvj1e8th`
Expansion of :term:`[derive macro]s` proceeds as follows:

#. :def_p:`fls_grtiwf7q8jah`
   The :codeterm:`proc_macro::TokenStream` of the related :term:`item` is passed
   to the sole :term:`function parameter` of the :term:`macro implementation
   function`. The :codeterm:`proc_macro::TokenStream` captures the :term:`item`
   subject to the :term:`derive macro` excluding the invoking :term:`attribute`
   :codeterm:`derive` as well as any preceding :codeterm:`derive`
   :term:`[attribute]s`.

#. :def_p:`fls_tbe2qq7whq10`
   The :term:`macro implementation function` produces its output
   :codeterm:`proc_macro::TokenStream`. It is a static error if the
   :term:`macro implementation function` fails to produce its output
   :codeterm:`proc_macro::TokenStream`.

#. :def_p:`fls_my93neopj9x0`
   The output :codeterm:`proc_macro::TokenStream` is appended to the
   enclosing :term:`block expression` or :term:`module` where the
   related :syntax:`EnumDeclaration`, :syntax:`StructDeclaration`, or
   :syntax:`UnionDeclaration` resides. It is a static error if the output
   :codeterm:`proc_macro::TokenStream` does not constitute zero or more
   :term:`[item]s`.

:def_p:`fls_zat7kwi5vc5c`
The expansion of :term:`[attribute macro]s` proceeds as follows:

#. :def_p:`fls_tjn92evtlflq`
   The :codeterm:`proc_macro::TokenStream` of the invoking :term:`attribute`
   is passed to the first :term:`function parameter` of the :term:`macro
   implementation function`. The :codeterm:`proc_macro::TokenStream` captures
   the :syntax:`DelimitedTokenTree` without the outer :syntax:`[Delimiter]s`.
   If no :syntax:`DelimitedTokenTree` is provided, then an empty
   :codeterm:`proc_macro::TokenStream` is passed.

#. :def_p:`fls_mpgh22bi8caz`
   The :codeterm:`proc_macro::TokenStream` of the related :term:`item`
   is passed to the second :term:`function parameter` of the :term:`macro
   implementation function`. The :codeterm:`proc_macro::TokenStream` captures
   the :term:`item` subject to the invoking :term:`attribute`, excluding the
   invoking :term:`attribute`.

#. :def_p:`fls_ul7nhfyvyzh`
   The :term:`macro implementation function` produces its output
   :codeterm:`proc_macro::TokenStream`. It is a static error if the
   :term:`macro implementation function` fails to produce its output
   :codeterm:`proc_macro::TokenStream`.

#. :def_p:`fls_z6xfhf71w10a`
   The :term:`item` is replaced with the output
   :codeterm:`proc_macro::TokenStream`. It is a static error if the output
   :codeterm:`proc_macro::TokenStream` does not constitute zero or more
   :term:`[item]s`.

Macro Matching
~~~~~~~~~~~~~~

Rule Matching
^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_77ucvwu6idms`
:term:`Rule matching` is the process of consuming a :syntax:`TokenTree` in an
attempt to fully satisfy the :term:`macro matcher` of a :term:`macro rule` that
belongs to a resolved :term:`declarative macro`.

:def_p:`fls_6h1jqhxzku5v`
:term:`Rule matching` proceeds as follows:

#. :def_p:`fls_r6i1ykrhb49j`
   The :term:`[macro matcher]s` of all :term:`[macro rule]s` that belong to
   a resolved :term:`macro` are tried against the :syntax:`TokenTree` of the
   :term:`macro invocation`, in declarative order. In the event of a static
   error, no further attempts at selecting a subsequent :term:`macro matcher`
   are made.

#. :def_p:`fls_3qzes4lr8yuv`
   The :term:`macro match` of a candidate :term:`macro matcher` is tried
   against the :syntax:`TokenTree` of the :term:`macro invocation` by matching
   individual :term:`[token]s`, in left-to-right order.

#. :def_p:`fls_lrpxlag31r3e`
   Matching does not employ lookahead. It is a static error if matching a
   candidate :term:`macro matcher` is ambiguous.

#. :def_p:`fls_ksy2h7ixf9ha`
   Matching does not employ backtracking. It is a static error if
   matching a candidate :term:`macro matcher` fails while parsing into a
   :term:`metavariable` and having consumed at least one :term:`token` while
   parsing the :term:`metavariable`.

#. :def_p:`fls_r878ysvsy4jb`
   It is a static error if no :term:`macro matcher` is selected.

Token Matching
^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_k6a24sbon5v9`
:term:`Token matching` is the process of consuming a :syntax:`TokenTree` in
an attempt to fully satisfy a :term:`macro match` of a selected :term:`macro
matcher` that belongs to a resolved :term:`declarative macro`.

:def_p:`fls_6uuxv91xgmfz`
:term:`Token matching` proceeds as follows:

:def_p:`fls_g1rml9tavh8v`
The outer :syntax:`[Delimiter]s` of a macro matcher match any outer
``Delimiter``\ s in the :term:`macro invocation`.

:def_p:`fls_h7x3tc208zpk`
A :term:`metavariable` in a :term:`macro matcher` is matched against a sequence
of :term:`[token]s` in the :term:`macro invocation` based on its :term:`fragment
specifier`:

* :def_p:`fls_p9eqa17d3dx`
  :term:`Fragment specifier` **block** requires a :term:`block expression`.

* :def_p:`fls_k00bck2k8tde`
  :term:`Fragment specifier` **expr** requires an :term:`expression`.

* :def_p:`fls_pf0qrz5nadl2`
  :term:`Fragment specifier` **ident** requires a :term:`pure identifier`.

* :def_p:`fls_9fioah171ojx`
  :term:`Fragment specifier` **item** requires an :term:`item`.

* :def_p:`fls_j2o0f52zyvyb`
  :term:`Fragment specifier` **lifetime** requires character sequence 0x27
  0x5F (apostrophe, low line), or character 0x27 (apostrophe) followed by an
  :term:`identifier`.

* :def_p:`fls_w5dzv3z4zd5a`
  :term:`Fragment specifier` **literal** requires optional character 0x2D
  (hyphen-minus), followed by a :term:`literal expression`.

* :def_p:`fls_wtol98rrqka5`
  :term:`Fragment specifier` **meta** requires an :term:`attribute content`.

* :def_p:`fls_iorqt9q4ie9j`
  :term:`Fragment specifier` **pat** requires a
  :term:`pattern-without-alternation`.

* :def_p:`fls_2zjed913qpvi`
  :term:`Fragment specifier` **pat_param** is the same as :term:`fragment
  specifier` **pat**.

* :def_p:`fls_3zdts0fsa36u`
  :term:`Fragment specifier` **path** requires a :term:`type path`.

* :def_p:`fls_mb3yr1j7npv5`
  :term:`Fragment specifier` **stmt** requires a :term:`statement` without
  trailing character 0x3B (semicolon), excluding :term:`[item]s` that require
  character 0x3B (semicolon).

* :def_p:`fls_xbuixjt9pum6`
  :term:`Fragment specifier` **tt** requires a :syntax:`TokenTree`.

* :def_p:`fls_6annifhk6cd8`
  :term:`Fragment specifier` **ty** requires a :term:`type specification`.

* :def_p:`fls_2zu22efr6ncy`
  :term:`Fragment specifier` **vis** requires a possibly empty visibility
  modifier.

:def_p:`fls_dqroklsaayzb`
Once a :term:`metavariable` is matched, the matching sequence of
:term:`[token]s` is bound to that :term:`metavariable`.

:def_p:`fls_ghqjk6xj85ng`
Repetition in a :term:`macro matcher` is matched based on how many times
the :term:`pattern` appears consecutively optionally separated by a
:term:`separator` in the :syntax:`TokenTree` of the :term:`macro invocation`,
as follows:

* :def_p:`fls_lzwl4en5wcw0`
  If the repeated :term:`pattern` includes a :term:`separator`, then the
  :term:`separator` must be able to follow the repeated :term:`pattern`.

* :def_p:`fls_cz44evkjzv29`
  If the repeated :term:`pattern` can appear multiple times, then the repeated
  :term:`pattern` must be able to follow itself.

* :def_p:`fls_o2exsai4m0gy`
  If the repeated :term:`pattern` can appear zero times, then the preceding
  :term:`pattern` must be able to follow the succeeding :term:`pattern`.

* :def_p:`fls_1ch299zp8h7`
  The repeated :term:`pattern` must be able to follow the preceding
  :term:`pattern`.

* :def_p:`fls_55ptfjlvoo8o`
  The succeeding :term:`pattern` must be able to follow the repeated
  :term:`pattern`.

:def_p:`fls_finzfb5ljkf8`
A repetition index is a monotonically increasing number that is initialized to
zero, and incremented by one.

:def_p:`fls_s1ccs6jocsgr`
Once a metavariable is matched, the matching sequence of tokens is treated as
follows:

#. :def_p:`fls_wpi2i6hoj3li`
   The matching sequence of tokens is stored in an ordered collection at the
   current repetition index.

#. :def_p:`fls_uuey421a8n96`
   The current repetition index is incremented by one.

:def_p:`fls_b5u47tuu136r`
Each matched :term:`metavariable` in a :term:`macro repetition in matching` is
bound separately, where the matches are stored in an ordered collection.

:def_p:`fls_rb1tu4e7dpma`
Any other :term:`token` in a :term:`macro matcher` is matched literally against
the :syntax:`TokenTree` of the :term:`macro invocation`\ ``.``

:def_p:`fls_c76sdvos5xeo`
It is a static error if the :syntax:`TokenTree` of the :term:`macro invocation`
contains leftover :term:`[token]s` after :term:`macro matching`.

Macro Transcription
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_y21i8062mft0`
:term:`Macro transcription` is the process of producing the expansion of a
:term:`declarative macro`.

:def_p:`fls_n2dx4ug5nd5w`
:term:`Macro transcription` proceeds as follows:

:def_p:`fls_iw7322ycvhkc`
Every :term:`metavariable indication` found in the :syntax:`DelimitedTokenTree`
of the :term:`macro transcriber` that belongs to a matched :term:`macro
rule` is replaced by the matched sequence of :term:`[token]s` of the
:term:`metavariable`.

:def_p:`fls_jgitbqmyixem`
Unresolved :term:`[metavariable indication]s` are kept as :term:`[token]s` in
the output verbatim.

:def_p:`fls_ihcwl6taptas`
Every :term:`macro repetition in transcription` found in the
:syntax:`DelimitedTokenTree` of the :term:`macro transcriber` shall be
transcribed by repeatedly transcribing the :term:`[token]s` inside of it.

:def_p:`fls_g3dtpw4rtgdr`
The number of transcription repetitions for a :term:`macro repetition in
transcription` shall depend on its :term:`repetition operator`, as follows:

* :def_p:`fls_pvp6dxykuv66`
  A :term:`repetition operator` denoted by ``+`` shall require one or more
  repetitions.

* :def_p:`fls_bd673n5awwbz`
  A :term:`repetition operator` denoted by ``*`` shall require zero or more
  repetitions.

* :def_p:`fls_zbtwrtcy7pzf`
  A :term:`repetition operator` denoted by ``?`` shall require zero or one
  repetition.

:def_p:`fls_eacyb6jap9ru`
A :term:`metavariable indication` that is matched inside of a repetition shall
not be used outside of a :term:`macro repetition in transcription`.

:def_p:`fls_y4podc7ee8lf`
A :term:`metavariable indication` shall be used in a :term:`macro
repetition in transcription` of the same nesting depth as its corresponding
:term:`metavariable` appears in the :term:`macro matcher`.

:def_p:`fls_wbys0m4a1omg`
A :term:`metavariable indication` within a :term:`macro repetition in
transcription` shall repeat the same number of times in its matching
:term:`repetition` if the :term:`repetition` occurs at the same nesting depth.

:def_p:`fls_g445ovedgo4q`
Multiple transcribed :term:`[metavariable indication]s` in the same :term:`macro
repetition in transcription` shall repeat the same number of times.

:def_p:`fls_ctzthi6keit2`
When transcribing a metavariable indication in a macro repetition in
transcription, the metavariable indication is replaced with the matched
sequence of :term:`[token]s` of the corresponding iteration of the repetition.
metavariable taken from the ordered collection.

:def_p:`fls_vqc2lsa9dozk`
When transcribing a metavariable

:def_p:`fls_9n46ugmcqmix`
A metavariable indication in a macro repetition in transcription shall be
transcribed to the matched tokens in order,

:def_p:`fls_u2lq0lr12kdt`
macro_rules! foo {

:def_p:`fls_q0fmdb243bbj`
( $($expr:expr)* ) => {

:def_p:`fls_5ybrepv7esk8`
$( $expr ; )*

:def_p:`fls_2624w1db6ln3`
// $expr is an error

:def_p:`fls_717qmew9z4vs`
};

:def_p:`fls_azsyrzry1gxs`
| ( $( $( $expr:expr )*  )*  )  => {
| 		$($($expr)*)*
|             }

:def_p:`fls_aup3whtatvpi`
}

:def_p:`fls_bh3bl0tz392e`
foo! {

:def_p:`fls_2kh21hqfbf30`
0

:def_p:`fls_5xluznklusm1`
1

:def_p:`fls_gsct98unzlne`
2

:def_p:`fls_wcyzipq58fm2`
}

:def_p:`fls_x5oa26asdh9q`
0;1;2;

:def_p:`fls_xu5esg3v2u6i`
Given a repetition in a macro invocation of the form

:def_p:`fls_95rn4cvgznmd`
Given a macro invocation with N metavariable actuals, a macro of the form

.. code-block:: text

   macro_rules! m {
       ( $(param: expr)* ) => {
           $( $param )*
       }
   }

:def_p:`fls_yg4c9x7049y4`
is equivalent to a macro of the form

.. code-block:: text

   macro_rules! m {
       ( $param_1: expr $param_2: expr ... $param_N: expr) => {
           $param_1 $param_2 ... $param_N
       }
   }

:def_p:`fls_o9rwz9z0a2h4`
where the metavariable of the macro repetition in matching are repeated N times,
and the metavariable indications of the macro repetition in transcription are
repeated N times. Invoking such a macro relates the first metavariable actual
of the macro invocation with the first metavariable of the macro repetition in
matching, the second metavariable actual with the second metavariable, etc.

Hygiene
-------

:def_p:`fls_7ezc7ncs678f`
:def_term:`Hygiene` is a property of macros and identifiers that appear within
them, which aims to eliminate the syntactic interference between a macro and
its environment.

.. rubric:: Legality Rules

:def_p:`fls_3axjf28xb1nt`
Hygiene is categorized as follows:

* :def_p:`fls_dz2mvodl818d`
  *Definition site hygiene*, which resolves to the ``MacroDeclaration``
  site. ``Identifier``\ s with definition site hygiene cannot reference
  the environment of the ``MacroDeclaration``, cannot be referenced by the
  environment of a ``MacroInvocation``, and are considered *hygienic*.

* :def_p:`fls_puqhytfzfsg6`
  *Call site hygiene*, which resolves to the ``MacroInvocation`` site.
  ``Identifier``\ s with call site hygiene can reference the environment
  of the ``MacroDeclaration``, can reference the environment of the
  ``MacroInvocation``, and are considered *unhygienic*.

* :def_p:`fls_uyvnq88y9gk3`
  *Mixed hygiene*, which resolves to either the ``MacroDeclaration`` or the
  ``MacroInvocation`` site, depending on the ``Identifier``, and is considered
  *partially hygienic*.

:def_p:`fls_yxqcr19dig18`
Every macro has associated hygiene that depends on its kind:

* :def_p:`fls_kx25olky1jov`
  Declarative macros have definition site hygiene only for locally declared
  variables, ``Label``\ s, and the ``$crate`` metavariable, otherwise they have
  mixed hygiene.

* :def_p:`fls_v46v0t2vh6x4`
  Procedural macros have call site hygiene.

:def_p:`fls_7eqqk2cj0clr`
When a macro references items within its defining crate, the macro shall use the
``$crate`` metavariable to fully qualify all paths.

:def_p:`fls_d6g5g1b8k8v5`
**Are there other rules?**

