.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_83182bfa9uqb:

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

.. _fls_xa7lp0zg1ol2:

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

.. code-block:: rust

   macro_rules! answer_to_life {
       () => { 42 };
   }

.. _fls_8nzypdu9j3ge:

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
a particular kind and bind its :t:`value` to a name for use during
:t:`macro transcription`.

:dp:`fls_4zdait30exvn`
A :t:`metavariable` is a :t:`macro match` that describes a :t:`variable`.

:dp:`fls_2HguXbL7DjKH`
A :t:`metavariable` is visible in the :t:`macro transcriber` of the
:t:`macro rule` of the :t:`macro matcher` it is declared in.

:dp:`fls_8zypylq60zba`
A :t:`fragment specifier` is a :t:`construct` that indicates the :t:`type` of
a :t:`metavariable`.

:dp:`fls_8o9mcV2KrKac`
:t:`Fragment specifier` kinds impose the following
:dt:`[fragment specifier restriction]s` on the :t:`[token]s` that follow them:

* :dp:`fls_PxR9vNHsaFnI`
  ``expr`` shall only be followed by ``=>``, ``,``, or ``;``.

* :dp:`fls_ePyoTeJJ11N0`
  ``pat`` shall only be followed by ``=>``, ``,``, ``=``, ``|``, ``if``, or
  ``in``.

* :dp:`fls_0j7VOV4ewfeY`
  ``path`` shall only be followed by ``=>``, ``,``, ``=``, ``|``, ``;``, ``:``,
  ``>``, ``>>``, ``[``, ``{``, ``as``, ``where``, or a :t:`metavariable` with
  the ``block`` :t:`fragment specifier` kind.

* :dp:`fls_80cOMpIMU2gx`
  ``pat_param`` shall only be followed by ``=>``, ``,``, ``=``, ``|``, ``if``,
  or ``in``.

* :dp:`fls_DFMRwsWI8e5z`
  ``stmt`` shall only be followed by ``=>``, ``,``, or ``;``.

* :dp:`fls_BoIGgrFdyhwH`
  ``ty`` shall only be followed by ``=>``, ``,``, ``=``, ``|``, ``;``, ``:``,
  ``>``, ``>>``, ``[``, ``{``, ``as``, ``where``, or a :t:`metavariable` with
  the ``block`` :t:`fragment specifier` kind.

* :dp:`fls_NBbygZwUxjFp`
  ``vis`` shall only be followed by ``,``, an :t:`identifier` except for
  ``priv``, any token that may begin a :s:`TypeSpecification`, or a
  :t:`metavariable` with the ``ident``, ``ty`` or ``block``
  :t:`fragment specifier` kind.

* :dp:`fls_lZ8F1zUJju33`
  Any other kind may be followed by any token.

:dp:`fls_ephlmLsGTMgw`
A :t:`metavariable indication` is a :t:`construct` that indicates a
:t:`metavariable`.

.. rubric:: Examples

.. code-block:: rust

   macro_rules! square {
       ($e:expr) => { $e * $e };
   }

.. _fls_k01lsksqtq1r:

Repetition
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   MacroRepetitionMatch ::=
       $$$$$ $$($$ MacroRepetitionMatchContent $$)$$ MacroRepetitionSeparator? MacroRepetitionOperator

   MacroRepetitionMatchContent ::=
       MacroMatch*

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

:dp:`fls_V1WRuzZUWUGj`
A :t:`macro repetition` is either a :t:`macro repetition in matching` or a
:t:`macro repetition in transcription`.

:dp:`fls_u86j0zm2jshf`
A :t:`repetition operator` is a :t:`construct` that indicates the number
of times a :t:`macro repetition in matching` or a
:t:`macro repetition in transcription` can be repeated.

:dp:`fls_h5f8x4jdnvbu`
The effects of a :t:`repetition operator` are as follows:

* :dp:`fls_hf4gj5pfl437`
  ``*`` - Zero or more repetitions.

* :dp:`fls_tm0w0680wf4x`
  ``+`` - One or more repetitions.

* :dp:`fls_10lsg5212ffb`
  ``?`` - Zero or one repetition.

:dp:`fls_UnfvR9NB1Nze`
A :t:`macro repetition` has the following additional restrictions:

* :dp:`fls_Sm4qVsHKYLY2`
  If the :t:`macro repetition` has a :t:`separator`, the :t:`separator` shall
  be allowed by the :s:`MacroRepetitionMatchContent`'s
  :t:`[fragment specifier restriction]s`.

* :dp:`fls_Rdvs8Dz6OUU7`
  If the :t:`repetition operator` is ``*`` or ``+``, then the
  possible beginnings of the :s:`MacroRepetitionMatchContent` shall be allowed
  by its :s:`MacroRepetitionMatchContent`'s
  :t:`[fragment specifier restriction]s`.

* :dp:`fls_UIlj6Csow81w`
  If the :t:`repetition operator` is ``?`` or ``*``, then the succeeding
  :s:`MacroMatch` must be allowed by the preceding :s:`MacroMatch`'s
  :t:`[fragment specifier restriction]s`.

* :dp:`fls_yp2XxDv4DzEi`
  The possible beginnings of the :s:`MacroRepetitionMatchContent` must be
  allowed by the preceding :s:`MacroMatch`'s
  :t:`[fragment specifier restriction]s`.

* :dp:`fls_n5TkJKWiDhCD`
  The succeeding :s:`MacroMatch` must be allowed by the possible endings of the
  :s:`MacroRepetitionMatchContent`'s :t:`[fragment specifier restriction]s`.

.. rubric:: Examples

.. code-block:: rust

   macro_rules! generate_pairs {
       ( $( $first:ident )* ; $( &second:ident )* )
           =>
       { $( $first, $second )* };
   }

.. _fls_wn1i6hzg2ff7:

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
A :t:`macro implementation function` enters the :t:`name` of the
:t:`procedural macro` into the :t:`macro namespace`.

.. _fls_2d6bqnpy6tvs:

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
  It shall be subject to :t:`attribute` :c:`proc_macro`,

* :dp:`fls_8a8qhzjw5hax`
  It shall be subject to :t:`visibility modifier` ``pub``,

* :dp:`fls_ofzql79i9if`
  It shall lack :t:`[function qualifier]s`,

* :dp:`fls_j1wsyzip2qb3`
  It shall lack :t:`[generic parameter]s`,

* :dp:`fls_etyo9bmzxby6`
  It shall have a single :t:`function parameter` whose :t:`type specification`
  indicates :t:`type` :std:`proc_macro::TokenStream`,

* :dp:`fls_mkl9b38m0sf1`
  It shall have a :t:`return type` whose :t:`type specification` indicates
  :t:`type` :std:`proc_macro::TokenStream`.

:dp:`fls_lfmb22bfnrye`
A :t:`function-like macro` is invoked using a :t:`macro invocation`.

:dp:`fls_fbgal48cgj44`
The sole parameter of the :t:`macro implementation function` captures the
:t:`token` stream produced from the :s:`DelimitedTokenTree` of the
:t:`macro invocation`, excluding outer :s:`[Delimiter]s`.

.. rubric:: Examples

.. code-block:: rust

   #[proc_macro]
   pub fn make_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

.. _fls_o8s3r7m90q59:

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
  It shall be subject to :t:`attribute` :c:`proc_macro_derive`,

* :dp:`fls_ef30ropg7dhx`
  It shall be subject to :t:`visibility modifier` ``pub``,

* :dp:`fls_mo00vqm9xfqc`
  It shall lack :t:`[function qualifier]s`,

* :dp:`fls_gr9wugeqyb3b`
  It shall lack :t:`[generic parameter]s`,

* :dp:`fls_npnze2cg8ae`
  It shall have a single :t:`function parameter` whose :t:`type specification`
  indicates :t:`type` :std:`proc_macro::TokenStream`,

* :dp:`fls_w2h4lk6bmht`
  It shall have a :t:`return type` whose :t:`type specification` indicates
  :t:`type` :std:`proc_macro::TokenStream`.

:dp:`fls_x96a0xzcyrko`
A :t:`derive macro` is invoked using :t:`attribute` :c:`derive`.

:dp:`fls_caa16usjxryg`
The sole parameter of the :t:`macro implementation function` captures
the :t:`token` stream produced from the related :s:`EnumDeclaration`,
:s:`StructDeclaration`, or :s:`UnionDeclaration`.

:dp:`fls_H5ipqqlH3pJh`
A :t:`derive macro` adds all its declared :t:`[derive helper attribute]s` into
the :t:`derive helper attribute` scope of the :t:`abstract data type` the
:t:`attribute` is attached to.

:dp:`fls_mobky5ck1mi`
A :dt:`derive helper attribute` is an :t:`inert attribute` that acts as a
hint to :t:`attribute` :c:`derive`.

.. rubric:: Examples

.. code-block:: rust

   #[proc_macro_derive(Answer)]
   pub fn derive_answer_to_life(_items: TokenStream) -> TokenStream {
       "fn answer_to_life() -> u32 { 42 }".parse().unwrap()
   }

.. _fls_4vjbkm4ceymk:

Attribute Macros
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_l3epi1dqpi8o`
An :t:`attribute macro` is a :t:`procedural macro` that consumes two streams
of :t:`[token]s` to produce a single stream of :t:`[token]s`, and defines a
new :t:`outer attribute` that can be attached to :t:`[item]s`.
:t:`[Attribute macro]s` are used to replace :t:`[item]s` with other
:t:`[item]s`.

:dp:`fls_3sublbi9bz7k`
The :t:`macro implementation function` of an :t:`attribute macro` shall be
subject to the following restrictions:

* :dp:`fls_eb8jxl70wmeh`
  It shall be subject to :t:`attribute` :c:`proc_macro_attribute`,

* :dp:`fls_7ugtmobgb2t9`
  It shall be subject to :t:`visibility modifier` ``pub``,

* :dp:`fls_y700oif45wum`
  It shall lack :t:`[function qualifier]s`,

* :dp:`fls_hhsf1a9p6o55`
  It shall lack :t:`[generic parameter]s`,

* :dp:`fls_4g932k8ueyqp`
  It shall have two :t:`[function parameter]s` whose :t:`[type specification]s`
  indicate :t:`type` :std:`proc_macro::TokenStream`,

* :dp:`fls_f5qy1pnlbpng`
  It shall have a :t:`return type` whose :t:`type specification` indicates type
  :std:`proc_macro::TokenStream`.

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

.. code-block:: rust

   #[proc_macro_attribute]
   pub fn output_and_return_item
       (attr: TokenStream, item: TokenStream) -> TokenStream
   {
       println!("attr: \"{}\"", attr.to_string());
       println!("item: \"{}\"", item.to_string());
       item
   }

.. _fls_vnvt40pa48n8:

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
:s:`LexicalElement`, except category :s:`Delimiter`.

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

.. code-block:: rust

   answer_to_life!();

:dp:`fls_lrr7gg8tian`
See :p:`20.1.1. <fls_mej9pty172v4>` for the declaration of ``square``.

.. code-block:: rust

   square!(5);

:dp:`fls_8qxwwf4trnl`
See :p:`20.1.2. <fls_b45ng0j84lli>` for the declaration of ``generate_pairs``.

.. code-block:: rust

   generate_pairs!(1, 2, 3; 9, 8, 7);

:dp:`fls_8z1sgtvchhhw`
See :p:`20.2.1. <fls_33w6tcb743j0>` for the declaration of
``make_answer_to_life``.

.. code-block:: rust

   make_answer_to_life!();

:dp:`fls_d9w3dn2yn7mo`
See :p:`20.2.2. <fls_uqp2svg2kntl>` for the declaration of ``Answer``.

.. code-block:: rust

   #[derive(Answer)]
   struct derive_macro_invoker;

:dp:`fls_1tftbd91yfpd`
See :p:`20.2.3. <fls_r5isidirsy03>` for the declaration of
``output_and_return_item``.

.. code-block:: rust

   #[output_and_return_item]
   fn attribute_macro_invoker() {}

.. _fls_wjldgtio5o75:

Macro Expansion
---------------

.. rubric:: Legality Rules

:dp:`fls_xscdaxvs4wx4`
:t:`Macro expansion` is the process of statically executing a
:t:`macro invocation` and replacing it with the produced output of the
:t:`macro invocation`.

:dp:`fls_nz5stwcc41gk`
:t:`Macro expansion` of :t:`[declarative macro]s` proceeds as follows:

#. :dp:`fls_40xq8Ri1OMZZ`
   The :s:`TokenTree` of the :t:`macro invocation` has all
   :t:`[outer block doc]s` and :t:`[outer line doc]s` contained within replaced
   by their equivalent :t:`attribute` :c:`doc` representation.

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
   The :t:`macro invocation` is replaced with the result of the
   :t:`macro transcriber`. It is a static error if the result cannot be parsed
   according to the expected expansion syntax of the context where the
   :t:`macro invocation` resides. The expected expansion syntax is as follows:

   #. :dp:`fls_y20pmwo3v3uu`
      If the :t:`macro invocation` appears as part of an :t:`associated item`,
      an :t:`item` within an :t:`external block`, or another
      :t:`macro invocation`, the output is required to constitute zero or more
      :t:`[item]s`.

   #. :dp:`fls_nsh2vwx8oiw`
      If the :t:`macro invocation` appears as part of an
      :t:`expression-without-block`, the output is required to constitute an
      :t:`expression`.

   #. :dp:`fls_tu6kmwm4v9nj`
      If the :t:`macro invocation` appears as part of a
      :t:`pattern-without-range`, the output is required to constitute a
      :t:`[pattern]`.

   #. :dp:`fls_3zn4dz19nyvq`
      If the :t:`macro invocation` appears as part of a :t:`statement`, the
      output is required to constitute zero or more :t:`[statement]s`.

   #. :dp:`fls_t89sw6az99z7`
      If the :t:`macro invocation` appears as part of a
      :t:`type specification` without :t:`[bound]s`, the output is required to
      constitute a :t:`type`.

:dp:`fls_417hvhvj2554`
:t:`Macro expansion` of :t:`[function-like macro]s` proceeds as follows:

#. :dp:`fls_nNrs4EC3ff5T`
   The :s:`TokenTree` of the :t:`macro invocation` has all :t:`[outer block
   doc]s` and :t:`[outer line doc]s` contained within replaced by their
   equivalent :t:`attribute` :c:`doc` representation.

#. :dp:`fls_srtqkdceaz5t`
   The :s:`TokenTree` of the :t:`macro invocation` is transformed into a
   corresponding :std:`proc_macro::TokenStream`.

#. :dp:`fls_mi92etjtpamu`
   The :t:`macro implementation function` is called with the
   :std:`proc_macro::TokenStream` as its sole argument. It is a static error
   if the :t:`macro implementation function` call fails.

#. :dp:`fls_n8beqlt54rhy`
   The :t:`macro invocation` is replaced with the returned
   :std:`proc_macro::TokenStream` of the :t:`macro implementation function`
   call. It is a static error if the result can not be parsed according
   to the expected expansion syntax of the context where the :t:`macro
   invocation` resides. The expected expansion syntax is as follows:

   #. :dp:`fls_vd3dzvr6re19`
      If the :t:`macro invocation` appears as part of an :t:`associated item`,
      an :t:`item` within an :t:`external block`, or another
      :t:`macro invocation`, the output is required to constitute zero or more
      :t:`[item]s`.

   #. :dp:`fls_l8j2jiuuao4f`
      If the :t:`macro invocation` appears as part of an
      :t:`expression-without-block`, the output is required to constitute an
      :t:`expression`.

   #. :dp:`fls_xvemyqj5gc6g`
      If the :t:`macro invocation` appears as part of a
      :t:`pattern-without-range`, the output is required to constitute zero or
      more :t:`[pattern]s`.

   #. :dp:`fls_stseor6tln22`
      If the :t:`macro invocation` appears as part of a :t:`statement`, the
      output is required to constitute zero or more :t:`[statement]s`.

   #. :dp:`fls_u11o90szy68s`
      If the :t:`macro invocation` appears as part of a
      :t:`type specification` without :t:`[bound]s`, the output is required to
      constitute a :t:`type`.

:dp:`fls_qi5kyvj1e8th`
:t:`Macro expansion` of :t:`[derive macro]s` proceeds as follows:

#. :dp:`fls_vqIZaEl4EKu5`
   The :t:`item` subject to the :t:`derive macro` has all
   :t:`[outer block doc]s` and :t:`[outer line doc]s` contained within replaced
   by their equivalent :t:`attribute` :c:`doc` representation.

#. :dp:`fls_grtiwf7q8jah`
   The :t:`item` subject to the :t:`derive macro` is transformed into a
   corresponding :std:`proc_macro::TokenStream` without the
   invoking :c:`derive` :t:`attribute` as well as any preceding :c:`derive`
   :t:`[attribute]s`.

#. :dp:`fls_tbe2qq7whq10`
   The :t:`macro implementation function` is called with the
   :std:`proc_macro::TokenStream` as its sole argument. It is a static error
   if the :t:`macro implementation function` call fails.

#. :dp:`fls_my93neopj9x0`
   The returned :std:`proc_macro::TokenStream` of the
   :t:`macro implementation function` call is appended to the enclosing
   :t:`block expression` or :t:`module` where the related :s:`EnumDeclaration`,
   :s:`StructDeclaration`, or :s:`UnionDeclaration` resides. It is a static
   error if the output :std:`proc_macro::TokenStream` does not constitute zero
   or more :t:`[item]s`.

:dp:`fls_zat7kwi5vc5c`
:t:`Macro expansion` of :t:`[attribute macro]s` proceeds as follows:

#. :dp:`fls_tjn92evtlflq`
   The :s:`DelimitedTokenTree` of the invoking :t:`attribute macro` is
   transformed into a corresponding :std:`proc_macro::TokenStream` without
   the outer :s:`[Delimiter]s`. If no :s:`DelimitedTokenTree` is provided,
   and empty :std:`proc_macro::TokenStream` is used. This
   :std:`proc_macro::TokenStream` constitutes the first :t:`function parameter`
   of the :t:`macro implementation function`.

#. :dp:`fls_AJmPrhHfZo6J`
   The :t:`item` subject to the :t:`attribute macro` has all
   :t:`[outer block doc]s` and :t:`[outer line doc]s` contained within replaced
   by their equivalent :t:`attribute` :c:`doc` representation.

#. :dp:`fls_mpgh22bi8caz`
   The :t:`item` subject to the :t:`attribute macro` is transformed into a
   corresponding :std:`proc_macro::TokenStream` without the invoking
   :t:`attribute`. This :std:`proc_macro::TokenStream` constitutes the second
   :t:`function parameter` of the :t:`macro implementation function`.

#. :dp:`fls_ul7nhfyvyzh`
   The :t:`macro implementation function` is called with the two
   :std:`[proc_macro::TokenStream]s` as the two arguments. It is a static error
   if the :t:`macro implementation function` call fails.

#. :dp:`fls_z6xfhf71w10a`
   The :t:`item` subject to the :t:`attribute macro` is replaced with the
   returned :std:`proc_macro::TokenStream` of the
   :t:`macro implementation function` call. It is a static error if the output
   :std:`proc_macro::TokenStream` does not constitute zero or more :t:`[item]s`.

.. _fls_4apk1exafxii:

Macro Matching
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_ZmQZ8HQWv77L`
:t:`Macro matching` is the process of performing :t:`rule matching` and
:t:`token matching`.

.. _fls_n3ktmjqf87qb:

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
   :t:`[token]s`, in left-to-right order. Matching does not employ lookahead.
   It is a static error if matching a candidate :t:`macro matcher` is ambiguous.
   Matching does not employ backtracking. It is a static error if matching a
   candidate :t:`macro matcher` fails while parsing into a :t:`metavariable` and
   having consumed at least one :t:`token` while parsing the :t:`metavariable`.

#. :dp:`fls_r878ysvsy4jb`
   It is a static error if no :t:`macro matcher` is selected.

.. _fls_qpx6lgapce57:

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
The outer :s:`[Delimiter]s` of a :t:`macro matcher` match any outer
:s:`[Delimiter]s` in the :t:`macro invocation`.

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
  :t:`Fragment specifier` **pat** requires a :t:`pattern`.

* :dp:`fls_2zjed913qpvi`
  :t:`Fragment specifier` **pat_param** requires a
  :t:`pattern-without-alternation`.

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
A :dt:`repetition index` is a monotonically increasing number that is
initialized to zero, and incremented by one.

:dp:`fls_s1ccs6jocsgr`
Once a :t:`metavariable` is matched, the matching sequence of :t:`[token]s` is
treated as follows:

#. :dp:`fls_wpi2i6hoj3li`
   The matching sequence of :t:`[token]s` is stored in an ordered collection at
   the current :t:`repetition index`.

#. :dp:`fls_uuey421a8n96`
   The current :t:`repetition index` is incremented by one.

:dp:`fls_b5u47tuu136r`
Each matched :t:`metavariable` in a :t:`macro repetition in matching` is bound
separately, where the matches are stored in an ordered collection.

:dp:`fls_rb1tu4e7dpma`
Any other :t:`token` in a :t:`macro matcher` is matched literally against the
:s:`TokenTree` of the :t:`macro invocation`.

:dp:`fls_c76sdvos5xeo`
It is a static error if the :s:`TokenTree` of the :t:`macro invocation` contains
leftover :t:`[token]s` after :t:`macro matching`.

.. _fls_ym00b6ewf4n3:

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
The number of transcription repetitions for a
:t:`macro repetition in transcription` shall depend on its
:t:`repetition operator`, as follows:

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
A :t:`metavariable indication` that is matched inside of a
:t:`macro repetition` shall not be used outside of a
:t:`macro repetition in transcription`.

:dp:`fls_y4podc7ee8lf`
A :t:`metavariable indication` shall be used in a
:t:`macro repetition in transcription` of the same nesting depth as its
corresponding :t:`metavariable` appears in the :t:`macro matcher`.

:dp:`fls_wbys0m4a1omg`
A :t:`metavariable indication` within a :t:`macro repetition in transcription`
shall repeat the same number of times in its matching :t:`macro repetition` if
the :t:`macro repetition` occurs at the same nesting depth.

:dp:`fls_g445ovedgo4q`
Multiple transcribed :t:`[metavariable indication]s` in the same :t:`macro
repetition in transcription` shall repeat the same number of times.

:dp:`fls_ctzthi6keit2`
When transcribing a :t:`metavariable indication` in a
:t:`macro repetition in transcription`, the :t:`metavariable indication` is
replaced with the matched sequence of :t:`[token]s` of the corresponding
iteration of the repetition :t:`metavariable` taken from the ordered collection.

:dp:`fls_9n46ugmcqmix`
A :t:`metavariable indication` in a :t:`macro repetition in transcription` shall
be transcribed to the matched :t:`[token]s` in order, as follows:

.. code-block:: rust

   macro_rules! foo {
       ( $($expr:expr)* ) => {
           $( $expr ; )*
           // $expr is an error
       };
       ( $( $( $expr:expr )*  )*  ) => {
           $($($expr)*)*
       }
   }

   foo! {
     0
     1
     2
   }

:dp:`fls_JinrPA0pMZCr`
yields ``0;1;2;``

:dp:`fls_95rn4cvgznmd`
Given a :t:`macro invocation` with ``N`` :t:`metavariable` arguments, a
:t:`macro` of the form

.. code-block:: rust

   macro_rules! m {
       ( $(param: expr)* ) => {
           $( $param )*
       }
   }

:dp:`fls_yg4c9x7049y4`
is equivalent to a :t:`macro` of the form

.. code-block:: rust

   macro_rules! m {
       ( $param_1: expr $param_2: expr ... $param_N: expr) => {
           $param_1 $param_2 ... $param_N
       }
   }

:dp:`fls_o9rwz9z0a2h4`
where the :t:`metavariable` of the :t:`macro repetition in matching` are
repeated ``N`` times, and the :t:`[metavariable indication]s` of the
:t:`macro repetition in transcription` are repeated ``N`` times. Invoking such
a :t:`macro` relates the first :t:`metavariable` argument of the
:t:`macro invocation` with the first :t:`metavariable` of the
:t:`macro repetition in matching`, the second :t:`metavariable` argument with
the second :t:`metavariable`, and so on.

.. _fls_xlfo7di0gsqz:

Hygiene
-------

:dp:`fls_7ezc7ncs678f`
:t:`Hygiene` is a property of :t:`[macro]s` and :t:`[identifier]s` that appear
within them, which aims to eliminate the syntactic interference between a
:t:`macro` and its environment.

.. rubric:: Legality Rules

:dp:`fls_3axjf28xb1nt`
:t:`Hygiene` is categorized as follows:

* :dp:`fls_dz2mvodl818d`
  :t:`Definition site hygiene`, which resolves to a :s:`MacroRulesDeclaration`
  site. :t:`[Identifier]s` with :t:`definition site hygiene` cannot reference
  the environment of the :s:`MacroRulesDeclaration`, cannot be referenced by the
  environment of a :s:`MacroInvocation`, and are considered :t:`hygienic`.

* :dp:`fls_puqhytfzfsg6`
  :t:`Call site hygiene`, which resolves to a :s:`MacroInvocation` site.
  :t:`[Identifier]s` with :t:`call site hygiene` can reference the environment
  of the :s:`MacroRulesDeclaration`, can reference the environment of the
  :s:`MacroInvocation`, and are considered :t:`unhygienic`.

* :dp:`fls_uyvnq88y9gk3`
  :t:`Mixed site hygiene`, which resolves to a :s:`MacroRulesDeclaration`
  site for :t:`[label]s`, :t:`[variable]s`, and the ``$crate``
  :t:`metavariable`, and to the :s:`MacroInvocation` site otherwise, and is
  considered :dt:`partially hygienic`.

:dp:`fls_yxqcr19dig18`
Every :t:`macro` has associated :t:`hygiene` that depends on its kind:

* :dp:`fls_kx25olky1jov`
  :t:`[Declarative macro]s` have :t:`mixed site hygiene`.

* :dp:`fls_v46v0t2vh6x4`
  :t:`[Procedural macro]s` have :t:`call site hygiene` and
  :t:`mixed site hygiene` depending on the implementation of the
  :t:`procedural macro`.

:dp:`fls_7eqqk2cj0clr`
The :t:`metavariable` ``$crate`` in a :t:`declarative macro`'s expansion refers
to the crate the :t:`declarative macro` was declared in.

