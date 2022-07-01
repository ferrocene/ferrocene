.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

General
=======

:def_p:`fls_c4ry0kgmvk9z`
This document is influenced by the `Ada Reference Manual
<http://www.ada-auth.org/standards/2xrm/html/RM-TTL.html>`_
Ada 202x edition, as well as the `Rust Language Reference
<https://doc.rust-lang.org/stable/reference/>`_, the `Rust
Guidebook <https://doc.rust-lang.org/book/>`_ and the `Rustonomicon
<https://doc.rust-lang.org/nomicon/>`_.

:def_p:`fls_gxqbj0qoiaio`
Parts of these documents have been copied, in whole or in part, in particular
but not limited to:

* :def_p:`fls_u8k9zr8da30`
  The outline and structure of the documents;

* :def_p:`fls_8mt9iigoboba`
  The title, outline, organization, and numbering of chapters;

* :def_p:`fls_suhf2g3fatfa`
  The structure, formality, wording, and numbering of paragraphs;

* :def_p:`fls_jjr5kbn0wuco`
  The definitions and uses of terms;

* :def_p:`fls_4dfcjyprkzbx`
  The categories of syntactic and semantic rules.

.. rubric:: Ada Reference Manual Copyright Notice

:def_p:`fls_3i93ausw1tdl`
**Fourth Edition (Ada **\ 202x\ **)**

:def_p:`fls_qdqemjcrgwcp`
Copyright © 2016, 2017, 2018, 2019, 2020, 2021 AXE Consultants. All Rights
Reserved.

:def_p:`fls_ud7wbv6l65ar`
This document may be copied, in whole or in part, in any form or by any means,
as is, or with alterations, provided that (1) alterations are clearly marked as
alterations and (2) this copyright notice is included unmodified in any copy.
Any other use or distribution of this document is prohibited without the prior
express permission of AXE.

:def_p:`fls_k2i3e6a5p6bp`
You use this document on the condition that you indemnify and hold harmless
AXE, its board, officers, agents, and employees, from any and all liability or
damages to yourself or your hardware or software, or third parties, including
attorneys' fees, court costs, and other related costs and expenses, arising out
of your use of this document irrespective of the cause of said liability.

:def_p:`fls_p2b5pjhghw2t`
AXE MAKES THIS DOCUMENT AVAILABLE ON AN "AS IS" BASIS AND MAKES NO WARRANTY,
EXPRESS OR IMPLIED, AS TO THE ACCURACY, CAPABILITY, EFFICIENCY MERCHANTABILITY,
OR FUNCTIONING OF THIS DOCUMENT. IN NO EVENT WILL AXE BE LIABLE FOR ANY GENERAL,
CONSEQUENTIAL, INDIRECT, INCIDENTAL, EXEMPLARY, OR SPECIAL DAMAGES, EVEN IF AXE
HAS BEEN ADVISED OF THE POSSIBILITY OF SUCH DAMAGES.

.. rubric:: Rust Reference MIT License

:def_p:`fls_wl1u8uwy8qds`
**Copyright (c) 2010 The Rust Project Developers**

:def_p:`fls_56bclza1art0`
Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights to
use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of
the Software, and to permit persons to whom the Software is furnished to do so,
subject to the following conditions:

:def_p:`fls_aw51e4hhiggp`
The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

:def_p:`fls_ksg0colbsz5q`
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS
FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS
OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY,
WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN
CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

.. rubric:: Rust Reference Apache License

:def_p:`fls_4wpem7yieo73`
Apache License

:def_p:`fls_8tomcasa39df`
Version 2.0, January 2004

:def_p:`fls_1zeggcpkiu0`
http://www.apache.org/licenses/

:def_p:`fls_z03bs2anqugq`
TERMS AND CONDITIONS FOR USE, REPRODUCTION, AND DISTRIBUTION

:def_p:`fls_tj9qsx9ir20r`
1. Definitions.

:def_p:`fls_ff7y4vyzf1d`
"License" shall mean the terms and conditions for use, reproduction, and
distribution as defined by Sections 1 through 9 of this document.

:def_p:`fls_efmvdtytq0qw`
"Licensor" shall mean the copyright owner or entity authorized by the copyright
owner that is granting the License.

:def_p:`fls_e5fc70m46o51`
"Legal Entity" shall mean the union of the acting entity and all other entities
that control, are controlled by, or are under common control with that entity.
For the purposes of this definition,  "control" means (i) the power, direct
or indirect, to cause the direction or management of such entity, whether by
contract or otherwise, or (ii) ownership of fifty percent (50%) or more of the
outstanding shares, or (iii) beneficial ownership of such entity.

:def_p:`fls_uf552qnxt3ia`
"You" (or "Your") shall mean an individual or Legal Entity exercising
permissions granted by this License.

:def_p:`fls_ormucunpda4e`
"Source" form shall mean the preferred form for making modifications, including
but not limited to software source code, documentation source, and configuration
files.

:def_p:`fls_pxpjv6qqblyz`
"Object" form shall mean any form resulting from mechanical transformation or
translation of a Source form, including but not limited to compiled object code,
generated documentation, and conversions to other media types.

:def_p:`fls_q6ciuzw3bkz2`
"Work" shall mean the work of authorship, whether in Source or Object form, made
available under the License, as indicated by a copyright notice that is included
in or attached to the work (an example is provided in the Appendix below).

:def_p:`fls_73cx0apabohl`
"Derivative Works" shall mean any work, whether in Source or Object form, that
is based on (or derived from) the Work and for which the editorial revisions,
annotations, elaborations, or other modifications represent, as a whole, an
original work of authorship. For the purposes of this License, Derivative Works
shall not include works that remain separable from, or merely link (or bind by
name) to the interfaces of, the Work and Derivative Works thereof.

:def_p:`fls_aosjsqjc1qbi`
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

:def_p:`fls_2ge6yg6zddeh`
"Contributor" shall mean Licensor and any individual or Legal Entity on
behalf of whom a contribution has been received by Licensor and subsequently
incorporated within the Work.

:def_p:`fls_8bgwwnuaeor7`
2. Grant of Copyright License. Subject to the terms and conditions of this
License, each Contributor hereby grants to You a perpetual, worldwide,
non-exclusive, no-charge, royalty-free, irrevocable copyright license to
reproduce, prepare Derivative Works of, publicly display, publicly perform,
sublicense, and distribute the Work and such Derivative Works in Source or
Object form.

:def_p:`fls_c51h6bcb3g4d`
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

:def_p:`fls_1xurgyi2bzs0`
4. Redistribution. You may reproduce and distribute copies of the Work or
Derivative Works thereof in any medium, with or without modifications, and in
Source or Object form, provided that You meet the following conditions:

:def_p:`fls_ypuj1r9nn4d`
(a) You must give any other recipients of the Work or Derivative Works a copy of
this License; and

:def_p:`fls_5wwlhv5ct1pu`
(b) You must cause any modified files to carry prominent notices stating that
You changed the files; and

:def_p:`fls_9sh4slvd3xu0`
(c) You must retain, in the Source form of any Derivative Works that You
distribute, all copyright, patent, trademark, and attribution notices from the
Source form of the Work, excluding those notices that do not pertain to any part
of the Derivative Works; and

:def_p:`fls_kzpj1wa8i0q4`
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

:def_p:`fls_sp1gb8gcr1s9`
You may add Your own copyright statement to Your modifications and may provide
additional or different license terms and conditions for use, reproduction, or
distribution of Your modifications, or for any such Derivative Works as a whole,
provided Your use, reproduction, and distribution of the Work otherwise complies
with the conditions stated in this License.

:def_p:`fls_dy3a79l0llrn`
5. Submission of Contributions. Unless You explicitly state otherwise, any
Contribution intentionally submitted for inclusion in the Work by You to the
Licensor shall be under the terms and conditions of this License, without any
additional terms or conditions. Notwithstanding the above, nothing herein shall
supersede or modify the terms of any separate license agreement you may have
executed with Licensor regarding such Contributions.

:def_p:`fls_bx3si7i3cf1h`
6. Trademarks. This License does not grant permission to use the trade names,
trademarks, service marks, or product names of the Licensor, except as required
for reasonable and customary use in describing the origin of the Work and
reproducing the content of the NOTICE file.

:def_p:`fls_biolm71uj0zc`
7. Disclaimer of Warranty. Unless required by applicable law or agreed to
in writing, Licensor provides the Work (and each Contributor provides its
Contributions) on an "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF
ANY KIND, either express or implied, including, without limitation, any
warranties or conditions of TITLE, NON-INFRINGEMENT, MERCHANTABILITY, or
FITNESS FOR A PARTICULAR PURPOSE. You are solely responsible for determining
the appropriateness of using or redistributing the Work and assume any risks
associated with Your exercise of permissions under this License.

:def_p:`fls_q8zc59e4k2jw`
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

:def_p:`fls_7830kufkv0ie`
9. Accepting Warranty or Additional Liability. While redistributing the
Work or Derivative Works thereof, You may choose to offer, and charge a
fee for, acceptance of support, warranty, indemnity, or other liability
obligations and/or rights consistent with this License. However, in accepting
such obligations, You may act only on Your own behalf and on Your sole
responsibility, not on behalf of any other Contributor, and only if You agree to
indemnify, defend, and hold each Contributor harmless for any liability incurred
by, or claims asserted against, such Contributor by reason of your accepting any
such warranty or additional liability.

:def_p:`fls_hekw14nk1xfo`
END OF TERMS AND CONDITIONS

:def_p:`fls_9xjr4eha5w7n`
APPENDIX: How to apply the Apache License to your work.

:def_p:`fls_abeqsyhlxee5`
To apply the Apache License to your work, attach the following boilerplate
notice, with the fields enclosed by brackets "[]" replaced with your own
identifying information. (Don't include the brackets!)  The text should
be enclosed in the appropriate comment syntax for the file format. We also
recommend that a file or class name and description of purpose be included on
the same "printed page" as the copyright notice for easier identification within
third-party archives.

:def_p:`fls_y4k1itkpzabk`
Copyright [yyyy] [name of copyright owner]

:def_p:`fls_txpmnwmmbuyf`
Licensed under the Apache License, Version 2.0 (the "License");

:def_p:`fls_8v90wijm7dr1`
you may not use this file except in compliance with the License.

:def_p:`fls_semi0wse3oj5`
You may obtain a copy of the License at

:def_p:`fls_sgeku3615457`
http://www.apache.org/licenses/LICENSE-2.0

:def_p:`fls_v9fscrw5fzgj`
Unless required by applicable law or agreed to in writing, software distributed
under the License is distributed on an "AS IS" BASIS, WITHOUT WARRANTIES OR
CONDITIONS OF ANY KIND, either express or implied. See the License for the
specific language governing permissions and limitations under the License.

Scope
-----

:def_p:`fls_srdq4mota5pr`
This document specifies the form and meaning of programs written in the
programming language Rust, as implemented by the :codeterm:`rustc` compiler
shipped with Ferrocene. It documents the current understanding for the purposes
of compiler validation. As such, given any doubt, it prefers documenting
behavior of :codeterm:`rustc` as included in the associated Ferrocene release
over claiming correctness as a specification.

:def_p:`fls_dv1qish8svc`
This document is made available for contribution and review as it is useful
outside of the Ferrocene effort and can be a place of shared understanding. It
is not intended as a discussion ground for language evolution. It is also not
indented as a document enabling conformance between compilers.

:def_p:`fls_osh9tiwpnsn1`
Contribution and review is managed by the Ferrocene project developers.

Extent
~~~~~~

:def_p:`fls_x78yd1sszydv`
This document specifies:

* :def_p:`fls_9e032738udnb`
  The form of a program written in Rust;

* :def_p:`fls_jk7scu5xs17z`
  The effect of translating and executing such a program;

* :def_p:`fls_jiryupa5fxgf`
  The manner in which :term:`module`\ s and :term:`crate`\ s may be combined to
  form Rust programs;

* :def_p:`fls_sph1a3sapinh`
  The language-defined libraries that a conforming tool is required to supply;

* :def_p:`fls_7tm19jxtffc8`
  The violations that a conforming tool is required to detect, and the effect of
  attempting to translate or execute a program containing such violations;

* :def_p:`fls_5pbrl8lhuth1`
  The violations that a conforming implementation is not required to detect.

:def_p:`fls_o8fc3e53vp7g`
This document does not specify:

* :def_p:`fls_rw0y5t13y6gs`
  The means by which a Rust program is transformed into object code executable
  by a processor;

* :def_p:`fls_x7c3o621qj9z`
  The means by which translation or execution of Rust programs is invoked and
  the executing units are controlled;

* :def_p:`fls_5y2b6yjcl1vz`
  The size or speed of the object code, or the relative execution speed of
  different language constructs;

* :def_p:`fls_8dennhk2dha`
  The form or contents of any listings produced by a tool; in particular, the
  form or contents of error or warning messages;

* :def_p:`fls_j2gs3hrbxtyx`
  The effect of undefined behavior;

* :def_p:`fls_gy2c7vfwkd8j`
  The size of a program or program unit that will exceed the capacity of a
  conforming tool.

Structure
~~~~~~~~~

:def_p:`fls_6lrqailxjb02`
This document contains 21 chapters, 4 appendices, and an index.

:def_p:`fls_tys7ciqnp8bn`
The specification of the Rust language is separated into:

* :def_p:`fls_3ubhkaheu8i1`
  Chapters 1 through 21,

* :def_p:`fls_xw3grr2g5zgi`
  Appendix A: The Rust Prelude,

* :def_p:`fls_3hu6x73g39yi`
  Appendix B: The Rust Core Library,

* :def_p:`fls_h29so7l54rrl`
  `Appendix C: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :def_p:`fls_rq8ejzuyi2ud`
  `Appendix D: Syntax Summary.
  <https://docs.google.com/document/d/1TzjQ-n2NS0ZUzwg6VDmD7-kAjW7iGID7h4KEdbfro
  Dk/edit#bookmark=id.h61cd8uat4jc>`_

:def_p:`fls_6srbinvnyd54`
The specification is normative, except for the material in each of the items
listed below, which is informative:

* :def_p:`fls_ciixfg9jhv42`
  Text under an Examples heading.

* :def_p:`fls_ej94lm2682kg`
  Each subchapter whose title starts with the word "Example" or "Examples".

:def_p:`fls_xgk91jrbpyoc`
The following appendices are informative:

* :def_p:`fls_7o7qh34bqahh`
  `Appendix C: Glossary
  <https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5R
  yI/edit#bookmark=id.bc2qwbfibrcs>`_,

* :def_p:`fls_w0mgss6ic60w`
  `Appendix D: Syntax Summary.
  <https://docs.google.com/document/d/1TzjQ-n2NS0ZUzwg6VDmD7-kAjW7iGID7h4KEdbfro
  Dk/edit#bookmark=id.h61cd8uat4jc>`_

:def_p:`fls_jc4upf6685bw`
Each chapter is divided into subchapters that have a common structure. Each
chapter is divided into subchapters that have a common structure. Each chapter
and subchapter is then organized to include the following segments as is
relevant to the topic:

.. rubric:: Syntax

:def_p:`fls_oxzjqxgejx9t`
The syntax representation of a :term:`construct`.

.. rubric:: Legality Rules

:def_p:`fls_gmx688d6ek1o`
Compile-time rules and facts for each :term:`construct`. A :term:`construct` is
legal if it obeys all of the Legality Rules.

:def_p:`fls_5zdjikp1jhc`
Legality Rules are verified after :term:`macro expansion` takes place.

.. rubric:: Dynamic Semantics

:def_p:`fls_as5bhc5t285g`
Run-time effects of each :term:`construct`.

.. rubric:: Undefined Behavior

:def_p:`fls_70qjvaqoz007`
Situations that result in unbounded errors.

.. rubric:: Implementation Requirements

Additional requirements for conforming tools.

.. rubric:: Examples

:def_p:`fls_w8j575w2hmc8`
Examples illustrating the possible forms of a :term:`construct`. This material
is informative.

Conformity
~~~~~~~~~~

.. rubric:: Implementation Requirements

:def_p:`fls_kdyqtnc6loam`
A conforming tool shall:

* :def_p:`fls_ctwsz8sl7lbq`
  Translate and correctly execute legal programs written in Rust, provided that
  they are not so large as to exceed the capacity of the tool,

* :def_p:`fls_bvpekhdaxctq`
  Identify all programs or program units that are so large as to exceed the
  capacity of the tool (or raise an appropriate exception at run time),

* :def_p:`fls_kfs8gsd36d91`
  Identify all programs or program units that contain errors whose detection is
  required by this document,

* :def_p:`fls_k5sozk8jhrmg`
  Supply all language-defined library units required by this document,

* :def_p:`fls_nwx1fdq6b4mg`
  Contain no variations except those explicitly permitted by this document, or
  those that are impossible or impractical to avoid given the tool's execution
  environment,

* :def_p:`fls_n3ypaile1a36`
  Specify all such variations in the manner prescribed by this document.

:def_p:`fls_nnmx2qsu14ft`
The external effect of the execution of a Rust program is defined in terms of
its interactions with its external environment. The following are defined as
external interactions:

* :def_p:`fls_gu3331rmv2ho`
  Any call on an foreign :term:`function`, including any :term:`argument
  operand`\ s passed to it;

* :def_p:`fls_3iekobt8qqi`
  Any result returned or :term:`panic` propagated from a :term:`main function`
  or an exported :term:`function` to an external caller;

* :def_p:`fls_qx9fxf4py0j0`
  The :term:`value`\ s of imported and exported :term:`object`\ s at the time of
  any other interaction with the external environment.

:def_p:`fls_pl0fyjcwslqm`
A tool that conforms to this document shall produce for the execution of a given
Rust program a set of interactions with the external environment whose order and
timing are consistent with the definitions and requirements of this document for
the semantics of the given program.

:def_p:`fls_lkdm0mdghppv`
A tool that conforms to this document shall support each capability required by
the language as specified.

:def_p:`fls_d07x1mbhgpsd`
A tool that conforms to this document may provide additional :term:`attribute`\
s as long as their names are not the same as the names of :term:`built-in
attribute`\ s.

Method of Description and Syntax Notation
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

:def_p:`fls_mc4a28do6kcp`
The form of a Rust program is described by means of a context-free syntax
together with context-dependent requirements expressed by narrative rules.

:def_p:`fls_ioyp4wux6skt`
The meaning of a Rust program is described by means of narrative rules defining
both the effects of each construct and the composition rules for constructs.

:def_p:`fls_jsflt7691ye4`
The context-free syntax of Rust is described using a simple variant of the
Backus-Naur form. In particular:

* :def_p:`fls_98fm7z04lq9`
  A ``monospaced`` font is used to denote Rust syntax.

* :def_p:`fls_ceb5a8t6cakr`
  Words in PascalCase font are used to denote a syntactic category, for example:

.. syntax::

   		FloatExponent

* :def_p:`fls_pts29mb5ld68`
  Words in **bold** font are used to indicate literal words and :term:`keyword`\
  s, for example:

.. syntax::

   		$$crate$$
   $$proc_macro_derive$$
   $$Self$$
   $$tt$$

* :def_p:`fls_gqjo5oh7vn3b`
  Characters in **bold** font are used to indicate literal characters and
  literal punctuation, for example:

.. syntax::

   		$$1$$
   $$F$$
   $${$$
   $$&&$$
   $$>>=$$

* :def_p:`fls_1dz634xp8xp5`
  A character preceded by ``\`` (bold reverse solidus) is used to denote an
  :term:`escaped character`, for example:

.. syntax::

   		$$\t$$
   $$\\$$

* :def_p:`fls_pp9vtjlyblrl`
  A prefix followed by ``?`` (question mark) is used to denote an optional
  prefix, for example:

.. syntax::

   		CrateRenaming?

* :def_p:`fls_6e2vd9fvhsmk`
  A prefix followed by ``*`` (asterisk) is used to denote zero or more
  repetitions of the prefix, for example:

.. syntax::

   		OuterAttributeOrDoc*

* :def_p:`fls_4onq0kkrt6qv`
  A prefix followed by ``+`` (plus sign) is used to denote one or more
  repetitions of the prefix, for example:

.. syntax::

   		MacroMatch+

* :def_p:`fls_qu4rsmnq659w`
  A prefix followed by ``L-H`` is used to denote the number of repetitions of
  the prefix within the range from L to H, inclusive. For example:

.. syntax::

   		HexadecimalDigit1-6

* :def_p:`fls_rllu7aksf17e`
  ``[ ]`` (square brackets) indicate any character within, for example:

.. syntax::

   		[$$8$$ $$a$$ $$\r$$ $$:$$]

* :def_p:`fls_blvsfqeevosr`
  ``~[ ]`` (square brackets preceded by tilde) indicate any character except the
  characters within, for example:

.. syntax::

   		~[$$8$$ $$a$$ $$\r$$ $$:$$]

* :def_p:`fls_lwcjq3wzjyvb`
  ``[ - ]`` indicates any character within the specified range, inclusive. For
  example:

.. syntax::

   		[$$a$$-$$f$$]

* :def_p:`fls_v7wd5yk00im6`
  A ``|`` (vertical line) separates alternative items, for example:

.. syntax::

   		$$self$$ | Identifier | $$_$$


* :def_p:`fls_nf8alga8uz6c`
  ``( )`` (parentheses) are used to group items, for example:

.. syntax::

   		($$,$$ ConfigurationPredicate)

:def_p:`fls_u5ryccs9cpex`
Whenever the run-time semantics define certain actions to happen in an arbitrary
order, this means that a tool arranges for these actions to occur in a way that
is equivalent to some sequential order, following the rules that result from
that sequential order. This can happen, for example, if two parameters of a
given call expression have side effects.

Versioning
----------

:def_p:`fls_l80e3kdwnldc`
Ferrocene is a qualified compiler and this is the accompanying language
specification for the qualified version of the compiler. This document will
be updated with each qualification to accurately reflect the behavior of
the compiler qualified under that version of Ferrocene. This specification
corresponds to Ferrocene 1.0.

Definitions
-----------

:def_p:`fls_sm2kexes5pr7`
Terms are defined throughout this document, indicated by *italic* type. Terms
explicitly defined in this document are not to be presumed to refer implicitly
to similar terms defined elsewhere.

:def_p:`fls_2o98zw29xc46`
Mathematical terms not defined in this document are to be interpreted according
to the CRC Concise Encyclopedia of Mathematics, Second Edition.

:def_p:`fls_lon5qffd65fi`
Other terms not defined in this document are to be interpreted according to the
Webster's Third New International Dictionary of the English Language.

:def_p:`fls_qeolgxvcy75`
The definitions of terms are available in `Appendix C: Glossary
<https://docs.google.com/document/d/1I5cxk43bG70JdhSJI2PZloQaj540ntY1IQSoFzo5RyI
/edit#bookmark=id.bc2qwbfibrcs>`_.

:def_p:`fls_h2m244agxaxs`
A rule is a requirement imposed on the programmer, stated in normative language
such as "shall", "shall not", "must", "must not", except for text under
Implementation Requirements heading.

:def_p:`fls_47svine904xk`
A fact is a requirement imposed on a conforming tool, stated in informative
language such as "is", "is not", "can", "cannot".

