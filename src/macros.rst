.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Macros
======

.. rubric:: Legality Rules

:dp:`fls_j1jc83erljo0`
A :t:`macro` is a custom definition that extends Rust by defining callable
syntactic transformations. The effects of a :t:`macro` are realized through
:t:`[macro invocation]s` or :t:`attribute` use. :t:`[Macro]s` come in two
distinct forms:

* :dp:`fls_23eapx3ckymf`
  :t:`[Declarative macro]s` define rules for recognizing syntactic patterns and
  generating direct syntax.

* :dp:`fls_a5uemz2hnbi8`
  :t:`[Procedural macro]s` define augmented :t:`[function]s` that operate on and
  return a stream of :t:`[lexical element]s`.

:dp:`fls_rnty1c8l5495`
:t:`[Token]s` are a subset of :t:`[lexical element]s` consumed by :t:`[macro]s`.

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

:dp:`fls_ikzjsq8heyk6`
A :ds:`MacroMatchToken` is any :t:`lexical element` in category
:s:`LexicalElement`, except punctuation ``$`` and category :s:`Delimiter`.

.. rubric:: Legality Rules

:dp:`fls_w44hav7mw3ao`
A :t:`declarative macro` is a :t:`macro` that associates a :t:`name` with a set
of syntactic transformation :t:`[macro rule]s`.

:dp:`fls_dw1nq4r9ghhd`
A :t:`macro rule` is a :t:`construct` that consists of a :t:`macro matcher` and
a :t:`macro transcriber`.

:dp:`fls_oq4xn8guos8f`
A :t:`macro matcher` is a :t:`construct` that describes a syntactic pattern that
a :t:`macro` must match.

:dp:`fls_cdaf8viwmdfe`
A :t:`macro match` is the most basic form of a satisfied :t:`macro matcher`.

:dp:`fls_ljavs0w61z3j`
A :t:`macro transcriber` is a :t:`construct` that describes the replacement
syntax of a :t:`macro`.

:dp:`fls_3jspk8obv7sd`
A :t:`declarative macro` is invoked using a :t:`macro invocation`.

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

:dp:`fls_g93r3teei8wo`
:t:`[Declarative macro]s` employ :t:`[metavariable]s` to match a :t:`token` of
a particular kind and bind its :t:`value` to a name for use during :t:`macro
transcription`.

:dp:`fls_4zdait30exvn`
A :t:`metavariable` is a :t:`macro match` that describes a :t:`variable`.

:dp:`fls_8zypylq60zba`
A :t:`fragment specifier` is a :t:`construct` that indicates the :t:`type` of
a :t:`metavariable`.

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

:dp:`fls_4ps4x4513xau`
A :ds:`MacroRepetitionSeparator` is any :t:`lexical element` in category
:s:`LexicalElement`, except punctuation ``+``, ``*``, ``?``, and category
:s:`Delimiter`.

.. rubric:: Legality Rules

:dp:`fls_8byjmlgum2f3`
A :t:`macro repetition in matching` allows for a syntactic pattern to be matched
zero or multiple times during :t:`macro matching`.

:dp:`fls_ltdp3zs60dzr`
A :t:`macro repetition in transcription` allows for a syntactic pattern to be
transcribed zero or multiple times during :t:`macro transcription`.

:dp:`fls_u86j0zm2jshf`
A :t:`repetition operator` is a :t:`construct` that indicates the number
of times a :t:`macro repetition in matching` or a :t:`macro repetition in
transcription` can be repeated.

:dp:`fls_h5f8x4jdnvbu`
The effects of a :t:`repetition operator` are as follows:

* :dp:`fls_hf4gj5pfl437`
  ``*`` - Zero or more repetitions.

* :dp:`fls_tm0w0680wf4x`
  ``+`` - One or more repetitions.

* :dp:`fls_10lsg5212ffb`
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

:dp:`fls_ejbddhggstd2`
A :t:`procedural macro` is a :t:`macro` that encapsulates syntactic
transformations in a :t:`function`. :t:`[Procedural macro]s` consume one or more
streams of :t:`[token]s` and produce a stream of :t:`[token]s`.

:dp:`fls_pcce9gmjpxba`
:t:`[Procedural macro]s` shall be defined in a :t:`crate` subject to
:t:`attribute` :c:`crate_type` where the type is ``proc-macro``.

:dp:`fls_vtzuplb1p3s`
A :t:`macro implementation function` is the :t:`function` that encapsulates the
syntactic transformations of a :t:`procedural macro`.

:dp:`fls_mewfehvgm16r`
A :t:`macro implementation function` enters the :t:`name` of the :t:`procedural
macro` into the :t:`macro namespace`.

Function-like Macros
~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_utd3zqczix`
A :t:`function-like macro` is a :t:`procedural macro` that consumes a stream of
:t:`[token]s` and produces a stream of :t:`[token]s`.

:dp:`fls_ojr30lf6jfx0`
The :t:`macro implementation function` of a :t:`function-like macro` shall be
subject to the following restrictions:

* :dp:`fls_ljkjmegynhiy`
  The :t:`macro implementation function` shall be subject to :t:`attribute`
  :c:`proc_macro`,

* :dp:`fls_8a8qhzjw5hax`
  The :t:`macro implementation function` shall be subject to visibility modifier
  ``pub``,

* :dp:`fls_ofzql79i9if`
  The :t:`macro implementation function` shall lack :t:`[function qualifier]s`,

* :dp:`fls_j1wsyzip2qb3`
  The :t:`macro implementation function` shall lack :t:`[generic parameter]s`,

* :dp:`fls_etyo9bmzxby6`
  The :t:`macro implementation function` shall have a single :t:`function
  parameter` whose :t:`type specification` indicates :t:`type`
  :std:`proc_macro::TokenStream`,

* :dp:`fls_mkl9b38m0sf1`
  The :t:`macro implementation function` shall have a :t:`return type` whose
  :t:`type specification` indicates :t:`type` :std:`proc_macro::TokenStream`.

:dp:`fls_lfmb22bfnrye`
A :t:`function-like macro` is invoked using a :t:`macro invocation`.

:dp:`fls_fbgal48cgj44`
The sole parameter of the :t:`macro implementation function` captures the
:t:`token` stream produced from the :s:`DelimitedTokenTree` of the :t:`macro
invocation`, excluding outer :s:`[Delimiter]s`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro]
   pub fn make_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

Derive Macros
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_e5x92q2rq8a0`
A :t:`derive macro` is a :t:`procedural macro` that consumes a stream of
:t:`[token]s` and produces a stream of :t:`[token]s`. :t:`[Derive macro]s` are
used to construct new syntax for :t:`[abstract data type]s`.

:dp:`fls_ldw75sy5uj7p`
The :t:`macro implementation function` of a :t:`derive macro` shall be subject
to the following restrictions:

* :dp:`fls_7gcnui9beky`
  The :t:`macro implementation function` shall be subject to :t:`attribute`
  :c:`proc_macro_derive`,

* :dp:`fls_ef30ropg7dhx`
  The :t:`macro implementation function` shall be subject to visibility modifier
  ``pub``,

* :dp:`fls_mo00vqm9xfqc`
  The :t:`macro implementation function` shall lack :t:`[function qualifier]s`,

* :dp:`fls_gr9wugeqyb3b`
  The :t:`macro implementation function` shall lack :t:`[generic parameter]s`,

* :dp:`fls_npnze2cg8ae`
  The :t:`macro implementation function` shall have a single :t:`function
  parameter` whose :t:`type specification` indicates :t:`type`
  :std:`proc_macro::TokenStream`,

* :dp:`fls_w2h4lk6bmht`
  The :t:`macro implementation function` shall have a :t:`return type` whose
  :t:`type specification` indicates :t:`type` :std:`proc_macro::TokenStream`.

:dp:`fls_x96a0xzcyrko`
A :t:`derive macro` is invoked using :t:`attribute` :c:`derive`.

:dp:`fls_caa16usjxryg`
The sole parameter of the :t:`macro implementation function` captures
the :t:`token` stream produced from the related :s:`EnumDeclaration`,
:s:`StructDeclaration`, or :s:`UnionDeclaration`.

:dp:`fls_mobky5ck1mi`
A :dt:`helper attribute` is an :t:`inert` :t:`attribute` that acts as a hint to
:t:`attribute` :c:`derive`.

.. rubric:: Examples

.. code-block:: text

   #[proc_macro_derive(Answer)]
   pub fn derive_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

Attribute Macros
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_l3epi1dqpi8o`
An :t:`attribute macro` is a :t:`procedural macro` that consumes two streams
of :t:`[token]s` to produce a single stream of :t:`[token]s`, and defines a
new :t:`outer attribute` that can be attached to :t:`[item]s`. :t:`[Attribute
macro]s` are used to replace :t:`[item]s` with other :t:`[item]s`.

:dp:`fls_3sublbi9bz7k`
The :t:`macro implementation function` of an :t:`attribute macro` shall be
subject to the following restrictions:

* :dp:`fls_eb8jxl70wmeh`
  The :t:`macro implementation function` shall be subject to :t:`attribute`
  :c:`proc_macro_attribute`,

* :dp:`fls_7ugtmobgb2t9`
  The :t:`macro implementation function` shall be subject to visibility modifier
  ``pub``,

* :dp:`fls_y700oif45wum`
  The :t:`macro implementation function` shall lack :t:`[function qualifier]s`,

* :dp:`fls_hhsf1a9p6o55`
  The :t:`macro implementation function` shall lack :t:`[generic parameter]s`,

* :dp:`fls_4g932k8ueyqp`
  The :t:`macro implementation function` shall have two :t:`[function
  parameter]s` whose :t:`[type specification]s` indicate :t:`type`
  :std:`proc_macro::TokenStream`,

* :dp:`fls_f5qy1pnlbpng`
  The :t:`macro implementation function` shall have a :t:`return type` whose
  :t:`type specification` indicates type :std:`proc_macro::TokenStream`.

:dp:`fls_rzn48xylk4yj`
An :t:`attribute macro` is invoked using an :t:`attribute` of the form

* :dp:`fls_78400zh02sdq`
  ``#[SimplePath]``, or

* :dp:`fls_eyesmvuwpjn1`
  ``#[SimplePath DelimitedTokenTree]``

:dp:`fls_fku5beu3mr4c`
The first :t:`function parameter` of the :t:`macro implementation function`
captures the :t:`token` stream produced from the :s:`DelimitedTokenTree`
of the invoking :t:`attribute`, excluding outer :s:`[Delimiter]s`. If no
:s:`DelimitedTokenTree` is provided, then the :t:`token` stream is considered
empty.

:dp:`fls_knjsslplv5ri`
The second :t:`function parameter` of the :t:`macro implementation function`
captures the :t:`token` stream produced from the related :t:`item`, including
all :t:`[outer attribute]s` that apply to that :t:`item`.

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

:dp:`fls_wushtmw9qt3y`
A :ds:`NonDelimitedToken` is any :t:`lexical element` in category
:s:`LexicalElement`, except delimiters ``(``, ``)``, ``[``, ``]``, ``{``, and
``}``.

.. rubric:: Legality Rules

:dp:`fls_snpxxcqhtjfv`
A :t:`macro invocation` is a call of a :t:`declarative macro` or
:t:`function-like macro` that is expanded statically and replaced with the
result of the :t:`macro`.

:dp:`fls_6v06zvi1ctub`
A :t:`terminated macro invocation` is a :t:`macro invocation` that may be used
as a :t:`statement`.

.. rubric:: Examples

:dp:`fls_338rmbazl67o`
See :p:`20.1. <fls_yrq1n547uzp>` for the declaration of ``answer_to_life``.

.. code-block:: text

   answer_to_life!();

:dp:`fls_lrr7gg8tian`
See :p:`20.1.1. <fls_mej9pty172v4>` for the declaration of ``square``.

.. code-block:: text

   square!(5);

:dp:`fls_8qxwwf4trnl`
See :p:`20.1.2. <fls_b45ng0j84lli>` for the declaration of ``generate_pairs``.

.. code-block:: text

   generate_pairs!(1, 2, 3; 9, 8, 7);

:dp:`fls_8z1sgtvchhhw`
See :p:`20.2.1. <fls_33w6tcb743j0>` for the declaration of
``make_answer_to_life``.

.. code-block:: text

   make_answer_to_life!();

:dp:`fls_d9w3dn2yn7mo`
See :p:`20.2.2. <fls_uqp2svg2kntl>` for the declaration of ``Answer``.

.. code-block:: text

   #[derive(Answer)]
   struct derive_macro_invoker;

:dp:`fls_1tftbd91yfpd`
See :p:`20.2.3. <fls_r5isidirsy03>` for the declaration of
``output_and_return_item``.

.. code-block:: text

   #[output_and_return_item]
   fn attribute_macro_invoker() {}

Macro Expansion
---------------

.. rubric:: Legality Rules

:dp:`fls_xscdaxvs4wx4`
:t:`Macro expansion` is the process of statically executing a :t:`macro
invocation` and replacing it with the produced output of the :t:`macro
invocation`.

:dp:`fls_nz5stwcc41gk`
:t:`Macro expansion` of :t:`[declarative macro]s` proceeds as follows:

#. :dp:`fls_76prdp6k1fga`
   The :s:`TokenTree` of the :t:`macro invocation` is matched against the
   :t:`[macro rule]s` of the resolved :t:`macro` by considering individual
   :t:`[macro matcher]s`. It is a static error if no :t:`macro matcher` is
   satisfied.

#. :dp:`fls_76u274l4kew8`
   The :t:`macro transcriber` of the satisfied :t:`macro rule` produces its
   result, with all :t:`[metavariable indication]s` resolved. It is a static
   error if the :t:`macro transcriber` fails to produce its result.

#. :dp:`fls_lakpily1zwfl`
   The :t:`macro invocation` is replaced with the result of the :t:`macro
   transcriber`. It is a static error if the result cannot be parsed according
   to the expected expansion syntax of the context where the :t:`macro
   invocation` resides. The expected expansion syntax is as follows:

   #. :dp:`fls_3zn4dz19nyvq`
      If the :t:`macro invocation` appears as part of a :t:`statement`, the
      output is required to constitute zero or more :t:`[statement]s`.

   #. :dp:`fls_nsh2vwx8oiw`
      If the :t:`macro invocation` appears as part of an
      :t:`expression-without-block`, the output is required to constitute an
      :t:`expression`.

   #. :dp:`fls_tu6kmwm4v9nj`
      If the :t:`macro invocation` appears as part of a
      :t:`pattern-without-range`, the output is required to constitute zero or
      more :t:`[pattern]s`.

   #. :dp:`fls_y20pmwo3v3uu`
      If the :t:`macro invocation` appears as part of an :t:`associated item`,
      an :t:`external item`, or a :t:`macro item`, the output is required to
      constitute zero or more :t:`[item]s`.

   #. :dp:`fls_t89sw6az99z7`
      If the :t:`macro invocation` appears as part of a
      :t:`type-specification-without-bounds`, the output is required to
      constitute a :t:`type`.

:dp:`fls_417hvhvj2554`
Expansion of :t:`[function-like macro]s` proceeds as follows:

#. :dp:`fls_srtqkdceaz5t`
   The :std:`proc_macro::TokenStream` of the :t:`macro invocation` is passed to
   the sole :t:`function parameter` of the :t:`macro implementation function`.
   The :std:`proc_macro::TokenStream` captures the :s:`DelimitedTokenTree`
   without the outer :s:`[Delimiter]s`.

#. :dp:`fls_mi92etjtpamu`
   The :t:`macro implementation function` produces its :t:`output
   proc_macro::TokenStream`. It is a static error if the
   :t:`macro implementation function` fails to produce its output
   :std:`proc_macro::TokenStream`.

#. :dp:`fls_n8beqlt54rhy`
   The :t:`macro invocation` is replaced with the result of the :t:`macro
   transcriber`. It is a static error if the result can not be parsed according
   to the expected expansion syntax of the context where the :t:`macro
   invocation` resides. The expected expansion syntax is as follows:

   #. :dp:`fls_stseor6tln22`
      If the :t:`macro invocation` appears as part of a :t:`statement`, the
      output is required to constitute zero or more :t:`[statement]s`.

   #. :dp:`fls_l8j2jiuuao4f`
      If the :t:`macro invocation` appears as part of an
      :t:`expression-without-block`, the output is required to constitute an
      :t:`expression`.

   #. :dp:`fls_xvemyqj5gc6g`
      If the :t:`macro invocation` appears as part of a
      :t:`pattern-without-range`, the output is required to constitute zero or
      more :t:`[pattern]s`.

   #. :dp:`fls_vd3dzvr6re19`
      If the :t:`macro invocation` appears as part of an :t:`associated item`,
      an :t:`external item`, or a :t:`macro item`, the output is required to
      constitute zero or more :t:`[item]s`.

   #. :dp:`fls_u11o90szy68s`
      If the :t:`macro invocation` appears as part of a
      :t:`type-specification-without-bounds`, the output is required to
      constitute a :t:`type`.

:dp:`fls_qi5kyvj1e8th`
Expansion of :t:`[derive macro]s` proceeds as follows:

#. :dp:`fls_grtiwf7q8jah`
   The :std:`proc_macro::TokenStream` of the related :t:`item` is passed to
   the sole :t:`function parameter` of the :t:`macro implementation function`.
   The :std:`proc_macro::TokenStream` captures the :t:`item` subject to the
   :t:`derive macro` excluding the invoking :t:`attribute` :c:`derive` as well
   as any preceding :c:`derive` :t:`[attribute]s`.

#. :dp:`fls_tbe2qq7whq10`
   The :t:`macro implementation function` produces its output
   :std:`proc_macro::TokenStream`. It is a static error if the
   :t:`macro implementation function` fails to produce its output
   :std:`proc_macro::TokenStream`.

#. :dp:`fls_my93neopj9x0`
   The output :std:`proc_macro::TokenStream` is appended to the enclosing
   :t:`block expression` or :t:`module` where the related :s:`EnumDeclaration`,
   :s:`StructDeclaration`, or :s:`UnionDeclaration` resides. It is a static
   error if the output :std:`proc_macro::TokenStream` does not constitute zero
   or more :t:`[item]s`.

:dp:`fls_zat7kwi5vc5c`
The expansion of :t:`[attribute macro]s` proceeds as follows:

#. :dp:`fls_tjn92evtlflq`
   The :std:`proc_macro::TokenStream` of the invoking :t:`attribute`
   is passed to the first :t:`function parameter` of the :t:`macro
   implementation function`. The :std:`proc_macro::TokenStream` captures
   the :s:`DelimitedTokenTree` without the outer :s:`[Delimiter]s`.
   If no :s:`DelimitedTokenTree` is provided, then an empty
   :std:`proc_macro::TokenStream` is passed.

#. :dp:`fls_mpgh22bi8caz`
   The :std:`proc_macro::TokenStream` of the related :t:`item` is passed to the
   second :t:`function parameter` of the :t:`macro implementation function`. The
   :std:`proc_macro::TokenStream` captures the :t:`item` subject to the invoking
   :t:`attribute`, excluding the invoking :t:`attribute`.

#. :dp:`fls_ul7nhfyvyzh`
   The :t:`macro implementation function` produces its output
   :std:`proc_macro::TokenStream`. It is a static error if the
   :t:`macro implementation function` fails to produce its output
   :std:`proc_macro::TokenStream`.

#. :dp:`fls_z6xfhf71w10a`
   The :t:`item` is replaced with the output :std:`proc_macro::TokenStream`.
   It is a static error if the output :std:`proc_macro::TokenStream` does not
   constitute zero or more :t:`[item]s`.

Macro Matching
~~~~~~~~~~~~~~

Rule Matching
^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_77ucvwu6idms`
:t:`Rule matching` is the process of consuming a :s:`TokenTree` in an attempt
to fully satisfy the :t:`macro matcher` of a :t:`macro rule` that belongs to a
resolved :t:`declarative macro`.

:dp:`fls_6h1jqhxzku5v`
:t:`Rule matching` proceeds as follows:

#. :dp:`fls_r6i1ykrhb49j`
   The :t:`[macro matcher]s` of all :t:`[macro rule]s` that belong to a resolved
   :t:`macro` are tried against the :s:`TokenTree` of the :t:`macro invocation`,
   in declarative order. In the event of a static error, no further attempts at
   selecting a subsequent :t:`macro matcher` are made.

#. :dp:`fls_3qzes4lr8yuv`
   The :t:`macro match` of a candidate :t:`macro matcher` is tried against
   the :s:`TokenTree` of the :t:`macro invocation` by matching individual
   :t:`[token]s`, in left-to-right order.

#. :dp:`fls_lrpxlag31r3e`
   Matching does not employ lookahead. It is a static error if matching a
   candidate :t:`macro matcher` is ambiguous.

#. :dp:`fls_ksy2h7ixf9ha`
   Matching does not employ backtracking. It is a static error if matching a
   candidate :t:`macro matcher` fails while parsing into a :t:`metavariable` and
   having consumed at least one :t:`token` while parsing the :t:`metavariable`.

#. :dp:`fls_r878ysvsy4jb`
   It is a static error if no :t:`macro matcher` is selected.

Token Matching
^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_k6a24sbon5v9`
:t:`Token matching` is the process of consuming a :s:`TokenTree` in an attempt
to fully satisfy a :t:`macro match` of a selected :t:`macro matcher` that
belongs to a resolved :t:`declarative macro`.

:dp:`fls_6uuxv91xgmfz`
:t:`Token matching` proceeds as follows:

:dp:`fls_g1rml9tavh8v`
The outer :s:`[Delimiter]s` of a macro matcher match any outer ``Delimiter``\ s
in the :t:`macro invocation`.

:dp:`fls_h7x3tc208zpk`
A :t:`metavariable` in a :t:`macro matcher` is matched against a sequence of
:t:`[token]s` in the :t:`macro invocation` based on its :t:`fragment specifier`:

* :dp:`fls_p9eqa17d3dx`
  :t:`Fragment specifier` **block** requires a :t:`block expression`.

* :dp:`fls_k00bck2k8tde`
  :t:`Fragment specifier` **expr** requires an :t:`expression`.

* :dp:`fls_pf0qrz5nadl2`
  :t:`Fragment specifier` **ident** requires a :t:`pure identifier`.

* :dp:`fls_9fioah171ojx`
  :t:`Fragment specifier` **item** requires an :t:`item`.

* :dp:`fls_j2o0f52zyvyb`
  :t:`Fragment specifier` **lifetime** requires character sequence 0x27
  0x5F (apostrophe, low line), or character 0x27 (apostrophe) followed by an
  :t:`identifier`.

* :dp:`fls_w5dzv3z4zd5a`
  :t:`Fragment specifier` **literal** requires optional character 0x2D
  (hyphen-minus), followed by a :t:`literal expression`.

* :dp:`fls_wtol98rrqka5`
  :t:`Fragment specifier` **meta** requires an :t:`attribute content`.

* :dp:`fls_iorqt9q4ie9j`
  :t:`Fragment specifier` **pat** requires a :t:`pattern-without-alternation`.

* :dp:`fls_2zjed913qpvi`
  :t:`Fragment specifier` **pat_param** is the same as :t:`fragment specifier`
  **pat**.

* :dp:`fls_3zdts0fsa36u`
  :t:`Fragment specifier` **path** requires a :t:`type path`.

* :dp:`fls_mb3yr1j7npv5`
  :t:`Fragment specifier` **stmt** requires a :t:`statement` without trailing
  character 0x3B (semicolon), excluding :t:`[item]s` that require character
  0x3B (semicolon).

* :dp:`fls_xbuixjt9pum6`
  :t:`Fragment specifier` **tt** requires a :s:`TokenTree`.

* :dp:`fls_6annifhk6cd8`
  :t:`Fragment specifier` **ty** requires a :t:`type specification`.

* :dp:`fls_2zu22efr6ncy`
  :t:`Fragment specifier` **vis** requires a possibly empty visibility modifier.

:dp:`fls_dqroklsaayzb`
Once a :t:`metavariable` is matched, the matching sequence of :t:`[token]s` is
bound to that :t:`metavariable`.

:dp:`fls_ghqjk6xj85ng`
Repetition in a :t:`macro matcher` is matched based on how many times the
:t:`pattern` appears consecutively optionally separated by a :t:`separator` in
the :s:`TokenTree` of the :t:`macro invocation`, as follows:

* :dp:`fls_lzwl4en5wcw0`
  If the repeated :t:`pattern` includes a :t:`separator`, then the
  :t:`separator` must be able to follow the repeated :t:`pattern`.

* :dp:`fls_cz44evkjzv29`
  If the repeated :t:`pattern` can appear multiple times, then the repeated
  :t:`pattern` must be able to follow itself.

* :dp:`fls_o2exsai4m0gy`
  If the repeated :t:`pattern` can appear zero times, then the preceding
  :t:`pattern` must be able to follow the succeeding :t:`pattern`.

* :dp:`fls_1ch299zp8h7`
  The repeated :t:`pattern` must be able to follow the preceding :t:`pattern`.

* :dp:`fls_55ptfjlvoo8o`
  The succeeding :t:`pattern` must be able to follow the repeated :t:`pattern`.

:dp:`fls_finzfb5ljkf8`
A repetition index is a monotonically increasing number that is initialized to
zero, and incremented by one.

:dp:`fls_s1ccs6jocsgr`
Once a metavariable is matched, the matching sequence of tokens is treated as
follows:

#. :dp:`fls_wpi2i6hoj3li`
   The matching sequence of tokens is stored in an ordered collection at the
   current repetition index.

#. :dp:`fls_uuey421a8n96`
   The current repetition index is incremented by one.

:dp:`fls_b5u47tuu136r`
Each matched :t:`metavariable` in a :t:`macro repetition in matching` is bound
separately, where the matches are stored in an ordered collection.

:dp:`fls_rb1tu4e7dpma`
Any other :t:`token` in a :t:`macro matcher` is matched literally against the
:s:`TokenTree` of the :t:`macro invocation`\ ``.``

:dp:`fls_c76sdvos5xeo`
It is a static error if the :s:`TokenTree` of the :t:`macro invocation` contains
leftover :t:`[token]s` after :t:`macro matching`.

Macro Transcription
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_y21i8062mft0`
:t:`Macro transcription` is the process of producing the expansion of a
:t:`declarative macro`.

:dp:`fls_n2dx4ug5nd5w`
:t:`Macro transcription` proceeds as follows:

:dp:`fls_iw7322ycvhkc`
Every :t:`metavariable indication` found in the :s:`DelimitedTokenTree` of the
:t:`macro transcriber` that belongs to a matched :t:`macro rule` is replaced by
the matched sequence of :t:`[token]s` of the :t:`metavariable`.

:dp:`fls_jgitbqmyixem`
Unresolved :t:`[metavariable indication]s` are kept as :t:`[token]s` in the
output verbatim.

:dp:`fls_ihcwl6taptas`
Every :t:`macro repetition in transcription` found in the
:s:`DelimitedTokenTree` of the :t:`macro transcriber` shall be transcribed by
repeatedly transcribing the :t:`[token]s` inside of it.

:dp:`fls_g3dtpw4rtgdr`
The number of transcription repetitions for a :t:`macro repetition in
transcription` shall depend on its :t:`repetition operator`, as follows:

* :dp:`fls_pvp6dxykuv66`
  A :t:`repetition operator` denoted by ``+`` shall require one or more
  repetitions.

* :dp:`fls_bd673n5awwbz`
  A :t:`repetition operator` denoted by ``*`` shall require zero or more
  repetitions.

* :dp:`fls_zbtwrtcy7pzf`
  A :t:`repetition operator` denoted by ``?`` shall require zero or one
  repetition.

:dp:`fls_eacyb6jap9ru`
A :t:`metavariable indication` that is matched inside of a repetition shall not
be used outside of a :t:`macro repetition in transcription`.

:dp:`fls_y4podc7ee8lf`
A :t:`metavariable indication` shall be used in a :t:`macro repetition in
transcription` of the same nesting depth as its corresponding :t:`metavariable`
appears in the :t:`macro matcher`.

:dp:`fls_wbys0m4a1omg`
A :t:`metavariable indication` within a :t:`macro repetition in transcription`
shall repeat the same number of times in its matching :t:`repetition` if the
:t:`repetition` occurs at the same nesting depth.

:dp:`fls_g445ovedgo4q`
Multiple transcribed :t:`[metavariable indication]s` in the same :t:`macro
repetition in transcription` shall repeat the same number of times.

:dp:`fls_ctzthi6keit2`
When transcribing a metavariable indication in a macro repetition in
transcription, the metavariable indication is replaced with the matched sequence
of :t:`[token]s` of the corresponding iteration of the repetition. metavariable
taken from the ordered collection.

:dp:`fls_vqc2lsa9dozk`
When transcribing a metavariable

:dp:`fls_9n46ugmcqmix`
A metavariable indication in a macro repetition in transcription shall be
transcribed to the matched tokens in order,

:dp:`fls_u2lq0lr12kdt`
macro_rules! foo {

:dp:`fls_q0fmdb243bbj`
( $($expr:expr)* ) => {

:dp:`fls_5ybrepv7esk8`
$( $expr ; )*

:dp:`fls_2624w1db6ln3`
// $expr is an error

:dp:`fls_717qmew9z4vs`
};

:dp:`fls_azsyrzry1gxs`
| ( $( $( $expr:expr )*  )*  )  => {
| 		$($($expr)*)*
|             }

:dp:`fls_aup3whtatvpi`
}

:dp:`fls_bh3bl0tz392e`
foo! {

:dp:`fls_2kh21hqfbf30`
0

:dp:`fls_5xluznklusm1`
1

:dp:`fls_gsct98unzlne`
2

:dp:`fls_wcyzipq58fm2`
}

:dp:`fls_x5oa26asdh9q`
0;1;2;

:dp:`fls_xu5esg3v2u6i`
Given a repetition in a macro invocation of the form

:dp:`fls_95rn4cvgznmd`
Given a macro invocation with N metavariable actuals, a macro of the form

.. code-block:: text

   macro_rules! m {
       ( $(param: expr)* ) => {
           $( $param )*
       }
   }

:dp:`fls_yg4c9x7049y4`
is equivalent to a macro of the form

.. code-block:: text

   macro_rules! m {
       ( $param_1: expr $param_2: expr ... $param_N: expr) => {
           $param_1 $param_2 ... $param_N
       }
   }

:dp:`fls_o9rwz9z0a2h4`
where the metavariable of the macro repetition in matching are repeated N times,
and the metavariable indications of the macro repetition in transcription are
repeated N times. Invoking such a macro relates the first metavariable actual
of the macro invocation with the first metavariable of the macro repetition in
matching, the second metavariable actual with the second metavariable, etc.

Hygiene
-------

:dp:`fls_7ezc7ncs678f`
:dt:`Hygiene` is a property of macros and identifiers that appear within them,
which aims to eliminate the syntactic interference between a macro and its
environment.

.. rubric:: Legality Rules

:dp:`fls_3axjf28xb1nt`
Hygiene is categorized as follows:

* :dp:`fls_dz2mvodl818d`
  *Definition site hygiene*, which resolves to the ``MacroDeclaration``
  site. ``Identifier``\ s with definition site hygiene cannot reference
  the environment of the ``MacroDeclaration``, cannot be referenced by the
  environment of a ``MacroInvocation``, and are considered *hygienic*.

* :dp:`fls_puqhytfzfsg6`
  *Call site hygiene*, which resolves to the ``MacroInvocation`` site.
  ``Identifier``\ s with call site hygiene can reference the environment
  of the ``MacroDeclaration``, can reference the environment of the
  ``MacroInvocation``, and are considered *unhygienic*.

* :dp:`fls_uyvnq88y9gk3`
  *Mixed hygiene*, which resolves to either the ``MacroDeclaration`` or the
  ``MacroInvocation`` site, depending on the ``Identifier``, and is considered
  *partially hygienic*.

:dp:`fls_yxqcr19dig18`
Every macro has associated hygiene that depends on its kind:

* :dp:`fls_kx25olky1jov`
  Declarative macros have definition site hygiene only for locally declared
  variables, ``Label``\ s, and the ``$crate`` metavariable, otherwise they have
  mixed hygiene.

* :dp:`fls_v46v0t2vh6x4`
  Procedural macros have call site hygiene.

:dp:`fls_7eqqk2cj0clr`
When a macro references items within its defining crate, the macro shall use the
``$crate`` metavariable to fully qualify all paths.

:dp:`fls_d6g5g1b8k8v5`
**Are there other rules?**

