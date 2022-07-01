.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Lexical Elements
================

:def_p:`fls_pqwpf87b84tr`
The text of a Rust program consists of :term:`module`\ s organized into
:term:`source file`\ s. The text of a :term:`source file` is a sequence of
:term:`lexical element`\ s, each composed of characters, whose rules are
presented in this chapter.

Character Set
-------------

:def_p:`fls_itcth8292ud6`
The program text of a Rust program is written using the :term:`Unicode`
character set.

.. rubric:: Syntax

:def_p:`fls_vfx8byq5zo8t`
A character is defined by this document for each cell in the coding space
described by :term:`Unicode`, regardless of whether or not :term:`Unicode`
allocates a character to that cell.

:def_p:`fls_pvslhm3chtlb`
A :def_term:`whitespace character` is one of the following characters:

* :def_p:`fls_a5ec9cpn4sc8`
  0x09 (horizontal tabulation)

* :def_p:`fls_dgyrj49y3c7c`
  0x0A (new line)

* :def_p:`fls_5ocmngyur7by`
  0x0B (vertical tabulation)

* :def_p:`fls_1aj0rgi9kpib`
  0x0C (form feed)

* :def_p:`fls_bfzdxsbq2c2q`
  0x0D (carriage return)

* :def_p:`fls_vw0kq2y1o63m`
  0x20 (space)

* :def_p:`fls_ao296bmamwzh`
  0x85 (next line)

* :def_p:`fls_6kymhq7embdh`
  0x200E (left-to-right mark)

* :def_p:`fls_8mxmrxvhn3by`
  0x200F (right-to-left mark)

* :def_p:`fls_zfs15iel08y0`
  0x2029 (paragraph separator)

:def_p:`fls_7eifv4ksunu1`
A :term:`whitespace string` is a string that consists of one or more
:term:`whitespace character`\ s.

.. rubric:: Legality Rules

:def_p:`fls_2brw13n9ldgy`
The coded representation of a character is tool-defined.

Lexical Elements, Separators, and Punctuation
---------------------------------------------

.. rubric:: Syntax

.. syntax::

   LexicalElement ::=
       Comment
     | Identifier
     | Keyword
     | Literal
     | Punctuation

   Punctuation ::=
       Delimiter
     | $$+$$
     | $$-$$
     | $$*$$
     | $$/$$
     | $$%$$
     | $$^$$
     | $$!$$
     | $$&$$
     | $$|$$
     | $$&&$$
     | $$||$$
     | $$<<$$
     | $$>>$$
     | $$+=$$
     | $$-=$$
     | $$*=$$
     | $$/=$$
     | $$&=$$
     | $$^=$$
     | $$&=$$
     | $$|=$$
     | $$<<=$$
     | $$>>=$$
     | $$=$$
     | $$==$$
     | $$!=$$
     | $$>$$
     | $$<$$
     | $$>=$$
     | $$<=$$
     | $$@$$
     | $$_$$
     | $$.$$
     | $$..$$
     | $$...$$
     | $$..=$$
     | $$,$$
     | $$;$$
     | $$:$$
     | $$::$$
     | $$->$$
     | $$=>$$
     | $$#$$
     | $$$$$
     | $$?$$

   Delimiter ::=
       $${$$
     | $$}$$
     | $$[$$
     | $$]$$
     | $$($$
     | $$)$$

.. rubric:: Legality Rules

:def_p:`fls_d4nvxsvxj537`
The text of a :term:`source file` is a sequence of separate :term:`lexical
element`\ s. The meaning of a program depends only on the particular sequence of
:term:`lexical element`\ s, excluding :term:`non-doc comment`\ s.

:def_p:`fls_a1zylpqha73x`
A :term:`lexical element` is the most basic syntactic element in program text.

:def_p:`fls_jy6wifn5r2bu`
The text of a :term:`source file` is divided into :term:`line`\ s.

:def_p:`fls_efdfq9nhpmp5`
A :term:`line` is a sequence of zero or more characters followed by an
:term:`end of line`.

:def_p:`fls_go25sisi5fdp`
The representation of an :term:`end of line` is tool-defined.

:def_p:`fls_a6t53o8h1vdk`
A :term:`separator` is a character or a string that separates adjacent
:term:`lexical element`\ s. A :term:`whitespace string` is a :term:`separator`.

:def_p:`fls_8fv63w6f4udl`
A :def_term:`simple punctuator` is one of the following characters:

.. syntax::

   	$$+$$
   $$-$$
   $$*$$
   $$/$$
   $$%$$
   $$^$$
   $$!$$
   $$&$$
   $$|$$
   $$=$$
   $$>$$
   $$<$$
   $$@$$
   $$_$$
   $$.$$
   $$,$$
   $$;$$
   $$:$$
   $$#$$
   $$$$$
   $$?$$
   $${$$
   $$}$$
   $$[$$
   $$]$$
   $$($$
   $$)$$

:def_p:`fls_es0tz1q9cmoo`
A :def_term:`compound punctuator` is one of the following two or more adjacent
special characters:

.. syntax::

   	$$&&$$
   $$||$$
   $$<<$$
   $$>>$$
   $$+=$$
   $$-=$$
   $$*=$$
   $$/=$$
   $$%=$$
   $$^=$$
   $$&=$$
   $$|=$$
   $$<<=$$
   $$>>=$$
   $$==$$
   $$!=$$
   $$>=$$
   $$<=$$
   $$..$$
   $$...$$
   $$..=$$
   $$::$$
   $$->$$
   $$=>$$

:def_p:`fls_vm86olkeecer`
The following :term:`compound punctuator`\ s are *flexible compound
punctuators*.

.. syntax::

   	$$&&$$
   $$||$$
   $$<<$$
   $$>>$$

:def_p:`fls_5zxdgxy8tjrq`
A :term:`flexible compound punctuator` may be treated as a single
:term:`compound punctuator` or two adjacent :term:`simple punctuator`\ s.

:def_p:`fls_x89vkq9rwlyt`
Each of the special characters listed for single character :term:`punctuator` is
a :term:`single punctuator` except if this character is used as a character of
a :term:`compound punctuator`, or a character of a :term:`character literal`, a
:term:`comment`, a :term:`numeric literal`, or a :term:`string literal`.

:def_p:`fls_bo3xh8r60ji1`
The following names are used when referring to :term:`punctuator`\ s:

.. list-table::

   * - .. rubric:: punctuator
     - .. rubric:: name
   * - .. rubric:: ``+``
     - :def_p:`fls_o0p0k7s1htgx`
       Plus
   * - .. rubric:: ``-``
     - :def_p:`fls_3mlq3zyt0kwy`
       Minus
   * - .. rubric:: ``*``
     - :def_p:`fls_ev6pe6molg3t`
       Star
   * - .. rubric:: ``/``
     - :def_p:`fls_vmdextc3u0t3`
       Slash
   * - .. rubric:: ``%``
     - :def_p:`fls_po1jyhaz5umy`
       Percent
   * - .. rubric:: ``^``
     - :def_p:`fls_kuqjk34aqx0f`
       Caret
   * - .. rubric:: ``!``
     - :def_p:`fls_nxzhksyqxmcg`
       Not
   * - .. rubric:: ``&``
     - :def_p:`fls_2ekl3lfdlpby`
       And
   * - .. rubric:: ``|``
     - :def_p:`fls_gqtnh8o60en3`
       Or
   * - .. rubric:: ``&&``
     - :def_p:`fls_tm34dx9px4j7`
       And and, lazy boolean and
   * - .. rubric:: ``||``
     - :def_p:`fls_7nb986dlm77i`
       Or or, lazy boolean or
   * - .. rubric:: ``<<``
     - :def_p:`fls_bhdfooj0t2dz`
       Shift left
   * - .. rubric:: ``>>``
     - :def_p:`fls_hmzqin3pxm37`
       Shift right
   * - .. rubric:: ``+=``
     - :def_p:`fls_k4r8gvob7uk6`
       Plus equals
   * - .. rubric:: ``-=``
     - :def_p:`fls_xijsv9vl73wj`
       Minus equals
   * - .. rubric:: ``*=``
     - :def_p:`fls_gyuswf1lrdaa`
       Start equals
   * - .. rubric:: ``/=``
     - :def_p:`fls_4ws4v0ow9epr`
       Slash equals
   * - .. rubric:: ``%=``
     - :def_p:`fls_b05knjvh40sx`
       Percent equals
   * - .. rubric:: ``^=``
     - :def_p:`fls_jxvijqju32y7`
       Caret equals
   * - .. rubric:: ``&=``
     - :def_p:`fls_2fds81qi64az`
       And equals
   * - .. rubric:: ``|=``
     - :def_p:`fls_h04jhzgwnajj`
       Or equals
   * - .. rubric:: ``<<=``
     - :def_p:`fls_wanmi8okel6p`
       Shift left equals
   * - .. rubric:: ``>>=``
     - :def_p:`fls_wufecquvnn1`
       Shift right equals
   * - .. rubric:: ``=``
     - :def_p:`fls_fxd70q9edhvf`
       Equals
   * - .. rubric:: ``==``
     - :def_p:`fls_kkvoncx8zqg0`
       Equals equals
   * - .. rubric:: ``!=``
     - :def_p:`fls_l96w0nsnaj2p`
       Not equals
   * - .. rubric:: ``>``
     - :def_p:`fls_kjx3xe1hve6b`
       Greater than
   * - .. rubric:: ``<``
     - :def_p:`fls_a1t0fkvtqhym`
       Less than
   * - .. rubric:: ``>=``
     - :def_p:`fls_c7c3tmfucz2a`
       Greater than equals
   * - .. rubric:: ``<=``
     - :def_p:`fls_3nto2gyi6x9m`
       Less than equals
   * - .. rubric:: ``@``
     - :def_p:`fls_aeoi7spez3y4`
       At
   * - .. rubric:: ``_``
     - :def_p:`fls_5oo1dhhjjv5v`
       Underscore
   * -
     - :def_p:`fls_lmgrlo42u1iq`
       Dot
   * -
     - :def_p:`fls_f4ivhxufu9jp`
       Dot dot, exclusive range
   * -
     - :def_p:`fls_nuy7q4b75owa`
       Dot dot dot, ellipsis
   * - .. rubric:: ``=``
     - :def_p:`fls_vgu5u6r95g9u`
       Dot dot equals, inclusive range
   * - .. rubric:: ``,``
     - :def_p:`fls_tm95hdtzn0pp`
       Comma
   * - .. rubric:: ``;``
     - :def_p:`fls_567xv1ojt810`
       Semicolon
   * - .. rubric:: ``:``
     - :def_p:`fls_c7q8ivzabguc`
       Colon
   * - .. rubric:: ``::``
     - :def_p:`fls_fnbkp67q3xo9`
       Path separator
   * - .. rubric:: ``->``
     - :def_p:`fls_hghwfv5njnr2`
       Right arrow
   * - .. rubric:: ``=>``
     - :def_p:`fls_8oszerfd3w89`
       Fat arrow
   * - .. rubric:: ``#``
     - :def_p:`fls_a8h0wxtsxwxl`
       Pound
   * - .. rubric:: ``$``
     - :def_p:`fls_wh9m39naxo2t`
       Dollar sign
   * - .. rubric:: ``?``
     - :def_p:`fls_qvvmcjp1k3e5`
       Question mark
   * - .. rubric:: ``{``
     - :def_p:`fls_4s8idmjzmsvj`
       Left curly brace
   * - .. rubric:: ``}``
     - :def_p:`fls_tpwmb5v1t8hz`
       Right curly brace
   * - .. rubric:: ``[``
     - :def_p:`fls_39pagkfcmbuv`
       Left square bracket
   * - .. rubric:: ``]``
     - :def_p:`fls_hec1sut23331`
       Right square bracket
   * - .. rubric:: ``(``
     - :def_p:`fls_3hc23jw55x6j`
       Left parenthesis
   * - .. rubric:: ``)``
     - :def_p:`fls_rvqg3ca3a7cs`
       Right parenthesis

Identifiers
-----------

.. rubric:: Syntax

.. syntax::


   Identifier ::=
       NonKeywordIdentifier
     | RawIdentifier

   IdentifierList ::=
       Identifier ($$,$$ Identifier)* $$,$$?

   NonKeywordIdentifier ::=
       PureIdentifier
     | WeakKeyword

   RawIdentifier ::=
       $$r#$$ (PureIdentifier | RawIdentifierKeyword)
   PureIdentifier ::=
       XID_Start XID_Continue*
     | $$_$$ XID_Continue+

   IdentifierOrUnderscore ::=
       Identifier
     | $$_$$

   Renaming ::=
       $$as$$ IdentifierOrUnderscore


:def_p:`fls_ls7ymvgd5kfa`
A :def_syntax:`RawIdentifierKeyword` is any :term:`keyword` in category
:syntax:`Keyword`, except ``crate``, ``self``, ``Self``, and ``super``.

.. rubric:: Legality Rules

:def_p:`fls_xsdmun5uqy4c`
An :term:`identifier` is a :term:`lexical element` that refers to a
:term:`name`.

:def_p:`fls_ktnf6zkrdy45`
A :term:`pure identifier` is an :term:`identifier` that does not include
:term:`weak keyword`\ s.

:def_p:`fls_jpecw46eh061`
A :term:`pure identifier` shall follow the specification in Unicode Standard
Annex #31 for :term:`Unicode` version 13.0, with the following profile:

* :def_p:`fls_lwcflgezgs5z`
  ``Start = XID_Start``, plus character 0x5F (low line).

* :def_p:`fls_uts0hywaw1rq`
  ``Continue = XID_Continue``

* :def_p:`fls_lju1avcn0pfd`
  ``Medial =`` empty

:def_p:`fls_cs6cbw625np1`
Characters 0x200C (zero width non-joiner) and 0x200D (zero width joiner) shall
not appear in a :term:`pure identifier`.

:def_p:`fls_irwcldiotei2`
A :term:`pure identifier` shall be restricted to characters in category
:syntax:`AsciiCharacter` in the following contexts:

* :def_p:`fls_6qo63nlkr0s8`
  :term:`External crate import`\ s,

* :def_p:`fls_w473jevurlt1`
  :term:`Name`\ s of :term:`external crate`\ s represented in a :term:`simple
  path`, when the :term:`simple path` that starts with namespace qualifier
  ``::``,

* :def_p:`fls_mt1u4m3simhc`
  :term:`Name`\ s of :term:`outline module`\ s that lack attribute
  :codeterm:`path`,

* :def_p:`fls_e2v58o233lvd`
  :term:`Name`\ s of :term:`item`\ s that are subject to attribute
  :codeterm:`no_mangle`,

* :def_p:`fls_op0lp1i065di`
  :term:`Name`\ s of :term:`item`\ s within :term:`external block`\ s.

:def_p:`fls_vde7gev5rz4q`
:term:`Identifier`\ s are normalized using Normalization Form C as defined in
Unicode Standard Annex #15.

:def_p:`fls_j9yh8j8jgdeu`
Two :term:`identifier`\ s are considered the same if they consist of the same
sequence of characters after performing normalization.

:def_p:`fls_jejt5z8m1yew`
:term:`Procedural macro`\ s and :term:`declarative macro`\ s shall receive
normalized :term:`identifier`\ s in their input.

.. rubric:: Examples

.. syntax::

   foo
   _identifier
   r#true
   Москва
   東京

Literals
--------

.. rubric:: Syntax

.. syntax::

   Literal ::=
       BooleanLiteral
     | ByteLiteral
     | CharacterLiteral
     | NumericLiteral
     | StringLiteral

.. rubric:: Legality Rules

:def_p:`fls_s76un78zyd0j`
A :term:`literal` is a fixed :term:`value` in program text.

Byte Literals
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ByteLiteral ::=
       ByteStringLiteral
     | RawByteStringLiteral
     | SimpleByteLiteral

.. rubric:: Legality Rules

:def_p:`fls_q0qwr83frszx`
A :term:`byte literal` is a :term:`literal` that denotes a fixed byte
:term:`value`.

Simple Byte Literals
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   SimpleByteLiteral ::=
       $$b'$$ SimpleByteContent $$'$$

   SimpleByteContent ::=
       ByteEscape
     | SimpleByteCharacter

   ByteEscape ::=
       AsciiControlEscape
     | $$\0$$
     | $$\"$$
     | $$\'$$
     | $$\t$$
     | $$\n$$
     | $$\r$$
     | $$\\$$

   AsciiControlEscape ::=
       $$\x$$ AsciiControlCharacter


:def_p:`fls_3hpzf12h60u4`
A :def_syntax:`SimpleByteCharacter` is any character in category
:syntax:`AsciiCharacter` except characters 0x09 (horizontal tabulation),
0x0A (new line), 0x0D (carriage return), 0x27 (apostrophe), and 0x5C (reverse
solidus).

.. rubric:: Legality Rules

:def_p:`fls_i67zy734o6e3`
A :term:`simple byte literal` is a :term:`byte literal` that consists of exactly
one byte character.

:def_p:`fls_fggytrv5jvw0`
The :term:`type` of a :term:`simple byte literal` is :codeterm:`u8`.

.. rubric:: Examples

.. code-block:: text

   b'h'
   b'\n'
   b'\x1B'

Byte String Literals
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   ByteStringLiteral ::=
       $$b"$$ ByteStringContent* $$"$$

   ByteStringContent ::=
       ByteEscape
     | ByteStringCharacter
     | StringContinuation


:def_p:`fls_3dcqhuosqb84`
A :def_syntax:`ByteStringCharacter` is any character in category
:syntax:`AsciiCharacter` except characters 0x0D (carriage return), 0x22
(quotation mark), and 0x5C (reverse solidus).

.. rubric:: Legality Rules

:def_p:`fls_moe3zfx39ox2`
A :term:`byte string literal` is a :term:`byte literal` that consists of
multiple byte characters.

:def_p:`fls_vffxb6arj9jf`
The :term:`type` of a :term:`byte string literal` of size ``N`` is ``&'static
[u8; N]``.

.. rubric:: Examples

.. code-block:: text

   b""
   b"a\tb"
   b"Multi\
   line"

Raw Byte String Literals
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   RawByteStringLiteral ::=
       $$br$$ RawByteStringContent

   RawByteStringContent ::=
       NestedRawByteStringContent
     | RawByteStringCharacter

   NestedRawByteStringContent ::=
       $$#$$ RawByteStringContent $$#$$

   RawByteStringCharacter ::=
       $$"$$ AsciiCharacter* $$"$$


.. rubric:: Legality Rules

:def_p:`fls_yyw7nv651580`
A :term:`raw byte string literal` is a :term:`simple byte literal` that does not
recognize :term:`escaped character`\ s.

:def_p:`fls_5ybq0euwya42`
The :term:`type` of a :term:`raw byte string literal` of size ``N`` is
``&'static [u8; N]``.

.. rubric:: Examples

.. code-block:: text

   br""
   br#""#
   br##"left #"# right"##

Numeric Literals
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   NumericLiteral ::=
       FloatLiteral
     | IntegerLiteral

.. rubric:: Legality Rules

:def_p:`fls_fqpqnku27v99`
A :term:`numeric literal` is a :term:`literal` that denotes a number.

Integer Literals
^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   IntegerLiteral ::=
       IntegerContent IntegerSuffix?

   IntegerContent ::=
       BinaryLiteral
     | DecimalLiteral
     | HexadecimalLiteral
     | OctalLiteral

   BinaryLiteral ::=
       $$0b$$ BinaryDigitOrUnderscore* BinaryDigit BinaryDigitOrUnderscore*

   BinaryDigitOrUnderscore ::=
       BinaryDigit
     | $$_$$

   BinaryDigit ::=
       [$$0$$-$$1$$]

   DecimalLiteral ::=
       DecimalDigit DecimalDigitOrUnderscore*

   DecimalDigitOrUnderscore ::=
       DecimalDigit
     | $$_$$

   DecimalDigit ::=
       [$$0$$-$$9$$]

   HexadecimalLiteral ::=
       $$0x$$ HexadecimalDigitOrUnderscore* HexadecimalDigit HexadecimalDigitOrUnderscore*
   HexadecimalDigitOrUnderscore ::=
       HexadecimalDigit
     | $$_$$
   HexadecimalDigit ::=
       [$$0$$-$$9$$ $$a$$-$$f$$ $$A$$-$$F$$]
   OctalLiteral ::=
       $$0o$$ OctalDigitOrUnderscore* OctalDigit OctalDigitOrUnderscore*
   OctalDigitOrUnderscore ::=
       OctalDigit
     | $$_$$
   OctalDigit ::=
       [$$0$$-$$7$$]

   IntegerSuffix ::=
       SignedIntegerSuffix
     | UnsignedIntegerSuffix

   SignedIntegerSuffix ::=
       $$i8$$
     | $$i16$$
     | $$i32$$
     | $$i64$$
     | $$i128$$
     | $$isize$$
   UnsignedIntegerSuffix ::=
       $$u8$$
     | $$u16$$
     | $$u32$$
     | $$u64$$
     | $$u128$$
     | $$usize$$


.. rubric:: Legality Rules

:def_p:`fls_vkk2krfn93ry`
An :term:`integer literal` is a :term:`numeric literal` that denotes a whole
number.

:def_p:`fls_nxqncu5yq4eu`
A :term:`binary literal` is an :term:`integer literal` in base 2.

:def_p:`fls_rn8xfd66yvst`
A :term:`decimal literal` is an :term:`integer literal` in base 10.

:def_p:`fls_2268lchxkzjp`
A :term:`hexadecimal literal` is an :term:`integer literal` in base 16.

:def_p:`fls_4v7awnutbpoe`
An :term:`octal literal` is an :term:`integer literal` in base 8.

:def_p:`fls_f1e29aj0sqvl`
An :term:`integer suffix` is a component of an :term:`integer literal` that
specifies an explicit :term:`integer type`.

:def_p:`fls_u83mffscqm6`
A :term:`suffixed integer` is an :term:`integer literal` with an :term:`integer
suffix`.

:def_p:`fls_g10nuv14q4jn`
An :term:`unsuffixed integer` is an :term:`integer literal` without an
:term:`integer suffix`.

:def_p:`fls_hpkkvuj1z1ez`
The :term:`type` of a :term:`suffixed integer` is determined by its
:term:`integer suffix` as follows:

* :def_p:`fls_7yq2fep848ky`
  Suffix ``i8`` specifies type :codeterm:`i8`.

* :def_p:`fls_bzm8lwq3qlat`
  Suffix ``i16`` specifies type :codeterm:`i16`.

* :def_p:`fls_l4cx36brc1r5`
  Suffix ``i32`` specifies type :codeterm:`i32`.

* :def_p:`fls_wthchinwx996`
  Suffix ``i64`` specifies type :codeterm:`i64`.

* :def_p:`fls_7uoaet2pm3am`
  Suffix ``i128`` specifies type :codeterm:`i128`.

* :def_p:`fls_p4rw583o2qbi`
  Suffix ``isize`` specifies type :codeterm:`isize`.

* :def_p:`fls_xrv4q56lmoo3`
  Suffix ``u8`` specifies type :codeterm:`u8`.

* :def_p:`fls_66e3q5um6cwc`
  Suffix ``u16`` specifies type :codeterm:`u16`.

* :def_p:`fls_5asyk66y7c9d`
  Suffix ``u32`` specifies type :codeterm:`u32`.

* :def_p:`fls_76fifqjka0lx`
  Suffix ``u64`` specifies type :codeterm:`u64`.

* :def_p:`fls_fsaimo419gf0`
  Suffix ``u128`` specifies type :codeterm:`u128`.

* :def_p:`fls_hvzacbu7yiwc`
  Suffix ``usize`` specifies type :codeterm:`usize`.

:def_p:`fls_50qipwqi3arw`
The :term:`type` of an :term:`unsuffixed integer` is determined by :term:`type
inference` as follows:

* :def_p:`fls_idzhusp2l908`
  If an :term:`integer type` can be uniquely determined from the surrounding
  program context, then the :term:`unsuffixed integer` has that :term:`type`.

* :def_p:`fls_qqrqyc6uhol`
  If the program context under-constrains the :term:`type`, then the
  :term:`inferred type` is :codeterm:`i32`.

* :def_p:`fls_pexi5jazthq6`
  If the program context over-constrains the :term:`type`, then this is
  considered a static type error.

.. rubric:: Examples

.. code-block:: text

   0b0010_1110_u8
   1___2_3
   0x4D8a
   0o77_52i128

Float Literals
^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   FloatLiteral ::=
       DecimalLiteral $$.$$
     | DecimalLiteral FloatExponent
     | DecimalLiteral $$.$$ DecimalLiteral FloatExponent?
     | DecimalLiteral ($$.$$ DecimalLiteral)? FloatExponent? FloatSuffix

   FloatExponent ::=
       ExponentLetter ExponentSign? ExponentMagnitude

   ExponentLetter ::=
       $$e$$
     | $$E$$

   ExponentSign ::=
       $$+$$
     | $$-$$

   ExponentMagnitude ::=
       DecimalDigitOrUnderscore* DecimalDigit DecimalDigitOrUnderscore*

   FloatSuffix ::=
       $$f32$$
     | $$f64$$


.. rubric:: Legality Rules

:def_p:`fls_rzi7oeqokd6e`
A :term:`float literal` is a :term:`numeric literal` that denotes a fractional
number.

:def_p:`fls_2ru1zyrykd37`
A :term:`float suffix` is a component of a :term:`float literal` that specifies
an explicit :term:`floating-point type`.

:def_p:`fls_21mhnhplzam7`
A :term:`suffixed float` is a :term:`float literal` with a :term:`float suffix`.

:def_p:`fls_drqh80k0sfkb`
An :term:`unsuffixed float` is a :term:`float literal` without a :term:`float
suffix`.

:def_p:`fls_cbs7j9pjpusw`
The :term:`type` of a :term:`suffixed float` is determined by the :term:`float
suffix` as follows:

* :def_p:`fls_b9w7teaw1f8f`
  Suffix ``f32`` specifies type :codeterm:`f32`.

* :def_p:`fls_eawxng4ndhv0`
  Suffix ``f64`` specifies type :codeterm:`f64`.

:def_p:`fls_yuhza1muo7o`
The :term:`type` of an :term:`unsuffixed float` is determined by :term:`type
inference` as follows:

* :def_p:`fls_4sxt1ct7fyen`
  If a :term:`floating-point type` can be uniquely determined from the
  surrounding program context, then the :term:`unsuffixed float` has that
  :term:`type`.

* :def_p:`fls_wa72rssp0jnt`
  If the program context under-constrains the :term:`type`, then the
  :term:`inferred type` is :codeterm:`f64`.

* :def_p:`fls_x2cw7g8g56f8`
  If the program context over-constrains the :term:`type`, then this is
  considered a static type error.

.. rubric:: Examples

.. code-block:: text

   45.
   8E+1_820
   3.14e5
   8_031.4_e-12f64

Character Literals
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   CharacterLiteral ::=
       $$'$$ CharacterContent $$'$$

   CharacterContent ::=
       AsciiEscape
     | CharacterLiteralCharacter

   AsciiEscape ::=
     | UnicodeEscape
     | $$\0$$
     | $$\"$$
     | $$\'$$
     | $$\t$$
     | $$\n$$
     | $$\r$$
     | $$\\$$
   UnicodeEscape ::=
       $$\u{$$ (HexadecimalDigit $$_$$*)1-6 $$}$$

   AsciiCharacter ::=
       [$$\u{00}$$-$$\u{7F}$$]

:def_p:`fls_5vi0uc2oshd`
An :def_syntax:`AsciiControlCharacter` is any character in category
:syntax:`AsciiCharacter` whose General Category is defined to be "Control".

:def_p:`fls_j9q9ton57rvl`
A :def_syntax:`CharacterLiteralCharacter` is any :term:`Unicode` character
except characters 0x09 (horizontal tabulation), 0x0A (new line), 0x0D (carriage
return), 0x27 (apostrophe), and 0x5c (reverse solidus).

.. rubric:: Legality Rules

:def_p:`fls_vag2oy4q7d4n`
A :term:`character literal` is a :term:`literal` that denotes a fixed
:term:`Unicode` character.

:def_p:`fls_n8z6p6g564r2`
The :term:`type` of a :term:`character literal` is :codeterm:`char`.

.. rubric:: Examples

.. syntax::

   'a'
   '\t'
   '\x1b'
   '\u1F30'

String Literals
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   StringLiteral ::=
       RawStringLiteral
     | SimpleStringLiteral

.. rubric:: Legality Rules

:def_p:`fls_7fuctvtvdi7x`
A :term:`string literal` is a :term:`literal` that consists of multiple
characters.

Simple String Literals
^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   SimpleStringLiteral ::=
       $$"$$ SimpleStringContent* $$"$$

   SimpleStringContent ::=
       AsciiEscape
     | SimpleStringCharacter
     | StringContinuation


:def_p:`fls_1pdzwkt5txfj`
A :def_syntax:`SimpleStringCharacter` is any :term:`Unicode` character except
characters 0x0D (carriage return), 0x22 (quotation mark), and 0x5C (reverse
solidus).

:def_p:`fls_wawtu6j3fiqn`
:def_syntax:`StringContinuation` is the character sequence 0x5C 0x0A (reverse
solidus, new line).

.. rubric:: Legality Rules

:def_p:`fls_ycy5ee6orjx`
A :term:`simple string literal` is a :term:`string literal` where the characters
are :term:`Unicode` characters.

:def_p:`fls_6nt5kls21xes`
The :term:`type` of a :term:`simple string literal` is ``&'static str``.

.. rubric:: Examples

.. code-block:: text

   ""
   "cat"
   "\tcol\nrow"
   "bell\x07"
   "\uB80a"
   "\
   multi\
   line\
   string"

Raw String Literals
^^^^^^^^^^^^^^^^^^^

.. rubric:: Syntax

.. syntax::

   RawStringLiteral ::=
       $$r$$ RawStringContent

   RawStringContent ::=
       NestedRawStringContent
     | RawStringCharacter

   NestedRawStringContent ::=
       $$#$$ RawStringContent $$#$$

   RawStringCharacter ::=
       $$"$$ ~[$$\r$$]* $$"$$

.. rubric:: Legality Rules

:def_p:`fls_36suwhbwmq1t`
A :term:`raw string literal` is a :term:`simple string literal` that does not
recognize :term:`escaped character`\ s.

:def_p:`fls_ms43w1towz40`
The :term:`type` of a :term:`raw string literal` is ``&'static str``.

.. rubric:: Examples

.. code-block:: text

   r""
   r#""#
   r##"left #"# right"##

Boolean Literals
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   BooleanLiteral ::=
       $$false$$
     | $$true$$

.. rubric:: Legality Rules

:def_p:`fls_1lll64ftupjd`
A :term:`boolean literal` is a :term:`literal` that denotes the truth
:term:`value`\ s of logic and Boolean algebra.

:def_p:`fls_pgngble3ilyx`
The :term:`type` of a :term:`boolean literal` is :codeterm:`bool`.

.. rubric:: Examples

.. code-block:: text

   true

Comments
--------

.. rubric:: Syntax

.. syntax::

   Comment ::=
       BlockCommentOrDoc
     | LineCommentOrDoc

   BlockCommentOrDoc ::=
       BlockComment
     | InnerBlockDoc
     | OuterBlockDoc

   LineCommentOrDoc ::=
       LineComment
     | InnerLineDoc
     | OuterLineDoc

   LineComment ::=
       $$//$$
     | $$//$$ (~[$$!$$ $$/$$] | $$//$$) ~[$$\n$$]*

   BlockComment ::=
       $$/*$$ (~[$$!$$ $$*$$] | $$**$$ | BlockCommentOrDoc) (BlockCommentOrDoc | ~[$$*/$$])* $$*/$$
     | $$/**/$$
     | $$/***/$$

   InnerBlockDoc ::=
       $$/*!$$ (BlockCommentOrDoc | ~[$$*/$$ $$\r$$])* $$*/$$

   InnerLineDoc ::=
       $$//!$$ ~[$$\n$$ $$\r$$]*

   OuterBlockDoc ::=
       $$/**$$ (~[$$*$$] | BlockCommentOrDoc) (BlockCommentOrDoc | ~[$$*/$$ $$\r$$])* $$*/$$

   OuterLineDoc ::=
       $$///$$ (~[$$/$$] ~[$$\n$$ $$\r$$]*)?

.. rubric:: Legality Rules

:def_p:`fls_8obn3dtzpe5f`
A :term:`comment` is a :term:`lexical element` that acts as an annotation or an
explanation in program text.

:def_p:`fls_qsbnl11be35s`
A :term:`block comment` is a :term:`comment` that spans one or more
:term:`line`\ s.

:def_p:`fls_nayisy85kyq2`
A :term:`line comment` is a :term:`comment` that spans exactly one :term:`line`.

:def_p:`fls_k3hj30hjkdhw`
An :term:`inner block doc` is a :term:`block comment` that applies to an
enclosing non-:term:`comment` :term:`construct`.

:def_p:`fls_tspijl68lduc`
An :term:`inner line doc` is a :term:`line doc` that applies to an enclosing
non-:term:`comment` :term:`construct`.

:def_p:`fls_63gzofa9ktic`
An :term:`outer block doc` is a :term:`block comment` that applies to a
subsequent non-:term:`comment` :term:`construct`.

:def_p:`fls_scko7crha0um`
An :term:`outer line doc` is a :term:`line comment` that applies to a subsequent
non-:term:`comment` :term:`construct`.

:def_p:`fls_7n6d3jx61ose`
A :term:`doc comment` is a :term:`comment` class that includes :term:`inner
block doc`\ s, :term:`inner line doc`\ s, :term:`outer block doc`\ s, and
:term:`outer line doc`\ s.

:def_p:`fls_6fxcs17n4kw`
Character 0x0D (carriage return) shall not appear in a :term:`comment`.

:def_p:`fls_uze7l7cxonk1`
:term:`Block comment`\ s, :term:`inner block doc`\ s, and :term:`outer block
doc`\ s shall extend one or more :term:`line`\ s.

:def_p:`fls_gy23lwlqw2mc`
:term:`Line comment`\ s, :term:`inner line doc`\ s, and :term:`outer line doc`\
s shall extend exactly one :term:`line`.

:def_p:`fls_w7d0skpov1is`
:term:`Outer block doc`\ s and :term:`outer line doc`\ s shall apply to a
subsequent non-:term:`comment` :term:`construct`.

:def_p:`fls_32ncjvj2kn7z`
:term:`Inner block doc`\ s and :term:`inner line doc`\ s shall apply to an
enclosing non-:term:`comment` :term:`construct`.

:def_p:`fls_ok0zvo9vcmzo`
:term:`Inner block doc`\ s, :term:`inner line doc`\ s, :term:`outer block
doc`\ s, and :term:`outer line doc`\ s are equivalent to  :term:`attribute`
:codeterm:`doc`.

.. rubric:: Examples

.. code-block:: text

   // This is a stand-alone line comment. So is the next line.

   ////

   /* This is a stand-alone
      block comment. */

   /// This outer line comment applies to commented_module.

   /** This outer block comment applies to commented_module,
       and is considered documentation. */

   pub mod commented_module {

       //! This inner line comment applies to commented_mode.

       /*! This inner block comment applies to commented_module,
           and is considered documentation. */
   }

Keywords
--------

.. rubric:: Syntax

.. syntax::

   Keyword ::=
       ReservedKeyword
     | StrictKeyword
     | WeakKeyword

.. rubric:: Legality Rules

:def_p:`fls_dti0uu7rz81w`
A :term:`keyword` is a word in program text that has special meaning.

:def_p:`fls_sxg1o4oxql51`
:term:`Keyword`\ s are case sensitive.

Strict Keywords
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   StrictKeyword ::=
       $$as$$
     | $$async$$
     | $$await$$
     | $$break$$
     | $$const$$
     | $$continue$$
     | $$crate$$
     | $$dyn$$
     | $$enum$$
     | $$extern$$
     | $$false$$
     | $$fn$$
     | $$for$$
     | $$if$$
     | $$impl$$
     | $$in$$
     | $$let$$
     | $$loop$$
     | $$match$$
     | $$mod$$
     | $$move$$
     | $$mut$$
     | $$pub$$
     | $$ref$$
     | $$return$$
     | $$self$$
     | $$Self$$
     | $$static$$
     | $$struct$$
     | $$super$$
     | $$trait$$
     | $$true$$
     | $$type$$
     | $$unsafe$$
     | $$use$$
     | $$where$$
     | $$while$$

.. rubric:: Legality Rules

:def_p:`fls_bsh7qsyvox21`
A :term:`strict keyword` is a :term:`keyword` that always holds its special
meaning.

Reserved Keywords
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ReservedKeyword ::=
       $$abstract$$
     | $$become$$
     | $$box$$
     | $$do$$
     | $$final$$
     | $$macro$$
     | $$override$$
     | $$priv$$
     | $$try$$
     | $$typeof$$
     | $$unsized$$
     | $$virtual$$
     | $$yield$$

.. rubric:: Legality Rules

:def_p:`fls_w4b97ewwnql`
A :term:`reserved keyword` is a :term:`keyword` that is not yet in use.

Weak Keywords
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   WeakKeyword ::=
       $$macro_rules$$
     | $$'static$$
     | $$union$$

.. rubric:: Legality Rules

:def_p:`fls_bv87t1gvj7bz`
A :term:`weak keyword` is a :term:`keyword` whose special meaning depends on
the context.

:def_p:`fls_bl55g03jmayf`
Word ``macro_rules`` acts as a :term:`keyword` only when used in the context of
a :syntax:`MacroRulesDefinition`.

:def_p:`fls_c354oryv513p`
Word ``'static`` acts as a :term:`keyword` only when used in the context of a
:syntax:`LifetimeIndication`.

:def_p:`fls_r9fhuiq1ys1p`
Word ``union`` acts as a :term:`keyword` only when used in the context of a
:syntax:`UnionDeclaration`.

