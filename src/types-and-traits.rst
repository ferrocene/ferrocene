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

   TypeSpecificationList ::=
       TypeSpecification (, TypeSpecification)* $$,$$?

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

Type Classification
-------------------

.. rubric:: Legality Rules

:def_p:`fls_c4xe3pkn0n3o`
:term:`[Type]s` are organized in the following categories:

* :def_p:`fls_69zyas59o8ff`
  :term:`[Scalar type]s`

  * :def_p:`fls_65hcyqizo1da`
    :term:`Bool` :term:`type`

  * :def_p:`fls_zge99l49az8w`
    :codeterm:`Char` :term:`type`

  * :def_p:`fls_vizoconv3ir`
    :term:`[Numeric type]s`

    * :def_p:`fls_ne6bgnh1eyrj`
      :term:`Floating-point type`

    * :def_p:`fls_jvj8l8366kl2`
      :term:`Integer type`

* :def_p:`fls_eek1jn1rwjh9`
  :term:`[Sequence type]s`

  * :def_p:`fls_s0aduyvz4i7f`
    :term:`Array type`

  * :def_p:`fls_zb5e79ai7w5i`
    :term:`Slice type`

  * :def_p:`fls_yjp19vt46asy`
    :codeterm:`Str` :term:`type`

  * :def_p:`fls_xflj5df6upc7`
    :term:`Tuple type`

* :def_p:`fls_u43jnp9jnw29`
  :term:`[Abstract data type]s`

  * :def_p:`fls_lric8bf631nw`
    :term:`Enum type`

  * :def_p:`fls_98djh9avlqc0`
    :term:`Struct type`

  * :def_p:`fls_b3ymsm8dmo4`
    :term:`Union type`

* :def_p:`fls_9x5atvhdq0j2`
  :term:`[Function type]s`

  * :def_p:`fls_n5rgqgnxk9to`
    :term:`Closure type`

  * :def_p:`fls_s7ndqc5sizdy`
    :term:`Function item type`

* :def_p:`fls_jrohsv7hx7yw`
  :term:`[Indirection type]s`

  * :def_p:`fls_1kg1mknf4yx7`
    :term:`Function pointer type`

  * :def_p:`fls_bw8zutjcteki`
    :term:`Raw pointer type`

  * :def_p:`fls_nqezuc9u6wpn`
    :term:`Reference type`

* :def_p:`fls_lh52q6f6snfh`
  :term:`[Trait type]s`

  * :def_p:`fls_qqg0uixrd1a4`
    :term:`Impl trait type`

  * :def_p:`fls_b8ecqp2argmn`
    :term:`Trait object type`

* :def_p:`fls_m5vtcars8aga`
  Other :term:`[type]s`

  * :def_p:`fls_lw38557rqikt`
    :term:`Inferred type`

  * :def_p:`fls_jxn63ow9xby3`
    :term:`Never type`

  * :def_p:`fls_a81tweobvm0p`
    :term:`Parenthesized type`

Scalar Types
------------

Bool Type
~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_h5994su1yft3`
:term:`Bool` is a :term:`type` whose :term:`[value]s` denote the truth
:term:`[value]s` of logic and Boolean algebra.

:def_p:`fls_v8atmrwz6wzk`
:term:`Type` :codeterm:`bool` appears in the :term:`language prelude` under the
name ``bool``.

:def_p:`fls_iye7ho2ynyhn`
Boolean :term:`value` ``false`` has bit pattern ``0x00``. Boolean :term:`value`
``true`` has bit pattern ``0x01``.

:def_p:`fls_7nd5tixyqir8`
The following operations are defined on :term:`type` :codeterm:`bool`:

:def_p:`fls_w2dzqq54fjhb`
**Logical not**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: !a
   * - :def_p:`fls_5allcjkjnon2`
       ``true``
     - :def_p:`fls_28d3kjlrybw2`
       ``false``
   * - :def_p:`fls_3bibysz95ktn`
       ``false``
     - :def_p:`fls_3ele57v22ez1`
       ``true``

:def_p:`fls_fxq19dqtmifj`
**Logical and**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a & b
   * - :def_p:`fls_v86qrsqcs3nd`
       ``true``
     - :def_p:`fls_yxiv2hfkcn5u`
       ``true``
     - :def_p:`fls_gjo12d9epo3c`
       ``true``
   * - :def_p:`fls_dd49lb2k3erc`
       ``true``
     - :def_p:`fls_i543afk8sxa0`
       ``false``
     - :def_p:`fls_e9kattvzbvcx`
       ``false``
   * - :def_p:`fls_t6ef5x4x5poi`
       ``false``
     - :def_p:`fls_5w2j2kpzq5mu`
       ``true``
     - :def_p:`fls_nl1c8r9oph3w`
       ``false``
   * - :def_p:`fls_kqtgjgn1hqrj`
       ``false``
     - :def_p:`fls_idb2j76pnzwa`
       ``false``
     - :def_p:`fls_7207szfdk6kk`
       ``false``

:def_p:`fls_ws15ilzf8n6z`
**Logical or**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a | b
   * - :def_p:`fls_6c9ax4qsr1gy`
       ``true``
     - :def_p:`fls_cktuofecs1xx`
       ``true``
     - :def_p:`fls_wfzn0p4q0wnv`
       ``true``
   * - :def_p:`fls_sqcgvpr4egtx`
       ``true``
     - :def_p:`fls_5nxvg1lsieic`
       ``false``
     - :def_p:`fls_x959i1hq1z6r`
       ``true``
   * - :def_p:`fls_9ys0itbp4okd`
       ``false``
     - :def_p:`fls_27stbjpm0h5k`
       ``true``
     - :def_p:`fls_d25yb2t91vn`
       ``true``
   * - :def_p:`fls_b46gbyid15zx`
       ``false``
     - :def_p:`fls_tlox3qwmxlx`
       ``false``
     - :def_p:`fls_yno5e974ggvj`
       ``false``

:def_p:`fls_f8ag276ecbze`
**Logical exclusive or (xor)**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a ^ b
   * - :def_p:`fls_wovu7330vdrq`
       ``true``
     - :def_p:`fls_4meevmqdii6b`
       ``true``
     - :def_p:`fls_8v5i2mszqxq9`
       ``false``
   * - :def_p:`fls_7xopdco6iy74`
       ``true``
     - :def_p:`fls_1y4y5wsopdh1`
       ``false``
     - :def_p:`fls_i609ntj8rxo6`
       ``true``
   * - :def_p:`fls_nb5cb6en2p5w`
       ``false``
     - :def_p:`fls_nqr85swog6mr`
       ``true``
     - :def_p:`fls_phe0zdpsq89q`
       ``true``
   * - :def_p:`fls_gd28wfcfs2pv`
       ``false``
     - :def_p:`fls_rwp9v53yvv3d`
       ``false``
     - :def_p:`fls_5uqcti5feipb`
       ``false``

:def_p:`fls_67a7p57nzbul`
**Equality**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a == b
   * - :def_p:`fls_o1e4tnh7v3db`
       ``true``
     - :def_p:`fls_iirq62fnjwem`
       ``true``
     - :def_p:`fls_shulk7qzw1rz`
       ``true``
   * - :def_p:`fls_6vnv3ygisjr`
       ``true``
     - :def_p:`fls_i73my1dyyc8t`
       ``false``
     - :def_p:`fls_kfzf8i5xbnvq`
       ``false``
   * - :def_p:`fls_s6m9abmmtc9i`
       ``false``
     - :def_p:`fls_kfgs6t1f9sdq`
       ``true``
     - :def_p:`fls_96v84bxvzjv3`
       ``false``
   * - :def_p:`fls_s19vu65z96y5`
       ``false``
     - :def_p:`fls_re21bv4wffnn`
       ``false``
     - :def_p:`fls_3b6pmw1y53ln`
       ``true``

:def_p:`fls_2d4aqspw0wlt`
**Greater than**

.. list-table::

   * - .. rubric:: a
     - .. rubric:: b
     - .. rubric:: a > b
   * - :def_p:`fls_w1oti03tm1y6`
       ``true``
     - :def_p:`fls_1u05y0gi1vjh`
       ``true``
     - :def_p:`fls_s42z6zd9u3n8`
       ``false``
   * - :def_p:`fls_9gqd7eevbknt`
       ``true``
     - :def_p:`fls_cywkqorrbvzx`
       ``false``
     - :def_p:`fls_wkshp64d6fxh`
       ``true``
   * - :def_p:`fls_r4o2rmhqg4br`
       ``false``
     - :def_p:`fls_w21l1h3s5ctn`
       ``true``
     - :def_p:`fls_pzwawtpzhj12`
       ``false``
   * - :def_p:`fls_1n7p6ij1dpm`
       ``false``
     - :def_p:`fls_32yhkk111539`
       ``false``
     - :def_p:`fls_dmmrw811fkmp`
       ``false``

:def_p:`fls_4x27kfiodb8`
Operation ``a != b`` is equivalent to ``!(a == b)``.

:def_p:`fls_me6bf9m2ypt`
Operation ``a >= b`` is equivalent to ``a == b | a > b``.

:def_p:`fls_2j659ns8wop4`
Operation ``a < b`` is equivalent to ``!(a >= b)``.

:def_p:`fls_d09l2rl0161l`
Operation ``a <= b`` shall be equivalent to ``a == b | a < b``.

.. rubric:: Undefined Behavior

:def_p:`fls_2sd39mj05mb9`
It is undefined behavior for a :term:`value` of :term:`type` :codeterm:`bool` to
have a bit pattern other than ``0x00`` and ``0x01``.

Char Type
~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_vnwbs0exbwcn`
:codeterm:`Char` is a :term:`type` whose :term:`[value]s` are represented as a
32-bit unsigned word in the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive
ranges of :term:`Unicode`.

.. rubric:: Undefined Behavior

:def_p:`fls_juysxea25owj`
It is undefined behavior for a :term:`value` of :term:`type` :codeterm:`char`
to be outside the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive ranges
of :term:`Unicode`.

Numeric Types
~~~~~~~~~~~~~

Floating Point Types
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_30yny2xb9b6b`
:term:`Type` :codeterm:`f32` is equivalent to the IEEE 754-2008 binary32
:term:`type`.

:def_p:`fls_yqflrq9s6p6n`
:term:`Type` :codeterm:`f64` is equivalent to the IEEE 754-2008 binary64
:term:`type`.

Integer Types
^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_cokwseo3nnr`
:term:`[Unsigned integer type]s` define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Minimum
     - .. rubric:: Maximum
   * - :def_p:`fls_iikexw8ps6mk`
       :codeterm:`u8`
     - :def_p:`fls_n5m1r48ip83l`
       0
     - :def_p:`fls_vndtgfpmm7nj`
       2\ :sup:`8` - 1
   * - :def_p:`fls_cavasxxlgs7g`
       :codeterm:`u16`
     - :def_p:`fls_ibb219r23ez1`
       0
     - :def_p:`fls_ui6fypgc1of6`
       2\ :sup:`16` - 1
   * - :def_p:`fls_7sx92xsjx3pl`
       :codeterm:`u32`
     - :def_p:`fls_itbp6ns6jwhn`
       0
     - :def_p:`fls_8t9n7i8ae445`
       2\ :sup:`32` - 1
   * - :def_p:`fls_q9f95uet7gq4`
       :codeterm:`u64`
     - :def_p:`fls_c355etbth6lj`
       0
     - :def_p:`fls_d9vsa8cqegom`
       2\ :sup:`64` - 1
   * - :def_p:`fls_yjb3kzijd19v`
       :codeterm:`u128`
     - :def_p:`fls_3i2cyla7czp6`
       0
     - :def_p:`fls_e5lm0119mvpu`
       2\ :sup:`128` - 1

:def_p:`fls_75lntwhg20l`
:term:`Type` :codeterm:`usize` has the same number of bits as the platform's
:term:`pointer type`, and at least 16-bits wide.

:def_p:`fls_p2shoji3xg5a`
:term:`[Signed integer type]s` define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Minimum
     - .. rubric:: Maximum
   * - :def_p:`fls_p9ffvtajr832`
       :codeterm:`i8`
     - :def_p:`fls_6us55j4jg57r`
       - (2\ :sup:`7`)
     - :def_p:`fls_46vs37d4pmuc`
       2\ :sup:`7` - 1
   * - :def_p:`fls_j6xan9f8udw7`
       :codeterm:`i16`
     - :def_p:`fls_mdszf32p34k7`
       - (2\ :sup:`15`)
     - :def_p:`fls_qh8gfywfjmbn`
       2\ :sup:`15` - 1
   * - :def_p:`fls_4t39p3ibkzu7`
       :codeterm:`i32`
     - :def_p:`fls_qhbl0nakee32`
       - (2\ :sup:`31`)
     - :def_p:`fls_8v64k4gzmuqn`
       2\ :sup:`31` - 1
   * - :def_p:`fls_egfoxke0lzje`
       :codeterm:`i64`
     - :def_p:`fls_9bjswmdhjs7s`
       - (2\ :sup:`63`)
     - :def_p:`fls_jum24vd36dgm`
       2\ :sup:`63` - 1
   * - :def_p:`fls_4c4qpel1tbqs`
       :codeterm:`i128`
     - :def_p:`fls_dj2j3i7vhfds`
       - (2\ :sup:`127`)
     - :def_p:`fls_p7blgsbcgou2`
       2\ :sup:`127` - 1

:def_p:`fls_t9oyfmgqka6u`
:term:`Type` :codeterm:`isize` has the same number of bits as the platform's
:term:`pointer type`, and at least 16-bits wide.

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

:def_p:`fls_fx7b3qv3ghca`
An :term:`array type` is a :term:`sequence type` that represents a fixed
sequence of elements.

:def_p:`fls_pkts1p2dnxo`
The :term:`element type` shall be a :term:`fixed sized type`.

:def_p:`fls_imr2jx6cbuzq`
The :term:`size operand` shall be a :term:`constant expression`.

:def_p:`fls_r8nqxry2dlww`
The :term:`type` of the :term:`size operand` is :term:`type` :codeterm:`usize`.

.. rubric:: Examples

:def_p:`fls_9vjijqi9w8wn`
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

:def_p:`fls_ftvua2hlvr08`
A :term:`slice type` is a :term:`sequence type` that provides a view into a
sequence of elements.

:def_p:`fls_acgtczhk8ci0`
The :term:`element type` shall be a :term:`fixed sized type`.

:def_p:`fls_5gl67ftc3m21`
A :term:`slice type` is a :term:`dynamically sized type`.

.. rubric:: Examples

:def_p:`fls_nsny832ap4v1`
See :p:`4.3.1. <fls_eyrdzuv0r9l4>` for the declaration of ``array``.

.. code-block:: text

   let slice: &[i32] = &array[0..1];

Str Type
~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_wlnoq1qoq2kr`
:codeterm:`Str` is a :term:`sequence type` that represents a :term:`slice` of
8-bit unsigned bytes.

:def_p:`fls_1xa6fas6laha`
:term:`Type` :codeterm:`str` is a :term:`dynamically sized type`.

:def_p:`fls_yu7r2077n9m7`
A :term:`value` of :term:`type` :codeterm:`str` shall denote a valid UTF-8
sequence of characters.

.. rubric:: Undefined Behavior

:def_p:`fls_wacoqrtzvrwu`
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

:def_p:`fls_bn7wmf681ngt`
A :term:`tuple type` is a :term:`sequence type` that represents a heterogeneous
list of other :term:`[type]s`.

:def_p:`fls_s9a36zsrfqew`
If the :term:`type` of a :term:`tuple field` is a :term:`dynamically-sized
type`, then the :term:`tuple field` shall be the last :term:`tuple field` in the
:syntax:`TupleFieldList`.

.. rubric:: Examples

.. code-block:: text

   ()
   (char,)
   (i32, f64, Vec<String>)

Abstract Data Types
-------------------

Enum Type
~~~~~~~~~

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

:def_p:`fls_gbdd37seqoab`
An :term:`enum type` is an :term:`abstract data type` that contains :term:`[enum
variant]s`. An :term:`enum variant` specifies a :term:`value` of an :term:`enum
type`.

:def_p:`fls_il9a1olqmu38`
A :term:`zero-variant enum type` has no :term:`[value]s`.

:def_p:`fls_g5qle7xzaoif`
The :term:`name` of an :term:`enum variant` shall denote a unique :term:`name`
within the related :syntax:`EnumDeclaration`.

:def_p:`fls_hp5frc752dam`
A :term:`discriminant initializer` shall be specified only when all :term:`[enum
variant]s` appear without an :syntax:`EnumVariantKind`.

:def_p:`fls_pijczoq4k9ij`
The :term:`type` of the :term:`expression` of a :term:`discriminant initializer`
shall be either:

* :def_p:`fls_x7nh42on06bg`
  The :term:`type` of the :term:`primitive representation` specified by
  :term:`attribute` :codeterm:`repr`, or

* :def_p:`fls_duqbzvpuehvv`
  :term:`Type` :codeterm:`isize`.

:def_p:`fls_ly183pj4fkgh`
The :term:`value` of the :term:`expression` of a :term:`discriminant
initializer` shall be a :term:`constant expression`.

:def_p:`fls_w7sggezgq9o4`
The :term:`value` of a :term:`discriminant` of an :term:`enum variant` is
determined as follows:

#. :def_p:`fls_93l5o6qar5p2`
   If the :term:`enum variant` contains a :term:`discriminant initializer`, then
   the :term:`value` is the value of its :term:`expression`.

#. :def_p:`fls_t36rk3wikq28`
   Else, if the :term:`enum variant` is the first :term:`enum variant` in the
   :syntax:`EnumVariantList`, then the :term:`value` is zero.

#. :def_p:`fls_8ajw5trd23wi`
   Otherwise the :term:`value` is one greater than the :term:`value` of the
   :term:`discriminant` of the previous :term:`enum variant`.

:def_p:`fls_w9xj26ej869w`
It is a static error if two :term:`[enum variant]s` have the same
:term:`[discriminant]s` with the same :term:`value`.

:def_p:`fls_wqbuof7kxsrg`
It is a static error if the :term:`value` of a :term:`discriminant` exceeds
the maximum :term:`value` of the :term:`type` of the :term:`expression` of a
:term:`discriminant initializer`.

.. rubric:: Undefined Behavior

:def_p:`fls_f046du2fkgr6`
It is undefined behavior for a :term:`value` of an :term:`enum type` to have
a :term:`discriminant` other than a :term:`discriminant` specified by the
:term:`enum type`.

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

:def_p:`fls_g1azfj548136`
A :term:`struct type` is an :term:`abstract data type` that is a product of
other :term:`[type]s`.

:def_p:`fls_r885av95eivp`
The :term:`name` of a :term:`record struct field` shall denote a unique
:term:`name` within the :syntax:`RecordStructDeclaration`.

:def_p:`fls_auurdv1zvzb`
If the :term:`type` of a :term:`record struct field` is a :term:`dynamically
sized type`, then :term:`the record struct field` shall be the last
:term:`record struct field` in the :syntax:`RecordStructFieldList`.

:def_p:`fls_vce7w0904du5`
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

:def_p:`fls_nskmnzq95yqm`
A :term:`union type` is an :term:`abstract data type` similar to a C-like union.

:def_p:`fls_1caus8ybmfli`
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

:def_p:`fls_bsykgnbatpmi`
A :term:`closure type` is a unique anonymous :term:`function type` that
encapsulates all :term:`[captured variable]s` of a :term:`closure expression`.

:def_p:`fls_zfj4l8bigdg0`
A :term:`closure type` implements the :codeterm:`core::ops::FnOnce`
:term:`trait`.

:def_p:`fls_bn0ueivujnqk`
A :term:`closure type` that does not move out its :term:`[captured variable]s`
implements the :codeterm:`core::ops::FnMut` :term:`trait`.

:def_p:`fls_u01kt5glbuz8`
A :term:`closure type` that does not move or mutate its :term:`[captured
variable]s` implements the :codeterm:`core::ops::Fn` :term:`trait`.

:def_p:`fls_3jeootwe6ucu`
A :term:`closure type` that does not encapsulate :term:`[captured variable]s` is
:term:`coercible` to a :term:`function pointer type`.

:def_p:`fls_63jqtyw0rz8c`
A :term:`closure type` implicitly implements the :codeterm:`core::marker::Copy`
:term:`trait` if

* :def_p:`fls_az5hkn72e3fz`
  It does not encapsulate :term:`[captured variable]s` :term:`by unique
  immutable borrow` or :term:`by mutable reference`, or

* :def_p:`fls_vvc8c910dmeh`
  The :term:`[type]s` of all :term:`[captured variable]s` implement the
  :codeterm:`core::marker::Copy` :term:`trait`.

:def_p:`fls_3c4g9njja5s5`
A :term:`closure type` implicitly implements the :codeterm:`core::marker::Clone`
:term:`trait` if

* :def_p:`fls_yr55fbspw7s9`
  It does not encapsulate :term:`[captured variable]s` :term:`by unique
  immutable borrow` or :term:`by mutable reference`, or

* :def_p:`fls_pt65037r6hjr`
  The :term:`[type]s` of all :term:`[captured variable]s` implement the
  :codeterm:`core::marker::Clone` :term:`trait`.

:def_p:`fls_2nuhy0ujgq18`
A :term:`closure type` implicitly implements the :codeterm:`core::marker::Send`
:term:`trait` if:

* :def_p:`fls_vamgwed199ct`
  The :term:`[type]s` of all :term:`[captured variable]s` that employ
  :term:`by immutable borrow`, :term:`by mutable borrow`, or :term:`by
  move` :term:`capture mode` implement the :codeterm:`core::marker::Sync`
  :term:`trait`, and

* :def_p:`fls_f96a5r1v7te7`
  The :term:`[type]s` of all :term:`[captured variable]s` that employ
  :term:`by unique immutable borrow`, :term:`by mutable reference`,
  :term:`by copy`, or :term:`by move` :term:`capture mode` implement the
  :codeterm:`core::marker::Send` :term:`trait`.

:def_p:`fls_5jh07heok8sy`
A :term:`closure type` implicitly implements the :codeterm:`core::marker::Sync`
:term:`trait` if the :term:`[type]s` of all :term:`[captured variable]s`
implement the :codeterm:`core::marker::Sync` :term:`trait`.

Function Item Type
~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_t24iojx7yc23`
A :term:`function item type` is a unique anonymous :term:`function type` that
identifies a :term:`function`.

:def_p:`fls_sas3ahcshnrh`
An :term:`external function item type` is a :term:`function item type` where the
related :term:`function` is an :term:`external function`.

:def_p:`fls_liwnzwu1el1i`
An :term:`unsafe function item type` is a :term:`function item type` where the
related :term:`function` is an :term:`unsafe function`.

:def_p:`fls_e9x4f7qxvvjv`
A :term:`function item type` is :term:`coercible` to a :term:`function pointer
type`.

:def_p:`fls_1941wid94hlg`
A :term:`function item type` implements the :codeterm:`core::ops::Fn`
:term:`trait`, the :codeterm:`core::ops::FnMut` :term:`trait`, the
:codeterm:`core::ops::FnOnce` :term:`trait`, the :codeterm:`core::marker::Copy`
:term:`trait`, the :codeterm:`core::marker::Clone` :term:`trait`,
the :codeterm:`core::marker::Send` :term:`trait`, and the
:codeterm:`core::marker::Sync` :term:`trait`.

Indirection Types
-----------------

Function Pointer Type
~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   FunctionPointerTypeSpecification ::=
       ForGenericParameterList? FunctionPointerTypeQualifierList $$fn$$ $$($$ FunctionPointerTypeParameterList? $$)$$ ReturnTypeWithoutBounds?

   FunctionPointerTypeQualifierList ::=
       $$unsafe$$? AbiSpecification?

   FunctionPointerTypeParameterList ::=
       FunctionPointerTypeParameter ($$,$$ FunctionPointerTypeParameter)* ($$,$$ VariadicPart | $$,$$?)

   VariadicPart ::=
       OuterAttributeOrDoc* $$...$$

   FunctionPointerTypeParameter ::=
       OuterAttributeOrDoc* (IdentifierOrUnderscore $$:$$)? TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_v2wrytr3t04h`
A :term:`function pointer type` is an :term:`indirection type` that refers to
a :term:`function`.

:def_p:`fls_5dd7icjcl3nt`
An :term:`unsafe function pointer type` is a function pointer type subject to
:term:`keyword` ``unsafe``.

:def_p:`fls_hbn1l42xmr3h`
A :syntax:`VariadicPart` shall be specified only when the :term:`ABI` of the
:term:`function pointer type` is either ``extern "C"`` or ``extern "cdecl"``.

.. rubric:: Undefined Behavior

:def_p:`fls_52thmi9hnoks`
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

:def_p:`fls_rpbhr0xukbx9`
A :term:`raw pointer type` is an :term:`indirection type` without validity
guarantees.

:def_p:`fls_hrum767l6dte`
Comparing two :term:`[value]s` of :term:`[raw pointer type]s` compares the
addresses of the :term:`[value]s`.

:def_p:`fls_k6ues2936pjq`
Comparing a :term:`value` of a :term:`raw pointer type` to a :term:`value` of a
:term:`dynamically sized type` compares the data being pointed to.

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

:def_p:`fls_twhq24s8kchh`
A :term:`reference type` is an :term:`indirection type` with :term:`ownership`.

:def_p:`fls_ie0avzljmxfm`
A :term:`shared reference type` prevents the direct mutation of a referenced
:term:`value`.

:def_p:`fls_15zdiqsm1q3p`
A :term:`shared reference type` implements the :codeterm:`core::marker::Copy`
:term:`trait`. Copying a :term:`shared reference` performs a shallow copy.

:def_p:`fls_csdjfwczlzfd`
Releasing a :term:`shared reference` has no effect on the :term:`value` it
refers to.

:def_p:`fls_vaas9kns4zo6`
A :term:`mutable reference type` allows the direct mutation of a referenced
:term:`value`.

:def_p:`fls_n6ffcms5pr0r`
A :term:`mutable reference type` does not implement the
:codeterm:`copy::marker::Copy` :term:`trait`.

.. rubric:: Undefined Behavior

:def_p:`fls_ezh8aq6fmdvz`
It is undefined behavior if a :term:`value` of a :term:`reference type` is
:codeterm:`null`.

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

:def_p:`fls_a6zlvyxpgsew`
An :term:`impl trait type` is a :term:`type` that implements a :term:`trait`,
where the :term:`type` is known at compile time.

:def_p:`fls_ieyqx5vzas2m`
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

:def_p:`fls_sgrvona1nb6h`
A :term:`trait object type` is a :term:`type` that implements a :term:`trait`,
where the :term:`type` is not known at compile time.

:def_p:`fls_9z8oleh0wdel`
The first :term:`trait bound` of a :term:`trait object type` shall denote an
:term:`object safe trait`. Any subsequent :term:`[trait bound]s` shall denote
:term:`[auto trait]s`.

:def_p:`fls_s0oy2c8t4yz9`
A :term:`trait object type` shall not contain :term:`[opt-out trait bound]s`.

:def_p:`fls_88b9bmhra55f`
A :term:`trait object type` is a :term:`dynamically sized type`. A :term:`trait
object type` permits late binding of :term:`[method]s`. A :term:`method` invoked
via a :term:`trait object type` involves dynamic dispatching.

.. rubric:: Examples

.. code-block:: text

   dyn MyTrait
   dyn MyTrait + Send
   dyn MyTrait + 'static + Copy

Other Types
-----------

Inferred Type
~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   InferredType ::=
       $$_$$

.. rubric:: Legality Rules

:def_p:`fls_xdtgr5toulpb`
An :term:`inferred type` is a placeholder for a :term:`type` deduced by
:term:`type inference`.

:def_p:`fls_3abhsuaa8nas`
An :term:`inferred type` shall not appear within an :term:`item signature`.

:def_p:`fls_9d8wbugmar1m`
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

:def_p:`fls_4u0v5uy95pyf`
The :term:`never type` is a :term:`type` that represents the result of a
computation that never completes.

:def_p:`fls_xmtc10qzw0ui`
The :term:`never type` has no :term:`[value]s`.

.. rubric:: Undefined Behavior

:def_p:`fls_22e8quna7ed5`
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

:def_p:`fls_1dvo1epstrdv`
A :term:`parenthesized type` is a :term:`type` that disambiguates the
interpretation of :term:`[lexical element]s`

.. rubric:: Examples

.. code-block:: text

   &'a (dyn MyTrait + Send)

Type Aliasing
-------------

.. rubric:: Syntax

.. syntax::

   TypeAliasDeclaration ::=
       $$type$$ Name GenericParameterList? ($$:$$ TypeParameterBoundList)? WhereClause? $$=$$ InitializationType WhereClause? $$;$$

   InitializationType ::=
       TypeSpecification

.. rubric:: Legality Rules

:def_p:`fls_bibigic4jjad`
A :term:`type alias` is an :term:`item` that defines a :term:`name` for a
:term:`type`.

:def_p:`fls_rosdkeck5ax2`
A :term:`type alias` shall not have a :syntax:`TypeParameterBoundList` unless it
is an :term:`associated item`.

:def_p:`fls_drxl7u3etfp9`
The last :term:`where clause` is rejected, but may still be consumed by
:term:`[macro]s`.

.. rubric:: Examples

.. code-block:: text

   type Point = (f64, f64);

Representation
--------------

Type Layout
~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_kdbq02iguzgl`
All :term:`[value]s` have an :term:`alignment` and a :term:`size`.

:def_p:`fls_muxfn9soi47l`
The :term:`alignment` of a :term:`value` specifies which addresses are valid for
storing the :term:`value`. :term:`Alignment` is measured in bytes, is at least
one, and always a power of two. A :term:`value` of :term:`alignment` ``N`` is
stored at an address that is a multiple of ``N``.

:def_p:`fls_1pbwigq6f3ha`
The :term:`size` of a :term:`value` is the offset in bytes between successive
elements in :term:`array type` ``[T, N]`` where ``T`` is the :term:`type` of the
:term:`value`, including any padding for :term:`alignment`. :term:`Size` is a
multiple of the :term:`alignment`.

:def_p:`fls_bk3nm2n47afu`
The :term:`size` of :term:`[scalar type]s` is as follows:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Size
   * - :def_p:`fls_uixe1ruv52be`
       :codeterm:`bool`
     - :def_p:`fls_doz523u32glf`
       1
   * - :def_p:`fls_7at60xlxm9u4`
       :codeterm:`u8`, :codeterm:`i8`
     - :def_p:`fls_49rdfbc47puw`
       1
   * - :def_p:`fls_395247pkxv48`
       :codeterm:`u16`, :codeterm:`i16`
     - :def_p:`fls_4qrcqxqxm9to`
       2
   * - :def_p:`fls_tbe9sc75timc`
       :codeterm:`u32`, :codeterm:`i32`
     - :def_p:`fls_8lpmceckeer8`
       4
   * - :def_p:`fls_7jaqx33re3hg`
       :codeterm:`u64`, :codeterm:`i64`
     - :def_p:`fls_eoq4zqz93piq`
       8
   * - :def_p:`fls_asys0iz6m0md`
       :codeterm:`u128`, :codeterm:`i128`
     - :def_p:`fls_t0t98d99p7g6`
       16
   * - :def_p:`fls_wfv5vcxl2lc7`
       :codeterm:`f32`
     - :def_p:`fls_z0s0oany7chy`
       4
   * - :def_p:`fls_x8dfw50z9c`
       :codeterm:`f64`
     - :def_p:`fls_x74py1sla7uo`
       8
   * - :def_p:`fls_nyxnnlwmt5gu`
       :codeterm:`char`
     - :def_p:`fls_upgffllf8g1m`
       4

:def_p:`fls_lwmrljw9m0pb`
Types :codeterm:`usize` and :codeterm:`isize` have :term:`size` big enough to
contain every address on the target platform.

:def_p:`fls_pzi6izljfv0f`
For :term:`string type` :codeterm:`str`, the :term:`layout` is that of
:term:`slice type` ``[u8]``.

:def_p:`fls_7cjbxleo998q`
For :term:`array type` ``[T; N]`` where ``T`` is the :term:`element type` and
``N`` is :term:`size operand`, the :term:`alignment` is that of ``T``, and the
:term:`size` is calculated as ``core::mem::size_of::<T>() * N``.

:def_p:`fls_veotnstzigw2`
For a :term:`slice type`, the :term:`layout` is that of the :term:`array type`
it slices.

:def_p:`fls_nmoqk7jo1kzf`
For a :term:`tuple type`, the :term:`layout` is tool-defined. For a :term:`unit
tuple`, the :term:`size` is zero and the :term:`alignment` is one.

:def_p:`fls_gd7wozpn2ecp`
For a :term:`closure type`, the :term:`layout` is tool-defined.

:def_p:`fls_18ke90udyp67`
For a :term:`thin pointer`, the :term:`size` and :term:`alignment` are those of
:term:`type` :codeterm:`usize`.

:def_p:`fls_e5hivr6m5s3h`
For a :term:`fat pointer`, the :term:`size` and :term:`alignment` are
tool-defined, but are at least those of a :term:`thin pointer`.

:def_p:`fls_hlbsjggfxnt2`
For a :term:`trait object type`, the :term:`layout` is the same as the
:term:`value` being :term:`coerced` into the :term:`trait object type` at
runtime.

:def_p:`fls_sdrb0k2r18my`
For a :term:`struct type`, the memory layout is undefined, unless the
:term:`struct type` is subject to :term:`attribute` :codeterm:`repr`.

:def_p:`fls_gt3tkbn4bsa6`
For a :term:`union type`, the memory layout is undefined, unless the
:term:`union type` is subject to :term:`attribute` :codeterm:`repr`. All
:term:`[union field]s` share a common storage.

:def_p:`fls_njvdevz0xqc0`
The :term:`size` of a :term:`recursive type` shall be finite.

Type Representation
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_mpqlyi3lgrfv`
:term:`Type representation` specifies the :term:`layout` of :term:`[field]s`
of :term:`[abstract data type]s`. :term:`Type representation` changes the bit
padding between :term:`[field]s` of :term:`[abstract data type]s` as well as
their order, but does not change the :term:`layout` of the :term:`[field]s`
themselves.

:def_p:`fls_9dhnanv21y9z`
:term:`Type representation` is classified into:

* :def_p:`fls_3dwtkr7vzha0`
  :term:`C representation`,

* :def_p:`fls_q465p1xuzxi`
  :term:`Default representation`,

* :def_p:`fls_hrsdn21jmgx2`
  :term:`Primitive representation`,

* :def_p:`fls_ergdb18tpx25`
  :term:`Transparent representation`.

:def_p:`fls_8s1vddh8vdhy`
:term:`C representation` lays out a :term:`type` such that the :term:`type` is
interoperable with the C language.

:def_p:`fls_b005bktrkrxy`
:term:`Default representation` makes no guarantees about the :term:`layout`.

:def_p:`fls_7plbkqlmed0r`
:term:`Primitive representation` is the :term:`type representation` of
individual :term:`[integer type]s`. :term:`Primitive representation` applies
only to an :term:`enum type` that is not a :term:`zero-variant enum type`.
It is possible to combine :term:`C representation` and :term:`primitive
representation`.

:def_p:`fls_ml4khttq3w5k`
:term:`Transparent representation` applies only to an :term:`enum type` with
a :term:`single variant` or a :term:`struct type` where the :term:`struct
type` has a single :term:`field` of non-zero :term:`size` and any number of
:term:`[field]s` of :term:`size` zero and :term:`alignment` one.

:def_p:`fls_9q2iqzbup8oy`
:term:`[Type]s` subject to :term:`transparent representation` have the same
:term:`type representation` as a :term:`struct type` with a single :term:`field`
of a non-zero :term:`size`.

:def_p:`fls_fsbf6ist38ix`
:term:`Type representation` may be specified using :term:`attribute`
:codeterm:`repr`. An :term:`enum type`, a :term:`struct type`, or a
:term:`union type` that is not subject to :term:`attribute` :codeterm:`repr` has
:term:`default representation`.

:def_p:`fls_qkkc8x2oghst`
:term:`Type representation` may be modified using :term:`attribute`
:codeterm:`[repr]'s` :codeterm:`align` and :codeterm:`packed`
:term:`[representation modifier]s`. A :term:`representation modifier` shall
apply only to a :term:`struct type` or a :term:`union type` subject to :term:`C
representation` or :term:`default representation`.

Enum Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_p0c62ejo1u1t`
:term:`[Zero-variant enum type]s` shall not be subject to :term:`C
representation`.

:def_p:`fls_efp1kfgkpba8`
The :term:`size` and :term:`alignment` of an :term:`enum type` without
:term:`[field]s` subject to :term:`C representation`, :term:`default
representation`, or :term:`primitive representation` are those of its
:term:`discriminant`.

:def_p:`fls_s9c0a0lg6c0p`
The :term:`discriminant type` of an :term:`enum type` with :term:`C
representation` is the :term:`type` of a C ``enum`` for the target platform's
C :term:`ABI`.

:def_p:`fls_slhvf3gmqz4h`
The :term:`discriminant type` of an :term:`enum type` with :term:`default
representation` is tool-defined.

:def_p:`fls_u1zy06510m56`
The :term:`discriminant type` of an :term:`enum type` with :term:`primitive
representation` is the :term:`primitive type` specified by the :term:`primitive
representation`.

:def_p:`fls_ryvqkcx48u74`
It is a static error if the :term:`discriminant type` cannot hold all the
:term:`discriminant` :term:`[value]s` of an :term:`enum type`.

:def_p:`fls_zhle0rb0vhpc`
An :term:`enum type` subject to :term:`transparent representation` shall have a
single :term:`variant` with

* :def_p:`fls_45f57s1gmmh5`
  a single :term:`field` of non-zero :term:`size`, or

* :def_p:`fls_hz012yus6b4g`
  any number of :term:`[field]s` of zero :term:`size` and :term:`alignment` one.

:def_p:`fls_q5akku2idrwh`
An :term:`enum type` subject to :term:`C representation` or :term:`primitive
representation` has the same :term:`type representation` as a :term:`union type`
with :term:`C representation` that is laid out as follows:

* :def_p:`fls_r6o1wv76yw6m`
  Each :term:`enum variant` corresponds to a :term:`struct` whose :term:`struct
  type` is subject to :term:`C representation` and laid out as follows:

  * :def_p:`fls_3k1tcfxp0g63`
    The :term:`type` of the first :term:`field` of the :term:`struct type` is
    the :term:`discriminant type` of the :term:`enum type`.

  * :def_p:`fls_ebs77rxvk9st`
    The remaining :term:`[field]s` of the :term:`struct type` are the
    :term:`[field]s` of the :term:`enum variant`, in the same declarative order.

:def_p:`fls_k907i6w83s2`
An :term:`enum type` subject to :term:`transparent representation` has the same
:term:`type representation` as the single :term:`field` of non-zero :term:`size`
of its :term:`variant` if one is present, otherwise the :term:`enum type` has
:term:`size` zero and :term:`alignment` one.

Struct Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_jr9dykj6rydn`
The :term:`alignment` of a :term:`struct type` subject to :term:`C
representation` is the :term:`alignment` of the most-aligned :term:`field` in
it.

:def_p:`fls_6ck71twmnbg5`
The :term:`size` of a :term:`struct type` subject to :term:`C representation` is
determined as follows:

#. :def_p:`fls_hydq3pvm00bn`
   Initialize a current offset to zero.

#. :def_p:`fls_yzcdffahxcz`
   For each :term:`field` of the :term:`struct type` in declarative order:

   #. :def_p:`fls_t2yqmphfd6he`
      Calculate the :term:`size` and :term:`alignment` of the :term:`field`.

   #. :def_p:`fls_fa5nkvu07jlp`
      If the current offset is not a multiple of the :term:`[field]'s`
      :term:`alignment`, add byte padding to the current offset until it is a
      multiple of the :term:`alignment`. The offset of the :term:`field` is the
      current offset.

   #. :def_p:`fls_x2pkmgbp63xx`
      Increase the current offset by the :term:`size` of the :term:`field`.

   #. :def_p:`fls_y6dwc1ndm395`
      Proceed with the next :term:`field`.

#. :def_p:`fls_2npku94ookdn`
   Round up the current offset to the nearest multiple of the :term:`[struct
   type]'s` :term:`alignment`.

#. :def_p:`fls_h7nvs25rsi0y`
   The :term:`size` of the :term:`struct type` is the current offset.

:def_p:`fls_iu93vpyihrpj`
A :term:`struct type` subject to :term:`transparent representation` shall have:

* :def_p:`fls_7sjkej5otxo`
  A single :term:`field` of non-zero :term:`size`, or

* :def_p:`fls_gwhceoy0m3or`
  Any number of :term:`[field]s` of :term:`size` zero and :term:`alignment` one.

:def_p:`fls_hvkalvr4e2v0`
A :term:`struct type` subject to :term:`transparent representation` has the same
:term:`type representation` as the single :term:`field` of non-zero :term:`size`
if one is present, otherwise the :term:`struct type` has :term:`size` zero and
:term:`alignment` one.

Union Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:def_p:`fls_opz1p1neldsg`
The :term:`size` of a :term:`union type` subject to :term:`C representation` is
the maximum of the :term:`[size]s` of all its :term:`[field]s`, rounded up to
:term:`alignment` of the :term:`union type`.

:def_p:`fls_y5qtvbx5m90g`
The :term:`alignment` of a :term:`union type` subject to :term:`C
representation` is the maximum of the :term:`[alignment]s` of all of its
:term:`[field]s`.

Type Model
----------

Recursive Types
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_z22std1crl49`
A :term:`recursive type` is a category of :term:`type` that may define other
:term:`[type]s` within its :term:`type specification`.

:def_p:`fls_eddnwlr0rz59`
A :term:`recursive type` shall include at least one :term:`abstract data type`
in its recursion.

.. rubric:: Examples

.. code-block:: text

   struct List<T> {
       Nil,
       Cons(T, Box<List<T>>)
   }

Type Unification
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_ryvdhkgm7vzj`
:term:`Type unification` is a measure of compatibility between two
:term:`[type]s`. A :term:`type` is said to :term:`unify` with another
:term:`type` when the domains, ranges and structures of both :term:`[type]s`
are compatible.

:def_p:`fls_aie0tr62vhw5`
Two types that :term:`unify` are said to be :term:`[unifiable type]s`.

:def_p:`fls_dhksyjrvx9a`
A :term:`scalar type` is unifiable only with itself.

:def_p:`fls_hf0cfkrmt655`
The :term:`never type` is unifiable with any other :term:`type`.

:def_p:`fls_k9dag68qpe93`
An :term:`array type` is unifiable only with another :term:`array type` when

* :def_p:`fls_m6d9qj9q9u1i`
  The :term:`[element type]s` of both :term:`[array type]s` are unifiable, and

* :def_p:`fls_gg3x25qvymmq`
  The sizes of both :term:`[array type]s` are the same.

:def_p:`fls_ni296ev8x9v9`
A :term:`slice type` is unifiable only with another :term:`slice type` when the
:term:`[element type]s` of both :term:`[slice type]s` are unifiable.

:def_p:`fls_i1m41c4wkfc0`
Type :codeterm:`str` is unifiable only with itself.

:def_p:`fls_mpq64eal9jo3`
A :term:`tuple type` is unifiable only with another :term:`tuple type` when:

* :def_p:`fls_kcr8npsmy0e5`
  The :term:`arity` of both :term:`[tuple type]s` is the same, and

* :def_p:`fls_kq3lv1zbangz`
  The :term:`[type]s` of the corresponding :term:`[tuple field]s` are unifiable.

:def_p:`fls_so2cgqmawlm7`
An :term:`abstract data type` is unifiable only with another :term:`abstract
data type` when:

* :def_p:`fls_vsax8w6y794m`
  The two :term:`[abstract data type]s` are the same :term:`type`, and

* :def_p:`fls_1j1wc3uxs7h6`
  The corresponding :term:`[generic substitution]s` are unifiable.

:def_p:`fls_9dpea9ty0c2l`
A :term:`closure type` is unifiable only with another :term:`closure type` when:

* :def_p:`fls_42oj1ekjihq1`
  The :term:`[two closure type]s` are the same closure, and

* :def_p:`fls_gebpqqqvvklf`
  The corresponding :term:`[generic substitution]s` are unifiable.

:def_p:`fls_i221hm7rssik`
A :term:`function item type` is unifiable only with another :term:`function item
type` when:

* :def_p:`fls_74cug5zfv2wv`
  The :term:`[two function item type]s` are the same function, and

* :def_p:`fls_keezxl8v4snf`
  The corresponding :term:`[generic substitution]s` are unifiable.

:def_p:`fls_wz2etmkpvxed`
A :term:`function pointer type` is unifiable only with another :term:`function
pointer type` when:

* :def_p:`fls_rmqcbb5ja4va`
  The :term:`[lifetime]s` are :term:`variant` conformant, and

* :def_p:`fls_uu8je75y5pss`
  The :term:`safety` is the same, and

* :def_p:`fls_oksjiq3nmq2k`
  The :term:`ABI` is the same, and

* :def_p:`fls_52ymp79ert2`
  The number of :term:`[function parameter]s` is the same, and

* :def_p:`fls_g2u1dfhphrrg`
  The :term:`[type]s` of the corresponding :term:`[function parameter]s` are
  unifiable, and

* :def_p:`fls_2xgq66qp3h95`
  The presence of a :term:`variadic part` is the same, and

* :def_p:`fls_5dh8c5gg0hmk`
  The :term:`[return type]s` are unifiable.

:def_p:`fls_ismr7wwvek4q`
A :term:`raw pointer type` is unifiable only with another :term:`raw pointer
type` when:

* :def_p:`fls_x9933rjecrna`
  The :term:`mutability` is the same, and

* :def_p:`fls_mizmcykgdisb`
  The :term:`[target type]s` are unifiable.

:def_p:`fls_paoh0wttde2z`
A :term:`reference type` is unifiable only with another :term:`reference type`
when:

* :def_p:`fls_akko4dmp4nkw`
  The :term:`mutability` is the same, and

* :def_p:`fls_8gldjjxbyyb4`
  The :term:`[target type]s` are unifiable.

:def_p:`fls_8jad1ztcuxha`
An :term:`anonymous return type` is unifiable with another :term:`type` when:

* :def_p:`fls_j3w9ap9zaqud`
  The :term:`[lifetime]s` are :term:`variant` conformant, and

* :def_p:`fls_yvllot5qnc4s`
  The other :term:`type` implements all :term:`[trait]s` specified by the
  :term:`anonymous return type`, and

:def_p:`fls_hza5n5eb18ta`
An :term:`impl trait type` is unifiabe only with itself.

:def_p:`fls_ww16urcjrj6i`
A :term:`trait object type` is unifiable only with another :term:`trait object
type` when:

* :def_p:`fls_bnp6or49voxp`
  The :term:`[bound]s` are unifiable, and

* :def_p:`fls_hdo4c849q3lo`
  The :term:`[lifetime]s` are unifiable.

:def_p:`fls_zh5hhq2x9h4q`
A :term:`general type variable` is unifiable with any other type.

:def_p:`fls_3xpp05fm0zpb`
A :term:`floating-point type variable` is unifiable only with a
:term:`floating-point type`.

:def_p:`fls_qtuq6q3ylic`
An :term:`integer type variable` is unifiable only with an :term:`integer type`.

:def_p:`fls_w9dx5h7m31sj`
A :term:`type alias` is unifiable with another :term:`type` when the aliased
:term:`type` is unifiable with the other :term:`type`.

Type Coercion
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_w5pjcj9qmgbv`
:term:`Type coercion` is an implicit operation that changes the :term:`type` of
a :term:`value`. Any implicit conversion allowed by :term:`type coercion` can be
made explicit using a :term:`type cast expression`.

:def_p:`fls_5v0n2a32bk95`
A :term:`type coercion` takes place at a :term:`coercion site` or within a
:term:`coercion-propagating expression`.

:def_p:`fls_j3kbaf43sgpj`
The following :term:`[construct]s` constitute a :def_term:`coercion site`:

* :def_p:`fls_sp794uzfiofr`
  A :term:`let statement` with an explicit :term:`type specification`.

* :def_p:`fls_bhzmble1itog`
  A :term:`constant` declaration.

* :def_p:`fls_xfqny6bwzsu9`
  A :term:`static` declaration.

* :def_p:`fls_wxrugvlazy6v`
  The :term:`[argument]s` of a :term:`call expression` or a :term:`method call
  expression`.

* :def_p:`fls_eu4bt3dw1b8c`
  The :term:`instantiation` of a :term:`field` of an :term:`abstract data type`.

* :def_p:`fls_apstt4elv2k7`
  A :term:`function` result.

:def_p:`fls_u0e42y7nvn7e`
The following :term:`[expression]s` constitute a :def_term:`coercion-propagating
expression`:

* :def_p:`fls_p8hp5y506nam`
  Each :term:`operand` of an :term:`array expression`.

* :def_p:`fls_fjc9xev8rcu6`
  The :term:`tail expression` of a :term:`block expression`.

* :def_p:`fls_n1kh3z8d4q8y`
  The :term:`operand` of a :term:`parenthesized expression`.

* :def_p:`fls_dgoypa3hcxc0`
  Each :term:`operand` of a :term:`tuple expression`.

:def_p:`fls_h8dkehit8rza`
:term:`Type coercion` from a source :term:`type` to a target :term:`type` is
allowed to occur when:

* :def_p:`fls_z00wtlna6grk`
  The source :term:`type` is a :term:`subtype` of the target :term:`type`.

* :def_p:`fls_rfjdh79k0wou`
  The source :term:`type` ``T`` coerces to intermediate :term:`type` ``W``, and
  intermediate :term:`type` ``W`` coerces to target :term:`type` ``U``.

* :def_p:`fls_e3lgrtqb7jwe`
  The source :term:`type` is ``&T`` and the target :term:`type` is ``*const T``.

* :def_p:`fls_fwy2z11c1sji`
  The source :term:`type` is ``&T`` and the target :term:`type` is ``&U``, where
  ``T`` implements the ``core::ops::Deref<Target = U>`` :term:`trait`.

* :def_p:`fls_aujb44849tq1`
  The source :term:`type` is ``&mut T`` and the target :term:`type` is ``&T``.

* :def_p:`fls_p3ym3ycrnd5m`
  The source :term:`type` is ``&mut T`` and the target :term:`type` is ``*mut
  T``.

* :def_p:`fls_jmo42qgix5uw`
  The source :term:`type` is ``&mut T`` and the target :term:`type` is ``&U``,
  where ``T`` implements the ``core::ops::Deref<Target = U>`` :term:`trait`.

* :def_p:`fls_tbt4236igdzb`
  The source :term:`type` is ``&mut T`` and the target :term:`type` is
  ``&mut U``, where ``T`` implements the ``core::ops::DerefMut<Target = U>``
  :term:`trait`.

* :def_p:`fls_7ri4jk2dydfn`
  The source :term:`type` is ``*mut T`` and the target :term:`type` is ``*const
  T``.

* :def_p:`fls_6r3kn0nk5b8o`
  The source :term:`type` is ``type_constructor(T)`` and the target :term:`type`
  is ``type_constructor(U)``, where ``type_constructor`` is one of ``&W``,
  ``&mut W``, ``*const W``, or ``*mut W``, and ``U`` can be obtained from ``T``
  using :term:`unsized coercion`.

* :def_p:`fls_ulcdetwp6x96`
  The source :term:`type` is a :term:`function item type` and the target
  :term:`type` is a :term:`function pointer type`.

* :def_p:`fls_2uv1r0gni1fk`
  The source :term:`type` is a non-capturing :term:`closure type` and the target
  :term:`type` is a :term:`function pointer type`.

* :def_p:`fls_sf0c3fbx8z57`
  The source :term:`type` is the :term:`never type` and the target :term:`type`
  is any :term:`type`.

:def_p:`fls_iiiu2q7pym4p`
An :term:`unsized coercion` is a :term:`type coercion` that converts a
:term:`sized type` into an :term:`unsized type`. :term:`Unsized coercion` from a
source :term:`type` to a target :term:`type` is allowed to occur when:

* :def_p:`fls_jte6n2js32af`
  The source :term:`type` is :term:`array type` ``[T; N]`` and the target
  :term:`type` is :term:`slice type` ``[T]``.

* :def_p:`fls_20pvqqayzqra`
  The source :term:`type` is ``T`` and the target :term:`type` is ``dyn U``,
  where ``T`` implements ``U + core::marker::Sized``, and ``U`` is :term:`object
  safe`.

* :def_p:`fls_j8rcy0xvd155`
  The source type is

.. code-block:: text

               S<..., T, ...> {
                   ...
                   last_field: X
               }


:def_p:`fls_wuka4uyo3oj7`
where

* :def_p:`fls_w15yo8yvuxq3`
  ``S`` is a :term:`struct type`,

* :def_p:`fls_7aw3ifbvfgbd`
  ``T`` implements ``core::marker::Unsize<U>``,

* :def_p:`fls_cnkth59djwgl`
  ``last_field`` is a :term:`struct field` of ``S``,

* :def_p:`fls_4wbk7pqj010i`
  The :term:`type` of ``last_field`` involves ``T`` and if the
  :term:`type` of ``last_field`` is ``W<T>``, then ``W<T>`` implements
  ``core::marker::Unsize<W<U>>``,

* :def_p:`fls_47u0039t0l8f`
  ``T`` is not part of any other :term:`struct field` of ``S``.

:def_p:`fls_bmh6g3jju7eq`
and the target ``type`` is ``S<..., U, ...>``.

:def_p:`fls_da4w32rsrwxc`
:def_term:`Least upper bound coercion` is a :term:`multi-[type coercion]` that
is used in the following scenarios:

* :def_p:`fls_zi5311z1w7re`
  To find the common :term:`type` of multiple :term:`if expression` branches.

* :def_p:`fls_zst5pa29rpt`
  To find the common :term:`type` of multiple :term:`if let expression`
  branches.

* :def_p:`fls_agw1aej616vf`
  To find the common :term:`type` for multiple :term:`match expression`
  :term:`[match arm]s`.

* :def_p:`fls_tnbga5dl4gz8`
  To find the common :term:`type` of :term:`array expression`
  :term:`[operand]s`.

* :def_p:`fls_yoreux8tn65x`
  To find the :term:`return type` of a :term:`closure expression` with multiple
  :term:`[return expression]s`.

* :def_p:`fls_r11shke69uu6`
  To find the :term:`return type` of a :term:`function` with multiple
  :term:`[return expression]s`.

:def_p:`fls_ky7ykpufb95t`
:term:`Least upper bound coercion` considers a set of source :term:`[type]s`
``T1``, ``T2``, ``...``, ``TN`` and target :term:`type` ``U``. The target
:term:`type` is obtained as follows:

#. :def_p:`fls_8kvme0u4u8r6`
   Initialize target :term:`type` ``U`` to source :term:`type` ``T1``.

#. :def_p:`fls_rl9yrdfwnu03`
   For each current source :term:`type` ``TC`` in the inclusive range ``T1``
   to ``TN``

   #. :def_p:`fls_iqtmhn8flws7`
      If ``TC`` can be coerced to ``U``, then continue with the next source
      :term:`type`.

   #. :def_p:`fls_sr8d5har4s03`
      Otherwise if ``U`` can be coerced to ``TC``, make ``TC`` the target
      :term:`type` ``U``.

   #. :def_p:`fls_92pwnd1xbp5r`
      Otherwise compute the mutual supertype of ``TC`` and ``U``, make the
      mutual supertype be target :term:`type` ``U``. It is a static error if the
      mutual supertype of ``TC`` and ``U`` cannot be computed.

   #. :def_p:`fls_ju4ypa5ysga0`
      Continue with the next source :term:`type`.

Type Inference
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_h8sedxew0d4u`
:term:`Constant` declarations, :term:`[let statement]s`, and :term:`[static
declaration]s` impose an :def_term:`expected type` on their respective
initialization :term:`[expression]s`. :term:`Type inference` is the process of
deducing the expected :term:`type` of an arbitrary :term:`value`.

:def_p:`fls_uvvn4usfsbhr`
A :term:`type variable` is a placeholder for a :term:`type`. A :term:`global
type variable` is a :term:`type variable` that can refer to any :term:`type`.

:def_p:`fls_5d4hw3gj4w4n`
The :term:`expected type` of the :term:`constant initializer` of a
:term:`constant` is the :term:`type` specified by its :term:`type ascription`.

:def_p:`fls_v6z48i1b7vxv`
The :term:`expected type` of the initialization :term:`expression` of a
:term:`let statement` is determined as follows:

#. :def_p:`fls_qob4wjgza3i8`
   If the :term:`let statement` appears with a :term:`type ascription`, then
   the :term:`expected type` is the :term:`type` specified by its :term:`type
   ascription`.

#. :def_p:`fls_7vdr0mh7kmpz`
   Otherwise the :term:`expected type` is a :term:`general type variable`.

:def_p:`fls_qlovdtcj1v1b`
The :term:`expected type` of the :term:`static initializer` of a :term:`static`
is the :term:`type` specified by its :term:`type ascription`.

:def_p:`fls_biyyicl3c3kn`
:term:`[Arithmetic expression]s`, :term:`[await expression]s`, :term:`[block
expression]s`, :term:`[borrow expression]s`, :term:`[dereference expression]s`,
:term:`[call expression]s`, :term:`[else expression]s`, :term:`[error
propagation expression]s`, :term:`[if expression]s`, :term:`[if let
expression]s`, :term:`[logical expression]s`, :term:`[loop expression]s`,
:term:`[match expression]s`, :term:`[negation expression]s`, and
:term:`[parenthesized expression]s` are :def_term:`type imposing expression`\ s.

:def_p:`fls_o94mhge1j3iw`
A :term:`type imposing expression` imposes its :term:`expected type` onto a
nested :term:`construct`, as follows:

* :def_p:`fls_3ihttknfccxr`
  An :term:`addition expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Add::Output`.

* :def_p:`fls_rta6ehkzp3hg`
  A :term:`division expression` imposes its :term:`expected type` onto
  :term:`associated type` :std:`core::ops::Div::Output`.

* :def_p:`fls_f2whukg3x1yo`
  A :term:`multiplication expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Mul::Output`.

* :def_p:`fls_w9fp1usbb15`
  A :term:`remainder expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Rem::Output`.

* :def_p:`fls_5s2eh0qjq6vk`
  A :term:`subtraction expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Sub::Output`.

* :def_p:`fls_rpxxg2u4hzhc`
  An :term:`await expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::future::Future::Output`.

* :def_p:`fls_vj1071lxoyyv`
  A :term:`bit and expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::BitAnd::Output`.

* :def_p:`fls_y6owsf8jnx35`
  A :term:`bit xor expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::BitXor::Output`.

* :def_p:`fls_i9dhdmiqde99`
  A :term:`bit or expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::BitOr::Output`.

* :def_p:`fls_bystnhv1olg5`
  A :term:`shift left expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Shl::Output`.

* :def_p:`fls_trvksnbx7opg`
  A :term:`shift right expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Shr::Output`.

* :def_p:`fls_8ct11ekq3p5q`
  A :term:`block expression` imposes its :term:`expected type` onto its
  :term:`tail expression`. If the :term:`block expression` is associated
  with a :term:`loop expression`, then the :term:`block expression` imposes
  its :term:`expected type` onto each :term:`break expression` within its
  :term:`statement` list. If the :term:`block expression` is associated
  with a :term:`function`, then the :term:`block expression` imposes its
  :term:`expected type` onto each :term:`return expression` within its
  :term:`statement` list.

* :def_p:`fls_eee1t7hynswa`
  A :term:`borrow expression` imposes its :term:`expected type` onto its
  :term:`operand`.

* :def_p:`fls_ax86vtmz4hrb`
  A :term:`dereference expression` imposes its :term:`expected type` onto its
  :term:`operand`.

* :def_p:`fls_kviulvlfvww2`
  A :term:`call expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::FnOnce::Output`.

* :def_p:`fls_4hsgi1voem9y`
  An :term:`error propagation expression` imposes its :term:`expected type` onto
  its operand.

* :def_p:`fls_8zpltmxy41rd`
  An :term:`if expression` imposes its :term:`expected type` onto its
  :term:`block expression` and else expression.

* :def_p:`fls_qdmyerpgnwha`
  An :term:`if let expression` imposes its :term:`expected type` onto its
  :term:`block expression` and :term:`else expression`.

* :def_p:`fls_gmojdinhct0b`
  A :term:`lazy boolean expression` imposes its :term:`expected type` onto its
  :term:`[operand]s`.

* :def_p:`fls_d8f7xb8r3aud`
  A :term:`loop expression` imposes its :term:`expected type` onto its
  :term:`block expression`.

* :def_p:`fls_ds3nkfar77in`
  A :term:`match expression` imposes its :term:`expected type` onto the
  :term:`expression-with-block` or :term:`expression-without-block` of every
  :term:`intermediate match arm` and the :term:`expression` of its :term:`final
  match arm`.

* :def_p:`fls_xhax58ebkqik`
  A :term:`negation expression` imposes its :term:`expected type` onto
  :term:`associated type` :codeterm:`core::ops::Neg::Output`.

* :def_p:`fls_m896wu8zax5k`
  A :term:`parenthesized expression` imposes its :term:`expected type` onto
  its :term:`operand`.

* :def_p:`fls_8ft8d4x1q08p`
  A :term:`return expression` imposes its :term:`expected type` onto its
  :term:`operand`.

:def_p:`fls_aaumn7viouu7`
:term:`[Array expression]s`, :term:`[array index expression]s`,
:term:`[assignment expression]s`, :term:`[closure expression]s`,
:term:`[comparison expression]s`, :term:`[compound assignment expression]s`,
:term:`[field access expression]s`, :term:`[lazy boolean expression]s`,
:term:`[method call expression]s`, :term:`[range expression]s`, :term:`[struct
expression]s`, :term:`[tuple expression]s`, and :term:`[type cast expression]s`
are :def_term:`type resolving expression`\ s.

:def_p:`fls_r7dyhfmdentz`
A :term:`type resolving expression` provides a :def_term:`resolving type`, which
is the :term:`type` of the :term:`expression` itself.

:def_p:`fls_3hv3wxkhjjp1`
A :term:`floating-point type variable` is a :term:`type variable` that can refer
only to :term:`[floating-point type]s`.

:def_p:`fls_8zkvwpkgob6d`
The :term:`resolving type` of a :term:`float literal` is determined as follows:

#. :def_p:`fls_1dvk2vvdw0oj`
   If the :term:`float literal` has a :term:`float suffix`, then the
   :term:`resolving type` is the :term:`type` specified by its :term:`float
   suffix`.

#. :def_p:`fls_gp9gcxiapfxv`
   Otherwise the :term:`resolving type` is a :term:`floating-point type
   variable`.

:def_p:`fls_7ov36fpd9mwe`
An :term:`integer type variable` is a :term:`type variable` that can refer only
to :term:`[integer type]s`.

:def_p:`fls_v9lyy98dgm98`
The :term:`resolving type` of an :term:`integer literal` is determined as
follows:

#. :def_p:`fls_i3v9yqp7j4n`
   If the :term:`integer literal` has an :term:`integer suffix`, then the
   :term:`resolving type` is the :term:`type` specified by its :term:`integer
   suffix`.

#. :def_p:`fls_z03x5pk7q9dd`
   Otherwise the :term:`resolving type` is an :term:`integer type variable`.

:def_p:`fls_ybvrhh96fc7y`
:term:`[Constant argument]s`, :term:`constant` declarations,
:term:`[function]s`, and :term:`static` declarations are referred to as
:def_term:`type inference root`\ s.

:def_p:`fls_j28usox2uzep`
:term:`Type inference` for a single :term:`type inference root` proceeds as
follows:

#. :def_p:`fls_7pwr5jeis2n8`
   Determine unique :term:`expected type` ``ET`` for the :term:`type inference
   root`.

#. :def_p:`fls_wqyw2u3tjzmv`
   Resolve the initialization :term:`expression` of the :term:`type inference
   root` against ``ET`` as follows:

   #. :def_p:`fls_a0d3x44wboz4`
      If the :term:`expression` is a :term:`type imposing expression`, then

      #. :def_p:`fls_62yj5vkp0iox`
         Make ``ET`` the :term:`type` of the :term:`expression`.

      #. :def_p:`fls_h0e7634x6go9`
         Impose ``ET`` on any nested :term:`construct` depending on the nature
         of the :term:`expression`, recursively.

   #. :def_p:`fls_7zzz1ao7k42e`
      If the :term:`expression` is a :term:`type resolving expression`, then

      #. :def_p:`fls_9swsddkfjw1r`
         Determine :term:`resolving type` ``RT`` the :term:`expression`.

      #. :def_p:`fls_59p9pd4jo8wt`
         Resolve ``ET`` against ``RT``.

#. :def_p:`fls_ynsjdua73fcl`
   If there are :term:`[expression]s` whose :term:`type` ``T`` is a
   :term:`floating-point type variable`, replace ``T`` with :term:`type`
   :codeterm:`f64`.

#. :def_p:`fls_oz057wsgk05e`
   If there are :term:`[expression]s` whose :term:`type` ``T`` is
   an :term:`integer type variable`, replace ``T`` with :term:`type`
   :codeterm:`i32`.

#. :def_p:`fls_2eu3zcuznfrk`
   If there are :term:`[expression]s` whose :term:`type` is a :term:`global type
   variable`, then this is a static error.

:def_p:`fls_iqf4muk5nrot`
Resolving :term:`expected type` ``ET`` against :term:`resolving type` ``RT`` for
an :term:`expression` proceeds as follows:

#. :def_p:`fls_qdpf7tahw1go`
   If both ``ET`` and ``RT`` denote a :term:`concrete type`, then ``ET`` and
   ``RT`` shall be :term:`unifiable`.

#. :def_p:`fls_yqsl1gg27b5o`
   If ``ET`` denotes a :term:`global type variable` and ``RT`` denotes a
   :term:`concrete type`, then ``ET`` is replaced with ``RT``, effectively
   changing the :term:`type` of all :term:`[expression]s` that previously held
   ``ET``.

#. :def_p:`fls_c4i80gd8cdub`
   If ``ET`` denotes a :term:`floating-point type variable` and ``RT`` denotes a
   :term:`floating point type`, then ``ET`` is replaced with ``RT``, effectively
   changing the :term:`type` of all :term:`[expression]s` that previously held
   ``ET``.

#. :def_p:`fls_acd7b3m1qm3a`
   If ``ET`` denotes an :term:`integer type variable` and ``RT`` denotes an
   :term:`integer type`, then ``ET`` is replaced with ``RT``, effectively
   changing the :term:`type` of all :term:`[expression]s` that previously held
   ``ET``.

#. :def_p:`fls_riivz4mlwr4y`
   Otherwise this is a static error.

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

:def_p:`fls_tani6lesan9u`
A :term:`trait` is an :term:`item` that describes an interface a :term:`type`
can implement.

:def_p:`fls_ytn5cdonytyn`
A :term:`subtrait` shall not be its own :term:`supertrait`.

:def_p:`fls_vucd1u38sq7i`
A :term:`trait` of the form

.. code-block:: text

   	trait T : Bound {}

:def_p:`fls_kyr81mi01me2`
is equivalent to a :term:`where clause` of the following form:

.. code-block:: text

   	trait T where Self: Bound {}

.. rubric:: Examples

.. code-block:: text

   trait Sequence<T> {
       fn length(&self) -> u32;
       fn element_at(&self, position: u32) -> T;
   }


:def_p:`fls_mjg7yrq66hh0`
Shape is a supertrait of Circle.

.. code-block:: text


   trait Shape {
       fn area(&self) -> f64;
   }


:def_p:`fls_ydowwijzirmm`
Circle is a subtrait of Shape.

.. code-block:: text


   trait Circle : Shape {
       fn radius(&self) -> f64;
   }

Object Safety
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_lrdki56hpc3k`
A trait is :term:`object safe` when:

* :def_p:`fls_5wlltclogfkw`
  Its :term:`[supertrait]s` are :term:`object safe`, and

* :def_p:`fls_droy0w5gtqaw`
  :codeterm:`core::marker::Sized` is not a :term:`supertrait`, and

* :def_p:`fls_46gd1q80c6bn`
  It lacks :term:`[associated constant]s`, and

* :def_p:`fls_kwo4cknx0yat`
  Its :term:`[associated function]s` are :term:`object safe`.

:def_p:`fls_uixekv82g2e5`
An :term:`associated function` is :term:`object safe` when it is either an
:term:`object safe` dispatchable :term:`function` or an :term:`object safe`
non-dispatchable :term:`function`.

:def_p:`fls_72tvfoemwpyy`
A dispatchable :term:`function` is :term:`object safe` when:

* :def_p:`fls_j7nb34o87l1z`
  It lacks :term:`[generic parameter]s`, and

* :def_p:`fls_k1vc9vd8at92`
  Is a :term:`method` that does not use :codeterm:`Self` except as the
  :term:`type` of its :term:`receiver`, and

* :def_p:`fls_32nk904hwjao`
  Is a :term:`method` whose :term:`receiver` is either ``&Self``, ``&mut Self``,
  or ``core::pin::Pin<T>`` where T is one of the previous :term:`[receiver]s`,
  and

* :def_p:`fls_kqylg31sm5wv`
  It lacks a :term:`where clause` that specifies the
  :codeterm:`core::marker::Sized` :term:`trait`.

:def_p:`fls_aer3gaur7avp`
A non-dispatchable :term:`function` is :term:`object safe` when it specifies a
:codeterm:`core::marker::Sized` :term:`t[rait bound]` for :codeterm:`Self`.

Trait and Lifetime Bounds
-------------------------

.. rubric:: Syntax

.. syntax::

   TypeBoundList ::=
       TypeBound ($$+$$ TypeBound)* $$+$$?

   TypeBound ::=
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
       $$($$ $$?$$? ForGenericParameterList? TypePath $$)$$

   TraitBound ::=
       $$?$$? ForGenericParameterList? TypePath

   ForGenericParameterList ::=
       $$for$$ GenericParameterList

.. rubric:: Legality Rules

:def_p:`fls_5g508z6c7q5f`
A :term:`bound` imposes a constraint on :term:`[generic parameter]s` by limiting
the set of possible :term:`[generic substitution]s`.

:def_p:`fls_grby8tmmd8sb`
A :term:`lifetime bound` is a :term:`bound` that imposes a constraint on the
:term:`[lifetime]s` of :term:`[generic parameter]s`.

:def_p:`fls_knut10hoz6wc`
A :term:`trait bound` is a :term:`bound` that imposes a constraint on the
:term:`[trait]s` of :term:`[generic parameter]s`.

:def_p:`fls_sf6zg0ez9hbb`
A :term:`ForGenericParameterList` shall not specify
:syntax:`[ConstantParameter]s` or :syntax:`[TypeParameter]s`.

:def_p:`fls_vujl3fblz6x2`
A :term:`higher-ranked trait bound` is a :term:`bound` that specifies an
infinite list of :term:`[bound]s` for all possible :term:`[lifetime]s` specified
by the ``ForGenericParameterList.``

:def_p:`fls_tx4uspewnk7w`
:term:`Bound` ``'a: 'b`` is read as ``'a`` outlives ``'b``, or in other words,
``'a`` lasts as long as ``'b``.

:def_p:`fls_5kj8bmvb8xfc`
:term:`Bound` ``T: 'a`` indicates that all :term:`[lifetime parameter]s` of
``T`` outlive ``'a``.

.. rubric:: Examples

.. code-block:: text

   fn draw<T: Shape>(shape: T) { ... }

Lifetime
~~~~~~~~

.. rubric:: Syntax

.. syntax::

   Lifetime ::=
       $$'$$ NonKeywordIdentifier

   AttributedLifetime ::=
       OuterAttributeOrDoc* Lifetime

   AttributedLifetimeList ::=
       AttributedLifetime ($$,$$ AttributedLifetime)* $$,$$?

.. rubric:: Legality Rules

:def_p:`fls_nne91at3143t`
A :term:`lifetime` specifies the expected longevity of a :term:`value`.

:def_p:`fls_vbclxg9dq4yo`
A :term:`lifetime bound` shall apply to :term:`[type]s` and other
:term:`[lifetime]s`.

.. rubric:: Examples

.. code-block:: text

   &'a i32
   &'static Shape

:def_p:`fls_gcszhqg6hnva`
See :p:`4.12. <fls_t515k9ywp2rd>` for the declaration of Shape.

Subtyping and Variance
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_atq2cltx487m`
:term:`Subtyping` is a property of :term:`[type]s`, allowing one :term:`type` to
be used where another :term:`type` is expected.

:def_p:`fls_df87d44kgwcv`
:term:`Variance` is a property of :term:`[lifetime parameter]s` and :term:`[type
parameter]s` that describes the circumstances under which a :term:`generic
type` is a :term:`subtype` of an instantiation of itself with different
:term:`[generic argument]s`.

:def_p:`fls_7ex941yysuhq`
A :term:`type` is its own :term:`subtype`.

:def_p:`fls_7qud6i05ze2`
``F<T>`` is said to be

* :def_p:`fls_wpm0p0gtctvi`
  :def_term:`Covariant` over ``T`` when ``T`` being a :term:`subtype` of ``U``
  implies that ``F<T>`` is a :term:`subtype` of ``F<U>``, or

* :def_p:`fls_3rfs58i2kg6l`
  :def_term:`Contravariant` over ``T`` when ``T`` being a :term:`subtype` of
  ``U`` implies that ``F<U>`` is a :term:`subtype` of ``F<T>``, or

* :def_p:`fls_kbo3e3bosr0m`
  :def_term:`Invariant` over ``T``.

:def_p:`fls_n36p6w2a75sm`
:term:`Variance` is determined as follows:

.. list-table::

   * - .. rubric:: Type
     - .. rubric:: Variance in ``'a``
     - .. rubric:: Variance in ``T``
   * - :def_p:`fls_qc6jma5g9vpn`
       ``&'a T``
     - :def_p:`fls_yk4ef4f7wjz2`
       :term:`covariant`
     - :def_p:`fls_c0iehspl2sp`
       :term:`covariant`
   * - :def_p:`fls_hpiiwxzg16rj`
       ``&'a mut T``
     - :def_p:`fls_b2scfmqk7inl`
       :term:`covariant`
     - :def_p:`fls_2pdbeauep5e2`
       :term:`invariant`
   * - :def_p:`fls_aspdlqluwh9w`
       ``*const T``
     -
     - :def_p:`fls_pjzvqon6di16`
       :term:`covariant`
   * - :def_p:`fls_8ohuze7hqtc1`
       ``*mut T``
     -
     - :def_p:`fls_kmpyaxg4ixus`
       :term:`invariant`
   * - :def_p:`fls_7pkqgxabojkn`
       ``[T]``
     -
     - :def_p:`fls_qhocsvxf1ga0`
       :term:`covariant`
   * - :def_p:`fls_ln9pqd4xu5e`
       ``[T; N]``
     -
     - :def_p:`fls_y9unquhmaqak`
       :term:`covariant`
   * - :def_p:`fls_z4jo3ojkcu9v`
       ``fn() -> T``
     -
     - :def_p:`fls_b4mmlxxqi3mv`
       :term:`covariant`
   * - :def_p:`fls_67w6yslr3e25`
       ``fn(T) -> ()``
     -
     - :def_p:`fls_pi7okdmu4dyp`
       :term:`contravariant`
   * - :def_p:`fls_ojal3qytkqql`
       ``fn(T) -> T``
     -
     - :def_p:`fls_ita7uk1h6nqm`
       :term:`invariant`
   * - :def_p:`fls_owp42z12l4lc`
       ``core::call::UnsafeCell<T>``
     -
     - :def_p:`fls_jtjnmm6zq4v5`
       :term:`invariant`
   * - :def_p:`fls_i1vuix3gj9ej`
       ``core::marker::PhantomData<T>``
     -
     - :def_p:`fls_s047wgv2h732`
       :term:`covariant`
   * - :def_p:`fls_mlf39pl0b931`
       ``dyn Trait<T> + 'a``
     - :def_p:`fls_lg0tygtion4p`
       :term:`covariant`
     - :def_p:`fls_j6xat2xxtua`
       :term:`invariant`

:def_p:`fls_yknymnlsasyw`
A :term:`trait` is :term:`invariant` in all inputs, including the :term:`Self`
parameter.

:def_p:`fls_xkzo7nj40rbn`
:term:`[Lifetime parameter]s` and :term:`[type parameter]s` are subject to
:term:`variance`.

:def_p:`fls_abn5ycx11zpm`
The :term:`variance` of a :term:`generic parameter` of an :term:`abstract data
type` or an :term:`tuple type` is determined as follows:

#. :def_p:`fls_hvfyog9ygn6q`
   For each :term:`generic parameter` ``G``

   #. :def_p:`fls_mduolmcawb30`
      Initialize :term:`variance` ``V`` of the :term:`generic parameter` to
      ``any``.

   #. :def_p:`fls_y81gmqweqc9w`
      For each :term:`field` of the :term:`abstract data type` or the
      :term:`tuple type`

      #. :def_p:`fls_etgfvgvymn8o`
         If :term:`field` :term:`type` ``T`` uses ``G``, then

         #. :def_p:`fls_4kjxxrsk1igf`
            If ``V`` is ``any``, set ``V`` to the :term:`variance` of ``T``
            over ``G``.

         #. :def_p:`fls_y4zmb3vrym7p`
            Otherwise if ``V`` and the :term:`variance` of ``T`` over ``G``
            differ, set ``V`` to :term:`invariant`.

   #. :def_p:`fls_9ae3idezsths`
      It is a static error if :term:`variance` ``V`` is ``any``.

Lifetime Elision
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:def_p:`fls_9wtuclhm7yz5`
:term:`Lifetime elision` is a set of relaxations on the use of
:term:`[lifetime]s`.

:def_p:`fls_dpudys82dhdc`
An :def_term:`input lifetime` is one of the following :term:`[lifetime]s`:

* :def_p:`fls_pjil71kk0r25`
  Any :term:`lifetime` related to a :term:`function parameter`.

* :def_p:`fls_1jnn9bsb71k7`
  Any :term:`lifetime` related to a :term:`function pointer type parameter`.

* :def_p:`fls_2p29p1fvi182`
  Any :term:`lifetime` related to the :term:`[function parameter]s`
  of the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`, and
  :codeterm:`core::ops::FnOnce` :term:`[trait]s`.

* :def_p:`fls_ks8wlufmhz6d`
  Any :term:`lifetime` related to an :term:`implementing type` and an
  :term:`implemented trait` of an :term:`implementation`.

:def_p:`fls_hsg9kfyvh35m`
An :def_term:`output lifetime` is one of the following :term:`[lifetime]s`:

* :def_p:`fls_ofqy10q4a9jk`
  Any :term:`lifetime` related to the :term:`return type` of a :term:`function`.

* :def_p:`fls_yofbo96tjppf`
  Any :term:`lifetime` related to the :term:`return type` of a :term:`function
  pointer type`.

* :def_p:`fls_vf7cxiir91ps`
  Any :term:`lifetime` related to the :term:`[return type]s` of
  the :codeterm:`core::ops::Fn`, :codeterm:`core::ops::FnMut`, and
  :codeterm:`core::ops::FnOnce` :term:`[trait]s`.

:def_p:`fls_g56br27hq2zj`
:term:`Lifetime elision` proceeds as follows:

#. :def_p:`fls_1j204m1wy333`
   Each :term:`elided` :term:`input lifetime` is a distinct :term:`lifetime
   parameter` in its related :term:`construct`.

#. :def_p:`fls_6km3cbchuxr2`
   If a :term:`construct` has exactly one :term:`input lifetime`, then
   that :term:`lifetime` is assigned to all :term:`elided` :term:`[output
   lifetime]s`.

#. :def_p:`fls_crb6m6b3cdwh`
   If a :term:`function` has a :term:`receiver` of the form ``&self``, ``&mut
   self``, or ``self: T`` where ``T`` is a :term:`type` with a :term:`lifetime`,
   then the :term:`lifetime` of the :term:`receiver` is assigned to all
   :term:`elided` :term:`[output lifetime]s`.

#. :def_p:`fls_ac9tdlfwp5et`
   Otherwise this is a static error.

:def_p:`fls_37udexenqv3p`
The :term:`lifetime` of an :term:`associated implementation constant` shall not
be :term:`elided`.

:def_p:`fls_xi86he5vvill`
The :term:`lifetime` of an :term:`associated trait constant` shall not be
:term:`elided`.

.. rubric:: Examples

:def_p:`fls_qtjc7334wzhj`
Given function ``f`` of the form

.. code-block:: text

   fn f <'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

:def_p:`fls_vcmmkp9uruhr`
its lifetime elided form is

.. code-block:: text

   fn f <T: ToCStr>(&mut self, args: &[T]) -> &mut Command;

