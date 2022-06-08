.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

Types and Traits
================

.. rubric:: Syntax

.. syntax::

   TypeSpecification ::=
       ImplTraitTypeSpecification
     | TraitObjectTypeSpecification
     | TypeSpecificationWithoutBounds

   TypeSpecificationWithoutBounds ::=
       ArrayTypeSpecification
     | FunctionPointerTypeSpecification
     | ImplTraitTypeSpecificationOneBound
     | InferredType
     | MacroInvocation
     | NeverType
     | ParenthesizedTypeSpecification
     | QualifiedPathInType
     | RawPointerTypeSpecification
     | ReferenceTypeSpecification
     | SliceTypeSpecification
     | TraitObjectTypeSpecificationOneBound
     | TupleTypeSpecification
     | TypePath

   TypeAscription ::=
   	$$:$$ TypeSpecification

   TypeSpecificationList ::=
       TypeSpecification (, TypeSpecification)* $$,$$?

Type Classification
-------------------

.. rubric:: Legality Rules

:def_p:`fls_dkts9ebfagr8`
:term:`Type`\ s are organized in the following categories:

:def_p:`fls_ujw3ct31dvsm`
:term:`Scalar type`\ s

:def_p:`fls_gx43u5pj26fj`
:term:`Boolean typ`\ e

:def_p:`fls_n4poyotxxclx`
:codeterm:`Char` type

:def_p:`fls_6d4ezphob2u0`
:term:`Numeric type`\ s

:def_p:`fls_2fjqh7un1535`
:term:`Floating-point type`

:def_p:`fls_um0670gtd3l9`
:term:`Integer type`

:def_p:`fls_ba1tlf272eih`
:term:`Sequence type`\ s

:def_p:`fls_67j9offwxgn2`
	:term:`Array type`

:def_p:`fls_o0su3jpk4hwt`
	:term:`Slice type`

:def_p:`fls_s4j4jynk8mzt`
	:codeterm:`Str` type

:def_p:`fls_b0a7jigth629`
	:term:`Tuple type`

:def_p:`fls_dwqfryso6rb1`
:term:`Abstract data type`\ s

:def_p:`fls_y75f3dhqz7rt`
	:term:`Enum type`

:def_p:`fls_qjn499m6rygh`
	:term:`Struct type`

:def_p:`fls_hn1i8d2ovadd`
	:term:`Union type`

:def_p:`fls_d9192rnvhuma`
:term:`Function type`\ s

:def_p:`fls_ow6bsfpokrbm`
	:term:`Closure type`

:def_p:`fls_rchaffig8cp6`
	:term:`Function item type`

:def_p:`fls_si96b7wdbtbx`
:term:`Indirection type`\ s

:def_p:`fls_y0aau5qoivg6`
	:term:`Function pointer type`

:def_p:`fls_m0nk1eusynr6`
	:term:`Raw pointer type`

:def_p:`fls_mhoqiffzed7b`
	:term:`Reference type`

:def_p:`fls_cwg1hidfhslc`
:term:`Trait type`\ s

:def_p:`fls_5y2eyg2eikn`
	:term:`Impl trait type`

:def_p:`fls_ny406ulgmjgl`
	:term:`Trait object type`

:def_p:`fls_28l9j7jxmc4k`
Other types

:def_p:`fls_lojs93t8oabt`
	:term:`Generic type`

:def_p:`fls_tf9h682j53fr`
	:term:`Inferred type`

:def_p:`fls_x4aa87pf6mws`
	:term:`Never type`

:def_p:`fls_vekae2esxiw3`
	:term:`Parenthesized type`

Scalar Types
------------

Boolean Type
~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_73i0z1e05gg9`
:term:`B``ool``\ ` is a :term:`type` whose :term:`value`\ s denote the
truth :term:`value`\ s of logic and Boolean algebra.

:def_p:`fls_5po4i81eu79w`
:term:`Type` :codeterm:`bool` appears in the :term:`language prelude` under the
name ``bool``.

:def_p:`fls_dew85dke73e`
Boolean :term:`value` ``false`` has bit pattern ``0x00``. Boolean :term:`value`
``true`` has bit pattern ``0x01``.

:def_p:`fls_5fzwz6ek6q6o`
The following operations are defined on :term:`type` :codeterm:`bool`:

:def_p:`fls_af84me14frcb`
**Logical not**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: !a
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          true

:def_p:`fls_wjfwanoz619l`
**Logical and**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a & b
   * - .. code-block:: text

          true
     - .. code-block:: text

          true
     - .. code-block:: text

          true
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          false
     - .. code-block:: text

          false

:def_p:`fls_btosbwbx6bnx`
**Logical or**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a | b
   * - .. code-block:: text

          true
     - .. code-block:: text

          true
     - .. code-block:: text

          true
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
     - .. code-block:: text

          true
   * - .. code-block:: text

          false
     - .. code-block:: text

          true
     - .. code-block:: text

          true
   * - .. code-block:: text

          false
     - .. code-block:: text

          false
     - .. code-block:: text

          false

:def_p:`fls_d3d7khom8h4d`
**Logical exclusive or (xor)**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a ^ b
   * - .. code-block:: text

          true
     - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
     - .. code-block:: text

          true
   * - .. code-block:: text

          false
     - .. code-block:: text

          true
     - .. code-block:: text

          true
   * - .. code-block:: text

          false
     - .. code-block:: text

          false
     - .. code-block:: text

          false

:def_p:`fls_7cdep49lbj2e`
**Equality**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a == b
   * - .. code-block:: text

          true
     - .. code-block:: text

          true
     - .. code-block:: text

          true
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          false
     - .. code-block:: text

          true

:def_p:`fls_f3fbdgpkqq9u`
**Greater than**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a > b
   * - .. code-block:: text

          true
     - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          true
     - .. code-block:: text

          false
     - .. code-block:: text

          true
   * - .. code-block:: text

          false
     - .. code-block:: text

          true
     - .. code-block:: text

          false
   * - .. code-block:: text

          false
     - .. code-block:: text

          false
     - .. code-block:: text

          false

:def_p:`fls_3mxztfhi5j3v`
Operation ``a != b`` is equivalent to ``!(a == b)``.

:def_p:`fls_ime5vorei2ij`
Operation ``a >= b`` is equivalent to ``a == b | a > b``.

:def_p:`fls_7mpxq0tw29dm`
Operation ``a < b`` is equivalent to ``!(a >= b)``.

:def_p:`fls_f6qcrh8aly3s`
Operation ``a <= b`` shall be equivalent to ``a == b | a < b``.

.. rubric:: Undefined Behavior

:def_p:`fls_ao4vote4cjoo`
It is undefined behavior for a :term:`value` of :term:`type` :codeterm:`bool` to
have a bit pattern other than ``0x00`` and ``0x01``.

Char Type
~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_1zspq1ey5860`
:codeterm:`Char` is a :term:`type` whose :term:`value`\ s are represented as a
32-bit unsigned word in the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive
ranges of :term:`Unicode`.

.. rubric:: Undefined Behavior

:def_p:`fls_ad2hjusj02gt`
It is undefined behavior for a :term:`value` of :term:`type` :codeterm:`char`
to be outside the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive ranges
of :term:`Unicode`.

Numeric Types
~~~~~~~~~~~~~

Floating-point Types
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_ry6phakjrifz`
:term:`Type` :codeterm:`f32` is equivalent to the IEEE 754-2008
binary32 :term:`type`.

:def_p:`fls_u2xc7tvu8zg0`
:term:`Type` :codeterm:`f64` is equivalent to the IEEE 754-2008
binary64 :term:`type`.

Integer Types
^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_up0cb6ilql5z`
:term:`Unsigned integer type`\ s define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Minimum
     - .. rubric:: Maximum
   * - .. code-block:: text

          u8
     - :def_p:`fls_j7zpu5acjogi`
       0
     - :def_p:`fls_e7trf5q2xefp`
       28 - 1
   * - .. code-block:: text

          u16
     - :def_p:`fls_15xve3e45mj9`
       0
     - :def_p:`fls_89vhcaz40u4z`
       216 - 1
   * - .. code-block:: text

          u32
     - :def_p:`fls_mhpow26n19hq`
       0
     - :def_p:`fls_mcf9uu9h30d8`
       232 - 1
   * - .. code-block:: text

          u64
     - :def_p:`fls_wtknl1cd39z6`
       0
     - :def_p:`fls_mrirr8pi1why`
       264 - 1
   * - .. code-block:: text

          u128
     - :def_p:`fls_4e4kiovmadyc`
       0
     - :def_p:`fls_r2u1zscr18ui`
       2128 - 1

:def_p:`fls_pe2lc0ste7wt`
:term:`Type` :codeterm:`usize` has the same number of bits as the
platform's :term:`pointer type`, and at least 16-bits wide.

:def_p:`fls_vek0sa5my1ew`
:term:`Signed integer type`\ s define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Minimum
     - .. rubric:: Maximum
   * - .. code-block:: text

          i8
     - :def_p:`fls_51osgqu2jms`
       - (27)
     - :def_p:`fls_tmomu432jxip`
       27 - 1
   * - .. code-block:: text

          i16
     - :def_p:`fls_w99k7bha949l`
       - (215)
     - :def_p:`fls_zh8x3o13yyqj`
       215 - 1
   * - .. code-block:: text

          i32
     - :def_p:`fls_8byd1p7jyemx`
       - (231)
     - :def_p:`fls_rcu2z6m071mn`
       231 - 1
   * - .. code-block:: text

          i64
     - :def_p:`fls_1mbdmm789xn1`
       - (263)
     - :def_p:`fls_zgipnchn1b1u`
       263 - 1
   * - .. code-block:: text

          i128
     - :def_p:`fls_1zcuy1cvdigx`
       - (2127)
     - :def_p:`fls_dvq6nasa43ox`
       2127 - 1

:def_p:`fls_dscs4vb9pojs`
:term:`Type` :codeterm:`isize` has the same number of bits as the
platform's :term:`pointer type`, and at least 16-bits wide.

Sequence Types
--------------

Array Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ArrayTypeSpecification ::=
       $$[$$ ElementType $$;$$ SizeOperand $$]$$

   ElementType ::=
       TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_om347oyy6j7k`
An :term:`array type` is a :term:`sequence type` that represents a fixed
sequence of elements.

:def_p:`fls_s98ayieesffy`
The :term:`element type` shall be a :term:`fixed sized type`.

:def_p:`fls_k97o6yhxs293`
The :term:`size operand` shall be a :term:`constant expression`.

:def_p:`fls_2cb1tjye506a`
The :term:`type` of the :term:`size operand` is :term:`type` :codeterm:`usize`.

.. rubric:: Examples

:def_p:`fls_2lbiu6mzsx9g`
An array type in the context of a let statement:

.. code-block:: text


   let array: [i32; 3] = [1, 2, 3];

Slice Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   SliceTypeSpecification ::=
       $$[$$ ElementType $$]$$

.. rubric:: Legality Rules

:def_p:`fls_cwrued2a8mky`
A :term:`slice type` is a :term:`sequence type` that provides a view into a
sequence of elements.

:def_p:`fls_1abpbov74tb3`
The :term:`element type` shall be a :term:`fixed sized type`.

:def_p:`fls_pdhhxqw6t1v6`
A :term:`slice type` is a :term:`dynamically sized type`.

.. rubric:: Examples

:def_p:`fls_p73aro6a0jgu`
See :p:`4.3.1. <fls_eyrdzuv0r9l4>` for the declaration of ``array``.

.. code-block:: text

   let slice: &[i32] = &array[0..1];

Str Type
~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_s43he3ejkovu`
:codeterm:`Str` is a :term:`sequence type` that represents a :term:`slice` of 8-
bit unsigned bytes.

:def_p:`fls_z0icxb4wlwxi`
:term:`Type` :codeterm:`str` is a :term:`dynamically sized type`.

:def_p:`fls_umfsz5bf6bzm`
A :term:`value` of :term:`type` :codeterm:`str` shall denote a valid UTF-8
sequence of characters.

.. rubric:: Undefined Behavior

:def_p:`fls_677r3odv8gtx`
It is undefined behavior for a :term:`value` of :term:`type` :codeterm:`str` to
denote an invalid UTF-8 sequence of characters.

Tuple Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TupleTypeSpecification ::=
       $$($$ TupleFieldList? $$)$$
   TupleFieldList ::=
       TupleField (, TupleField)* ,?

   TupleField ::=
       TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_vptrk1jisw8z`
A :term:`tuple type` is a :term:`sequence type` that represents a heterogeneous
list of other :term:`type`\ s.

:def_p:`fls_t6xyop4gzmge`
If the :term:`type` of a :term:`tuple field` is a :term:`dynamically-sized
type`, then the :term:`tuple field` shall be the last :term:`tuple field` in
the :syntax:`TupleFieldList`.

.. rubric:: Examples

.. code-block:: text

   ()
   (char,)
   (i32, f64, Vec<String>)

Abstract Data Types
-------------------

Enumerated Type
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   EnumDeclaration ::=
       $$enum$$ Name GenericParameterList? WhereClause? $${$$ EnumVariantList? $$}$$

   EnumVariantList ::=
       EnumVariant ($$,$$ EnumVariant)* $$,$$?

   EnumVariant ::=
       OuterAttributeOrDoc* VisibilityModifier? Name EnumVariantKind?

   EnumVariantKind ::=
       DiscriminantInitializer
     | RecordEnumVariant
     | TupleEnumVariant

   DiscriminantInitializer ::=
       $$=$$ Expression

   RecordEnumVariant ::=
       $${$$ RecordStructFieldList? $$}$$

   TupleEnumVariant ::=
       $$($$ TupleStructFieldList? $$)$$

.. rubric:: Legality Rules

:def_p:`fls_lmh70mkpbumq`
An :term:`enum type` is an :term:`abstract data type` that contains :term:`enum
variant`\ s. An :term:`enum variant` specifies a :term:`value` of an :term:`enum
type`.

:def_p:`fls_gjxpwjdrkw73`
A :term:`zero-variant enum type` has no :term:`value`\ s.

:def_p:`fls_sznf8l3pcb2l`
The :term:`name` of an :term:`enum variant` shall denote a unique :term:`name`
within the related :syntax:`EnumDeclaration`.

:def_p:`fls_g364uorjxamc`
A :term:`discriminant initializer` shall be specified only when all :term:`enum
variant`\ s appear without an :syntax:`EnumVariantKind`.

:def_p:`fls_k6cixt755hbw`
The :term:`type` of the :term:`expression` of a :term:`discriminant initializer`
shall be either:

* :def_p:`fls_6icxfl3i01mp`
  The :term:`type` of the :term:`primitive representation` specified
  by :term:`attribute` :codeterm:`repr`, or

* :def_p:`fls_f2mkt3d36gjm`
  :term:`Type` :codeterm:`isize`.

:def_p:`fls_n3a1ypbf8amy`
The :term:`value` of the :term:`expression` of a :term:`discriminant
initializer` shall be a :term:`constant expression`.

:def_p:`fls_q7c9c01o1gr`
The :term:`value` of a :term:`discriminant` of an :term:`enum variant` is
determined as follows:

#. :def_p:`fls_yhf2fjikc142`
   If the :term:`enum variant` contains a :term:`discriminant initializer`, then
   the :term:`value` is the value of its :term:`expression`.

#. :def_p:`fls_xetzn4or8by`
   Else, if the :term:`enum variant` is the first :term:`enum variant` in
   the :syntax:`EnumVariantList`, then the :term:`value` is zero.

#. :def_p:`fls_a0xju8tceo6t`
   Otherwise the :term:`value` is one greater than the :term:`value` of
   the :term:`discriminant` of the previous :term:`enum variant`.

:def_p:`fls_9u2f466ln3l0`
It is a static error if two :term:`enum variant`\ s have the
same :term:`discriminant`\ s with the same :term:`value`.

:def_p:`fls_9p2ckx7yv3ff`
It is a static error if the :term:`value` of a :term:`discriminant` exceeds
the maximum :term:`value` of the :term:`type` of the :term:`expression` of
a :term:`discriminant initializer`.

.. rubric:: Undefined Behavior

:def_p:`fls_95r3puet64ft`
It is undefined behavior for a :term:`value` of an :term:`enum type` to
have a :term:`discriminant` other than a :term:`discriminant` specified by
the :term:`enum type`.

.. rubric:: Examples

.. code-block:: text

   enum ZeroVariantEnumType {}

   enum Animal {
      Cat,
      Dog(String),
      Otter { name: String, weight: f64, age: u8 }
   }

   enum Discriminants {
       First,       // The discriminant is 0.
       Second,      // The discriminant is 1.
       Third = 12,  // The discriminant is 12.
       Fourth,      // The discriminant is 13.
       Fifth = 34,  // The discriminant is 34.
       Sixth        // The discriminant is 35.
   }

Struct Type
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   StructDeclaration ::=
       RecordStructDeclaration
     | TupleStructDeclaration

   RecordStructDeclaration ::=
       $$struct$$ Name GenericParameterList? WhereClause? (RecordStructSpecification | $$;$$)

   RecordStructSpecification ::=
       $${$$ RecordStructFieldList? $$}$$

   RecordStructFieldList ::=
       RecordStructField ($$,$$ RecordStructField)* $$,$$?

   RecordStructField ::=
       OuterAttributeOrDoc* VisibilityModifier? Name TypeAscription

   TupleStructDeclaration ::=
       $$struct$$ Name GenericParameterList? $$($$ TupleStructFieldList? $$)$$ WhereClause? $$;$$

   TupleStructFieldList ::=
       TupleStructField ($$,$$ TupleStructField)* $$,$$?

   TupleStructField ::=
       OuterAttributeOrDoc* VisibilityModifier? TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_xrngz1s0sgrl`
A :term:`struct type` is an :term:`abstract data type` that is a product of
other :term:`type`\ s.

:def_p:`fls_enqcp4c40ai6`
The memory layout of a struct type is undefined, unless the struct type is
subject to attribute :codeterm:`repr`.

:def_p:`fls_ws5yi21isqo7`
The :term:`name` of a :term:`record struct field` shall denote a
unique :term:`name` within the :syntax:`RecordStructDeclaration`.

:def_p:`fls_j9sm39dr24nl`
If the :term:`type` of a :term:`record struct field` is a :term:`dynamically
sized type`, then :term:`the record struct field` shall be the
last :term:`record struct field` in the :syntax:`RecordStructFieldList`.

:def_p:`fls_mszd8fererpy`
If the :term:`type` of a :term:`tuple struct field` is a :term:`dynamically
sized type`, then the :term:`tuple struct field` shall be the last :term:`record
tuple field` in the :syntax:`TupleStructFieldList`.

.. rubric:: Examples

.. code-block:: text

   struct UnitStruct;

   struct AnimalRecordStruct {
       name: String,
       weight: f64,
       age: u8
   }

   struct AnimalTupleStruct (
       String,
       f64,
       u8
   );

Union Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   UnionDeclaration ::=
       $$union$$ Name GenericParameterList? WhereClause? RecordStructSpecification

.. rubric:: Legality Rules

:def_p:`fls_4qeqg2472po`
A :term:`union type` is an :term:`abstract data type` similar to a C-like union.

:def_p:`fls_6hahllqaxosb`
The memory :term:`layout` of a :term:`union type` is undefined, unless
the :syntax:`UnionDeclaration` is subject to :term:`attribute` :codeterm:`repr`.
All :term:`union field`\ s share a common storage.

:def_p:`fls_yzu706h2xyh9`
The :term:`size` of a :term:`union type` is determined by the :term:`size` of
its largest :term:`union field`.

:def_p:`fls_lllzb4biyl44`
The :term:`name` of a :term:`union field` shall denote a unique :term:`name`
within the related :syntax:`RecordStructDeclaration`.

.. rubric:: Examples

.. code-block:: text

   union LeafNode {
       int: i32,
       float: f32,
       double: f64
   }

Function Types
--------------

Closure Type
~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_js6nbvwzueep`
A :term:`closure type` is a unique anonymous :term:`function type` that
encapsulates all :term:`captured variable`\ s of a :term:`closure expression`.

:def_p:`fls_fsff7yijp0b6`
A :term:`closure type` implements
the :codeterm:`core::ops::FnOnce` :term:`trait`.

:def_p:`fls_3ekjj6owhtwr`
A :term:`closure type` that does not move out its :term:`captured variable`\ s
implements the :codeterm:`core::ops::FnMut` :term:`trait`.

:def_p:`fls_8lzolietnco3`
A :term:`closure type` that does not move or mutate its :term:`captured
variable`\ s implements the :codeterm:`core::ops::Fn` :term:`trait`.

:def_p:`fls_q35lm2anlxg6`
A :term:`closure type` that does not encapsulate :term:`captured variable`\ s
is :term:`coercible` to a :term:`function pointer type`.

:def_p:`fls_98xyl7ijujo6`
A :term:`closure type` implicitly implements
the :codeterm:`core::marker::Copy` :term:`trait` if

* :def_p:`fls_6tv172d6vmsv`
  It does not encapsulate :term:`captured variable`\ s :term:`by unique
  immutable borrow` or :term:`by mutable reference`, or

* :def_p:`fls_qsqa6158xkqk`
  The :term:`type`\ s of all :term:`captured variable`\ s implement
  the :codeterm:`core::marker::Copy` :term:`trait`.

:def_p:`fls_wg2a04xgomrd`
A :term:`closure type` implicitly implements
the :codeterm:`core::marker::Clone` :term:`trait` if

* :def_p:`fls_hihltu2pgjse`
  It does not encapsulate :term:`captured variable`\ s :term:`by unique
  immutable borrow` or :term:`by mutable reference`, or

* :def_p:`fls_if3f7yx5gd1w`
  The :term:`type`\ s of all :term:`captured variable`\ s implement
  the :codeterm:`core::marker::Clone` :term:`trait`.

:def_p:`fls_ox2b8p2o0q34`
A :term:`closure type` implicitly implements
the :codeterm:`core::marker::Send` :term:`trait` if:

* :def_p:`fls_f05b2b7zyutp`
  The :term:`type`\ s of all :term:`captured variable`\ s
  that employ :term:`by immutable borrow`, :term:`by mutable
  borrow`, or :term:`by move` :term:`capture mode` implement
  the :codeterm:`core::marker::Sync` :term:`trait`, and

* :def_p:`fls_6va77ci2c78o`
  The :term:`type`\ s of all :term:`captured variable`\ s that
  employ :term:`by unique immutable borrow`, :term:`by mutable
  reference`, :term:`by copy`, or :term:`by move` :term:`capture mode` implement
  the :codeterm:`core::marker::Send` :term:`trait`.

:def_p:`fls_89zhbs350wga`
A :term:`closure type` implicitly implements
the :codeterm:`core::marker::Sync` :term:`trait` if
the :term:`type`\ s of all :term:`captured variable`\ s implement
the :codeterm:`core::marker::Sync` :term:`trait`.

Function Item Type
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_v1kbnjidy842`
A :term:`function item type` is a unique anonymous :term:`function type` that
identifies a :term:`function`.

:def_p:`fls_xklsqcxqau76`
A :term:`function item type` is :term:`coercible` to a :term:`function pointer
type`.

:def_p:`fls_ku9bo6yck0fw`
A :term:`function item type` implements
the :codeterm:`core::ops::Fn` :term:`trait`,
the :codeterm:`core::ops::FnMut` :term:`trait`,
the :codeterm:`core::ops::FnOnce` :term:`trait`,
the :codeterm:`core::marker::Copy` :term:`trait`,
the :codeterm:`core::marker::Clone` :term:`trait`,
the :codeterm:`core::marker::Send` :term:`trait`, and
the :codeterm:`core::marker::Sync` :term:`trait`.

Indirection Types
-----------------

Function Pointer Type
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   FunctionPointerTypeSpecification ::=
       ForLifetimeList? FunctionPointerTypeQualifierList $$fn$$ $$($$ FunctionPointerTypeParameterList? $$)$$ ReturnTypeWithoutBounds?

   FunctionPointerTypeQualifierList ::=
       $$unsafe$$? AbiSpecification?

   FunctionPointerTypeParameterList ::=
       FunctionPointerTypeParameter ($$,$$ FunctionPointerTypeParameter)* ($$,$$ VariadicPart | $$,$$?)

   VariadicPart ::=
       OuterAttributeOrDoc* $$...$$

   FunctionPointerTypeParameter ::=
       OuterAttributeOrDoc* (IdentifierOrUnderscore $$:$$)? TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_bx9f28hmc0g9`
A :term:`function pointer type` is an :term:`indirection type` that refers to
a :term:`function`.

:def_p:`fls_x8f818ersssf`
A :syntax:`VariadicPart` shall be specified only when the :term:`ABI` of
the :term:`function pointer type` is either ``**extern** "C"`` or ``**extern**
"cdecl"``.

.. rubric:: Undefined Behavior

:def_p:`fls_qvtbc8og9hie`
It is undefined behavior to have a :term:`value` of a :term:`function pointer
type` that is :codeterm:`null`.

.. rubric:: Examples

.. code-block:: text

   unsafe extern "C" fn (value: i32, ...) -> f64

Raw Pointer Type
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   RawPointerTypeSpecification ::=
       $$*$$ ($$const$$ | $$mut$$) TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:def_p:`fls_hv7mqvn6sux0`
A :term:`raw pointer type` is an :term:`indirection type` without safety and
liveness guarantees.

:def_p:`fls_6v7hgk5somf8`
Comparing two :term:`value`\ s of :term:`raw pointer type`\ s compares the
addresses of the :term:`value`\ s.

:def_p:`fls_3f3jacc4szu6`
Comparing a :term:`value` of a :term:`raw pointer type` to a :term:`value` of
a :term:`dynamically sized type` compares the data being pointed to.

.. rubric:: Examples

.. code-block:: text

   *const i128
   *mut bool

Reference Type
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ReferenceTypeSpecification ::=
       $$&$$ LifetimeIndication? $$mut$$? TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:def_p:`fls_csnzkorynbfw`
A :term:`reference type` is an :term:`indirection type` with :term:`ownership`.

:def_p:`fls_auaov8atxgfv`
A :term:`shared reference type` prevents the direct mutation of a
referenced :term:`value`.

:def_p:`fls_7rz4dg2z9u7f`
A :term:`shared reference type` implements
the :codeterm:`core::marker::Copy` :term:`trait`. Copying a :term:`shared
reference` performs a shallow copy.

:def_p:`fls_7t9s5yg318f9`
Releasing a :term:`shared reference` has no effect on the :term:`value` it
refers to.

:def_p:`fls_lt784kdsttgm`
A :term:`mutable reference type` allows the direct mutation of a
referenced :term:`value`.

:def_p:`fls_5snts4braabw`
A :term:`mutable reference type` does not implement
the :codeterm:`copy::marker::Copy` :term:`trait`.

.. rubric:: Undefined Behavior

:def_p:`fls_v1wjl8h1m8f`
It is undefined behavior if a :term:`value` of a :term:`reference type`
is :codeterm:`null`.

.. rubric:: Examples

.. code-block:: text

   &i16
   &'a mut f32

Trait Types
-----------

Impl Trait Type
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ImplTraitTypeSpecification ::=
       $$impl$$ TypeParameterBoundList

   ImplTraitTypeSpecificationOneBound ::=
       $$impl$$ TraitBound

.. rubric:: Legality Rules

:def_p:`fls_8y68d7dyall6`
An :term:`impl trait type` is a :term:`trait type` that implements
a :term:`trait` at compile-time.

:def_p:`fls_33l95bx3f13x`
An :term:`impl trait type` shall appear only within a :term:`function parameter`
or the :term:`return type` of a :term:`function`.

.. rubric:: Examples

.. code-block:: text

   fn anonymous_type_parameter
       (arg: impl Copy + Send + Sync) { ... }

   fn anonymous_return_type () -> impl MyTrait { ... }

Trait Object Type
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TraitObjectTypeSpecification ::=
       $$dyn$$? TypeParameterBoundList

   TraitObjectTypeSpecificationOneBound ::=
       $$dyn$$? TraitBound

.. rubric:: Legality Rules

:def_p:`fls_mjh5zn58s52z`
A :term:`trait object type` is a :term:`trait type` that implements
a :term:`trait` at run-time.

:def_p:`fls_6bth55hje6qd`
The first :term:`trait bound` of a :term:`trait object type` shall denote
an :term:`object safe trait`. Any subsequent :term:`trait bound`\ s shall
denote :term:`auto trait`\ s.

:def_p:`fls_hrh5ejleelrj`
A :term:`trait object type` shall not contain :term:`opt-out trait bound`\ s.

:def_p:`fls_n6vh6gq0zi8h`
A :term:`trait object type` is a :term:`dynamically sized type`. A :term:`trait
object type` permits late binding of :term:`method`\ s. A :term:`method` invoked
via a :term:`trait object type` involves dynamic dispatching.

.. rubric:: Examples

.. code-block:: text

   dyn MyTrait
   dyn MyTrait + Send
   dyn MyTrait + 'static + Copy

Other Types
-----------

Generic Parameter Type
~~~~~~~~~~~~~~~~~~~~~~

:def_p:`fls_5517a437l8gs`
**We need additional Rust expertise.**

Inferred Type
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   InferredType ::=
       $$_$$

.. rubric:: Legality Rules

:def_p:`fls_fpbhm3dihrku`
An :term:`inferred type` is a :term:`type` that indicates the need
for :term:`type inference`.

:def_p:`fls_yiv6nj5y1q2k`
An :term:`inferred type` shall not appear within an :term:`item signature`.

:def_p:`fls_z994umg72oan`
An :term:`inferred type` forces a tool to :term:`infer` a :term:`type`, if
possible.

.. rubric:: Examples

.. code-block:: text

   let values: Vec<_> = (0 .. 10).collect();

Never Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   NeverType ::=
       $$!$$

.. rubric:: Legality Rules

:def_p:`fls_6nn8ek6e3lg8`
The :term:`never type` is a :term:`type` that represents the result of a
computation that never completes.

:def_p:`fls_jj691ocd2gfu`
The :term:`never type` has no :term:`value`\ s.

.. rubric:: Undefined Behavior

:def_p:`fls_jph88w6h8p5l`
It is undefined behavior to have a :term:`value` of the :term:`never type`.

.. rubric:: Examples

.. code-block:: text

   let never_completes: ! = panic!();

Parenthesized Type
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ParenthesizedTypeSpecification ::=
       $$($$ TypeSpecification $$)$$

.. rubric:: Legality Rules

:def_p:`fls_jux8v335y31o`
A :term:`parenthesized type` is a :term:`type` that disambiguates the
interpretation of :term:`lexical element`\ s

.. rubric:: Examples

.. code-block:: text

   &'a (dyn MyTrait + Send)

Type Aliasing
-------------

.. rubric:: Syntax

.. syntax::

   TypeAliasDeclaration ::=
       $$type$$ Name GenericParameterList? WhereClause? $$=$$ InitializationType $$;$$

   InitializationType ::=
       TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_e1on287i0awn`
A :term:`type alias` is an :term:`item` that defines a :term:`name` for
a :term:`type`.

.. rubric:: Examples

.. code-block:: text

   type Point = (f64, f64);

Representation
--------------

Type Layout
~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_rplm3jcwrpkk`
All :term:`value`\ s have an :term:`alignment` and a :term:`size`.

:def_p:`fls_r25n5medhrjy`
The :term:`alignment` of a :term:`value` specifies which addresses are valid for
storing the :term:`value`. :term:`Alignment` is measured in bytes, is at least
one, and always a power of two. A :term:`value` of :term:`alignment` ``N`` is
stored at an address that is a multiple of ``N``.

:def_p:`fls_ls2c4s35rd7g`
The :term:`size` of a :term:`value` is the offset in bytes between successive
elements in an :term:`array type` with the same :term:`element type`,
including any padding for :term:`alignment`. :term:`Size` is a multiple of
the :term:`alignment`.

:def_p:`fls_1cybhigkumu`
The :term:`size` of :term:`scalar type`\ s is as follows:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Size
   * - .. code-block:: text

          bool
     - :def_p:`fls_c7z4wx4lmndn`
       1
   * - :def_p:`fls_46roga6qp3ii`
       :codeterm:`u8`, :codeterm:`i8`
     - :def_p:`fls_4o8vofalj7wk`
       1
   * - :def_p:`fls_6xifyrrcntxs`
       :codeterm:`u16`, :codeterm:`i16`
     - :def_p:`fls_y87cfvs4kbsr`
       2
   * - :def_p:`fls_gu5awptddrte`
       :codeterm:`u32`, :codeterm:`i32`
     - :def_p:`fls_nr7mofuy5xz6`
       4
   * - :def_p:`fls_vpqrtkz4pz2d`
       :codeterm:`u64`, :codeterm:`i64`
     - :def_p:`fls_tgxjiwf3gb86`
       8
   * - :def_p:`fls_y3vs3aktk7xl`
       :codeterm:`u128`, :codeterm:`i128`
     - :def_p:`fls_vr6fssoeeci0`
       16
   * - .. code-block:: text

          f32
     - :def_p:`fls_xzavhxiifvfa`
       4
   * - .. code-block:: text

          f64
     - :def_p:`fls_nswizj47bijo`
       8
   * - .. code-block:: text

          char
     - :def_p:`fls_swco99k6gfgv`
       4

:def_p:`fls_c70fyolfdp2l`
Types :codeterm:`usize` and :codeterm:`isize` have :term:`size` big enough to
contain every address on the target platform.

:def_p:`fls_ibxk532v44jy`
For :term:`string type` :codeterm:`str`, the :term:`layout` is that
of :term:`slice type` ``[u8]``.

:def_p:`fls_dm0435hrs6mn`
For :term:`array type` ``[T; N]`` where ``T`` is the :term:`element type` and
``N`` is :term:`size operand`, the :term:`alignment` is that of ``T``, and
the :term:`size` is calculated as ``:term:`core::mem::size_of`::<T>() * N``.

:def_p:`fls_mrmxwt130mzl`
For a :term:`slice type`, the :term:`layout` is that of the :term:`array type`
it slices.

:def_p:`fls_hp3vucqcxnvy`
For a :term:`tuple type`, the :term:`layout` is tool-defined. For a :term:`unit
tuple`, the :term:`size` is zero and the :term:`alignment` is one.

:def_p:`fls_3kbhtlqf51sf`
For a :term:`closure type`, the :term:`layout` is tool-defined.

:def_p:`fls_z1o91t9fs3l3`
For a :term:`thin pointer`, the :term:`size` and :term:`alignment` are those
of :term:`type` :codeterm:`usize`.

:def_p:`fls_ed443qnwz8ka`
For a :term:`fat pointer`, the :term:`size` and :term:`alignment` are toll-
defined, but is at least those of a :term:`thin pointer`.

:def_p:`fls_t3gqhtrsmi7g`
For a :term:`trait object type`, the :term:`layout` is the same as
the :term:`value` being :term:`coerced` into the :term:`trait object type` at
runtime.

:def_p:`fls_lhvybd59nqbq`
The :term:`size` of a :term:`recursive type` shall be finite.

Type Representation
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_k2mhmu3rkh3b`
:term:`Type representation` specifies the :term:`layout` of :term:`field`\
s of :term:`abstract data type`\ s. :term:`Type representation` changes the
bit padding between :term:`field`\ s of :term:`abstract data type`\ s as well
as their order, but does not change the :term:`layout` of the :term:`field`\
s themselves.

:def_p:`fls_wu1nicv8oavh`
:term:`Type representation` is classified into:

* :def_p:`fls_p7mbylv0rfcb`
  :term:`C representation`,

* :def_p:`fls_hh2nbxupqq1u`
  :term:`Default representation`,

* :def_p:`fls_ftsnfdla4x94`
  :term:`Primitive representation`,

* :def_p:`fls_iqujr7zcfrej`
  :term:`Transparent representation`.

:def_p:`fls_9540m3f6o1vp`
:term:`C representation` lays out a :term:`type` such that the :term:`type` is
interoperable with the C language.

:def_p:`fls_9u4fjb6tewbs`
:term:`Default representation` makes no guarantees about the :term:`layout`.

:def_p:`fls_m20ylvhfk0os`
:term:`Primitive representation` is the :term:`type representation` of
individual :term:`integer type`\ s. :term:`Primitive representation` applies
only to an :term:`enum type` that is not a :term:`zero-variant enum type`.
It is possible to combine :term:`C representation` and :term:`primitive
representation`.

:def_p:`fls_w9qb6zcfj74f`
:term:`Transparent representation` applies only to an :term:`enum type` with
a :term:`single variant` or a :term:`struct type` where the :term:`struct
type` has a single :term:`field` of non-zero :term:`size` and any number
of :term:`field`\ s of :term:`size` zero and :term:`alignment` one.

:def_p:`fls_t9cekw6b3z2r`
:term:`Type`\ s subject to :term:`transparent representation` have
the same :term:`type representation` as a :term:`struct type` with a
single :term:`field` of a non-zero :term:`size`.

:def_p:`fls_wqrlth58wwzu`
:term:`Type representation` may be specified
using :term:`attribute` :codeterm:`repr`. An :term:`enum type`,
a :term:`struct type`, or a :term:`union type` that is not subject
to :term:`attribute` :codeterm:`repr` has :term:`default representation`.

:def_p:`fls_mqmnwaell4o`
:term:`Type representation` may be modified
using :term:`attribute` :codeterm:`repr`'s :codeterm:`align`
and :codeterm:`packed` :term:`representation modifier`\ s.
A :term:`representation modifier` shall apply only to a :term:`struct type`
or a :term:`union type` subject to :term:`C representation` or :term:`default
representation`.

Enumerated Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_o61kq434u2z6`
:term:`Zero-variant enum type`\ s shall not be subject to :term:`C
representation`.

:def_p:`fls_5tzmm05s6fa5`
The :term:`size` and :term:`alignment` of an :term:`enum type`
without :term:`field`\ s subject to :term:`C representation`, :term:`default
representation`, or :term:`primitive representation` are those of
its :term:`discriminant`.

:def_p:`fls_537di7fxb4ds`
The :term:`discriminant type` of an :term:`enum type` with :term:`C
representation` is the :term:`type` of a C ``enum`` for the target platform's
C :term:`ABI`.

:def_p:`fls_3jnqjuidch2e`
The :term:`discriminant type` of an :term:`enum type` with :term:`default
representation` is the smallest :term:`integer type` that can hold all
the :term:`discriminant` :term:`value`\ s of the :term:`enum type`.

:def_p:`fls_3yllrdxon1qo`
The :term:`discriminant type` of an :term:`enum type` with :term:`primitive
representation, which specifies the ` is the :term:`primitive type` specified by
the :term:`primitive representation`.

:def_p:`fls_kuczvsoq5tau`
It is a static error if the :term:`discriminant type` cannot hold all
the :term:`discriminant` :term:`value`\ s of an :term:`enum type`.

:def_p:`fls_l1u9p6w93o7j`
An :term:`enum type` subject to :term:`transparent representation` shall have a
single :term:`variant` with

* :def_p:`fls_agq5khae3yki`
  a single :term:`field` of non-zero :term:`size`, or

* :def_p:`fls_jd5h8jkakade`
  any number of :term:`field`\ s of zero :term:`size` and :term:`alignment` one.

:def_p:`fls_1wia2hc7s6gi`
An :term:`enum type` subject to :term:`C representation` or :term:`primitive
representation` has the same :term:`type representation` as a :term:`union type`
with :term:`C representation` that is laid out as follows:

* :def_p:`fls_37f4be3fjito`
  Each :term:`enum variant` corresponds to a :term:`struct` whose :term:`struct
  type` is subject to :term:`C representation` and laid out as follows:

*    * :def_p:`fls_l4j9yqlwvuwh`
       The :term:`type` of the first :term:`field` of the :term:`struct type` is
       the :term:`discriminant type` of the :term:`enum type`.

*    * :def_p:`fls_gq3tlif8tz4u`
       The remaining :term:`field`\ s of the :term:`struct type` are
       the :term:`field`\ s of the :term:`enum variant`, in the same declarative
       order.

:def_p:`fls_nrsro91mw9jo`
An :term:`enum type` subject to :term:`transparent representation` has
the same :term:`type representation` as the single :term:`field` of non-
zero :term:`size` of its :term:`variant` if one is present, otherwise
the :term:`enum type` has :term:`size` zero and :term:`alignment` one.

Struct Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_mdd00ba1tphv`
The :term:`alignment` of a :term:`struct type` subject to :term:`C
representation` is the :term:`alignment` of the most-aligned :term:`field` in
it.

:def_p:`fls_haqc7plkacff`
The :term:`size` of a :term:`struct type` subject to :term:`C representation` is
determined as follows:

#. :def_p:`fls_m2m6zi3q92gq`
   Initialize a current offset to zero.

#. :def_p:`fls_4f3plpcw5cre`
   For each :term:`field` of the :term:`struct type` in declarative order:

#.    #. :def_p:`fls_j9ha6w2wvszb`
         Calculate the :term:`size` and :term:`alignment` of the :term:`field`.

#.    #. :def_p:`fls_r6yzpovqtsa3`
         If the current offset is not a multiple of
         the :term:`field`'s :term:`alignment`, add byte padding to the current
         offset until it is a multiple of the :term:`alignment`. The offset of
         the :term:`field` is the current offset.

#.    #. :def_p:`fls_5eyy3z87wwx`
         Increase the current offset by the :term:`size` of the :term:`field`.

#.    #. :def_p:`fls_6osdngs4gyky`
         Proceed with the next :term:`field`.

#. :def_p:`fls_43mtjogvrcja`
   Round up the current offset to the nearest multiple of the :term:`struct
   type`'s :term:`alignment`.

#. :def_p:`fls_lfxpoes7ztwm`
   The :term:`size` of the :term:`struct type` is the current offset.

:def_p:`fls_f0ijvgnqeet9`
The offset of a :term:`field` of a :term:`struct type` subject to :term:`C
representation` is the current offset calculated by the aforementioned algorithm
up to the field.

:def_p:`fls_of56p0whi0x6`
A :term:`struct type` subject to :term:`transparent representation` shall have:

* :def_p:`fls_7ln4xh4qjav7`
  A single :term:`field` of non-zero :term:`size`, or

* :def_p:`fls_ns0w5e1m04bz`
  Any number of :term:`field`\ s of :term:`size` zero and :term:`alignment` one.

:def_p:`fls_a3e0z4n9nnzb`
A :term:`struct type` subject to :term:`transparent representation` has
the same :term:`type representation` as the single :term:`field` of non-
zero :term:`size` if one is present, otherwise the :term:`struct type`
has :term:`size` zero and :term:`alignment` one.

Union Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_vimtcmj46mm6`
The :term:`size` of a :term:`union type` subject to :term:`C representation`
is the maximum of the :term:`size`\ s of all its :term:`field`\ s, rounded up
to :term:`alignment` of the :term:`union type`.

:def_p:`fls_600egq51ppn7`
The :term:`alignment` of a :term:`union type` subject to :term:`C
representation` is the maximum of the :term:`alignment`\ s of all of
its :term:`field`\ s.

Type Model
----------

Recursive Types
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_m9kdiiuklme2`
A :term:`recursive type` is a :term:`type` that may define other :term:`type`\ s
within its :term:`type specification`.

:def_p:`fls_tgq1p6zbky6j`
A nested :term:`recursive type` shall not cross crate boundaries and visibility
boundaries.

.. rubric:: Examples

.. code-block:: text

   struct List<T> {
       Nil,
       Cons(T, Box<List<T>>)
   }

Type Unification
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_lkddnw4joc68`
:def_term:`Type unification` is a measure of compatibility between
two :term:`type`\ s. A type is said to :def_term:`unify` with another type when
the domains, ranges and structures of both types are compatible. Two types that
unify are said to be *:term:`unifiable type`\ s*.

:def_p:`fls_6mylzdlcz2nk`
A :term:`scalar type` is unifiable only with itself.

:def_p:`fls_hc6wnmeu1o7t`
The :term:`never type` is unifiable with any other type.

:def_p:`fls_1hzwbnogoiub`
An :term:`array type` is unifiable only with another array type when

* :def_p:`fls_17cs1v353kvi`
  The :term:`element type`\ s of both array types are unifiable, and

* :def_p:`fls_hzl15q79k78b`
  The sizes of both array types are the same.

:def_p:`fls_nhqamu4f2sl9`
A :term:`slice type` is unifiable only with another slice type when the element
types of both slice types are unifiable.

:def_p:`fls_s6ylm2qqn3uo`
Type :codeterm:`str` is unifiable only with itself.

:def_p:`fls_whs4iy58h0rv`
A :term:`tuple type` is unifiable only with another tuple type when:

* :def_p:`fls_oyofes218xh5`
  The :term:`arity` of both tuple types is the same, and

* :def_p:`fls_chmrgy6u81v7`
  The types of the corresponding :term:`tuple field`\ s are unifiable.

:def_p:`fls_445heyp4y0xj`
An :term:`abstract data type` is unifiable only with another abstract data type
when:

* :def_p:`fls_87so0n73uo17`
  The two abstract data types are the same type, and

* :def_p:`fls_yowew4pqu5vc`
  The corresponding :term:`generic substitution`\ s are unifiable.

:def_p:`fls_33tbclcgjzrw`
A :term:`closure type` is unifiable only with another closure type when:

* :def_p:`fls_3jdjs7l5wml4`
  The two closure types are the same type, and

* :def_p:`fls_f1p1zv9aqdxp`
  The corresponding generic substitutions are unifiable.

:def_p:`fls_ocopnrt5gzn`
A :term:`function item type` is unifiable only with another function item type
when:

* :def_p:`fls_ymefpxuir9nd`
  The two function item types are the same type, and

* :def_p:`fls_eamqn1bun5e`
  The corresponding generic substitutions are unifiable.

:def_p:`fls_nccy149v7yl7`
A :term:`function pointer type` is unifiable only with another function pointer
type when:

* :def_p:`fls_i76ps4wqxjre`
  **Lifetimes need to be taken into account.**

* :def_p:`fls_nw74kjr5ky00`
  The :term:`safety` is the same, and

* :def_p:`fls_4s7yjoyigcnw`
  The :term:`ABI` is the same, and

* :def_p:`fls_jw6utjctsc5p`
  The number of parameters is the same, and

* :def_p:`fls_u1feg4ejzhgp`
  The types of the corresponding parameters are unifiable, and

* :def_p:`fls_76mo8auk2ks1`
  The presence of a variadic part is the same, and

* :def_p:`fls_4eji1x8hsr2k`
  The return types are unifiable.

:def_p:`fls_bgbb215eo4jt`
A :term:`raw pointer type` is unifiable only with another raw pointer type when:

* :def_p:`fls_jkrvr3rahpu7`
  The mutability is the same, and

* :def_p:`fls_2g842t8yody7`
  The target types are unifiable.

:def_p:`fls_3g5z4h3ant2j`
A :term:`reference type` is unifiable only with another reference type when:

* :def_p:`fls_8wdy51l0lc8a`
  The mutability is the same, and

* :def_p:`fls_9usii7qskpxz`
  The target types are unifiable.

:def_p:`fls_5x0xk5a8a9p6`
An :term:`impl trait type` is unifiable with another type when:

* :def_p:`fls_8z9l2uz2ptce`
  The other type implements all :term:`trait`\ s specified by the impl trait
  type, and

* :def_p:`fls_d3dixybjfdl4`
  The lifetimes are unifiable.

:def_p:`fls_3m8ctxces23f`
A :term:`trait object type` is unifiable only with another trait object type
when:

* :def_p:`fls_ftoy2cltlm68`
  The :term:`bound`\ s are unifiable, and

* :def_p:`fls_oh6gbzap2n1c`
  The lifetimes are unifiable.

:def_p:`fls_12x7sp5l9hza`
A :term:`general type variable` is unifiable with any other type.

:def_p:`fls_pdhty4nbcs7d`
A :term:`floating-point type variable` is unifiable only with a :term:`floating-
point type`.

:def_p:`fls_efbtu1q1qpu9`
An :term:`integer type variable` shall be unifiable only with an :term:`integer
type`.

:def_p:`fls_naw8wirwqf75`
A :term:`type alias` is unifiable with another type when the aliased type is
unifiable with the other type.

Type Coercion
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_gktaw1jp93ss`
:def_term:`Type coercion` is an implicit operation that changes the :term:`type`
of a value. Any implicit conversion allowed by type coercion can be made
explicit using a :term:`type cast expression`.

:def_p:`fls_uq2k0osxfjs0`
A type coercion takes place at a coercion site or within a coercion-propagating
expression.

:def_p:`fls_q29uh0h31618`
The following :term:`construct`\ s constitute a :def_term:`coercion site`:

* :def_p:`fls_lu783aj8ufy6`
  A :term:`let statement` with an explicit type specification.

* :def_p:`fls_hd564w8rnkeh`
  A :term:`const declaration`.

* :def_p:`fls_s299ldqbxylk`
  A :term:`static declaration`.

* :def_p:`fls_rn7533nfvw8f`
  The :term:`argument`\ s of a :term:`call expression` or a :term:`method call
  expression`.

* :def_p:`fls_dpek8vqhe630`
  The :term:`instantiation` of an :term:`enum variant field`, a :term:`struct
  type`, or a :term:`union type`.

* :def_p:`fls_5ftq4jk86eyq`
  A :term:`function` result.

:def_p:`fls_fz3qxif88gix`
The following :term:`expression`\ s constitute a :def_term:`coercion-propagating
expression`:

* :def_p:`fls_ktdo0z1dlc8n`
  Each :term:`operand` of an :term:`array expression`.

* :def_p:`fls_3o5un4gqwetc`
  The last expression of a :term:`block expression`.

* :def_p:`fls_3io3fs486w8p`
  The operand of a :term:`parenthesized expression`.

* :def_p:`fls_paj06s9ecepb`
  Each operand of a :term:`tuple expression`.

:def_p:`fls_3zvbd5o4ia4k`
Type coercion from a source type to a target type is allowed to occur when:

* :def_p:`fls_qr5q6tgewfdh`
  The source type is a :term:`subtype` of the target type.

* :def_p:`fls_6uj3mog8c085`
  The source type ``T`` coerces to intermediate type ``W``, and intermediate
  type ``W`` coerces to target type ``U``.

* :def_p:`fls_e93vtnsltmry`
  The source type is ``&T`` and the target type is ``*const T``.

* :def_p:`fls_eoh2ftxdu60p`
  The source type is ``&T`` and the target type is ``&U``, where ``T``
  implements the ``:term:`core::ops::Deref`\ <Target = U>`` trait.

* :def_p:`fls_8x8kgi9n9780`
  The source type is ``&mut T`` and the target type is ``&T``.

* :def_p:`fls_ep3wfv1p2b5u`
  The source type is ``&mut T`` and the target type is ``*mut T``.

* :def_p:`fls_20hu66uelw4t`
  The source type is ``&mut T`` and the target type is ``&U``, where ``T``
  implements the ``core::ops::Deref<Target = U>`` trait.

* :def_p:`fls_x7te9ag96esc`
  The source type is ``&mut T`` and the target type is ``&mut U``, where ``T``
  implements the ``:term:`core::ops::DerefMut`\ <Target = U>`` trait.

* :def_p:`fls_jvxafhpc831u`
  The source type is ``***mut** T`` and the target type is ``***const** T``.

* :def_p:`fls_57etj2pp62bt`
  The source type is ``type_constructor(T)`` and the target type is
  ``type_constructor(U)``, where ``type_constructor`` is one of ``&W``, ``&mut
  W``, ``*const W``, or ``*mut W``, and ``U`` can be obtained from ``T`` using
  unsized coercion.

* :def_p:`fls_7c3etvhjjklr`
  The source type is a :term:`function item type` and the target type is
  a :term:`function pointer type`.

* :def_p:`fls_2x6xw0v9xa3w`
  The source type is a non-capturing :term:`closure type` and the target type is
  a function pointer type.

* :def_p:`fls_f6du5op4njvp`
  The source type is the :term:`never type` and the target type is any type.

:def_p:`fls_d85uv5rrrdd5`
:def_term:`Unsized coercion` is a type coercion that converts a :term:`sized
type` into an :term:`unsized type`. Unsized coercion from a source type to a
target type is allowed to occur when:

* :def_p:`fls_rvjnaglpypm0`
  The source type is array type ``[T; N]`` and the target type is slice type
  ``[T]``.

* :def_p:`fls_pif0khplfuxf`
  The source type is ``T`` and the target type is ``dyn U``, where ``T``
  implements ``U + :term:`core::marker::Sized`\ ``, and ``U`` is :term:`object
  safe`.

:def_p:`fls_ty5d40ofxyq4`
**I have no idea how to describe**

#. :def_p:`fls_rticctwsn3cc`
   **Foo<..., T, ...> to Foo<..., U, ...>, when:**

      #. :def_p:`fls_c2t2zciqfdog`
         **Foo is a struct.**

      #. :def_p:`fls_bt1gaa55z6nb`
         **T implements Unsize<U>.**

      #. :def_p:`fls_p1h8izv9pfeb`
         **The last field of Foo has a type involving T.**

      #. :def_p:`fls_s2ctx2bu145c`
         **If that field has type Bar<T>, then Bar<T> implements
         Unsized<Bar<U>>.**

      #. :def_p:`fls_f32wb4pbqsam`
         **T is not part of the type of any other fields.**

:def_p:`fls_a09k1zi5gtyf`
**Additionally, a type Foo<T> can implement CoerceUnsized<Foo<U>> when T
implements Unsize<U> or CoerceUnsized<Foo<U>>. This allows it to provide a
unsized coercion to Foo<U>.**

:def_p:`fls_iwp3qdfaonz0`
:def_term:`Least upper bound coercion` is a multi-type coercion that is used in
the following scenarios:

* :def_p:`fls_tpqrvsjgeimr`
  To find the common type of multiple :term:`if expression` branches.

* :def_p:`fls_1ir6znpjpbtb`
  To find the common type of multiple :term:`if let expression` branches.

* :def_p:`fls_v4ctf5q60pkh`
  To find the common type for multiple :term:`match expression` :term:`match
  arm`\ s.

* :def_p:`fls_oyg63id5bqul`
  To find the common type of :term:`array expression` operands.

* :def_p:`fls_g9oi4m36vog`
  To find the return type of a :term:`closure expression` with
  multiple :term:`return expression`\ s.

* :def_p:`fls_bbro10brcev4`
  To find the return type of a :term:`function` with multiple return
  expressions.

:def_p:`fls_mmuxrusol2dc`
Least upper bound coercion considers a set of source types ``T1``, ``T2``,
``...``, ``TN`` and target type ``U``. The target type is obtained as follows:

#. :def_p:`fls_ueld0z53no34`
   Initialize target type ``U`` to source type ``T1``.

#. :def_p:`fls_wt06vdqz87sb`
   For each current source type ``TC`` in the inclusive range ``T1`` to ``TN``

#.    #. :def_p:`fls_6rktdk7ycng7`
         If ``TC`` can be coerced to ``U``, then continue with the next source
         type.

#.    #. :def_p:`fls_gtlow26uc3dz`
         Otherwise if ``U`` can be coerced to ``TC``, make ``TC`` the target
         type ``U``.

#.    #. :def_p:`fls_c1io6s988thh`
         Otherwise compute the mutual supertype (**where is this done?**) of
         ``TC`` and ``U``, make the mutual supertype be target type ``U``. It
         is a static error if the mutual supertype of ``TC`` and ``U`` cannot
         be computed.

#.    #. :def_p:`fls_bpok1t12dyow`
         Continue with the next source type.

Type Inference
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_t439pw1wgj23`
:term:`Const declaration`\ s, :term:`let statement`\ s, and :term:`static
declaration`\ s impose an :def_term:`expected type` on their respective
initialization :term:`expression`\ s. The process of deducing the expected type
of an arbitrary expression is referred to as :def_term:`type inference`.

:def_p:`fls_wbbv5sjj6uzw`
A :def_term:`type variable` is a placeholder for a :term:`type`.
A :def_term:`global type variable` is a type variable that can refer to any
type.

:def_p:`fls_2as1jwpg4jz0`
The expected type of the initialization expression of a const declaration is the
type specified by its :term:`type specification`.

:def_p:`fls_s9uylwpy2jwb`
The expected type of the initialization expression of a let statement is
determined as follows:

#. :def_p:`fls_iy7a9gs09nze`
   If the let statement appears with a type specification, then the expected
   type is the type specified by its type specification.

#. :def_p:`fls_fw39kf45un8k`
   Otherwise the expected type is a general type variable.

:def_p:`fls_v6a69bt87ua`
The expected type of the initialization expression of a static declaration is
the type specified by its type specification.

:def_p:`fls_rkjfwxbpaz62`
:term:`Arithmetic expression`\ s, :term:`await expression`\ s, :term:`block
expression`\ s, :term:`borrow expression`\ s, :term:`dereference expression`\
s, :term:`call expression`\ s, :term:`else expression`\ s, :term:`error
propagation expression`\ s, :term:`if expression`\ s, :term:`if let expression`\
s, :term:`logical expression`\ s, :term:`loop expression`\ s, :term:`match
expression`\ s, :term:`negation expression`\ s, and :term:`parenthesized
expression`\ s are :def_term:`type imposing expression`\ s.

:def_p:`fls_g3423g61pzrk`
A type imposing expression imposes its expected type onto a nested construct,
as follows:

* :def_p:`fls_i7nh7u4zm5cp`
  An :term:`addition expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Add::Output`.

* :def_p:`fls_m4qfu9dppv9d`
  An :term:`await expression` imposes its expected type onto associated
  type :codeterm:`core::future::Future::Output`.

* A :term:`bit and expression` imposes its expected type onto associated
  type :codeterm:`core::ops::BitAnd::Output`.

* A :term:`bit xor expression` imposes its expected type onto associated
  type :codeterm:`core::ops::BitXor::Output`.

* A :term:`bit or expression` imposes its expected type onto associated
  type :codeterm:`core::ops::BitOr::Output`.

* :def_p:`fls_13q3la1p9xav`
  A block expression imposes its expected type onto the last expression
  within its statement list. If the block expression is associated with a loop
  expression, then the block expression imposes its type onto each :term:`break
  expression` within its statement list. If the block expression is associated
  with a :term:`function`, then the block expression imposes its type onto
  each :term:`return expression` within its statement list.

* :def_p:`fls_n413alwb5ce`
  A borrow expression imposes its expected type onto its operand. (**this is
  probably incomplete**)

* :def_p:`fls_ubrb08oij8us`
  A dereference expression imposes its expected type onto its operand. (**this
  is probably incomplete**)

* :def_p:`fls_n2eongbbsy2l`
  A :term:`call expression` imposes its expected type onto associated
  type :codeterm:`core::ops::FnOnce::Output`.

* :def_p:`fls_3z7rqge15h3u`
  A :term:`division expression` imposes its expected type onto associated
  type :std:`core::ops::Div::Output`.

* :def_p:`fls_zcqbipp19yjm`
  An error propagation expression imposes its expected type onto its operand.

* :def_p:`fls_wkoggpl8hztm`
  An if expression imposes its expected type onto its block expression and else
  expression.

* :def_p:`fls_2i93tlj1rrgd`
  An if let expression imposes its expected type onto its block expression and
  else expression.

* :def_p:`fls_pbeg0wkmyn0w`
  A lazy boolean expression imposes its expected type onto its operands.

* :def_p:`fls_xhts9gsuw8fw`
  A loop expression imposes its expected type onto its block expression.

* :def_p:`fls_pra47kh2cd0n`
  A match expression imposes its expected type onto the :term:`expression-with-
  block` or :term:`expression-without-block` of every :term:`intermediate match
  arm` and the expression of its :term:`final match arm`.

* :def_p:`fls_pesvopliida1`
  A :term:`multiplication expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Mul::Output`.

* :def_p:`fls_e4sotuv9b1jo`
  A negation expression imposes its expected type onto associated
  type :codeterm:`core::ops::Neg::Output`.

* :def_p:`fls_w5llya7balb7`
  A parenthesized expression imposes its expected type onto its operand.

* :def_p:`fls_izt4hxjjpk6j`
  A :term:`remainder expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Rem::Output`.

* :def_p:`fls_jv71kxyq3m6e`
  A return expression imposes its expected type onto its operand.

* :def_p:`fls_o91t1be7jp1a`
  A :term:`shift left expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Shl::Output`.

* :def_p:`fls_k4vsr5p0ewxn`
  A :term:`shift right expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Shr::Output`.

* :def_p:`fls_107h7yh1gu25`
  A :term:`subtraction expression` imposes its expected type onto associated
  type :codeterm:`core::ops::Sub::Output`.

:def_p:`fls_1okrcz2yu7iu`
:term:`Array expression`\ s, :term:`array index expression`\
s, :term:`assignment expression`\ s, :term:`closure expression`\
s, :term:`comparison expression`\ s, :term:`compound assignment expression`\
s, :term:`field access expression`\ s, :term:`lazy boolean expression`\
s, :term:`method call expression`\ s, :term:`range expression`\ s, :term:`struct
expression`\ s, :term:`tuple expression`\ s, and :term:`type cast expression`\ s
are :def_term:`type resolving expression`\ s.

:def_p:`fls_2i5z0kv6082u`
A type resolving expression provides a :def_term:`resolving type`, which is the
type of the expression itself.

:def_p:`fls_p1qkpxez9xn6`
A :def_term:`floating-point type variable` is a type variable that can refer
only to :term:`floating-point type`\ s.

:def_p:`fls_fwkazyv0uk86`
The resolving type of a :term:`float literal` is determined as follows:

#. :def_p:`fls_7onnjsh8jin5`
   If the float literal has a :term:`float suffix`, then the resolving type is
   the type specified by its float suffix.

#. :def_p:`fls_85kjc0levu5v`
   Otherwise the resolving type is a floating-point type variable.

:def_p:`fls_s7m491l7u91r`
An :def_term:`integer type variable` is a type variable that can refer only
to :term:`integer type`\ s.

:def_p:`fls_z4232iriua1h`
The resolving type of an :term:`integer literal` is determined as follows:

#. :def_p:`fls_9pe2cte0f87b`
   If the integer literal has an :term:`integer suffix`, then the resolving type
   is the type specified by its integer suffix.

#. :def_p:`fls_7qcp2gpzlrc5`
   Otherwise the resolving type is an integer type variable.

:def_p:`fls_i22wyuxsvmlu`
Const declarations, let statements, and static declarations are referred to
as :def_term:`type inference root`\ s.

:def_p:`fls_jdtvlkfss8ey`
Type inference for a single type inference root proceeds as follows:

#. :def_p:`fls_kerahloj1qyd`
   Determine unique expected type ``ET`` for the type inference root.

#. :def_p:`fls_7qr021wuh9m4`
   Resolve the initialization expression of the type inference root against
   ``ET`` as follows:

#.    #. :def_p:`fls_2zsimvtdt8ht`
         If the expression is a type imposing expression, then

#.    #.    #. :def_p:`fls_2irv3spwhar`
               Make ``ET`` the type of the expression.

#.    #.    #. :def_p:`fls_bdu40w4ukjfm`
               Impose ``ET`` on any nested construct depending on the nature of
               the expression, recursively.

#.    #. :def_p:`fls_79ja9mqri7kg`
         If the expression is a type resolving expression, then

#.    #.    #. :def_p:`fls_2wn592vu11ev`
               Determine resolving type ``RT`` the expression.

#.    #.    #. :def_p:`fls_i3obczu1h1ar`
               Resolve ``ET`` against ``RT``.

#. :def_p:`fls_jzmd8aq064u7`
   If there are expressions whose type ``T`` is a floating-point type variable,
   replace ``T`` with type :codeterm:`f64`.

#. :def_p:`fls_4iyqp3k09boe`
   If there are expressions whose type ``T`` is an integer type variable,
   replace ``T`` with type :codeterm:`i32`.

#. :def_p:`fls_9famy7rzmn9h`
   If there are expressions whose type is a global type variable, then this is a
   static error.

:def_p:`fls_vjkzlud3qpyu`
Resolving expected type ``ET`` against resolving type ``RT`` for an expression
proceeds as follows:

#. :def_p:`fls_p27jsmew3tcw`
   If both ``ET`` and ``RT`` denote a :term:`concrete type`, then ``ET`` and
   ``RT`` shall be :term:`unifiable`. (**how does type coercion fit in?**)

#. :def_p:`fls_870ll2etp13d`
   If ``ET`` denotes a global type variable and ``RT`` denotes a concrete type,
   then ``ET`` is replaced with ``RT``, effectively changing the type of all
   expressions that previously held ``ET``. (**the "changing" part probably
   needs a better explanation**)

#. :def_p:`fls_5dzvzp6yiwee`
   If ``ET`` denotes a floating-point type variable and ``RT`` denotes a
   floating point type, then ``ET`` is replaced with ``RT``, effectively
   changing the type of all expressions that previously held ``ET``.

#. :def_p:`fls_dwqtpod7868u`
   If ``ET`` denotes an integer type variable and ``RT`` denotes an integer
   type, then ``ET`` is replaced with ``RT``, effectively changing the type of
   all expressions that previously held ``ET``.

#. :def_p:`fls_84s1wkp9c05a`
   Otherwise this is a static error.

#. :def_p:`fls_gqpzlc8ijz05`
   **Are other combinations possible?**

Traits
------

.. rubric:: Syntax

.. syntax::

   TraitDeclaration ::=
       $$unsafe$$? $$trait$$ Name GenericParameterList? ($$:$$ SupertraitList?)? WhereClause? $${$$
         InnerAttributeOrDoc*
         AssociatedItem*
       $$}$$

   SupertraitList ::=
       TypeParameterBoundList

.. rubric:: Legality Rules

:def_p:`fls_dxfuhy6mpqh`
A :term:`trait` is an :term:`item` that describes an interface a :term:`type`
can implement.

:def_p:`fls_c3j5o06ccngx`
A :term:`subtrait` shall not be its own :term:`supertrait`.

:def_p:`fls_fr1oev58xvud`
A :term:`trait` of the form

.. code-block:: text

   	trait T : Bound {}

:def_p:`fls_up0g22y1jjnp`
is equivalent to a :term:`where clause` of the following form:

.. code-block:: text

   	trait T where Self: Bound {}

.. rubric:: Examples

.. code-block:: text

   trait Sequence<T> {
       fn length(&self) -> u32;
       fn element_at(&self, position: u32) -> T;
   }


:def_p:`fls_gn7l7dile45q`
Shape is a supertrait of Circle.

.. code-block:: text


   trait Shape {
       fn area(&self) -> f64;
   }


:def_p:`fls_ucdwelt9b2vg`
Circle is a subtrait of Shape.

.. code-block:: text


   trait Circle : Shape {
       fn radius(&self) -> f64;
   }

Object Safety
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_ygsni8lalu6u`
A trait is :term:`object safe` when

* :def_p:`fls_60ogkcw99150`
  Its :term:`supertrait`\ s are :term:`object safe`, and

* :def_p:`fls_jp0h875d5dae`
  :codeterm:`core::marker::Sized` is not a :term:`supertrait`, and

* :def_p:`fls_e3o5p95k7ypo`
  It lacks :term:`associated constant`\ s, and

* :def_p:`fls_qcm5t8pipood`
  Its :term:`associated function`\ s are :term:`object safe`.

:def_p:`fls_wvn0hvpznwnu`
An :term:`associated function` is :term:`object safe` when it is either
an :term:`object safe` dispatchable :term:`function` or an :term:`object safe`
non-dispatchable :term:`function`.

:def_p:`fls_b09qnvkqrsu`
A dispatchable :term:`function` is :term:`object safe` when

* :def_p:`fls_zggieuyj6w4o`
  It lacks :term:`generic parameter`\ s, and

* :def_p:`fls_lm8rdu21zfnn`
  Is a :term:`method` that does not use :codeterm:`Self` except as
  the :term:`type` of its :term:`receiver`, and

* :def_p:`fls_5t463gzi7w7`
  Is a :term:`method` whose :term:`receiver` is either ``&:term:`Self`\ ``,
  ``&mut :term:`Self`\ ``, or ``:term:`core::pin::Pin`\ <T>`` where T is one of
  the previous :term:`receiver`\ s, and

* :def_p:`fls_heklhfxxdh6p`
  It lacks a :term:`where clause` that specifies
  the :codeterm:`core::marker::Sized` :term:`trait`.

:def_p:`fls_c8cwsrl12hgx`
A non-dispatchable :term:`function` is :term:`object safe` when it specifies
a :codeterm:`core::marker::Sized` t:term:`rait bound` for :codeterm:`Self`.

Trait and Lifetime Bounds
-------------------------

.. rubric:: Syntax

.. syntax::

   TypeParameterBoundList ::=
       TypeParameterBound ($$+$$ TypeParameterBound)* $$+$$?

   TypeParameterBound ::=
       LifetimeIndication
     | ParenthesizedTraitBound
     | TraitBound

   LifetimeIndication ::=
       Lifetime
     | $$'_$$
     | $$'static$$

   LifetimeIndicationList ::=
       LifetimeIndication ($$+$$ LifetimeIndication)* $$+$$?

   ParenthesizedTraitBound ::=
       $$($$ $$?$$? ForLifetimeList? TypePath $$)$$

   TraitBound ::=
       $$?$$? ForLifetimeList? TypePath

   ForLifetimeList ::=
       $$for$$ $$<$$ AttributedLifetimeList? $$>$$

.. rubric:: Legality Rules

:def_p:`fls_6yymzv8s5prt`
A :term:`bound` imposes a constraint on :term:`generic parameter`\ s by limiting
the set of possible :term:`generic substitution`\ s.

:def_p:`fls_k6i8q318fpfr`
A :term:`lifetime bound` is a :term:`bound` that imposes a constraint on
the :term:`lifetime`\ s of :term:`generic parameter`\ s.

:def_p:`fls_mu3gsqe1i10h`
A :term:`trait bound` is a :term:`bound` that imposes a constraint on
the :term:`trait`\ s of :term:`generic parameter`\ s.

:def_p:`fls_qtrp88yywxcy`
A higher-ranked trait bound is a bound that specifies an infinite list of bounds
for all possible lifetimes specified by the ``:term:`ForLifetimeList`.``

:def_p:`fls_g5v19ci7iqdr`
:term:`Bound` ``'a: 'b`` is read as ``'a`` outlives ``'b``, or in other words,
``'a`` lasts as long as ``'b``.

:def_p:`fls_wk5vfpuqjhmg`
:term:`Bound` ``T: 'a`` indicates that all :term:`lifetime parameter`\ s of
``T`` outlive ``'a``.

:def_p:`fls_gvkrdsu11w7g`
**I can't understand and explain `this <https://doc.rust-lang.org/stable/
reference/trait-bounds.html#higher-ranked-trait-bounds>`_.**

.. rubric:: Examples

.. code-block:: text

   fn draw<T: Shape>(shape: T) { ... }

Lifetime
~~~~~~~~

.. rubric:: Syntax

.. syntax::

   Lifetime ::=
       $$'$$ NonKeywordIdentifier

   AttributedLifetimeList ::=
       AttributedLifetime ($$,$$ AttributedLifetime)* $$,$$?

   AttributedLifetime ::=
       OuterAttributeOrDoc* Lifetime

.. rubric:: Legality Rules

:def_p:`fls_5i8rjbg96yx3`
A :term:`lifetime` specifies the expected longevity of a :term:`value`.

:def_p:`fls_gnzm5t9ctlt1`
A :term:`lifetime bound` shall apply to :term:`type`\ s and
other :term:`lifetime`\ s.

.. rubric:: Examples

.. code-block:: text

   &'a i32
   &'static Shape

:def_p:`fls_yumlyctyrtf4`
See :p:`4.12. <fls_t515k9ywp2rd>` for the declaration of Shape.

Subtyping and Variance
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_rs1zfa5499wy`
**I have no idea how to describe subtyping.**

:def_p:`fls_uf8qppqspnjx`
Subtyping defines a relationship between types allowing for the use of one type
where another is expected.

:def_p:`fls_s2pje9rn9oll`
Every type is a subtype of itself.

:def_p:`fls_rpbmc9bgnxiq`
**Which should be introduced first - subtyping or variance?**

:def_p:`fls_uepb9f6t1q1w`
``F<T>`` is said to be

* :def_p:`fls_8m9j71tmuqfx`
  :def_term:`Covariant` over ``T`` when ``T`` being a subtype of ``U`` implies
  that ``F<T>`` is a subtype of ``F<U>``, or

* :def_p:`fls_5basw2872m2p`
  :def_term:`Contravariant` over ``T`` when ``T`` being a subtype of ``U``
  implies that ``F<U>`` is a subtype of ``F<T>``, or

* :def_p:`fls_s6n3e2rvgfxi`
  :def_term:`Invariant` over ``T``.

:def_p:`fls_9y63ay1chjf8`
:term:`Variance` is determined as follows:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Variance in ``'a``
     - .. rubric:: Variance in ``T``
   * - .. code-block:: text

          &'a T
     - :def_p:`fls_9lsh9mgdit38`
       :term:`covariant`
     - :def_p:`fls_3o1lt4or1fq6`
       :term:`covariant`
   * - .. code-block:: text

          &'a mut T
     - :def_p:`fls_f2dw164sj1l0`
       :term:`covariant`
     - :def_p:`fls_mvgpkah921md`
       :term:`invariant`
   * - .. code-block:: text

          *const T
     -- :def_p:`fls_6f5fh3ivzlmi`
       :term:`covariant`
   * - .. code-block:: text

          *mut T
     -- :def_p:`fls_e0b3bxv4exl8`
       :term:`invariant`
   * - .. code-block:: text

          [T]
     -- :def_p:`fls_nji53xcr3z59`
       :term:`covariant`
   * - .. code-block:: text

          [T; N]
     -- :def_p:`fls_8fz5ajampwv8`
       :term:`covariant`
   * - .. code-block:: text

          fn() -> T
     -- :def_p:`fls_wzzld1diii4b`
       :term:`covariant`
   * - .. code-block:: text

          fn(T) -> ()
     -- :def_p:`fls_szkuqgxi0z27`
       :term:`contravariant`
   * - .. code-block:: text

          fn(T) -> T
     -- :def_p:`fls_e4fd6kbmsjkb`
       :term:`invariant`
   * - .. code-block:: text

          core::call::UnsafeCell<T>
     -- :def_p:`fls_tqd5nmwqpcgv`
       :term:`invariant`
   * - .. code-block:: text

          core::marker::PhantomData<T>
     -- :def_p:`fls_p9tgqf7xz76u`
       :term:`covariant`
   * - .. code-block:: text

          dyn Trait<T> + 'a
     - :def_p:`fls_agzdgfeduxlx`
       :term:`covariant`
     - :def_p:`fls_fpf3h3eox0r6`
       :term:`invariant`

:def_p:`fls_1n740f6ua9yo`
:term:`Lifetime parameter`\ s and :term:`type parameter`\ s are subject
to :term:`variance`.

:def_p:`fls_rgxcvtyxodke`
The :term:`variance` of a :term:`generic parameter` of an :term:`abstract data
type` or an :term:`tuple type` is determined as follows:

#. :def_p:`fls_k5xt607wgu2g`
   For each :term:`generic parameter` ``G``

#.    #. :def_p:`fls_vh81a7xm9ro0`
         Initialize :term:`variance` ``V`` of the :term:`generic parameter`
         to ``any``.

#.    #. :def_p:`fls_f9xs5f3ppbuj`
         For each :term:`field` of the :term:`abstract data type` or
         the :term:`tuple type`

#.    #.    #. :def_p:`fls_yihccn44ngwk`
               If :term:`field` :term:`type` ``T`` uses ``G``, then

#.    #.    #.    #. :def_p:`fls_dwyeiafr5cyw`
                     If ``V`` is ``any``, set ``V`` to the :term:`variance` of
                     ``T`` over ``G``.

#.    #.    #.    #. :def_p:`fls_t8mf335qlb73`
                     Otherwise if ``V`` and the :term:`variance` of ``T`` over
                     ``G`` differ, set ``V`` to :term:`invariant`.

#.    #. :def_p:`fls_ccjq6kzdwmdp`
         It is a static error if :term:`variance` ``V`` is ``any``.

Lifetime Elision
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_cvcz8r6m3n73`
:term:`Lifetime elision` is a set of relaxations on the use of :term:`lifetime`\
s.

:def_p:`fls_grfmqm7bfee0`
An :def_term:`input lifetime` is one of the following :term:`lifetime`\ s:

* :def_p:`fls_eryxhmljr0wc`
  Any :term:`lifetime` related to a :term:`function parameter`.

* :def_p:`fls_qjjdg795l46n`
  Any :term:`lifetime` related to a :term:`function pointer type parameter`.

* :def_p:`fls_4c1pskqlhunf`
  Any :term:`lifetime` related to the :term:`function parameter`\
  s of the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`,
  and :codeterm:`core::ops::FnOnce` :term:`trait`\ s.

* :def_p:`fls_gcbme4o2g62z`
  Any :term:`lifetime` related to an :term:`implementing type` and
  an :term:`implemented trait` of an :term:`implementation`.

:def_p:`fls_3pbjjw912b4j`
An :def_term:`output lifetime` is one of the following :term:`lifetime`\ s:

* :def_p:`fls_btssbgt0x45j`
  Any :term:`lifetime` related to the :term:`return type` of a :term:`function`.

* :def_p:`fls_ru00sjwmx01f`
  Any :term:`lifetime` related to the :term:`return type` of a :term:`function
  pointer type`.

* :def_p:`fls_y0ze7b8e7o3c`
  Any :term:`lifetime` related to the :term:`return type`\ s of
  the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`,
  and :codeterm:`core::ops::FnOnce` :term:`trait`\ s.

:def_p:`fls_bvva87gorxpk`
:term:`Lifetime elision` proceeds as follows:

#. :def_p:`fls_nt42ozrbsj25`
   Each :term:`elided` :term:`input lifetime` is a distinct :term:`lifetime
   parameter` in its related :term:`construct`.

#. :def_p:`fls_xc73479ji4kz`
   If a :term:`construct` has exactly one :term:`input lifetime`, then
   that :term:`lifetime` is assigned to all :term:`elided` :term:`output
   lifetime`\ s.

#. :def_p:`fls_fsmsz2307y18`
   If a :term:`function` has a :term:`receiver` of the form ``&self``, ``&mut
   self``, or ``self: T`` where ``T`` is a :term:`type` with a :term:`lifetime`,
   then the :term:`lifetime` of the :term:`receiver` is assigned to
   all :term:`elided` :term:`output lifetime`\ s.

#. :def_p:`fls_au6m8xzho9xv`
   Otherwise this is a static error.

.. rubric:: Examples

