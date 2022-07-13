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

:dp:`fls_imegtsi224ts`
A :t:`pattern` is a :t:`construct` that matches a :t:`value` which satisfies all
the criteria of the :t:`pattern`.

:dp:`fls_mp6i4blzexnu`
A :t:`pattern-without-alternation` is a :t:`pattern` that cannot be alternated.

:dp:`fls_6xx34zr069bj`
A :t:`subpattern` is a :t:`pattern` nested within another pattern.

:dp:`fls_8xzjb0yzftkd`
A :t:`pattern` has a :t:`type`, with the exception of the :t:`rest pattern` if
it is not the inner :t:`pattern` of a :t:`slice pattern` or the :t:`pattern` of
a possibly nested :t:`identifier pattern` of a :t:`slice pattern`\ ``.``

:dp:`fls_cma5t8waon0x`
The :t:`expected type` of a :t:`pattern` is the :t:`type` of the :t:`value` the
:t:`pattern` is being matched against.

:dp:`fls_8luyomzppck`
Any two :t:`[pattern-without-alternation]s` that are or-ed using character 0x7C
(vertical line) are subject to the following restrictions:

* :dp:`fls_rpvdfmy3n05a`
  The :t:`[type]s` of the two :t:`[pattern-without-alternation]s` shall be
  :t:`unifiable`.

* :dp:`fls_kv533rntni1x`
  The :t:`[binding]s` of the two :t:`[pattern-without-alternation]s` shall
  be the same, shall have :t:`[unifiable type]s`, and shall have the same
  :t:`[binding mode]s`.

Identifier Patterns
~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   IdentifierPattern ::=
       $$ref$$? $$mut$$? Binding BoundPattern?

   BoundPattern ::=
       $$@$$ Pattern

.. rubric:: Legality Rules

:dp:`fls_uljdw9rf7ies`
An :t:`identifier pattern` is a :t:`pattern` that binds the :t:`value` it
matches to a :t:`binding`.

:dp:`fls_vy9uw586wy0d`
A :t:`bound pattern` is a :t:`pattern` that imposes a constraint on a related
:t:`identifier pattern`.

:dp:`fls_hqwt3fvr063y`
An :t:`identifier pattern` yields a :t:`binding`. An :t:`identifier pattern`
with :t:`keyword` ``mut`` yields a :t:`mutable binding`.

:dp:`fls_24c95c56tugl`
The :t:`identifier pattern` enters its :t:`binding` into :t:`pattern scope` in
the :t:`value namespace` if it does not resolve to a :t:`constant`.

:dp:`fls_twcavjk7iquy`
It is a static error if the :t:`identifier pattern` consists of anything other
than a :t:`binding` when the :t:`binding` resolves to a :t:`constant`.

:dp:`fls_hw26hy33guk5`
An :t:`identifier pattern` is :t:`irrefutable` when:

* :dp:`fls_svfxwz4yy5i`
  It has a :t:`bound pattern` and the :t:`bound pattern` is :t:`irrefutable`, or

* :dp:`fls_x6f6q22b5jpc`
  It does not have a :t:`bound pattern` and its :t:`binding` resolves to an
  :t:`irrefutable constant`.

* :dp:`fls_r2mb8v2lh3x0`
  It does not have a :t:`bound pattern` and its :t:`binding` does not resolve to
  a :t:`constant`.

:dp:`fls_7oioaitb075g`
If the :t:`identifier pattern` does not have a :t:`bound pattern`, then the
:t:`type` of its :t:`binding` is determined as follows:

* :dp:`fls_40qin0ss5sqd`
  If the :t:`identifier pattern` has only :t:`keyword` ``ref``, then the
  :t:`type` is ``& inferred_type``, where ``inferred_type`` is the :t:`type`
  determined by :t:`type inference`.

* :dp:`fls_pivz0v7ey6sw`
  If the :t:`identifier pattern` has :t:`keywords` ``ref`` ``mut``, then the
  :t:`type` is ``&mut inferred_type``, where ``inferred_type`` is the :t:`type`
  determined by :t:`type inference`.

* :dp:`fls_2ahkrddxwj1n`
  Otherwise the :t:`type` is ``inferred_type``, where ``inferred_type`` is the
  :t:`type` determined by :t:`type inference`.

:dp:`fls_eucnafj3uedy`
If the :t:`identifier pattern` has a :t:`bound pattern`, then the :t:`type` of
its :t:`binding` is determined as follows:

* :dp:`fls_f8zo4scodhcr`
  If the :t:`identifier pattern` has only :t:`keyword` ``ref``, then the
  :t:`type` ``& bound_pattern_type``, where ``bound_pattern_type`` is the
  :t:`type` of the :t:`bound pattern`.

* :dp:`fls_d3fs2h7oqjl0`
  If the :t:`identifier pattern` has :t:`keywords` ``ref mut``, then the
  :t:`type` is ``&mut bound_pattern_type``, where ``bound_pattern_type`` is the
  :t:`type` of the :t:`bound pattern`.

* :dp:`fls_exo8asevh5x1`
  Otherwise the :t:`type` is ``inferred_type``, where ``inferred_type`` is the
  :t:`type` determined by :t:`type inference`.

.. rubric:: Examples

:dp:`fls_sfyfdxhvhk44`
An identifier pattern in the context of a let expression.

.. code-block:: text

   let x = 42;

:dp:`fls_as0pqqmo1des`
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

:dp:`fls_pah15qa54irs`
A :t:`literal pattern` is a :t:`pattern` that matches a :t:`literal`.

:dp:`fls_v7iv7x9gy9qm`
A :t:`literal pattern` is always :t:`refutable`.

:dp:`fls_co60bzvwashg`
The :t:`type` of a :t:`literal pattern` is the :t:`type` of the specified
:t:`literal`.

.. rubric:: Examples

:dp:`fls_fqclaznjgtb1`
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

:dp:`fls_kvqzmt7my5dh`
A :t:`parenthesized pattern` is a :t:`pattern` that controls the precedence of
its :t:`[subpattern]s`.

:dp:`fls_mrjhpiq5refe`
A :t:`parenthesized pattern` is :t:`irrefutable` when its nested :t:`pattern`
is :t:`irrefutable`.

:dp:`fls_pe5kh8y8u664`
The :t:`type` of a :t:`parenthesized pattern` is the :t:`type` of its nested
:t:`pattern`.

.. rubric:: Examples

:dp:`fls_2xq8852gihn9`
See :p:`5.1.1. <fls_yeajwokikkdi>` for the declaration of ``x``.

.. code-block:: text

   let ref_x = &x;

:dp:`fls_2dmeukyjqz9y`
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

:dp:`fls_1crq0mexo5r1`
A :t:`path pattern` is a :t:`pattern` that matches a :t:`constant`, an :t:`enum
variant` without :t:`[field]s`, or a :t:`unit struct` indicated by a :t:`path`.

:dp:`fls_xz5otkhogn31`
A :t:`path pattern` expressed as a :t:`path-in-expression` shall refer to either
an :t:`associated constant`, or a :t:`constant`.

:dp:`fls_t8sjzsif2ilf`
When a :t:`path pattern` expressed as a :t:`path-in-expression` refers to
a :t:`constant`, the :t:`constant` shall not be of a :t:`union type`. If
the :t:`constant` is of an :t:`enum type` or :t:`struct type`, then the
:t:`constant` shall be subject to :t:`attribute` :c:`derive` with arguments
:std:`core::cmp::Eq` and :std:`core::cmp::PartialEq`.

:dp:`fls_bv9psmitxfuw`
A :t:`path pattern` expressed as a :t:`qualified path-in-expression` shall refer
to an :t:`associated constant`.

:dp:`fls_sl47k9oj5p7t`
A :t:`path pattern` is :t:`irrefutable` when it refers to:

* :dp:`fls_cfoy86mkmqa4`
  A :t:`constant` whose :t:`type` is :t:`irrefutable`, or

* :dp:`fls_rnppz6y5z8pi`
  An :t:`enum variant` of an :t:`enum type` with a single :t:`enum variant`
  where the :t:`[type]s` of all :t:`[field]s` are :t:`irrefutable`, or

* :dp:`fls_ag6m4mvpturw`
  A :t:`struct` where the :t:`[type]s` of all :t:`[field]s` are
  :t:`irrefutable`.

:dp:`fls_pedy2pqrvnx7`
The :t:`type` of a :t:`path pattern` is the :t:`type` of the :t:`constant`,
:t:`enum`, or :t:`struct` the :t:`path` resolved to.

.. rubric:: Examples

.. code-block:: text

   mod module {
   	pub const ZERO: i32 = 0;
   }

   enum Enum { Variant }

:dp:`fls_u59rilepu8z9`
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

:dp:`fls_okupyoav13rm`
A :t:`range pattern` is a :t:`pattern` that matches :t:`[value]s` which fall
within a range.

:dp:`fls_jhchm7dy927k`
A :t:`half-open range pattern` is a :t:`range pattern` with only a :t:`range
pattern low bound`.

:dp:`fls_q86j23iiqv8w`
An :t:`inclusive range pattern` is a :t:`range pattern` with both a :t:`range
pattern low bound` and a :t:`range pattern high bound`.

:dp:`fls_akf9x5r6e0ta`
An :t:`obsolete range pattern` is a :t:`range pattern` that uses obsolete syntax
to express an :t:`inclusive range pattern`.

:dp:`fls_vrpr6ttpfpal`
A :t:`range pattern bound` is a constraint on the range of a :t:`range pattern`.

:dp:`fls_nk48gregn3me`
A :t:`range pattern low bound` is a :t:`range pattern bound` that specifies the
start of a range.

:dp:`fls_83v1xqbebs58`
A :t:`range pattern high bound` is a :t:`range pattern bound` that specifies the
end of a range.

:dp:`fls_2hpuccwh2xml`
A :t:`half-open range pattern` shall appear within a :t:`parenthesized pattern`
when context is a :t:`slice pattern`.

:dp:`fls_9kk81isk0mlp`
The :t:`range pattern low bound` of an :t:`inclusive range pattern` shall be
less than or equal to its :t:`range pattern high bound`.

:dp:`fls_s2b5n4snc4d7`
An :t:`obsolete range pattern` is equivalent to an :t:`inclusive range pattern`.

:dp:`fls_4o4ge6x9a8rs`
A :t:`range pattern` is :t:`irrefutable` only when it spans the entire set of
possible :t:`[value]s` of a :t:`type`.

:dp:`fls_6o995ak4hywq`
The :t:`[type]s` of the :t:`range pattern low bound` and the :t:`range pattern
high bound` of a :t:`range pattern` shall be :t:`unifiable`.

:dp:`fls_3js1645tgh31`
The :t:`type` of a :t:`range pattern` is determined as follows:

* :dp:`fls_wfqrbwrogjnq`
  If the :t:`range pattern` is expressed as an :t:`inclusive range pattern` or
  an :t:`obsolete range pattern`, then the :t:`type` is the :t:`unified type` of
  the :t:`[type]s` of the :t:`range pattern low bound` and the :t:`range pattern
  high bound`.

* :dp:`fls_rgr7t33s0m7m`
  Otherwise the :t:`type` is the :t:`type` of the :t:`range pattern low bound`.

:dp:`fls_5ey5mj8t8knd`
A :t:`path-in-expression` of a :t:`range pattern` shall refer to a :t:`constant`
of a :t:`scalar type`.

:dp:`fls_z4js96mchcsv`
A :t:`qualified path-in-expression` of a :t:`range pattern` shall refer to an
:t:`associated constant` of a :t:`scalar type`.

.. rubric:: Examples

:dp:`fls_3wwpq8i6mo2a`
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

:dp:`fls_fhahcc1mz2qh`
A :t:`reference pattern` is a :t:`pattern` that dereferences a :t:`pointer` that
is being matched.

:dp:`fls_x0bmzl1315gq`
A :t:`reference pattern` is always :t:`irrefutable`.

:dp:`fls_fedo8zhgpla5`
The :t:`type` of a :t:`reference pattern` is determined as follows:

* :dp:`fls_30u9ij164ww3`
  If the :t:`reference pattern` appears with :t:`keyword` ``mut``,
  then the :t:`type` is ``&mut pattern_without_range_type``,
  where ``pattern_without_range_type`` is the :t:`type` of the
  :s:`PatternWithoutRange`.

* :dp:`fls_d1kc73hpncpo`
  If the :t:`reference pattern` appears without :t:`keyword`
  ``mut``, then the :t:`type` is ``& pattern_without_range_type``,
  where ``pattern_without_range_type`` is the :t:`type` of the
  :s:`PatternWithoutRange`.

.. rubric:: Examples

:dp:`fls_mpeuhov0umfa`
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

:dp:`fls_eso51epfofxb`
A :t:`rest pattern` is a :t:`pattern` that matches zero or more elements that
have not already been matched.

:dp:`fls_5a75a2y43uev`
A :t:`rest pattern` shall appear at most once within a :t:`slice pattern`, an
:t:`identifier pattern` of a :t:`slice pattern`, a :t:`tuple pattern`, and a
:t:`tuple struct pattern`.

:dp:`fls_rsqyza99vl3x`
A :t:`rest pattern` is always :t:`irrefutable`.

:dp:`fls_w1pw40phsv2o`
If a :t:`rest pattern` appears within a :t:`slice pattern` or the :t:`identifier
pattern` of a :t:`slice pattern`, then the :t:`type` of the :t:`rest pattern` is
determined as follows:

* :dp:`fls_x8ylgxrf9ca`
  If the :t:`type` of the :t:`slice pattern` is an :t:`array type`, then the
  :t:`type` is ``[T; N]`` where ``T`` is the :t:`element type` of the :t:`array
  type`, and ``N`` is the :t:`[array type]'s` size minus the number of matched
  elements of the :t:`slice pattern`.

* :dp:`fls_zgoke73xrhk3`
  If the :t:`type` of the :t:`slice pattern` is a :t:`slice type`, then the
  :t:`type` is that :t:`slice type`.

.. rubric:: Examples

:dp:`fls_bdcv6rwx0fsv`
A rest pattern in an identifier pattern of a slice pattern, followed by a rest
pattern in a slice pattern.

.. code-block:: text

   match slice {
       [1, 5, .., 7] => (),
       [start, end @ ..] => (),
   }

:dp:`fls_qz9guhlg19j3`
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

:dp:`fls_qqiu594hki8g`
A :t:`slice pattern` is a :t:`pattern` that matches :t:`[array]s` of fixed size
and :t:`[slice]s` of dynamic size.

:dp:`fls_h6x9xlxi7y5n`
A :t:`slice pattern` is :t:`irrefutable` when it refers to:

* :dp:`fls_jbmxu7y5fnm6`
  An :t:`array`, where each :t:`subpattern` is :t:`irrefutable`, or

* :dp:`fls_r78zzw7yyg34`
  A :t:`slice`, where the :s:`PatternList` consists of a single :t:`rest
  pattern`, or a single possibly nested :t:`identifier pattern` whose last
  :t:`bound pattern` is a :t:`rest pattern`.

:dp:`fls_ndor56nou676`
The :t:`type` of a :t:`slice pattern` is the same as the :t:`expected type`.

.. rubric:: Examples

.. syntax::

   let v = vec![1, 2, 3];

:dp:`fls_9yuobz1jsehf`
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

:dp:`fls_e093i77g3ju7`
A :t:`struct pattern` is a :t:`pattern` that matches a :t:`struct`.

:dp:`fls_mqhest816lo2`
A :t:`tuple struct pattern` is a :t:`struct pattern` that matches :t:`[enum
variant]s` and :t:`[tuple struct]s`.

:dp:`fls_d8cyh5v25s0x`
A :t:`union pattern` is a :t:`record struct pattern` that matches a :t:`union`.

:dp:`fls_aolvi75ck60i`
A :t:`unit struct pattern` is a :t:`struct pattern` that matches a :t:`unit
struct`.

:dp:`fls_77qdyt1lpd`
An :t:`indexed deconstructor` is a :t:`construct` that matches the position of a
:t:`field` of a :t:`tuple`.

:dp:`fls_khhozbtc23l1`
A :t:`named deconstructor` is a :t:`construct` that matches the :t:`name` of a
:t:`field` of a :t:`struct`.

:dp:`fls_1zi6zmayw792`
A :t:`shorthand deconstructor` is a :t:`construct` that matches the :t:`name` of
a :t:`field` of a :t:`struct`.

:dp:`fls_8ersn39rt5pd`
The :t:`deconstruction type` indicates the :t:`type` of the :t:`struct` being
deconstructed by a :t:`struct pattern`.

:dp:`fls_tfwwrovxeomp`
A :s:`RecordStructPattern` without a
:s:`RecordStructIndexedFieldDeconstructorList` is a :t:`record struct pattern`.

:dp:`fls_1tjp0z3v9ukg`
A :s:`TupleStructPattern` and a :s:`RecordStructPattern` without a
:s:`RecordStructFieldDeconstructorList` are :t:`[tuple struct pattern]s`.

:dp:`fls_y8qclm82nun8`
A :s:`RecordStructPattern` with a :s:`RecordStructFieldDeconstructorList` is a
:t:`union pattern`.

:dp:`fls_j1lux391rmgg`
A :s:`UnitStructPattern` and a :s:`RecordStructPattern`
without a :s:`RecordStructFieldDeconstructorList` and a
:s:`RecordStructIndexedFieldDeconstructorList` are :t:`[unit struct pattern]s`.

:dp:`fls_2rgip6uruvt5`
A :t:`struct pattern` is interpreted based on the :t:`deconstruction type`. It
is a static error if a :t:`struct pattern` cannot be interpreted.

:dp:`fls_wi3yo3z5mn5w`
A :t:`shorthand deconstructor` binds the :t:`value` of a matched :t:`field` to a
:t:`variable`. A :t:`shorthand deconstructor` with :t:`keyword` ``mut`` yields a
:t:`mutable` :t:`variable`.

:dp:`fls_g5t53fj9ghk0`
It is a static error if a :t:`shorthand deconstructor` has only :t:`keyword`
``ref`` or :t:`keywords` ``ref`` ``mut``, and its :t:`variable` shadows a
:t:`constant`.

:dp:`fls_5vjoxrgeq3bg`
A :t:`struct pattern` is :t:`irrefutable` when all of its :t:`[subpattern]s`
are :t:`irrefutable`.

Record Struct Patterns
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_y10l03ogbs1s`
The :t:`deconstruction type` of a :t:`record struct pattern` shall resolve to a
:t:`record struct type`.

:dp:`fls_tzbjwdk0xxui`
A :t:`named deconstructor` matches a :t:`field` of the :t:`deconstruction type`
when its :t:`identifier` and the :t:`name` of the :t:`field` are the same. Such
a :t:`named deconstructor` is a :dt:`matched named deconstructor`.

:dp:`fls_n5xx6urvj7xg`
The :t:`type` of a :t:`named deconstructor` and the :t:`type` of a matched
:t:`field` shall be :t:`unifiable`.

:dp:`fls_njzp6q2kfjb8`
A :t:`shorthand deconstructor` matches a :t:`field` of the :t:`deconstruction
type` when its :t:`name` and the :t:`name` of the :t:`field` are the same. Such
a :t:`shorthand deconstructor` is a :dt:`matched shorthand deconstructor`.

:dp:`fls_emq0uil5w7xm`
The :t:`type` of a :t:`shorthand deconstructor` and the :t:`type` of the matched
:t:`field` shall be :t:`unifiable`.

:dp:`fls_vai6qhy39zmz`
A :t:`shorthand deconstructor` is equivalent to a :t:`named deconstructor` where
the :t:`name` of the :t:`shorthand deconstructor` denotes the :t:`identifier`
of the :t:`named deconstructor` and the entire content of the :t:`shorthand
deconstructor` denotes the pattern of the :t:`named deconstructor`.

:dp:`fls_pzvz6l540atp`
For each :t:`field` of the :t:`deconstruction type`, the :t:`record struct
deconstructor` shall either:

* :dp:`fls_uoedp3g89mg`
  Contain a :t:`matched named deconstructor`, or

* :dp:`fls_rspzc5jqbysa`
  Contain a :t:`matched shorthand deconstructor`, or

* :dp:`fls_2l9wbc8sqtlo`
  Has a :s:`RecordStructRestPattern` or a
  :s:`RecordStructFieldDeconstructorList` with a :s:`RecordStructRestPattern`.

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

:dp:`fls_ec0o57hwg8ic`
The :t:`deconstruction type` of a :t:`tuple struct pattern` shall resolve to a
:t:`tuple struct type`.

:dp:`fls_3e2zaeqo1s96`
A :t:`tuple struct pattern` shall contain one :t:`subpattern` for each
:t:`field` of the :t:`deconstruction type`.

:dp:`fls_w936pvga6lgn`
A :t:`subpattern` of a :t:`tuple struct pattern` matches a :t:`field` of the
:t:`deconstruction type` when its position and the position of the :t:`field`
in the :t:`deconstruction type` are the same. Such a :t:`subpattern` is a
:dt:`matched subpattern`.

:dp:`fls_aeh8bzh59m05`
The :t:`type` of the :t:`subpattern` of a :t:`tuple struct pattern` and the
:t:`type` of the matched :t:`field` shall be :t:`unifiable`.

:dp:`fls_s7u5ghr13ib7`
An :t:`index deconstructor` matches a :t:`field` of the :t:`deconstruction
type` when its :t:`tuple index` and the position of the :t:`field` in the
:t:`deconstruction type` are the same. Such an :t:`index deconstructor` is a
:dt:`matched index deconstructor`.

:dp:`fls_x33civd9eptg`
For each :t:`field` of the :t:`deconstruction type`, the :t:`tuple struct
deconstructor` shall either:

* :dp:`fls_gr3kc7k1j2ou`
  Contain a :t:`matched index deconstructor`, or

* :dp:`fls_oc293y7fmn9f`
  Contain a :t:`matched subpattern`, or

* :dp:`fls_n5w52m48v8fh`
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

:dp:`fls_eytubf3jp1vy`
The :t:`deconstruction type` of a :t:`union pattern` shall resolve to a
:t:`union type`.

:dp:`fls_51yggy3rohm8`
The :s:`RecordStructFieldDeconstructorList` of a :t:`union pattern`
shall contain exactly one :s:`RecordStructFieldDeconstructor` and no
:s:`RecordStructRestPattern`.

:dp:`fls_cb5au9tab68o`
For the single :t:`field` of the :t:`deconstruction type`, a :t:`union
deconstructor` shall either:

* :dp:`fls_r0d6w9di8ega`
  Contain a :t:`matched named deconstructor`, or

* :dp:`fls_sm8o7cfb3q1k`
  Contain a :t:`matched shorthand deconstructor`.

:dp:`fls_gm45psu7l64e`
A :t:`union pattern` shall require :t:`unsafe context`.

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

:dp:`fls_9ilkcejibsjd`
The :t:`deconstruction type` of a :t:`unit struct deconstructor` shall resolve
to a :t:`unit struct type`.

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

:dp:`fls_e2manugp4e0b`
A :t:`tuple pattern` is a :t:`pattern` that matches a :t:`tuple` which satisfies
all criteria defined by its :t:`[subpattern]s`.

:dp:`fls_xk8udu4k61kj`
A :t:`tuple pattern` is :t:`irrefutable` when all of its :t:`[subpattern]s`
are :t:`irrefutable`.

:dp:`fls_yhcaz6v49ub2`
The :t:`type` of a :t:`tuple pattern` is the :t:`type` of the :t:`tuple` being
destructured.

.. rubric:: Examples

.. code-block:: text

   let pair = (1, "two");

:dp:`fls_8r81vtv5hnrd`
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

:dp:`fls_dreny9e0ei6r`
A :t:`wildcard pattern` is a :t:`pattern` that matches any single :t:`value`.

:dp:`fls_42fye1v0th8l`
A :t:`wildcard pattern` is always :t:`irrefutable`.

:dp:`fls_b87mvrcc13f2`
The :t:`type` of a :t:`wildcard pattern` is the :t:`type` of the :t:`value`
it matches.

.. rubric:: Examples

:dp:`fls_j3u6x1ensrbe`
A wildcard pattern in the context of a let statement. See :p:`5.1.10.
<fls_fo48m62q2y0v>` for the declaration of ``pair``.

.. code-block:: text

   let (first, _) = pair;

Refutability
------------

.. rubric:: Legality Rules

:dp:`fls_9ntc4qmjmo90`
:t:`Refutability` is a property of :t:`[pattern]s` that expresses the ability to
match all possible values of a :t:`type`.

:dp:`fls_9fjspnefoyvz`
An :t:`irrefutable pattern` is a :t:`pattern` that always matches the :t:`value`
it is being matched against.

:dp:`fls_uq7ftuuq1sig`
A :t:`refutable pattern` is a :t:`pattern` that has a possibility of not
matching the :t:`value` it is being matched against.

:dp:`fls_mtkx414qk66c`
An :t:`irrefutable type` is a :t:`type` that has at most one :t:`value`.

:dp:`fls_sccfjvu95qfr`
A :t:`refutable type` is a :t:`type` that has more than one :t:`value`.

:dp:`fls_l76ycteulo8e`
An :t:`irrefutable constant` is a :t:`constant` of an :t:`irrefutable type`.

:dp:`fls_lh0d85tl4qvy`
A :t:`refutable constant` is a :t:`constant` of a :t:`refutable type`.

.. rubric:: Examples

:dp:`fls_sgu9bnp7xajv`
``x`` is an irrefutable pattern because it always matches ``42``.

.. code-block:: text

   let x = 42;

:dp:`fls_cl1g4fxfa020`
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

:dp:`fls_vnh9wfrvumdz`
A :t:`binding` is a :t:`variable` of an :t:`identifier pattern` or a
:t:`shorthand deconstructor` that binds a matched :t:`value`.

:dp:`fls_jrv4ghj4fr20`
A :t:`binding pattern` is either an :t:`identifier pattern` or a :t:`shorthand
deconstructor`.

:dp:`fls_dqe75i8h2fie`
A :t:`non-reference pattern` is any :t:`pattern` except :t:`non-[binding
pattern]s`, :t:`[path pattern]s`, :t:`[reference pattern]s`, and :t:`[wildcard
pattern]s`.

:dp:`fls_y3wuvj1y5j20`
If a :t:`binding pattern` does not explicitly specify :t:`keyword` ``ref``,
:t:`keyword` ``mut``, or :t:`[keyword]s` ``ref mut``, then its :t:`binding mode`
uses the current :t:`binding mode` of :t:`pattern matching`.

:dp:`fls_55jtzh6a292x`
Initially, the :t:`binding mode` of a :t:`binding` is "by value".

:dp:`fls_qcaf2kup7zn0`
During the process of :t:`pattern matching`, each time a :t:`reference`
is matched against a :t:`non-[reference pattern]`, the :t:`reference` is
dereferenced and the :t:`binding mode` is updated as follows:

* :dp:`fls_6acdqz8rwnn`
  If the :t:`reference` is an :t:`immutable reference`, then the :t:`binding
  mode` is updated to "by reference".

* :dp:`fls_tv0avib387bv`
  If the :t:`reference` is a :t:`mutable reference` and the :t:`binding mode` is
  "by value", then the :t:`binding mode` is updated to "by mutable reference".

:dp:`fls_dbgmwldye42e`
The process repeats if the dereferenced :t:`value` is a :t:`reference`.

.. rubric:: Dynamic Semantics

:dp:`fls_t34oqarwcusu`
A :t:`[binding pattern]s` binds its :t:`binding` to a matched :t:`value` as
follows:

* :dp:`fls_7gxb74u1np36`
  If the :t:`binding mode` is "by reference" or the :t:`binding pattern`
  appears only with :t:`keyword` ``ref``, then the :t:`binding` is bound to a
  :t:`reference` of the matched :t:`value`.

* :dp:`fls_7y56d0ulxomf`
  If the :t:`binding mode` is "by mutable reference" or the :t:`binding pattern`
  appears with keywords ``ref`` ``mut``, then the :t:`binding` is bound to a
  :t:`mutable reference` of the matched :t:`value`.

* :dp:`fls_pxvtqxke1enp`
  If the :t:`binding mode` is "by value", then the :t:`binding` is bound
  to a copy of the matched :t:`value` if its :t:`type` implements the
  :std:`core::marker::Copy` :t:`trait`, otherwise the :t:`binding` is bound to
  the move of the matched :t:`value`.

Pattern Matching
----------------

.. rubric:: Dynamic Semantics

:dp:`fls_tlwr4u7bjhh5`
:t:`Pattern matching` that involves a :t:`pattern` and a context :t:`value`
proceeds as follows:

#. :dp:`fls_67ajub7d2b4c`
   For each :t:`pattern-without-alternation` of the :t:`pattern`

   #. :dp:`fls_62626ws222op`
      If the :t:`pattern-without-alternation` is an :t:`identifier pattern`,
      then perform :t:`identifier pattern matching`.

   #. :dp:`fls_q0z46h1gnzez`
      If the :t:`pattern-without-alternation` is a :t:`literal pattern`, then
      perform :t:`literal pattern matching`.

   #. :dp:`fls_1r0vm6rg13o9`
      If the :t:`pattern-without-alternation` is a :t:`parenthesized pattern`,
      then perform :t:`parenthesized pattern matching`.

   #. :dp:`fls_am5h8r887bz5`
      If the :t:`pattern-without-alternation` is a :t:`path pattern`, then
      perform :t:`path pattern matching`.

   #. :dp:`fls_eppmiloh7bgg`
      If the :t:`pattern-without-alternation` is a :t:`range pattern`, then
      perform :t:`range pattern matching`.

   #. :dp:`fls_gwc08xayno7q`
      If the :t:`pattern-without-alternation` is a :t:`reference pattern`, then
      perform :t:`reference pattern matching`.

   #. :dp:`fls_19iygu12s315`
      If the :t:`pattern-without-alternation` is a :t:`slice pattern`, then
      perform :t:`slice pattern matching`.

   #. :dp:`fls_r307spfk6cs9`
      If the :t:`pattern-without-alternation` is a :t:`struct pattern`, then
      perform :t:`struct pattern matching`.

   #. :dp:`fls_drb114dtvlpt`
      If the :t:`pattern-without-alternation` is a :t:`tuple pattern`, then
      perform :t:`tuple pattern matching`.

   #. :dp:`fls_qhdofvbso3gl`
      If the :t:`pattern-without-alternation` is a :t:`tuple struct pattern`,
      then perform :t:`tuple struct pattern matching`.

   #. :dp:`fls_uxysntb3u03j`
      If the :t:`pattern-without-alternation` is a :t:`wildcard pattern`, then
      perform :t:`wildcard pattern matching`.

   #. :dp:`fls_wh201rmh6u6d`
      Otherwise :t:`pattern matching` fails.

:dp:`fls_vstdqifqipbh`
Only the :t:`[binding]s` of a matched :t:`pattern-without-alternation` are
introduced to the corresponding :t:`scope`.

Identifier Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_4f3lzw64myhk`
An :t:`identifier pattern` with :t:`keyword` ``mut`` shall require that the
context :t:`value` is a :t:`mutable place expression`.

.. rubric:: Dynamic Semantics

:dp:`fls_wauqwmdbcpna`
:dt:`Identifier pattern matching` proceeds as follows:

#. :dp:`fls_3jyog8n6x2aa`
   If the :t:`identifier pattern` has a :t:`bound pattern`, then

   #. :dp:`fls_w637uvlbzsyo`
      Performed :t:`pattern matching` with the :t:`bound pattern` and the same
      context :t:`value`.

   #. :dp:`fls_arz8ik3gf6u4`
      If matching the :t:`bound pattern` fails, then matching fails.

#. :dp:`fls_u6o5ndnezwbe`
   The context :t:`value` is bound to the :t:`binding` of the :t:`identifier
   pattern` according to the :t:`binding mode`.

#. :dp:`fls_h1er04t0yta7`
   Matching succeeds.

Literal Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_fqkhhgushje9`
:dt:`Literal pattern matching` proceeds as follows:

#. :dp:`fls_m01eo9sa55s`
   If the :t:`literal` of the :t:`literal pattern` and the context :t:`value`
   are equal, then matching succeeds.

#. :dp:`fls_294jtwbfq3p9`
   Otherwise matching fails.

Parenthesized Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_jajvvwoy3399`
:dt:`Parenthesized pattern matching` performs :t:`pattern matching` with its
:t:`subpattern` and the same context :t:`value`.

Path Pattern Matching
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_4faltss0xbn4`
:dt:`Path pattern matching` proceeds as follows:

#. :dp:`fls_fqt5w3qsykca`
   If the :t:`constant` the :t:`path` of the :t:`path pattern` resolved to and
   the context :t:`value` are equal, then matching succeeds.

#. :dp:`fls_h3y8r4298s53`
   Otherwise matching fails.

Range Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_mrh9vfdek5fi`
:dt:`Range pattern matching` proceeds as follows:

#. :dp:`fls_7nxkgls0a5os`
   If the :t:`range pattern` is expressed as a :t:`half-open range pattern` and
   the context :t:`value` is in the inclusive range from the :t:`range pattern
   low bound` to the maximum :t:`value` of the :t:`[range pattern low bound]'s`
   :t:`type`, then matching succeeds.

#. :dp:`fls_6kgj2fjccoig`
   If the :t:`range pattern` is expressed as either an :t:`inclusive range
   pattern` or an :t:`obsolete range pattern` and the context :t:`value` is in
   the inclusive range from the :t:`range pattern low bound` to the :t:`range
   pattern high bound`, then matching succeeds.

#. :dp:`fls_n4t3xah1pk7i`
   Otherwise matching fails.

Reference Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_ysfgdzaiww8z`
:dt:`Reference pattern matching` proceeds as follows:

#. :dp:`fls_7rxnxd4ybxbt`
   Dereference the context :t:`value`.

#. :dp:`fls_l2nwz166curc`
   Perform :t:`pattern matching` with its :t:`subpattern` and the dereferenced
   :t:`value`.

Slice Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_hzyv4ofu0ny`
:dt:`Slice pattern matching` proceeds as follows if the expected :t:`type` is an
:t:`array type` or a :t:`slice type`:

#. :dp:`fls_69bnxrtj0nar`
   If the number of :t:`[subpattern]s` of the :t:`slice pattern` is greater than
   the length of the context :t:`value`, then matching fails.

#. :dp:`fls_twhwiy213ibf`
   If the number of :t:`[subpattern]s` of the :t:`slice pattern` is less than
   the size of the context :t:`value` and one of those :t:`[subpattern]s` is not
   a :t:`rest pattern`, then matching fails.

#. :dp:`fls_ei7y4ul6n6hu`
   For each :t:`subpattern` of the :t:`slice pattern`

   #. :dp:`fls_ad2jud5h1rfp`
      Perform :t:`pattern matching` with the :t:`subpattern` and the
      corresponding :t:`value` from the context :t:`value`, ignoring :t:`[rest
      pattern]s`.

   #. :dp:`fls_pc97m47p34wq`
      If matching the :t:`subpattern` fails, then matching fails.

Struct Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_evuhau2rwm8i`
:dt:`Struct pattern matching` proceeds as follows:

#. :dp:`fls_osc8aj1htgqo`
   If the number of :t:`[subpattern]s` of the :t:`struct pattern` is greater
   than the number of :t:`[field]s` of the context :t:`value`, then this is a
   static error.

#. :dp:`fls_bde1hpvrosui`
   If the number of :t:`[subpattern]s` of the :t:`struct pattern` is less
   than the number of :t:`[field]s` of the context :t:`value` and one of those
   :t:`[subpattern]s` is not a :t:`rest pattern`, then matching fails.

#. :dp:`fls_447s4hc07ozn`
   For each :t:`subpattern` of the :t:`struct pattern`

   #. :dp:`fls_vfdb1i5l41yk`
      If the :t:`subpattern` is a :t:`shorthand deconstructor`, then the
      corresponding :t:`field` of the context :t:`value` is bound to the
      :t:`binding` of the :t:`shorthand deconstructor` according to the
      :t:`binding mode`.

   #. :dp:`fls_yfk52fr7trw3`
      Otherwise perform :t:`pattern matching` with the :t:`subpattern` and the
      corresponding :t:`field` from the context :t:`value`, ignoring :t:`[rest
      pattern]s`.

   #. :dp:`fls_6sdcykdrpe5d`
      If matching the :t:`subpattern` fails, then matching fails.

Tuple Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_w4xypnrnhycb`
:dt:`Tuple pattern matching` proceeds as follows:

#. :dp:`fls_9lujsrwrrvqs`
   If the number of :t:`[subpattern]s` of the :t:`tuple pattern` is greater than
   the number of :t:`[field]s` of the context :t:`value`, then this is a static
   error.

#. :dp:`fls_wsd605jlyzs2`
   If the number of :t:`[subpattern]s` of the :t:`tuple pattern` is less than
   the number of :t:`[field]s` of the context :t:`value` and one of those
   :t:`[subpattern]s` is not a :t:`rest pattern`, then this is a static error.

#. :dp:`fls_vnx1bpval595`
   For each :t:`subpattern` of the :t:`tuple pattern`

   #. :dp:`fls_dzf32f40y7fr`
      Perform :t:`pattern matching` with the :t:`subpattern` and the
      corresponding :t:`field` from the context :t:`value`, ignoring :t:`[rest
      pattern]s`.

   #. :dp:`fls_krl32txvxxkz`
      If matching the :t:`subpattern` fails, then matching fails.

Tuple Struct Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_dexg9g9cct30`
:dt:`Tuple struct pattern matching` proceeds as follows:

#. :dp:`fls_daj9ds77r27b`
   If the number of :t:`[subpattern]s` of the :t:`tuple struct pattern` is
   greater than the number of :t:`[field]s` of the context :t:`value`, then this
   is a static error.

#. :dp:`fls_z8dqrgmk24n0`
   If the number of :t:`[subpattern]s` of the :t:`tuple struct pattern` is less
   than the number of :t:`[field]s` of the context :t:`value` and one of those
   :t:`[subpattern]s` is not a :t:`rest pattern`, then this is a static error.

#. :dp:`fls_boc7juqj69hw`
   For each :t:`subpattern` of the :t:`tuple struct pattern`

   #. :dp:`fls_4dr1stiw82v9`
      Otherwise perform :t:`pattern matching` with the :t:`subpattern` and the
      corresponding :t:`field` from the context :t:`value`, ignoring :t:`[rest
      pattern]s`.

   #. :dp:`fls_h14emtt6iyk3`
      If matching the :t:`subpattern` fails, then matching fails.

Wildcard Pattern Matching
~~~~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Dynamic Semantics

:dp:`fls_dvk7r1gf7pwp`
:dt:`Wildcard pattern matching` proceeds as follows:

#. :dp:`fls_e0uprihqn1y6`
   The context :t:`value` is matched unconditionally.

#. :dp:`fls_ljcq2vyo052q`
   Matching succeeds.

