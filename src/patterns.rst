.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Patterns
========

Patterns
--------

.. rubric:: Syntax

.. syntax::

   Pattern ::=
       $$|$$? PatternWithoutAlternation ($$|$$ PatternWithoutAlternation)*

   PatternList ::=
       Pattern ($$,$$ Pattern)+ $$,$$?

   PatternWithoutAlternation ::=
       PatternWithoutRange
     | RangePattern

   PatternWithoutRange ::=
       IdentifierPattern
     | LiteralPattern
     | MacroInvocation
     | ParenthesizedPattern
     | PathPattern
     | ReferencePattern
     | RestPattern
     | SlicePattern
     | StructPattern
     | TuplePattern
     | TupleStructPattern
     | WildcardPattern

.. rubric:: Legality Rules

:def_p:`fls_imegtsi224ts`
A :term:`pattern` is a :term:`construct` that matches a :term:`value` which
satisfies all the criteria of the :term:`pattern`.

:def_p:`fls_mp6i4blzexnu`
A :term:`pattern-without-alternation` is a :term:`pattern` that cannot be
alternated.

:def_p:`fls_6xx34zr069bj`
A :term:`subpattern` is a :term:`pattern` nested within another pattern.

:def_p:`fls_8xzjb0yzftkd`
A :term:`pattern` has a :term:`type`, with the exception of the :term:`rest
pattern` if it is not the inner :term:`pattern` of a :term:`slice pattern`
or the :term:`pattern` of a possibly nested :term:`identifier pattern` of a
:term:`slice pattern`\ ``.``

:def_p:`fls_cma5t8waon0x`
The :term:`expected type` of a :term:`pattern` is the :term:`type` of the
:term:`value` the :term:`pattern` is being matched against.

:def_p:`fls_8luyomzppck`
Any two :term:`pattern-without-alternation`\ s that are or-ed using character
0x7C (vertical line) are subject to the following restrictions:

* :def_p:`fls_rpvdfmy3n05a`
  The :term:`type`\ s of the two :term:`pattern-without-alternation`\ s shall
  be :term:`unifiable`.

* :def_p:`fls_kv533rntni1x`
  The :term:`binding`\ s of the two :term:`pattern-without-alternation`\ s shall
  be the same, shall have :term:`unifiable type`\ s, and shall have the same
  :term:`binding mode`\ s.

Identifier Patterns
~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IdentifierPattern ::=
       $$ref$$? $$mut$$? Binding BoundPattern?

   BoundPattern ::=
       $$@$$ Pattern

.. rubric:: Legality Rules

:def_p:`fls_uljdw9rf7ies`
An :term:`identifier pattern` is a :term:`pattern` that binds the :term:`value`
it matches to a :term:`binding`.

:def_p:`fls_vy9uw586wy0d`
A :term:`bound pattern` is a :term:`pattern` that imposes a constraint on a
related :term:`identifier pattern`.

:def_p:`fls_hqwt3fvr063y`
An :term:`identifier pattern` yields a :term:`binding`. An :term:`identifier
pattern` with :term:`keyword` ``mut`` yields a :term:`mutable binding`.

:def_p:`fls_24c95c56tugl`
The :term:`identifier pattern` enters its :term:`binding` into :term:`pattern
scope` in the :term:`value namespace` if it does not resolve to a
:term:`constant`.

:def_p:`fls_twcavjk7iquy`
It is a static error if the :term:`identifier pattern` consists of anything
other than a :term:`binding` when the :term:`binding` resolves to a
:term:`constant`.

:def_p:`fls_hw26hy33guk5`
An :term:`identifier pattern` is :term:`irrefutable` when:

* :def_p:`fls_svfxwz4yy5i`
  It has a :term:`bound pattern` and the :term:`bound pattern` is
  :term:`irrefutable`, or

* :def_p:`fls_x6f6q22b5jpc`
  It does not have a :term:`bound pattern` and its :term:`binding` resolves to
  an :term:`irrefutable constant`.

* :def_p:`fls_r2mb8v2lh3x0`
  It does not have a :term:`bound pattern` and its :term:`binding` does not
  resolve to a :term:`constant`.

:def_p:`fls_7oioaitb075g`
If the :term:`identifier pattern` does not have a :term:`bound pattern`, then
the :term:`type` of its :term:`binding` is determined as follows:

* :def_p:`fls_40qin0ss5sqd`
  If the :term:`identifier pattern` has only :term:`keyword` ``ref``, then
  the :term:`type` is ``& inferred_type``, where ``inferred_type`` is the
  :term:`type` determined by :term:`type inference`.

* :def_p:`fls_pivz0v7ey6sw`
  If the :term:`identifier pattern` has :term:`keywords` ``ref`` ``mut``, then
  the :term:`type` is ``&mut inferred_type``, where ``inferred_type`` is the
  :term:`type` determined by :term:`type inference`.

* :def_p:`fls_2ahkrddxwj1n`
  Otherwise the :term:`type` is ``inferred_type``, where ``inferred_type`` is
  the :term:`type` determined by :term:`type inference`.

:def_p:`fls_eucnafj3uedy`
If the :term:`identifier pattern` has a :term:`bound pattern`, then the
:term:`type` of its :term:`binding` is determined as follows:

* :def_p:`fls_f8zo4scodhcr`
  If the :term:`identifier pattern` has only :term:`keyword` ``ref``, then the
  :term:`type` ``& bound_pattern_type``, where ``bound_pattern_type`` is the
  :term:`type` of the :term:`bound pattern`.

* :def_p:`fls_d3fs2h7oqjl0`
  If the :term:`identifier pattern` has :term:`keywords` ``ref mut``, then the
  :term:`type` is ``&mut bound_pattern_type``, where ``bound_pattern_type`` is
  the :term:`type` of the :term:`bound pattern`.

* :def_p:`fls_exo8asevh5x1`
  Otherwise the :term:`type` is ``inferred_type``, where ``inferred_type`` is
  the :term:`type` determined by :term:`type inference`.

.. rubric:: Examples

:def_p:`fls_sfyfdxhvhk44`
An identifier pattern in the context of a let expression.

.. code-block:: text

   let x = 42;

:def_p:`fls_as0pqqmo1des`
An identifier pattern with a bound pattern in the context of a match expression.

.. code-block:: text

   match x {
       small @ 1 ..= 5 => (),
       _ => (),
   }

Literal Patterns
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   LiteralPattern ::=
       BooleanLiteral
     | ByteLiteral
     | ByteStringLiteral
     | CharacterLiteral
     | $$-$$? NumericLiteral
     | RawByteStringLiteral
     | RawStringLiteral
     | SimpleStringLiteral

.. rubric:: Legality Rules

:def_p:`fls_pah15qa54irs`
A :term:`literal pattern` is a :term:`pattern` that matches a :term:`literal`.

:def_p:`fls_v7iv7x9gy9qm`
A :term:`literal pattern` is always :term:`refutable`.

:def_p:`fls_co60bzvwashg`
The :term:`type` of a :term:`literal pattern` is the :term:`type` of the
specified :term:`literal`.

.. rubric:: Examples

:def_p:`fls_fqclaznjgtb1`
Two literal patterns in the context of a match expression. See :p:`5.1.1.
<fls_yeajwokikkdi>` for the declaration of ``x``.

.. code-block:: text

   match x {
       -2 => (),
       36 => (),
       _  => (),
   }


Parenthesized Patterns
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ParenthesizedPattern ::=
       $$($$ Pattern $$)$$

.. rubric:: Legality Rules

:def_p:`fls_kvqzmt7my5dh`
A :term:`parenthesized pattern` is a :term:`pattern` that controls the
precedence of its :term:`subpattern`\ s.

:def_p:`fls_mrjhpiq5refe`
A :term:`parenthesized pattern` is :term:`irrefutable` when its nested
:term:`pattern` is :term:`irrefutable`.

:def_p:`fls_pe5kh8y8u664`
The :term:`type` of a :term:`parenthesized pattern` is the :term:`type` of its
nested :term:`pattern`.

.. rubric:: Examples

:def_p:`fls_2xq8852gihn9`
See :p:`5.1.1. <fls_yeajwokikkdi>` for the declaration of ``x``.

.. code-block:: text

   let ref_x = &x;

:def_p:`fls_2dmeukyjqz9y`
A parenthesized pattern inside a reference pattern in the context of a match
expression.

.. code-block:: text

   match ref_x {
       &(1 ..= 5) => (),
       _ => (),
   }

Path Patterns
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   PathPattern ::=
       PathInExpression
     | QualifiedPathInExpression

.. rubric:: Legality Rules

:def_p:`fls_1crq0mexo5r1`
A :term:`path pattern` is a :term:`pattern` that matches a :term:`constant`,
an :term:`enum variant` without :term:`field`\ s, or a :term:`unit struct`
indicated by a :term:`path`.

:def_p:`fls_xz5otkhogn31`
A :term:`path pattern` expressed as a :term:`path-in-expression` shall refer to
either an :term:`associated constant`, or a :term:`constant`.

:def_p:`fls_t8sjzsif2ilf`
When a :term:`path pattern` expressed as a :term:`path-in-expression` refers to
a :term:`constant`, the :term:`constant` shall not be of a :term:`union type`.
If the :term:`constant` is of an :term:`enum type` or :term:`struct type`, then
the :term:`constant` shall be subject to :term:`attribute` :codeterm:`derive`
with arguments :codeterm:`core::cmp::Eq` and :codeterm:`core::cmp::PartialEq`.

:def_p:`fls_bv9psmitxfuw`
A :term:`path pattern` expressed as a :term:`qualified path-in-expression` shall
refer to an :term:`associated constant`.

:def_p:`fls_sl47k9oj5p7t`
A :term:`path pattern` is :term:`irrefutable` when it refers to:

* :def_p:`fls_cfoy86mkmqa4`
  A :term:`constant` whose :term:`type` is :term:`irrefutable`, or

* :def_p:`fls_rnppz6y5z8pi`
  An :term:`enum variant` of an :term:`enum type` with a single :term:`enum
  variant` where the :term:`type`\ s of all :term:`field`\ s are
  :term:`irrefutable`, or

* :def_p:`fls_ag6m4mvpturw`
  A :term:`struct` where the :term:`type`\ s of all :term:`field`\ s are
  :term:`irrefutable`.

:def_p:`fls_pedy2pqrvnx7`
The :term:`type` of a :term:`path pattern` is the :term:`type` of the
:term:`constant`, :term:`enum`, or :term:`struct` the :term:`path` resolved to.

.. rubric:: Examples

.. code-block:: text

   mod module {
   	pub const ZERO: i32 = 0;
   }

   enum Enum { Variant }


:def_p:`fls_u59rilepu8z9`
See :p:`5.1.1. <fls_yeajwokikkdi>` for the declaration of ``x``.

.. code-block:: text

   match x {
       module::ZERO => (),
       Enum::Variant => (),
       _  => (),
   }

Range Patterns
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   RangePattern ::=
       HalfOpenRangePattern
     | InclusiveRangePattern
     | ObsoleteRangePattern

   HalfOpenRangePattern ::=
       RangePatternLowBound $$..$$

   InclusiveRangePattern ::=
       RangePatternLowBound $$..=$$ RangePatternHighBound

   ObsoleteRangePattern ::=
       RangePatternLowBound $$...$$ RangePatternHighBound

   RangePatternLowBound ::=
       RangePatternBound

   RangePatternHighBound ::=
       RangePatternBound

   RangePatternBound ::=
       ByteLiteral
     | CharacterLiteral
     | $$-$$? NumericLiteral
     | PathInExpression
     | QualifiedPathInExpression

.. rubric:: Legality Rules

:def_p:`fls_okupyoav13rm`
A :term:`range pattern` is a :term:`pattern` that matches :term:`value`\ s which
fall within a range.

:def_p:`fls_jhchm7dy927k`
A :term:`half-open range pattern` is a :term:`range pattern` with only a
:term:`range pattern low bound`.

:def_p:`fls_q86j23iiqv8w`
An :term:`inclusive range pattern` is a :term:`range pattern` with both a
:term:`range pattern low bound` and a :term:`range pattern high bound`.

:def_p:`fls_akf9x5r6e0ta`
An :term:`obsolete range pattern` is a :term:`range pattern` that uses obsolete
syntax to express an :term:`inclusive range pattern`.

:def_p:`fls_vrpr6ttpfpal`
A :term:`range pattern bound` is a constraint on the range of a :term:`range
pattern`.

:def_p:`fls_nk48gregn3me`
A :term:`range pattern low bound` is a :term:`range pattern bound` that
specifies the start of a range.

:def_p:`fls_83v1xqbebs58`
A :term:`range pattern high bound` is a :term:`range pattern bound` that
specifies the end of a range.

:def_p:`fls_2hpuccwh2xml`
A :term:`half-open range pattern` shall appear within a :term:`parenthesized
pattern` when context is a :term:`slice pattern`.

:def_p:`fls_9kk81isk0mlp`
The :term:`range pattern low bound` of an :term:`inclusive range pattern` shall
be less than or equal to its :term:`range pattern high bound`.

:def_p:`fls_s2b5n4snc4d7`
An :term:`obsolete range pattern` is equivalent to an :term:`inclusive range
pattern`.

:def_p:`fls_4o4ge6x9a8rs`
A :term:`range pattern` is :term:`irrefutable` only when it spans the entire set
of possible :term:`value`\ s of a :term:`type`.

:def_p:`fls_6o995ak4hywq`
The :term:`type`\ s of the :term:`range pattern low bound` and the :term:`range
pattern high bound` of a :term:`range pattern` shall be :term:`unifiable`.

:def_p:`fls_3js1645tgh31`
The :term:`type` of a :term:`range pattern` is determined as follows:

* :def_p:`fls_wfqrbwrogjnq`
  If the :term:`range pattern` is expressed as an :term:`inclusive range
  pattern` or an :term:`obsolete range pattern`, then the :term:`type` is the
  :term:`unified type` of the :term:`type`\ s of the :term:`range pattern low
  bound` and the :term:`range pattern high bound`.

* :def_p:`fls_rgr7t33s0m7m`
  Otherwise the :term:`type` is the :term:`type` of the :term:`range pattern
  low bound`.

:def_p:`fls_5ey5mj8t8knd`
A :term:`path-in-expression` of a :term:`range pattern` shall refer to a
:term:`constant` of a :term:`scalar type`.

:def_p:`fls_z4js96mchcsv`
A :term:`qualified path-in-expression` of a :term:`range pattern` shall refer to
an :term:`associated constant` of a :term:`scalar type`.

.. rubric:: Examples

:def_p:`fls_3wwpq8i6mo2a`
Two range patterns in the context of a match expression. See :p:`5.1.1.
<fls_yeajwokikkdi>` for the declaration of ``x``.

.. code-block:: text

   match x {
       -30 ..= 2 => (),
       57 .. => (),
       _ => (),
   }

Reference Patterns
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ReferencePattern ::=
       $$&$$ $$mut$$? PatternWithoutRange

.. rubric:: Legality Rules

:def_p:`fls_fhahcc1mz2qh`
A :term:`reference pattern` is a :term:`pattern` that dereferences a
:term:`pointer` that is being matched.

:def_p:`fls_x0bmzl1315gq`
A :term:`reference pattern` is always :term:`irrefutable`.

:def_p:`fls_fedo8zhgpla5`
The :term:`type` of a :term:`reference pattern` is determined as follows:

* :def_p:`fls_30u9ij164ww3`
  If the :term:`reference pattern` appears with :term:`keyword`
  ``mut``, then the :term:`type` is ``&mut pattern_without_range_type``,
  where ``pattern_without_range_type`` is the :term:`type` of the
  :syntax:`PatternWithoutRange`.

* :def_p:`fls_d1kc73hpncpo`
  If the :term:`reference pattern` appears without :term:`keyword`
  ``mut``, then the :term:`type` is ``& pattern_without_range_type``,
  where ``pattern_without_range_type`` is the :term:`type` of the
  :syntax:`PatternWithoutRange`.

.. rubric:: Examples

:def_p:`fls_mpeuhov0umfa`
A reference pattern in the context of a match expression. See :p:`5.1.3.
<fls_yowuqu7bcu7b>` for the declaration of ``ref_x``.

.. code-block:: text

   match ref_x {
       &23 => (),
       _ => (),
   }

Rest Patterns
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   RestPattern ::=
       $$..$$

.. rubric:: Legality Rules

:def_p:`fls_eso51epfofxb`
A :term:`rest pattern` is a :term:`pattern` that matches zero or more elements
that have not already been matched.

:def_p:`fls_5a75a2y43uev`
A :term:`rest pattern` shall appear at most once within a :term:`slice pattern`,
an :term:`identifier pattern` of a :term:`slice pattern`, a :term:`tuple
pattern`, and a :term:`tuple struct pattern`.

:def_p:`fls_rsqyza99vl3x`
A :term:`rest pattern` is always :term:`irrefutable`.

:def_p:`fls_w1pw40phsv2o`
If a :term:`rest pattern` appears within a :term:`slice pattern` or the
:term:`identifier pattern` of a :term:`slice pattern`, then the :term:`type` of
the :term:`rest pattern` is determined as follows:

* :def_p:`fls_x8ylgxrf9ca`
  If the :term:`type` of the :term:`slice pattern` is an :term:`array type`,
  then the :term:`type` is ``[T; N]`` where ``T`` is the :term:`element type` of
  the :term:`array type`, and ``N`` is the :term:`array type`'s size minus the
  number of matched elements of the :term:`slice pattern`.

* :def_p:`fls_zgoke73xrhk3`
  If the :term:`type` of the :term:`slice pattern` is a :term:`slice type`, then
  the :term:`type` is that :term:`slice type`.

.. rubric:: Examples

:def_p:`fls_bdcv6rwx0fsv`
A rest pattern in an identifier pattern of a slice pattern, followed by a rest
pattern in a slice pattern.

.. code-block:: text

   match slice {
       [1, 5, .., 7] => (),
       [start, end @ ..] => (),
   }


:def_p:`fls_qz9guhlg19j3`
Rest patterns in tuple patterns.

.. syntax::


   match tuple {
       (1, .., y) => (),
       (.., 5) => (),
       (..) => (),
   }

Slice Patterns
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   SlicePattern ::=
       $$[$$ PatternList? $$]$$

.. rubric:: Legality Rules

:def_p:`fls_qqiu594hki8g`
A :term:`slice pattern` is a :term:`pattern` that matches :term:`array`\ s of
fixed size and :term:`slice`\ s of dynamic size.

:def_p:`fls_h6x9xlxi7y5n`
A :term:`slice pattern` is :term:`irrefutable` when it refers to:

* :def_p:`fls_jbmxu7y5fnm6`
  An :term:`array`, where each :term:`subpattern` is :term:`irrefutable`, or

* :def_p:`fls_r78zzw7yyg34`
  A :term:`slice`, where the :syntax:`PatternList` consists of a single
  :term:`rest pattern`, or a single possibly nested :term:`identifier pattern`
  whose last :term:`bound pattern` is a :term:`rest pattern`.

:def_p:`fls_ndor56nou676`
The :term:`type` of a :term:`slice pattern` is the same as the :term:`expected
type`.

.. rubric:: Examples

.. syntax::


   let v = vec![1, 2, 3];


:def_p:`fls_9yuobz1jsehf`
A slice pattern in the context of a match expression.

.. syntax::


   match v {
       [a, b, c] => (),
       _ => ()
   }


Struct Patterns
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   StructPattern ::=
       RecordStructPattern
     | TupleStructPattern
     | UnitStructPattern

   RecordStructPattern ::=
       DeconstructionType $${$$ RecordStructDeconstructor? $$}$$

   RecordStructDeconstructor ::=
       RecordStructRestPattern
     | RecordStructFieldDeconstructorList
     | RecordStructIndexedFieldDeconstructorList

   RecordStructRestPattern ::=
       OuterAttributeOrDoc* RestPattern

   RecordStructFieldDeconstructorList ::=
       RecordStructFieldDeconstructor (, RecordStructFieldDeconstructor)* (, RecordStructRestPattern | ,?)

   RecordStructFieldDeconstructor ::=
       OuterAttributeOrDoc* (
           NamedDeconstructor
         | ShorthandDeconstructor
       )

   NamedDeconstructor ::=
       Identifier $$:$$ Pattern

   ShorthandDeconstructor ::=
       $$ref$$? $$mut$$? Binding

   RecordStructIndexedFieldDeconstructorList ::=
       IndexedDeconstructor (, IndexedDeconstructor)* (, RecordStructRestPattern | ,?)

   IndexedDeconstructor ::=
       OuterAttributeOrDoc* TupleIndex $$:$$ Pattern
   TupleStructPattern ::=
       DeconstructionType $$($$ PatternList? $$)$$
   UnitStructPattern ::=
       DeconstructionType
   DeconstructionType ::=
       PathInExpression


.. rubric:: Legality Rules

:def_p:`fls_e093i77g3ju7`
A :term:`struct pattern` is a :term:`pattern` that matches a :term:`struct`.

:def_p:`fls_mqhest816lo2`
A :term:`tuple struct pattern` is a :term:`struct pattern` that matches
:term:`enum variant`\ s and :term:`tuple struct`\ s.

:def_p:`fls_d8cyh5v25s0x`
A :term:`union pattern` is a :term:`record struct pattern` that matches a
:term:`union`.

:def_p:`fls_aolvi75ck60i`
A :term:`unit struct pattern` is a :term:`struct pattern` that matches a
:term:`unit struct`.

:def_p:`fls_77qdyt1lpd`
An :term:`indexed deconstructor` is a :term:`construct` that matches the
position of a :term:`field` of a :term:`tuple`.

:def_p:`fls_khhozbtc23l1`
A :term:`named deconstructor` is a :term:`construct` that matches the
:term:`name` of a :term:`field` of a :term:`struct`.

:def_p:`fls_1zi6zmayw792`
A :term:`shorthand deconstructor` is a :term:`construct` that matches the
:term:`name` of a :term:`field` of a :term:`struct`.

:def_p:`fls_8ersn39rt5pd`
The :term:`deconstruction type` indicates the :term:`type` of the :term:`struct`
being deconstructed by a :term:`struct pattern`.

:def_p:`fls_tfwwrovxeomp`
A :syntax:`RecordStructPattern` without a
:syntax:`RecordStructIndexedFieldDeconstructorList` is a :term:`record struct
pattern`.

:def_p:`fls_1tjp0z3v9ukg`
A :syntax:`TupleStructPattern` and a :syntax:`RecordStructPattern` without a
:syntax:`RecordStructFieldDeconstructorList` are :term:`tuple struct pattern`\
s.

:def_p:`fls_y8qclm82nun8`
A :syntax:`RecordStructPattern` with a
:syntax:`RecordStructFieldDeconstructorList` is a :term:`union pattern`.

:def_p:`fls_j1lux391rmgg`
A :syntax:`UnitStructPattern` and a :syntax:`RecordStructPattern`
without a :syntax:`RecordStructFieldDeconstructorList` and a
:syntax:`RecordStructIndexedFieldDeconstructorList` are :term:`unit struct
pattern`\ s.

:def_p:`fls_2rgip6uruvt5`
A :term:`struct pattern` is interpreted based on the :term:`deconstruction
type`. It is a static error if a :term:`struct pattern` cannot be interpreted.

:def_p:`fls_wi3yo3z5mn5w`
A :term:`shorthand deconstructor` binds the :term:`value` of a matched
:term:`field` to a :term:`variable`. A :term:`shorthand deconstructor` with
:term:`keyword` ``mut`` yields a :term:`mutable` :term:`variable`.

:def_p:`fls_g5t53fj9ghk0`
It is a static error if a :term:`shorthand deconstructor` has only
:term:`keyword` ``ref`` or :term:`keywords` ``ref`` ``mut``, and its
:term:`variable` shadows a :term:`constant`.

:def_p:`fls_5vjoxrgeq3bg`
A :term:`struct pattern` is :term:`irrefutable` when all of its
:term:`subpattern`\ s are :term:`irrefutable`.

Record Struct Patterns
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_y10l03ogbs1s`
The :term:`deconstruction type` of a :term:`record struct pattern` shall resolve
to a :term:`record struct type`.

:def_p:`fls_tzbjwdk0xxui`
A :term:`named deconstructor` matches a :term:`field` of the
:term:`deconstruction type` when its :term:`identifier` and the :term:`name`
of the :term:`field` are the same. Such a :term:`named deconstructor` is a
:def_term:`matched named deconstructor`.

:def_p:`fls_n5xx6urvj7xg`
The :term:`type` of a :term:`named deconstructor` and the :term:`type` of a
matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_njzp6q2kfjb8`
A :term:`shorthand deconstructor` matches a :term:`field` of the
:term:`deconstruction type` when its :term:`name` and the :term:`name` of
the :term:`field` are the same. Such a :term:`shorthand deconstructor` is a
:def_term:`matched shorthand deconstructor`.

:def_p:`fls_emq0uil5w7xm`
The :term:`type` of a :term:`shorthand deconstructor` and the :term:`type` of
the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_vai6qhy39zmz`
A :term:`shorthand deconstructor` is equivalent to a :term:`named deconstructor`
where the :term:`name` of the :term:`shorthand deconstructor` denotes the
:term:`identifier` of the :term:`named deconstructor` and the entire content
of the :term:`shorthand deconstructor` denotes the pattern of the :term:`named
deconstructor`.

:def_p:`fls_pzvz6l540atp`
For each :term:`field` of the :term:`deconstruction type`, the :term:`record
struct deconstructor` shall either:

* :def_p:`fls_uoedp3g89mg`
  Contain a :term:`matched named deconstructor`, or

* :def_p:`fls_rspzc5jqbysa`
  Contain a :term:`matched shorthand deconstructor`, or

* :def_p:`fls_2l9wbc8sqtlo`
  Has a :syntax:`RecordStructRestPattern` or a
  :syntax:`RecordStructFieldDeconstructorList` with a
  :syntax:`RecordStructRestPattern`.

.. rubric:: Examples

.. syntax::


   struct Struct {
   	field: u32,
   	other: u32,
   }

   let Struct { field, other };
   let Struct { field, .. };
   let Struct { .. };

Tuple Struct Patterns
^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_ec0o57hwg8ic`
The :term:`deconstruction type` of a :term:`tuple struct pattern` shall resolve
to a :term:`tuple struct type`.

:def_p:`fls_3e2zaeqo1s96`
A :term:`tuple struct pattern` shall contain one :term:`subpattern` for each
:term:`field` of the :term:`deconstruction type`.

:def_p:`fls_w936pvga6lgn`
A :term:`subpattern` of a :term:`tuple struct pattern` matches a :term:`field`
of the :term:`deconstruction type` when its position and the position of
the :term:`field` in the :term:`deconstruction type` are the same. Such a
:term:`subpattern` is a :def_term:`matched subpattern`.

:def_p:`fls_aeh8bzh59m05`
The :term:`type` of the :term:`subpattern` of a :term:`tuple struct pattern` and
the :term:`type` of the matched :term:`field` shall be :term:`unifiable`.

:def_p:`fls_s7u5ghr13ib7`
An :term:`index deconstructor` matches a :term:`field` of the
:term:`deconstruction type` when its :term:`tuple index` and the position of
the :term:`field` in the :term:`deconstruction type` are the same. Such an
:term:`index deconstructor` is a :def_term:`matched index deconstructor`.

:def_p:`fls_x33civd9eptg`
For each :term:`field` of the :term:`deconstruction type`, the :term:`tuple
struct deconstructor` shall either:

* :def_p:`fls_gr3kc7k1j2ou`
  Contain a :term:`matched index deconstructor`, or

* :def_p:`fls_oc293y7fmn9f`
  Contain a :term:`matched subpattern`, or

* :def_p:`fls_n5w52m48v8fh`
  Has a ``RecordStructRestPattern.``

.. rubric:: Examples

.. syntax::


   struct Tuple(u32, f32);

   let Tuple(first, second);
   let Tuple(first, ..);
   let Tuple(..);

Union Patterns
^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_eytubf3jp1vy`
The :term:`deconstruction type` of a :term:`union pattern` shall resolve to a
:term:`union type`.

:def_p:`fls_51yggy3rohm8`
The :syntax:`RecordStructFieldDeconstructorList` of a :term:`union pattern`
shall contain exactly one :syntax:`RecordStructFieldDeconstructor` and no
:syntax:`RecordStructRestPattern`.

:def_p:`fls_cb5au9tab68o`
For the single :term:`field` of the :term:`deconstruction type`, a :term:`union
deconstructor` shall either:

* :def_p:`fls_r0d6w9di8ega`
  Contain a :term:`matched named deconstructor`, or

* :def_p:`fls_sm8o7cfb3q1k`
  Contain a :term:`matched shorthand deconstructor`.

:def_p:`fls_gm45psu7l64e`
A :term:`union pattern` shall require :term:`unsafe context`.

.. rubric:: Examples

.. syntax::


   union Union {
       int: u32,
       float: f32,
   }

   unsafe {
       let Union { int } = Union { int: 0 };
   }

Unit Struct Patterns
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_9ilkcejibsjd`
The :term:`deconstruction type` of a :term:`unit struct deconstructor` shall
resolve to a :term:`unit struct type`.

.. rubric:: Examples

.. syntax::


   struct Empty;

   let Empty = Empty;
   let Empty = Empty{};

Tuple Patterns
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TuplePattern ::=
       $$($$ TuplePatternElementList? $$)$$
   TuplePatternElementList ::=
       Pattern $$,$$
     | PatternList
     | RestPattern

.. rubric:: Legality Rules

:def_p:`fls_e2manugp4e0b`
A :term:`tuple pattern` is a :term:`pattern` that matches a :term:`tuple` which
satisfies all criteria defined by its :term:`subpattern`\ s.

:def_p:`fls_xk8udu4k61kj`
A :term:`tuple pattern` is :term:`irrefutable` when all of its
:term:`subpattern`\ s are :term:`irrefutable`.

:def_p:`fls_yhcaz6v49ub2`
The :term:`type` of a :term:`tuple pattern` is the :term:`type` of the
:term:`tuple` being destructured.

.. rubric:: Examples

.. code-block:: text

   let pair = (1, "two");


:def_p:`fls_8r81vtv5hnrd`
A tuple pattern in the context of a let statement.

.. syntax::


   let (first, second) = pair;

Wildcard Patterns
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WildcardPattern ::=
       $$_$$

.. rubric:: Legality Rules

:def_p:`fls_dreny9e0ei6r`
A :term:`wildcard pattern` is a :term:`pattern` that matches any single
:term:`value`.

:def_p:`fls_42fye1v0th8l`
A :term:`wildcard pattern` is always :term:`irrefutable`.

:def_p:`fls_b87mvrcc13f2`
The :term:`type` of a :term:`wildcard pattern` is the :term:`type` of the
:term:`value` it matches.

.. rubric:: Examples

:def_p:`fls_j3u6x1ensrbe`
A wildcard pattern in the context of a let statement. See :p:`5.1.10.
<fls_fo48m62q2y0v>` for the declaration of ``pair``.

.. code-block:: text

   let (first, _) = pair;

Refutability
------------

.. rubric:: Legality Rules

:def_p:`fls_9ntc4qmjmo90`
:term:`Refutability` is a property of :term:`pattern`\ s that expresses the
ability to match all possible values of a :term:`type`.

:def_p:`fls_9fjspnefoyvz`
An :term:`irrefutable pattern` is a :term:`pattern` that always matches the
:term:`value` it is being matched against.

:def_p:`fls_uq7ftuuq1sig`
A :term:`refutable pattern` is a :term:`pattern` that has a possibility of not
matching the :term:`value` it is being matched against.

An :term:`irrefutable type` is a :term:`type` that has at most one
:term:`value`.

A :term:`refutable type` is a :term:`type` that has more than one :term:`value`.

:def_p:`fls_l76ycteulo8e`
An :term:`irrefutable constant` is a :term:`constant` of an :term:`irrefutable
type`.

:def_p:`fls_lh0d85tl4qvy`
A :term:`refutable constant` is a :term:`constant` of a :term:`refutable type`.

.. rubric:: Examples

``x`` is an irrefutable pattern because it always matches ``42``.

.. code-block:: text

   let x = 42;


``y`` is a refutable pattern because it does not match ``value`` when ``value``
denotes :std:`core::option::Option::None`.

.. code-block:: text

   if let core::option::Option::Some(y) = value {

Binding Modes
-------------

.. rubric:: Syntax

.. syntax::

   Binding ::=
       Name

.. rubric:: Legality Rules

:def_p:`fls_vnh9wfrvumdz`
A :term:`binding` is a :term:`variable` of an :term:`identifier pattern` or a
:term:`shorthand deconstructor` that binds a matched :term:`value`.

:def_p:`fls_jrv4ghj4fr20`
A :term:`binding pattern` is either an :term:`identifier pattern` or a
:term:`shorthand deconstructor`.

:def_p:`fls_dqe75i8h2fie`
A :term:`non-reference pattern` is any :term:`pattern` except non-:term:`binding
pattern`\ s, :term:`path pattern`\ s, :term:`reference pattern`\ s, and
:term:`wildcard pattern`\ s.

:def_p:`fls_y3wuvj1y5j20`
If a :term:`binding pattern` does not explicitly specify :term:`keyword`
``ref``, :term:`keyword` ``mut``, or :term:`keyword`\ s ``ref mut``, then its
:term:`binding mode` uses the current :term:`binding mode` of :term:`pattern
matching`.

:def_p:`fls_55jtzh6a292x`
Initially, the :term:`binding mode` of a :term:`binding` is "by value".

:def_p:`fls_qcaf2kup7zn0`
During the process of :term:`pattern matching`, each time a :term:`reference`
is matched against a non-:term:`reference pattern`, the :term:`reference` is
dereferenced and the :term:`binding mode` is updated as follows:

* :def_p:`fls_6acdqz8rwnn`
  If the :term:`reference` is an :term:`immutable reference`, then the
  :term:`binding mode` is updated to "by reference".

* :def_p:`fls_tv0avib387bv`
  If the :term:`reference` is a :term:`mutable reference` and the :term:`binding
  mode` is "by value", then the :term:`binding mode` is updated to "by mutable
  reference".

:def_p:`fls_dbgmwldye42e`
The process repeats if the dereferenced :term:`value` is a :term:`reference`.

.. rubric:: Dynamic Semantics

:def_p:`fls_t34oqarwcusu`
A :term:`binding pattern`\ s binds its :term:`binding` to a matched
:term:`value` as follows:

* :def_p:`fls_7gxb74u1np36`
  If the :term:`binding mode` is "by reference" or the :term:`binding pattern`
  appears only with :term:`keyword` ``ref``, then the :term:`binding` is bound
  to a :term:`reference` of the matched :term:`value`.

* :def_p:`fls_7y56d0ulxomf`
  If the :term:`binding mode` is "by mutable reference" or the :term:`binding
  pattern` appears with keywords ``ref`` ``mut``, then the :term:`binding` is
  bound to a :term:`mutable reference` of the matched :term:`value`.

* :def_p:`fls_pxvtqxke1enp`
  If the :term:`binding mode` is "by value", then the :term:`binding` is bound
  to a copy of the matched :term:`value` if its :term:`type` implements the
  :codeterm:`core::marker::Copy` :term:`trait`, otherwise the :term:`binding` is
  bound to the move of the matched :term:`value`.

Pattern Matching
----------------

.. rubric:: Dynamic Semantics

:def_p:`fls_tlwr4u7bjhh5`
:term:`Pattern matching` that involves a :term:`pattern` and a context
:term:`value` proceeds as follows:

#. :def_p:`fls_67ajub7d2b4c`
   For each :term:`pattern-without-alternation` of the :term:`pattern`

   #. :def_p:`fls_62626ws222op`
      If the :term:`pattern-without-alternation` is an :term:`identifier
      pattern`, then perform :term:`identifier pattern matching`.

   #. :def_p:`fls_q0z46h1gnzez`
      If the :term:`pattern-without-alternation` is a :term:`literal pattern`,
      then perform :term:`literal pattern matching`.

   #. :def_p:`fls_1r0vm6rg13o9`
      If the :term:`pattern-without-alternation` is a :term:`parenthesized
      pattern`, then perform :term:`parenthesized pattern matching`.

   #. :def_p:`fls_am5h8r887bz5`
      If the :term:`pattern-without-alternation` is a :term:`path pattern`, then
      perform :term:`path pattern matching`.

   #. :def_p:`fls_eppmiloh7bgg`
      If the :term:`pattern-without-alternation` is a :term:`range pattern`,
      then perform :term:`range pattern matching`.

   #. :def_p:`fls_gwc08xayno7q`
      If the :term:`pattern-without-alternation` is a :term:`reference pattern`,
      then perform :term:`reference pattern matching`.

   #. :def_p:`fls_19iygu12s315`
      If the :term:`pattern-without-alternation` is a :term:`slice pattern`,
      then perform :term:`slice pattern matching`.

   #. :def_p:`fls_r307spfk6cs9`
      If the :term:`pattern-without-alternation` is a :term:`struct pattern`,
      then perform :term:`struct pattern matching`.

   #. :def_p:`fls_drb114dtvlpt`
      If the :term:`pattern-without-alternation` is a :term:`tuple pattern`,
      then perform :term:`tuple pattern matching`.

   #. :def_p:`fls_qhdofvbso3gl`
      If the :term:`pattern-without-alternation` is a :term:`tuple struct
      pattern`, then perform :term:`tuple struct pattern matching`.

   #. :def_p:`fls_uxysntb3u03j`
      If the :term:`pattern-without-alternation` is a :term:`wildcard pattern`,
      then perform :term:`wildcard pattern matching`.

   #. :def_p:`fls_wh201rmh6u6d`
      Otherwise :term:`pattern matching` fails.

:def_p:`fls_vstdqifqipbh`
Only the :term:`binding`\ s of a matched :term:`pattern-without-alternation` are
introduced to the corresponding :term:`scope`.

Identifier Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_4f3lzw64myhk`
An :term:`identifier pattern` with :term:`keyword` ``mut`` shall require that
the context :term:`value` is a :term:`mutable place expression`.

.. rubric:: Dynamic Semantics

:def_p:`fls_wauqwmdbcpna`
:def_term:`Identifier pattern matching` proceeds as follows:

#. :def_p:`fls_3jyog8n6x2aa`
   If the :term:`identifier pattern` has a :term:`bound pattern`, then

   #. :def_p:`fls_w637uvlbzsyo`
      Performed :term:`pattern matching` with the :term:`bound pattern` and the
      same context :term:`value`.

   #. :def_p:`fls_arz8ik3gf6u4`
      If matching the :term:`bound pattern` fails, then matching fails.

#. :def_p:`fls_u6o5ndnezwbe`
   The context :term:`value` is bound to the :term:`binding` of the
   :term:`identifier pattern` according to the :term:`binding mode`.

#. :def_p:`fls_h1er04t0yta7`
   Matching succeeds.

Literal Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_fqkhhgushje9`
:def_term:`Literal pattern matching` proceeds as follows:

#. :def_p:`fls_m01eo9sa55s`
   If the :term:`literal` of the :term:`literal pattern` and the context
   :term:`value` are equal, then matching succeeds.

#. :def_p:`fls_294jtwbfq3p9`
   Otherwise matching fails.

Parenthesized Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_jajvvwoy3399`
:def_term:`Parenthesized pattern matching` performs :term:`pattern matching`
with its :term:`subpattern` and the same context :term:`value`.

Path Pattern Matching
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_4faltss0xbn4`
:def_term:`Path pattern matching` proceeds as follows:

#. :def_p:`fls_fqt5w3qsykca`
   If the :term:`constant` the :term:`path` of the :term:`path pattern` resolved
   to and the context :term:`value` are equal, then matching succeeds.

#. :def_p:`fls_h3y8r4298s53`
   Otherwise matching fails.

Range Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_mrh9vfdek5fi`
:def_term:`Range pattern matching` proceeds as follows:

#. :def_p:`fls_7nxkgls0a5os`
   If the :term:`range pattern` is expressed as a :term:`half-open range
   pattern` and the context :term:`value` is in the inclusive range from
   the :term:`range pattern low bound` to the maximum :term:`value` of the
   :term:`range pattern low bound`'s :term:`type`, then matching succeeds.

#. :def_p:`fls_6kgj2fjccoig`
   If the :term:`range pattern` is expressed as either an :term:`inclusive range
   pattern` or an :term:`obsolete range pattern` and the context :term:`value`
   is in the inclusive range from the :term:`range pattern low bound` to the
   :term:`range pattern high bound`, then matching succeeds.

#. :def_p:`fls_n4t3xah1pk7i`
   Otherwise matching fails.

Reference Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_ysfgdzaiww8z`
:def_term:`Reference pattern matching` proceeds as follows:

#. :def_p:`fls_7rxnxd4ybxbt`
   Dereference the context :term:`value`.

#. :def_p:`fls_l2nwz166curc`
   Perform :term:`pattern matching` with its :term:`subpattern` and the
   dereferenced :term:`value`.

Slice Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_hzyv4ofu0ny`
:def_term:`Slice pattern matching` proceeds as follows if the expected
:term:`type` is an :term:`array type` or a :term:`slice type`:

#. :def_p:`fls_69bnxrtj0nar`
   If the number of :term:`subpattern`\ s of the :term:`slice pattern` is
   greater than the length of the context :term:`value`, then matching fails.

#. :def_p:`fls_twhwiy213ibf`
   If the number of :term:`subpattern`\ s of the :term:`slice pattern`
   is less than the size of the context :term:`value` and one of those
   :term:`subpattern`\ s is not a :term:`rest pattern`, then matching fails.

#. :def_p:`fls_ei7y4ul6n6hu`
   For each :term:`subpattern` of the :term:`slice pattern`

   #. :def_p:`fls_ad2jud5h1rfp`
      Perform :term:`pattern matching` with the :term:`subpattern` and the
      corresponding :term:`value` from the context :term:`value`, ignoring
      :term:`rest pattern`\ s.

   #. :def_p:`fls_pc97m47p34wq`
      If matching the :term:`subpattern` fails, then matching fails.

Struct Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_evuhau2rwm8i`
:def_term:`Struct pattern matching` proceeds as follows:

#. :def_p:`fls_osc8aj1htgqo`
   If the number of :term:`subpattern`\ s of the :term:`struct pattern` is
   greater than the number of :term:`field`\ s of the context :term:`value`,
   then this is a static error.

#. :def_p:`fls_bde1hpvrosui`
   If the number of :term:`subpattern`\ s of the :term:`struct pattern` is less
   than the number of :term:`field`\ s of the context :term:`value` and one
   of those :term:`subpattern`\ s is not a :term:`rest pattern`, then matching
   fails.

#. :def_p:`fls_447s4hc07ozn`
   For each :term:`subpattern` of the :term:`struct pattern`

   #. :def_p:`fls_vfdb1i5l41yk`
      If the :term:`subpattern` is a :term:`shorthand deconstructor`, then the
      corresponding :term:`field` of the context :term:`value` is bound to the
      :term:`binding` of the :term:`shorthand deconstructor` according to the
      :term:`binding mode`.

   #. :def_p:`fls_yfk52fr7trw3`
      Otherwise perform :term:`pattern matching` with the :term:`subpattern` and
      the corresponding :term:`field` from the context :term:`value`, ignoring
      :term:`rest pattern`\ s.

   #. :def_p:`fls_6sdcykdrpe5d`
      If matching the :term:`subpattern` fails, then matching fails.

Tuple Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_w4xypnrnhycb`
:def_term:`Tuple pattern matching` proceeds as follows:

#. :def_p:`fls_9lujsrwrrvqs`
   If the number of :term:`subpattern`\ s of the :term:`tuple pattern` is
   greater than the number of :term:`field`\ s of the context :term:`value`,
   then this is a static error.

#. :def_p:`fls_wsd605jlyzs2`
   If the number of :term:`subpattern`\ s of the :term:`tuple pattern` is less
   than the number of :term:`field`\ s of the context :term:`value` and one of
   those :term:`subpattern`\ s is not a :term:`rest pattern`, then this is a
   static error.

#. :def_p:`fls_vnx1bpval595`
   For each :term:`subpattern` of the :term:`tuple pattern`

   #. :def_p:`fls_dzf32f40y7fr`
      Perform :term:`pattern matching` with the :term:`subpattern` and the
      corresponding :term:`field` from the context :term:`value`, ignoring
      :term:`rest pattern`\ s.

   #. :def_p:`fls_krl32txvxxkz`
      If matching the :term:`subpattern` fails, then matching fails.

Tuple Struct Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_dexg9g9cct30`
:def_term:`Tuple struct pattern matching` proceeds as follows:

#. :def_p:`fls_daj9ds77r27b`
   If the number of :term:`subpattern`\ s of the :term:`tuple struct pattern`
   is greater than the number of :term:`field`\ s of the context :term:`value`,
   then this is a static error.

#. :def_p:`fls_z8dqrgmk24n0`
   If the number of :term:`subpattern`\ s of the :term:`tuple struct pattern` is
   less than the number of :term:`field`\ s of the context :term:`value` and one
   of those :term:`subpattern`\ s is not a :term:`rest pattern`, then this is a
   static error.

#. :def_p:`fls_boc7juqj69hw`
   For each :term:`subpattern` of the :term:`tuple struct pattern`

   #. :def_p:`fls_4dr1stiw82v9`
      Otherwise perform :term:`pattern matching` with the :term:`subpattern` and
      the corresponding :term:`field` from the context :term:`value`, ignoring
      :term:`rest pattern`\ s.

   #. :def_p:`fls_h14emtt6iyk3`
      If matching the :term:`subpattern` fails, then matching fails.

Wildcard Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:def_p:`fls_dvk7r1gf7pwp`
:def_term:`Wildcard pattern matching` proceeds as follows:

#. :def_p:`fls_e0uprihqn1y6`
   The context :term:`value` is matched unconditionally.

#. :def_p:`fls_ljcq2vyo052q`
   Matching succeeds.

