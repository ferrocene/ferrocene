.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

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

.. rubric:: Ada Reference Manual Copyright Notice

:dp:`fls_3i93ausw1tdl`
**Fourth Edition (Ada** 202x\ **)**

:dp:`fls_qdqemjcrgwcp`
Copyright © 2016, 2017, 2018, 2019, 2020, 2021 AXE Consultants. All Rights
Reserved.

:dp:`fls_ud7wbv6l65ar`
This document may be copied, in whole or in part, in any form or by any means,
as is, or with alterations, provided that (1) alterations are clearly marked as
alterations and (2) this copyright notice is included unmodified in any copy.
Any other use or distribution of this document is prohibited without the prior
express permission of AXE.

:dp:`fls_k2i3e6a5p6bp`
You use this document on the condition that you indemnify and hold harmless
AXE, its board, officers, agents, and employees, from any and all liability or
damages to yourself or your hardware or software, or third parties, including
attorneys' fees, court costs, and other related costs and expenses, arising out
of your use of this document irrespective of the cause of said liability.

:dp:`fls_p2b5pjhghw2t`
AXE MAKES THIS DOCUMENT AVAILABLE ON AN "AS IS" BASIS AND MAKES NO WARRANTY,
EXPRESS OR IMPLIED, AS TO THE ACCURACY, CAPABILITY, EFFICIENCY MERCHANTABILITY,
OR FUNCTIONING OF THIS DOCUMENT. IN NO EVENT WILL AXE BE LIABLE FOR ANY GENERAL,
CONSEQUENTIAL, INDIRECT, INCIDENTAL, EXEMPLARY, OR SPECIAL DAMAGES, EVEN IF AXE
HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.

.. rubric:: Rust Reference MIT License

:dp:`fls_wl1u8uwy8qds`
**Copyright (c) 2010 The Rust Project Developers**

:dp:`fls_56bclza1art0`
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

:dp:`fls_aw51e4hhiggp`
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

:dp:`fls_ksg0colbsz5q`
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

.. rubric:: Rust Reference Apache License

:dp:`fls_4wpem7yieo73`
Apache License

:dp:`fls_8tomcasa39df`
Version 2.0, January 2004

:dp:`fls_1zeggcpkiu0`
http://www.apache.org/licenses/

:dp:`fls_z03bs2anqugq`
TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

:dp:`fls_tj9qsx9ir20r`
1. Definitions.

:dp:`fls_ff7y4vyzf1d`
"License" shall mean the terms and conditions for use, reproduction, and
distribution as defined by Sections 1 through 9 of this document.

:dp:`fls_efmvdtytq0qw`
"Licensor" shall mean the copyright owner or entity authorized by the copyright
owner that is granting the License.

:dp:`fls_e5fc70m46o51`
"Legal Entity" shall mean the union of the acting entity and all other entities
that control, are controlled by, or are under common control with that entity.
For the purposes of this definition,  "control" means (i) the power, direct
or indirect, to cause the direction or management of such entity, whether by
contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the
outstanding shares, or (iii) beneficial ownership of such entity.

:dp:`fls_uf552qnxt3ia`
"You" (or "Your") shall mean an individual or Legal Entity exercising
permissions granted by this License.

:dp:`fls_ormucunpda4e`
"Source" form shall mean the preferred form for making modifications, including
but not limited to software source code, documentation source, and configuration
files.

:dp:`fls_pxpjv6qqblyz`
"Object" form shall mean any form resulting from mechanical transformation or
translation of a Source form, including but not limited to compiled object code,
generated documentation, and conversions to other media types.

:dp:`fls_q6ciuzw3bkz2`
"Work" shall mean the work of authorship, whether in Source or Object form, made
available under the License, as indicated by a copyright notice that is included
in or attached to the work (an example is provided in the Appendix below).

:dp:`fls_73cx0apabohl`
"Derivative Works" shall mean any work, whether in Source or Object form, that
is based on (or derived from) the Work and for which the editorial revisions,
annotations, elaborations, or other modifications represent, as a whole, an
original work of authorship. For the purposes of this License, Derivative Works
shall not include works that remain separable from, or merely link (or bind by
name) to the interfaces of, the Work and Derivative Works thereof.

:dp:`fls_aosjsqjc1qbi`
"Contribution" shall mean any work of authorship, including the original version
of the Work and any modifications or additions to that Work or Derivative
Works thereof, that is intentionally submitted to Licensor for inclusion in the
Work by the copyright owner or by an individual or Legal Entity authorized to
submit on behalf of the copyright owner. For the purposes of this definition,
"submitted" means any form of electronic, verbal, or written communication
sent to the Licensor or its representatives, including but not limited to
communication on electronic mailing lists, source code control systems, and
issue tracking systems that are managed by, or on behalf of, the Licensor for
the purpose of discussing and improving the Work, but excluding communication
that is conspicuously marked or otherwise designated in writing by the copyright
owner as "Not a Contribution."

:dp:`fls_2ge6yg6zddeh`
"Contributor" shall mean Licensor and any individual or Legal Entity on
behalf of whom a contribution has been received by Licensor and subsequently
incorporated within the Work.

:dp:`fls_8bgwwnuaeor7`
2. Grant of Copyright License. Subject to the terms and conditions of this
License, each Contributor hereby grants to You a perpetual, worldwide,
non-exclusive, no-charge, royalty-free, irrevocable copyright license to
reproduce, prepare Derivative Works of, publicly display, publicly perform,
sublicense, and distribute the Work and such Derivative Works in Source or
Object form.

:dp:`fls_c51h6bcb3g4d`
3. Grant of Patent License. Subject to the terms and conditions of this License,
each Contributor hereby grants to You a perpetual, worldwide, non-exclusive,
no-charge, royalty-free, irrevocable (except as stated in this section)
patent license to make, have made, use, offer to sell, sell, import, and
otherwise transfer the Work, where such license applies only to those patent
claims licensable by such Contributor that are necessarily infringed by their
Contribution(s) alone or by combination of their Contribution(s) with the
Work to which such Contribution(s) was submitted. If You institute patent
litigation against any entity (including a cross-claim or counterclaim in a
lawsuit) alleging that the Work or a Contribution incorporated within the Work
constitutes direct or contributory patent infringement, then any patent licenses
granted to You under this License for that Work shall terminate as of the date
such litigation is filed.

:dp:`fls_1xurgyi2bzs0`
4. Redistribution. You may reproduce and distribute copies of the Work or
Derivative Works thereof in any medium, with or without modifications, and in
Source or Object form, provided that You meet the following conditions:

:dp:`fls_ypuj1r9nn4d`
(a) You must give any other recipients of the Work or Derivative Works a copy of
this License; and

:dp:`fls_5wwlhv5ct1pu`
(b) You must cause any modified files to carry prominent notices stating that
You changed the files; and

:dp:`fls_9sh4slvd3xu0`
(c) You must retain, in the Source form of any Derivative Works that You
distribute, all copyright, patent, trademark, and attribution notices from the
Source form of the Work, excluding those notices that do not pertain to any part
of the Derivative Works; and

:dp:`fls_kzpj1wa8i0q4`
(d) If the Work includes a "NOTICE" text file as part of its distribution, then
any Derivative Works that You distribute must include a readable copy of the
attribution notices contained within such NOTICE file, excluding those notices
that do not pertain to any part of the Derivative Works, in at least one of
the following places: within a NOTICE text file distributed as part of the
Derivative Works; within the Source form or documentation, if provided along
with the Derivative Works; or, within a display generated by the Derivative
Works, if and wherever such third-party notices normally appear. The contents
of the NOTICE file are for informational purposes only and do not modify the
License. You may add Your own attribution notices within Derivative Works
that You distribute, alongside or as an addendum to the NOTICE text from the
Work, provided that such additional attribution notices cannot be construed as
modifying the License.

:dp:`fls_sp1gb8gcr1s9`
You may add Your own copyright statement to Your modifications and may provide
additional or different license terms and conditions for use, reproduction, or
distribution of Your modifications, or for any such Derivative Works as a whole,
provided Your use, reproduction, and distribution of the Work otherwise complies
with the conditions stated in this License.

:dp:`fls_dy3a79l0llrn`
5. Submission of Contributions. Unless You explicitly state otherwise, any
Contribution intentionally submitted for inclusion in the Work by You to the
Licensor shall be under the terms and conditions of this License, without any
additional terms or conditions. Notwithstanding the above, nothing herein shall
supersede or modify the terms of any separate license agreement you may have
executed with Licensor regarding such Contributions.

:dp:`fls_bx3si7i3cf1h`
6. Trademarks. This License does not grant permission to use the trade names,
trademarks, service marks, or product names of the Licensor, except as required
for reasonable and customary use in describing the origin of the Work and
reproducing the content of the NOTICE file.

:dp:`fls_biolm71uj0zc`
7. Disclaimer of Warranty. Unless required by applicable law or agreed to
in writing, Licensor provides the Work (and each Contributor provides its
Contributions) on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF
ANY KIND, either express or implied, including, without limitation, any
warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or
FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining
the appropriateness of using or redistributing the Work and assume any risks
associated with Your exercise of permissions under this License.

:dp:`fls_q8zc59e4k2jw`
8. Limitation of Liability. In no event and under no legal theory, whether
in tort (including negligence), contract, or otherwise, unless required by
applicable law (such as deliberate and grossly negligent acts) or agreed to
in writing, shall any Contributor be liable to You for damages, including any
direct, indirect, special, incidental, or consequential damages of any character
arising as a result of this License or out of the use or inability to use the
Work (including but not limited to damages for loss of goodwill, work stoppage,
computer failure or malfunction, or any and all other commercial damages or
losses), even if such Contributor has been advised of the possibility of such
damages.

:dp:`fls_7830kufkv0ie`
9. Accepting Warranty or Additional Liability. While redistributing the
Work or Derivative Works thereof, You may choose to offer, and charge a
fee for, acceptance of support, warranty, indemnity, or other liability
obligations and/or rights consistent with this License. However, in accepting
such obligations, You may act only on Your own behalf and on Your sole
responsibility, not on behalf of any other Contributor, and only if You agree to
indemnify, defend, and hold each Contributor harmless for any liability incurred
by, or claims asserted against, such Contributor by reason of your accepting any
such warranty or additional liability.

:dp:`fls_hekw14nk1xfo`
END OF TERMS AND CONDITIONS

:dp:`fls_9xjr4eha5w7n`
APPENDIX: How to apply the Apache License to your work.

:dp:`fls_abeqsyhlxee5`
To apply the Apache License to your work, attach the following boilerplate
notice, with the fields enclosed by brackets "[]" replaced with your own
identifying information. (Don't include the brackets!)  The text should
be enclosed in the appropriate comment syntax for the file format. We also
recommend that a file or class name and description of purpose be included on
the same "printed page" as the copyright notice for easier identification within
third-party archives.

:dp:`fls_y4k1itkpzabk`
Copyright [yyyy] [name of copyright owner]

:dp:`fls_txpmnwmmbuyf`
Licensed under the Apache License, Version 2.0 (the "License");

:dp:`fls_8v90wijm7dr1`
you may not use this file except in compliance with the License.

:dp:`fls_semi0wse3oj5`
You may obtain a copy of the License at

:dp:`fls_sgeku3615457`
http://www.apache.org/licenses/LICENSE-2.0

:dp:`fls_v9fscrw5fzgj`
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.

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

Structure
~~~~~~~~~

:dp:`fls_6lrqailxjb02`
This document contains 21 chapters, 4 appendices, and an index.

:dp:`fls_tys7ciqnp8bn`
The specification of the Rust language is separated into:

* :dp:`fls_3ubhkaheu8i1`
  Chapters 1 through 21,

* :dp:`fls_xw3grr2g5zgi`
  Appendix A: The Rust Prelude,

* :dp:`fls_3hu6x73g39yi`
  Appendix B: The Rust Core Library,

* :dp:`fls_h29so7l54rrl`
  `Appendix C: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :dp:`fls_rq8ejzuyi2ud`
  `Appendix D: Syntax Summary.
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

* :dp:`fls_7o7qh34bqahh`
  `Appendix C: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :dp:`fls_w0mgss6ic60w`
  `Appendix D: Syntax Summary.
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

Versioning
----------

:dp:`fls_l80e3kdwnldc`
Ferrocene is a qualified compiler and this is the accompanying language
specification for the qualified version of the compiler. This document will
be updated with each qualification to accurately reflect the behavior of
the compiler qualified under that version of Ferrocene. This specification
corresponds to Ferrocene 1.0.

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
The definitions of terms are available in `Appendix C: Glossary
<https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5RyI
/edit#bookmark=id.bc2qwbfibrcs>`_.

:dp:`fls_h2m244agxaxs`
A rule is a requirement imposed on the programmer, stated in normative language
such as "shall", "shall not", "must", "must not", except for text under
Implementation Requirements heading.

:dp:`fls_47svine904xk`
A fact is a requirement imposed on a conforming tool, stated in informative
language such as "is", "is not", "can", "cannot".

