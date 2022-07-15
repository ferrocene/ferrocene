.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_48qldfwwh493:

General
=======

:dp:`fls_c4ry0kgmvk9z`
This document is influenced by the `Ada Reference Manual
<http://www.ada-auth.org/standards/2xrm/html/RM-TTL.html>`_
Ada 202x edition, as well as the `Rust Language Reference
<https://doc.rust-lang.org/stable/reference/>`_, the `Rust
Guidebook <https://doc.rust-lang.org/book/>`_ and the `Rustonomicon
<https://doc.rust-lang.org/nomicon/>`_.

:dp:`fls_gxqbj0qoiaio`
Parts of these documents have been copied, in whole or in part, in particular
but not limited to:

* :dp:`fls_u8k9zr8da30`
  The outline and structure of the documents;

* :dp:`fls_8mt9iigoboba`
  The title, outline, organization, and numbering of chapters;

* :dp:`fls_suhf2g3fatfa`
  The structure, formality, wording, and numbering of paragraphs;

* :dp:`fls_jjr5kbn0wuco`
  The definitions and uses of terms;

* :dp:`fls_4dfcjyprkzbx`
  The categories of syntactic and semantic rules.

:dp:`fls_tq9jcv5ddvwn`
Consult `Appendix A: Licenses
<https://docs.google.com/document/d/1MXGXAwVo714GETP447NuFKaeqjxO4vMYLhLHTDwOTbc
/edit#bookmark=id.kd7fcvfrwks0>`_ for the relevant copyright notices and
licenses.

.. _fls_fo1c7pg2mw1:

Scope
-----

:dp:`fls_srdq4mota5pr`
This document specifies the form and meaning of programs written in the
programming language Rust, as implemented by the :c:`rustc` compiler shipped
with Ferrocene. It documents the current understanding for the purposes of
compiler validation. As such, given any doubt, it prefers documenting behavior
of :c:`rustc` as included in the associated Ferrocene release over claiming
correctness as a specification.

:dp:`fls_dv1qish8svc`
This document is made available for contribution and review as it is useful
outside of the Ferrocene effort and can be a place of shared understanding. It
is not intended as a discussion ground for language evolution. It is also not
indented as a document enabling conformance between compilers.

:dp:`fls_osh9tiwpnsn1`
Contribution and review is managed by the Ferrocene project developers.

.. _fls_10yukmkhl0ng:

Extent
~~~~~~

:dp:`fls_x78yd1sszydv`
This document specifies:

* :dp:`fls_9e032738udnb`
  The form of a program written in Rust;

* :dp:`fls_jk7scu5xs17z`
  The effect of translating and executing such a program;

* :dp:`fls_jiryupa5fxgf`
  The manner in which :t:`[module]s` and :t:`[crate]s` may be combined to form
  Rust programs;

* :dp:`fls_sph1a3sapinh`
  The language-defined libraries that a conforming tool is required to supply;

* :dp:`fls_7tm19jxtffc8`
  The violations that a conforming tool is required to detect, and the effect of
  attempting to translate or execute a program containing such violations;

* :dp:`fls_5pbrl8lhuth1`
  The violations that a conforming implementation is not required to detect.

:dp:`fls_o8fc3e53vp7g`
This document does not specify:

* :dp:`fls_rw0y5t13y6gs`
  The means by which a Rust program is transformed into object code executable
  by a processor;

* :dp:`fls_x7c3o621qj9z`
  The means by which translation or execution of Rust programs is invoked and
  the executing units are controlled;

* :dp:`fls_5y2b6yjcl1vz`
  The size or speed of the object code, or the relative execution speed of
  different language constructs;

* :dp:`fls_8dennhk2dha`
  The form or contents of any listings produced by a tool; in particular, the
  form or contents of error or warning messages;

* :dp:`fls_j2gs3hrbxtyx`
  The effect of undefined behavior;

* :dp:`fls_gy2c7vfwkd8j`
  The size of a program or program unit that will exceed the capacity of a
  conforming tool.

.. _fls_xscgklvg1wd2:

Structure
~~~~~~~~~

:dp:`fls_6lrqailxjb02`
This document contains 21 chapters, 4 appendices, and an index.

:dp:`fls_tys7ciqnp8bn`
The specification of the Rust language is separated into:

* :dp:`fls_3ubhkaheu8i1`
  Chapters 1 through 21,

* :dp:`fls_xw3grr2g5zgi`
  `Appendix A: Licenses
  <https://docs.google.com/document/d/1MXGXAwVo714GETP447NuFKaeqjxO4vMYLhLHTDwOT
  bc/edit#bookmark=id.kd7fcvfrwks0>`_,

* :dp:`fls_k6obg07c1i71`
  `Appendix B: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :dp:`fls_az3jhtedgvyi`
  `Appendix C: Syntax Summary.
  <https://docs.google.com/document/d/1TzjQ-n2NS0ZUzwg6VDmD7-kAjW7iGID7h4KEdbfro
  Dk/edit#bookmark=id.h61cd8uat4jc>`_

:dp:`fls_6srbinvnyd54`
The specification is normative, except for the material in each of the items
listed below, which is informative:

* :dp:`fls_ciixfg9jhv42`
  Text under an Examples heading.

* :dp:`fls_ej94lm2682kg`
  Each subchapter whose title starts with the word "Example" or "Examples".

:dp:`fls_xgk91jrbpyoc`
The following appendices are informative:

* :dp:`fls_enkvrkfqwyt8`
  `Appendix A: Licenses
  <https://docs.google.com/document/d/1MXGXAwVo714GETP447NuFKaeqjxO4vMYLhLHTDwOT
  bc/edit#bookmark=id.kd7fcvfrwks0>`_,

* :dp:`fls_yfyiaipc9omp`
  `Appendix B: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :dp:`fls_qt3pzt1xhoup`
  `Appendix C: Syntax Summary.
  <https://docs.google.com/document/d/1TzjQ-n2NS0ZUzwg6VDmD7-kAjW7iGID7h4KEdbfro
  Dk/edit#bookmark=id.h61cd8uat4jc>`_

:dp:`fls_jc4upf6685bw`
Each chapter is divided into subchapters that have a common structure. Each
chapter is divided into subchapters that have a common structure. Each chapter
and subchapter is then organized to include the following segments as is
relevant to the topic:

.. rubric:: Syntax

:dp:`fls_oxzjqxgejx9t`
The syntax representation of a :t:`construct`.

.. rubric:: Legality Rules

:dp:`fls_gmx688d6ek1o`
Compile-time rules and facts for each :t:`construct`. A :t:`construct` is legal
if it obeys all of the Legality Rules.

:dp:`fls_5zdjikp1jhc`
Legality Rules are verified after :t:`macro expansion` takes place.

.. rubric:: Dynamic Semantics

:dp:`fls_as5bhc5t285g`
Run-time effects of each :t:`construct`.

.. rubric:: Undefined Behavior

:dp:`fls_70qjvaqoz007`
Situations that result in unbounded errors.

.. rubric:: Implementation Requirements

:dp:`fls_o4rdsbc7u98`
Additional requirements for conforming tools.

.. rubric:: Examples

:dp:`fls_w8j575w2hmc8`
Examples illustrating the possible forms of a :t:`construct`. This material
is informative.

.. _fls_99b7xi1bkgih:

Conformity
~~~~~~~~~~

.. rubric:: Implementation Requirements

:dp:`fls_kdyqtnc6loam`
A conforming tool shall:

* :dp:`fls_ctwsz8sl7lbq`
  Translate and correctly execute legal programs written in Rust, provided that
  they are not so large as to exceed the capacity of the tool,

* :dp:`fls_bvpekhdaxctq`
  Identify all programs or program units that are so large as to exceed the
  capacity of the tool (or raise an appropriate exception at run time),

* :dp:`fls_kfs8gsd36d91`
  Identify all programs or program units that contain errors whose detection is
  required by this document,

* :dp:`fls_k5sozk8jhrmg`
  Supply all language-defined library units required by this document,

* :dp:`fls_nwx1fdq6b4mg`
  Contain no variations except those explicitly permitted by this document, or
  those that are impossible or impractical to avoid given the tool's execution
  environment,

* :dp:`fls_n3ypaile1a36`
  Specify all such variations in the manner prescribed by this document.

:dp:`fls_nnmx2qsu14ft`
The external effect of the execution of a Rust program is defined in terms of
its interactions with its external environment. The following are defined as
external interactions:

* :dp:`fls_gu3331rmv2ho`
  Any call on an foreign :t:`function`, including any :t:`[argument operand]s`
  passed to it;

* :dp:`fls_3iekobt8qqi`
  Any result returned or :t:`panic` propagated from a :t:`main function` or an
  exported :t:`function` to an external caller;

* :dp:`fls_qx9fxf4py0j0`
  The :t:`[value]s` of imported and exported :t:`[object]s` at the time of any
  other interaction with the external environment.

:dp:`fls_pl0fyjcwslqm`
A tool that conforms to this document shall produce for the execution of a given
Rust program a set of interactions with the external environment whose order and
timing are consistent with the definitions and requirements of this document for
the semantics of the given program.

:dp:`fls_lkdm0mdghppv`
A tool that conforms to this document shall support each capability required by
the language as specified.

:dp:`fls_d07x1mbhgpsd`
A tool that conforms to this document may provide additional :t:`[attribute]s`
as long as their names are not the same as the names of :t:`[built-in
attribute]s`.

.. _fls_79rl6ylmct07:

Method of Description and Syntax Notation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

:dp:`fls_mc4a28do6kcp`
The form of a Rust program is described by means of a context-free syntax
together with context-dependent requirements expressed by narrative rules.

:dp:`fls_ioyp4wux6skt`
The meaning of a Rust program is described by means of narrative rules defining
both the effects of each construct and the composition rules for constructs.

:dp:`fls_jsflt7691ye4`
The context-free syntax of Rust is described using a simple variant of the
Backus-Naur form. In particular:

* :dp:`fls_98fm7z04lq9`
  A ``monospaced`` font is used to denote Rust syntax.

* :dp:`fls_ceb5a8t6cakr`
  Words in PascalCase font are used to denote a syntactic category, for example:

.. syntax::

   FloatExponent

* :dp:`fls_pts29mb5ld68`
  Words in **bold** font are used to indicate literal words and :t:`[keyword]s`,
  for example:

.. syntax::

   $$crate$$
   $$proc_macro_derive$$
   $$Self$$
   $$tt$$

* :dp:`fls_gqjo5oh7vn3b`
  Characters in **bold** font are used to indicate literal characters and
  literal punctuation, for example:

.. syntax::

   $$1$$
   $$F$$
   $${$$
   $$&&$$
   $$>>=$$

* :dp:`fls_1dz634xp8xp5`
  A character preceded by ``\`` (bold reverse solidus) is used to denote an
  :t:`escaped character`, for example:

.. syntax::

   $$\t$$
   $$\\$$

* :dp:`fls_pp9vtjlyblrl`
  A prefix followed by ``?`` (question mark) is used to denote an optional
  prefix, for example:

.. syntax::

   CrateRenaming?

* :dp:`fls_6e2vd9fvhsmk`
  A prefix followed by ``*`` (asterisk) is used to denote zero or more
  repetitions of the prefix, for example:

.. syntax::

   OuterAttributeOrDoc*

* :dp:`fls_4onq0kkrt6qv`
  A prefix followed by ``+`` (plus sign) is used to denote one or more
  repetitions of the prefix, for example:

.. syntax::

   MacroMatch+

* :dp:`fls_qu4rsmnq659w`
  A prefix followed by ``L-H`` is used to denote the number of repetitions of
  the prefix within the range from L to H, inclusive. For example:

.. syntax::

   HexadecimalDigit1-6

* :dp:`fls_rllu7aksf17e`
  ``[ ]`` (square brackets) indicate any character within, for example:

.. syntax::

   [$$8$$ $$a$$ $$\r$$ $$:$$]

* :dp:`fls_blvsfqeevosr`
  ``~[ ]`` (square brackets preceded by tilde) indicate any character except the
  characters within, for example:

.. syntax::

   ~[$$8$$ $$a$$ $$\r$$ $$:$$]

* :dp:`fls_lwcjq3wzjyvb`
  ``[ - ]`` indicates any character within the specified range, inclusive. For
  example:

.. syntax::

   [$$a$$-$$f$$]

* :dp:`fls_v7wd5yk00im6`
  A ``|`` (vertical line) separates alternative items, for example:

.. syntax::

   $$self$$ | Identifier | $$_$$

* :dp:`fls_nf8alga8uz6c`
  ``( )`` (parentheses) are used to group items, for example:

.. syntax::

   ($$,$$ ConfigurationPredicate)

:dp:`fls_u5ryccs9cpex`
Whenever the run-time semantics define certain actions to happen in an arbitrary
order, this means that a tool arranges for these actions to occur in a way that
is equivalent to some sequential order, following the rules that result from
that sequential order. This can happen, for example, if two parameters of a
given call expression have side effects.

.. _fls_9cd746qe40ag:

Versioning
----------

:dp:`fls_l80e3kdwnldc`
Ferrocene is a qualified compiler and this is the accompanying language
specification for the qualified version of the compiler. This document will
be updated with each qualification to accurately reflect the behavior of the
compiler qualified under that version of Ferrocene.

.. _fls_ijzgf4h0mp3c:

Definitions
-----------

:dp:`fls_sm2kexes5pr7`
Terms are defined throughout this document, indicated by *italic* type. Terms
explicitly defined in this document are not to be presumed to refer implicitly
to similar terms defined elsewhere.

:dp:`fls_2o98zw29xc46`
Mathematical terms not defined in this document are to be interpreted according
to the CRC Concise Encyclopedia of Mathematics, Second Edition.

:dp:`fls_lon5qffd65fi`
Other terms not defined in this document are to be interpreted according to the
Webster's Third New International Dictionary of the English Language.

:dp:`fls_qeolgxvcy75`
The definitions of terms are available in `Appendix B: Glossary
<https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5RyI
/edit#bookmark=id.bc2qwbfibrcs>`_.

:dp:`fls_h2m244agxaxs`
A rule is a requirement imposed on the programmer, stated in normative language
such as "shall", "shall not", "must", "must not", except for text under
Implementation Requirements heading.

:dp:`fls_47svine904xk`
A fact is a requirement imposed on a conforming tool, stated in informative
language such as "is", "is not", "can", "cannot".

