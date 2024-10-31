.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: spec

.. _fls_vgb6ev541b2r:

Types and Traits
================

.. _fls_kwsBxMQNTRnL:

Types
-----

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
     | QualifiedTypePath
     | RawPointerTypeSpecification
     | ReferenceTypeSpecification
     | SliceTypeSpecification
     | TraitObjectTypeSpecificationOneBound
     | TupleTypeSpecification
     | TypePath

   TypeAscription ::=
       $$:$$ TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_4rhjpdu4zfqj`
A :t:`type` defines a set of :t:`[value]s` and a set of operations that act on
those :t:`[value]s`.

:dp:`fls_0yaYKnFrJkhG`
A :t:`local type` is a :t:`type` that is defined in the current :t:`crate`.

.. _fls_963gsjp2jas2:

Type Classification
-------------------

.. informational-section::

.. rubric:: Legality Rules

:dp:`fls_c4xe3pkn0n3o`
:t:`[Type]s` are organized in the following categories:

* :dp:`fls_69zyas59o8ff`
  :t:`[Scalar type]s`

  * :dp:`fls_65hcyqizo1da`
    :c:`Bool` :t:`type`

  * :dp:`fls_zge99l49az8w`
    :c:`Char` :t:`type`

  * :dp:`fls_vizoconv3ir`
    :t:`[Numeric type]s`

    * :dp:`fls_ne6bgnh1eyrj`
      :t:`Floating-point type`

    * :dp:`fls_jvj8l8366kl2`
      :t:`Integer type`

* :dp:`fls_eek1jn1rwjh9`
  :t:`[Sequence type]s`

  * :dp:`fls_s0aduyvz4i7f`
    :t:`[Array type]s`

  * :dp:`fls_zb5e79ai7w5i`
    :t:`[Slice type]s`

  * :dp:`fls_yjp19vt46asy`
    :c:`Str` :t:`type`

  * :dp:`fls_xflj5df6upc7`
    :t:`[Tuple type]s`

* :dp:`fls_u43jnp9jnw29`
  :t:`[Abstract data type]s`

  * :dp:`fls_lric8bf631nw`
    :t:`[Enum type]s`

  * :dp:`fls_98djh9avlqc0`
    :t:`[Struct type]s`

  * :dp:`fls_b3ymsm8dmo4`
    :t:`[Union type]s`

* :dp:`fls_9x5atvhdq0j2`
  :t:`[Function type]s`

  * :dp:`fls_n5rgqgnxk9to`
    :t:`[Closure type]s`

  * :dp:`fls_s7ndqc5sizdy`
    :t:`[Function item type]s`

* :dp:`fls_jrohsv7hx7yw`
  :t:`[Indirection type]s`

  * :dp:`fls_1kg1mknf4yx7`
    :t:`[Function pointer type]s`

  * :dp:`fls_bw8zutjcteki`
    :t:`[Raw pointer type]s`

  * :dp:`fls_nqezuc9u6wpn`
    :t:`[Reference type]s`

* :dp:`fls_lh52q6f6snfh`
  :t:`[Trait type]s`

  * :dp:`fls_qqg0uixrd1a4`
    :t:`[Impl trait type]s`

  * :dp:`fls_b8ecqp2argmn`
    :t:`[Trait object type]s`

* :dp:`fls_m5vtcars8aga`
  Other :t:`[type]s`

  * :dp:`fls_lw38557rqikt`
    :t:`[Inferred type]s`

  * :dp:`fls_jxn63ow9xby3`
    :t:`Never type`

  * :dp:`fls_a81tweobvm0p`
    :t:`[Parenthesized type]s`

.. _fls_id66vnaqw0zt:

Scalar Types
------------

.. _fls_tiqp1gxf116z:

Bool Type
~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_h5994su1yft3`
:c:`Bool` is a :t:`type` whose :t:`[value]s` denote the truth :t:`[value]s` of
logic and Boolean algebra.

:dp:`fls_v8atmrwz6wzk`
:t:`Type` :c:`bool` appears in the :t:`language prelude` under the name
``bool``.

:dp:`fls_iye7ho2ynyhn`
Boolean :t:`value` ``false`` has bit pattern ``0x00``. Boolean :t:`value`
``true`` has bit pattern ``0x01``.

:dp:`fls_7nd5tixyqir8`
The following operations are defined on :t:`type` :c:`bool`:

:dp:`fls_w2dzqq54fjhb`
**Logical not**

.. list-table::

   * - :dp:`fls_ufmd38hi9t9y`
     - **a**
     - **!a**
   * - :dp:`fls_5allcjkjnon2`
     - ``true``
     - ``false``
   * - :dp:`fls_3bibysz95ktn`
     - ``false``
     - ``true``

:dp:`fls_fxq19dqtmifj`
**Logical and**

.. list-table::

   * - :dp:`fls_drhpcwoblcux`
     - **a**
     - **b**
     - **a & b**
   * - :dp:`fls_v86qrsqcs3nd`
     - ``true``
     - ``true``
     - ``true``
   * - :dp:`fls_dd49lb2k3erc`
     - ``true``
     - ``false``
     - ``false``
   * - :dp:`fls_t6ef5x4x5poi`
     - ``false``
     - ``true``
     - ``false``
   * - :dp:`fls_kqtgjgn1hqrj`
     - ``false``
     - ``false``
     - ``false``

:dp:`fls_ws15ilzf8n6z`
**Logical or**

.. list-table::

   * - :dp:`fls_ni4mgq3mouek`
     - **a**
     - **b**
     - **a | b**
   * - :dp:`fls_6c9ax4qsr1gy`
     - ``true``
     - ``true``
     - ``true``
   * - :dp:`fls_sqcgvpr4egtx`
     - ``true``
     - ``false``
     - ``true``
   * - :dp:`fls_9ys0itbp4okd`
     - ``false``
     - ``true``
     - ``true``
   * - :dp:`fls_b46gbyid15zx`
     - ``false``
     - ``false``
     - ``false``

:dp:`fls_f8ag276ecbze`
**Logical exclusive or (xor)**

.. list-table::

   * - :dp:`fls_twwjcrcfirdi`
     - **a**
     - **b**
     - **a ^ b**
   * - :dp:`fls_wovu7330vdrq`
     - ``true``
     - ``true``
     - ``false``
   * - :dp:`fls_7xopdco6iy74`
     - ``true``
     - ``false``
     - ``true``
   * - :dp:`fls_nb5cb6en2p5w`
     - ``false``
     - ``true``
     - ``true``
   * - :dp:`fls_gd28wfcfs2pv`
     - ``false``
     - ``false``
     - ``false``

:dp:`fls_67a7p57nzbul`
**Equality**

.. list-table::

   * - :dp:`fls_cq0qunw51m94`
     - **a**
     - **b**
     - **a == b**
   * - :dp:`fls_o1e4tnh7v3db`
     - ``true``
     - ``true``
     - ``true``
   * - :dp:`fls_6vnv3ygisjr`
     - ``true``
     - ``false``
     - ``false``
   * - :dp:`fls_s6m9abmmtc9i`
     - ``false``
     - ``true``
     - ``false``
   * - :dp:`fls_s19vu65z96y5`
     - ``false``
     - ``false``
     - ``true``

:dp:`fls_2d4aqspw0wlt`
**Greater than**

.. list-table::

   * - :dp:`fls_msjo2zd67zn1`
     - **a**
     - **b**
     - **a > b**
   * - :dp:`fls_w1oti03tm1y6`
     - ``true``
     - ``true``
     - ``false``
   * - :dp:`fls_9gqd7eevbknt`
     - ``true``
     - ``false``
     - ``true``
   * - :dp:`fls_r4o2rmhqg4br`
     - ``false``
     - ``true``
     - ``false``
   * - :dp:`fls_1n7p6ij1dpm`
     - ``false``
     - ``false``
     - ``false``

:dp:`fls_4x27kfiodb8`
Operation ``a != b`` is equivalent to ``!(a == b)``.

:dp:`fls_me6bf9m2ypt`
Operation ``a >= b`` is equivalent to ``a == b | a > b``.

:dp:`fls_2j659ns8wop4`
Operation ``a < b`` is equivalent to ``!(a >= b)``.

:dp:`fls_d09l2rl0161l`
Operation ``a <= b`` is equivalent to ``a == b | a < b``.

.. rubric:: Undefined Behavior

:dp:`fls_2sd39mj05mb9`
It is a :t:`validity invariant` for a :t:`value` of :t:`type` :c:`bool` to have
a bit pattern of ``0x00`` and ``0x01``.

.. _fls_wrvjizrqf3po:

Char Type
~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_vnwbs0exbwcn`
:c:`Char` is a :t:`type` whose :t:`[value]s` are represented as a 32-bit
unsigned word in the 0x000 - 0xD7FF or the 0xE000 - 0x10FFFF inclusive ranges
of :t:`Unicode`.

.. rubric:: Undefined Behavior

:dp:`fls_juysxea25owj`
It is a :t:`validity invariant` for a :t:`value` of :t:`type` :c:`char` to be
inside the 0x000 - 0xD7FF or the 0xE000 - 0x10FFFF inclusive ranges of
:t:`Unicode`.

.. _fls_qwljwqr07slp:

Numeric Types
~~~~~~~~~~~~~

.. _fls_b4xporvr64s:

Floating Point Types
^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_30yny2xb9b6b`
:t:`Type` :c:`f32` is equivalent to the IEEE 754-2008 binary32 :t:`type`.

:dp:`fls_yqflrq9s6p6n`
:t:`Type` :c:`f64` is equivalent to the IEEE 754-2008 binary64 :t:`type`.

.. _fls_3qnpv2z7yjil:

Integer Types
^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_cokwseo3nnr`
:t:`[Unsigned integer type]s` define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - :dp:`fls_vk1skn6ek36u`
     - **Type**
     - **Minimum**
     - **Maximum**
   * - :dp:`fls_iikexw8ps6mk`
     - :c:`u8`
     - 0
     - 2\ :sup:`8` - 1
   * - :dp:`fls_cavasxxlgs7g`
     - :c:`u16`
     - 0
     - 2\ :sup:`16` - 1
   * - :dp:`fls_7sx92xsjx3pl`
     - :c:`u32`
     - 0
     - 2\ :sup:`32` - 1
   * - :dp:`fls_q9f95uet7gq4`
     - :c:`u64`
     - 0
     - 2\ :sup:`64` - 1
   * - :dp:`fls_yjb3kzijd19v`
     - :c:`u128`
     - 0
     - 2\ :sup:`128` - 1

:dp:`fls_75lntwhg20l`
:t:`Type` :c:`usize` has the same number of bits as the platform's
:t:`pointer type`, and is at least 16-bits wide.

:dp:`fls_p2shoji3xg5a`
:t:`[Signed integer type]s` define the following inclusive ranges over the
domain of whole numbers:

.. list-table::

   * - :dp:`fls_fsyt05u9y4sl`
     - **Type**
     - **Minimum**
     - **Maximum**
   * - :dp:`fls_p9ffvtajr832`
     - :c:`i8`
     - \- (2\ :sup:`7`)
     - 2\ :sup:`7` - 1
   * - :dp:`fls_j6xan9f8udw7`
     - :c:`i16`
     - \- (2\ :sup:`15`)
     - 2\ :sup:`15` - 1
   * - :dp:`fls_4t39p3ibkzu7`
     - :c:`i32`
     - \- (2\ :sup:`31`)
     - 2\ :sup:`31` - 1
   * - :dp:`fls_egfoxke0lzje`
     - :c:`i64`
     - \- (2\ :sup:`63`)
     - 2\ :sup:`63` - 1
   * - :dp:`fls_4c4qpel1tbqs`
     - :c:`i128`
     - \- (2\ :sup:`127`)
     - 2\ :sup:`127` - 1

:dp:`fls_t9oyfmgqka6u`
:t:`Type` :c:`isize` has the same number of bits as the platform's
:t:`pointer type`, and is at least 16-bits wide.

.. _fls_fbchw64p6n2x:

Sequence Types
--------------

.. _fls_uj0kpjwyld60:

Array Types
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ArrayTypeSpecification ::=
       $$[$$ ElementType $$;$$ SizeOperand $$]$$

   ElementType ::=
       TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_fx7b3qv3ghca`
An :t:`array type` is a :t:`sequence type` that represents a fixed sequence
of elements.

:dp:`fls_pkts1p2dnxo`
The :t:`element type` shall be a :t:`fixed sized type`.

:dp:`fls_imr2jx6cbuzq`
The :t:`size operand` shall be a :t:`constant expression`.

:dp:`fls_r8nqxry2dlww`
The :t:`type` of the :t:`size operand` is :t:`type` :c:`usize`.

.. rubric:: Examples

:dp:`fls_9vjijqi9w8wn`
An array type in the context of a let statement:

.. code-block:: rust

   let array: [i32; 3] = [1, 2, 3];

.. _fls_vpbikb73dw4k:

Slice Types
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   SliceTypeSpecification ::=
       $$[$$ ElementType $$]$$

.. rubric:: Legality Rules

:dp:`fls_ftvua2hlvr08`
A :t:`slice type` is a :t:`sequence type` that provides a view into a sequence
of elements.

:dp:`fls_acgtczhk8ci0`
The :t:`element type` shall be a :t:`fixed sized type`.

:dp:`fls_5gl67ftc3m21`
A :t:`slice type` is a :t:`dynamically sized type`.

.. rubric:: Examples

:dp:`fls_nsny832ap4v1`
A slice type in the context of a let statement:

.. code-block:: rust

   let array: [i32; 3] = [1, 2, 3];
   let slice: &[i32] = &array[0..1];

.. _fls_4agmmu5al6gt:

Str Type
~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_wlnoq1qoq2kr`
:c:`Str` is a :t:`sequence type` that represents a :t:`slice` of 8-bit unsigned
bytes.

:dp:`fls_1xa6fas6laha`
:t:`Type` :c:`str` is a :t:`dynamically sized type`.

:dp:`fls_yu7r2077n9m7`
A :t:`value` of :t:`type` :c:`str` shall denote a valid UTF-8 sequence of
characters.

.. rubric:: Undefined Behavior

:dp:`fls_wacoqrtzvrwu`
It is a :t:`safety invariant` for a :t:`value` of :t:`type` :c:`str` to denote
a valid UTF-8 sequence of characters.

.. _fls_4ckl3n2ko3i4:

Tuple Types
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TupleTypeSpecification ::=
       $$($$ TupleFieldList? $$)$$

   TupleFieldList ::=
       TupleField (, TupleField)* ,?

   TupleField ::=
       TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_bn7wmf681ngt`
A :t:`tuple type` is a :t:`sequence type` that represents a heterogeneous list
of other :t:`[type]s`.

:dp:`fls_s9a36zsrfqew`
If the :t:`type` of a :t:`tuple field` is a :t:`dynamically-sized type`, then
the :t:`tuple field` shall be the last :t:`tuple field` in the
:s:`TupleFieldList`.

.. rubric:: Examples

.. code-block:: rust

   ()
   (char,)
   (i32, f64, Vec<String>)

.. _fls_wdec78luqh5b:

Abstract Data Types
-------------------

.. _fls_szibmtfv117b:

Enum Types
~~~~~~~~~~

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
     | RecordStructFieldList
     | TupleStructFieldList

   DiscriminantInitializer ::=
       $$=$$ Expression

.. rubric:: Legality Rules

:dp:`fls_gbdd37seqoab`
An :t:`enum type` is an :t:`abstract data type` that contains
:t:`[enum variant]s`.

:dp:`fls_il9a1olqmu38`
A :t:`zero-variant enum type` has no :t:`[value]s`.

:dp:`fls_wQTFwl88VujQ`
An :t:`enum variant` is a :t:`construct` that declares one of the
possible variations of an :t:`enum`.

:dp:`fls_g5qle7xzaoif`
The :t:`name` of an :t:`enum variant` shall be unique within the related
:s:`EnumDeclaration`.

:dp:`fls_t4yeovFm83Wo`
A :t:`discriminant` is an opaque integer that identifies an :t:`enum variant`.

:dp:`fls_hp5frc752dam`
A :t:`discriminant initializer` shall be specified only when all :t:`[enum
variant]s` appear without an :s:`EnumVariantKind`.

:dp:`fls_pijczoq4k9ij`
The :t:`type` of the :t:`expression` of a :t:`discriminant initializer` shall
be either:

* :dp:`fls_x7nh42on06bg`
  The :t:`type` of the :t:`primitive representation` specified by :t:`attribute`
  :c:`repr`, or

* :dp:`fls_duqbzvpuehvv`
  :t:`Type` :c:`isize`.

:dp:`fls_ly183pj4fkgh`
The :t:`value` of the :t:`expression` of a :t:`discriminant initializer` shall
be a :t:`constant expression`.

:dp:`fls_w7sggezgq9o4`
The :t:`value` of a :t:`discriminant` of an :t:`enum variant` is determined
as follows:

#. :dp:`fls_93l5o6qar5p2`
   If the :t:`enum variant` contains a :t:`discriminant initializer`, then the
   :t:`value` is the value of its :t:`expression`.

#. :dp:`fls_t36rk3wikq28`
   Otherwise, if the :t:`enum variant` is the first :t:`enum variant` in the
   :s:`EnumVariantList`, then the :t:`value` is zero.

#. :dp:`fls_8ajw5trd23wi`
   Otherwise the :t:`value` is one greater than the :t:`value` of the
   :t:`discriminant` of the previous :t:`enum variant`.

:dp:`fls_w9xj26ej869w`
It is a static error if two :t:`[enum variant]s` have :t:`[discriminant]s`
with the same :t:`value`.

:dp:`fls_wqbuof7kxsrg`
It is a static error if the :t:`value` of a :t:`discriminant` exceeds the
maximum :t:`value` of the :t:`type` of the :t:`expression` of a :t:`discriminant
initializer`.

.. rubric:: Undefined Behavior

:dp:`fls_f046du2fkgr6`
It is a :t:`validity invariant` for a :t:`value` of an :t:`enum type` to have a
:t:`discriminant` specified by the :t:`enum type`.

.. rubric:: Examples

.. code-block:: rust

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

.. _fls_9ucqbbd0s2yo:

Struct Types
~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   StructDeclaration ::=
       RecordStructDeclaration
     | TupleStructDeclaration
     | UnitStructDeclaration

   RecordStructDeclaration ::=
       $$struct$$ Name GenericParameterList? WhereClause? RecordStructFieldList

   RecordStructFieldList ::=
       $${$$ (RecordStructField ($$,$$ RecordStructField)* $$,$$?)? $$}$$

   RecordStructField ::=
       OuterAttributeOrDoc* VisibilityModifier? Name TypeAscription

   TupleStructDeclaration ::=
       $$struct$$ Name GenericParameterList? TupleStructFieldList WhereClause? $$;$$

   TupleStructFieldList ::=
       $$($$ (TupleStructField ($$,$$ TupleStructField)* $$,$$?)? $$)$$

   TupleStructField ::=
       OuterAttributeOrDoc* VisibilityModifier? TypeSpecification

   UnitStructDeclaration ::=
       $$struct$$ Name GenericParameterList? WhereClause? $$;$$

.. rubric:: Legality Rules

:dp:`fls_g1azfj548136`
A :t:`struct type` is an :t:`abstract data type` that is a product of other
:t:`[type]s`.

:dp:`fls_r885av95eivp`
The :t:`name` of a :t:`record struct field` shall be unique within the
related :s:`RecordStructDeclaration`.

:dp:`fls_auurdv1zvzb`
If the :t:`type` of a :t:`record struct field` is a :t:`dynamically sized type`,
then the :t:`record struct field` shall be the last :t:`record struct field` in
the :s:`RecordStructFieldList`.

:dp:`fls_vce7w0904du5`
If the :t:`type` of a :t:`tuple struct field` is a :t:`dynamically sized type`,
then the :t:`tuple struct field` shall be the last :t:`tuple struct field` in
the :s:`TupleStructFieldList`.

.. rubric:: Examples

.. code-block:: rust

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

.. _fls_fmdn7n7s413d:

Union Types
~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   UnionDeclaration ::=
       $$union$$ Name GenericParameterList? WhereClause? RecordStructFieldList

.. rubric:: Legality Rules

:dp:`fls_nskmnzq95yqm`
A :t:`union type` is an :t:`abstract data type` that is a sum of other
:t:`[type]s`.

:dp:`fls_I5fN5Fmo5CyK`
A :t:`union` without any :t:`[union field]s` is rejected, but may still be consumed by
:t:`[macro]s`.

:dp:`fls_1caus8ybmfli`
The :t:`name` of a :t:`union field` shall be unique within the related
:s:`RecordStructDeclaration`.

:dp:`fls_ZJG2Q6lJYXhY`
The :t:`type` of a :t:`union field` shall be either:

* :dp:`fls_hLTnHnZuaHve`
  A :t:`copy type`, or

* :dp:`fls_JWgSckDtN13c`
  A :t:`mutable reference type`, or

* :dp:`fls_sXZknxozJxtC`
  :std:`core::mem::ManuallyDrop`, or

* :dp:`fls_vgNK01SXacnx`
  A :t:`tuple type` whose :t:`[tuple field]s`' :t:`[type]s` are all valid
  :t:`union field` :t:`[type]s`, or

* :dp:`fls_bQhh3zHAKjSu`
  An :t:`array type` whose :t:`element type` is a valid :t:`union field`
  :t:`[type]s`.

.. rubric:: Examples

.. code-block:: rust

   union LeafNode {
       int: i32,
       float: f32,
       double: f64
   }

.. _fls_hbbek3z4wtcs:

Function Types
--------------

.. _fls_xd2oxlebhs14:

Closure Types
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_bsykgnbatpmi`
A :t:`closure type` is a unique anonymous :t:`function type` that encapsulates
all :t:`[capture target]s` of a :t:`closure expression`.

:dp:`fls_zfj4l8bigdg0`
A :t:`closure type` implements the :std:`core::ops::FnOnce` :t:`trait`.

:dp:`fls_bn0ueivujnqk`
A :t:`closure type` that does not move out its :t:`[capture target]s`
implements the :std:`core::ops::FnMut` :t:`trait`.

:dp:`fls_u01kt5glbuz8`
A :t:`closure type` that does not move out or mutate its :t:`[capture target]s`
implements the :std:`core::ops::Fn` :t:`trait`.

:dp:`fls_3jeootwe6ucu`
A :t:`closure type` that does not encapsulate :t:`[capture target]s` is
coercible to a :t:`function pointer type`.

:dp:`fls_63jqtyw0rz8c`
A :t:`closure type` implicitly implements the :std:`core::marker::Copy`
:t:`trait` if all the :t:`[type]s` of the :t:`[value]s` of the
:t:`capturing environment` implement the :std:`core::marker::Copy` :t:`trait`.

:dp:`fls_3c4g9njja5s5`
A :t:`closure type` implicitly implements the :std:`core::clone::Clone`
:t:`trait` if all the :t:`[type]s` of the :t:`[value]s` of the
:t:`capturing environment` implement the :std:`core::clone::Clone` :t:`trait`.

:dp:`fls_2nuhy0ujgq18`
A :t:`closure type` implicitly implements the :std:`core::marker::Send`
:t:`trait` if all the :t:`[type]s` of the :t:`[value]s` of the
:t:`capturing environment` implement the :std:`core::marker::Send` :t:`trait`.

:dp:`fls_5jh07heok8sy`
A :t:`closure type` implicitly implements the :std:`core::marker::Sync`
:t:`trait` if all the :t:`[type]s` of the :t:`[value]s` of the :t:`capturing
environment` implement the :std:`core::marker::Send` :t:`trait`.

.. _fls_airvr79xkcag:

Function Item Types
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_t24iojx7yc23`
A :t:`function item type` is a unique anonymous :t:`function type` that
identifies a :t:`function`.

:dp:`fls_sas3ahcshnrh`
An :t:`external function item type` is a :t:`function item type` where the
related :t:`function` is an :t:`external function`.

:dp:`fls_liwnzwu1el1i`
An :t:`unsafe function item type` is a :t:`function item type` where the related
:t:`function` is an :t:`unsafe function`.

:dp:`fls_e9x4f7qxvvjv`
A :t:`function item type` is coercible to a :t:`function pointer type`.

:dp:`fls_1941wid94hlg`
A :t:`function item type` implements the :std:`core::clone::Clone` :t:`trait`,
the :std:`core::marker::Copy` :t:`trait`, the :std:`core::ops::Fn` :t:`trait`,
the :std:`core::ops::FnMut` :t:`trait`, the :std:`core::ops::FnOnce` :t:`trait`,
the :std:`core::marker::Send` :t:`trait`, and the :std:`core::marker::Sync`
:t:`trait`.

.. _fls_3i4ou0dq64ny:

Indirection Types
-----------------

.. _fls_xztr1kebz8bo:

Function Pointer Types
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   FunctionPointerTypeSpecification ::=
       ForGenericParameterList? FunctionPointerTypeQualifierList $$fn$$
         $$($$ FunctionPointerTypeParameterList? $$)$$ ReturnTypeWithoutBounds?

   FunctionPointerTypeQualifierList ::=
       $$unsafe$$? AbiSpecification?

   FunctionPointerTypeParameterList ::=
       FunctionPointerTypeParameter ($$,$$ FunctionPointerTypeParameter)*
         ($$,$$ VariadicPart | $$,$$?)

   VariadicPart ::=
       OuterAttributeOrDoc* $$...$$

   FunctionPointerTypeParameter ::=
       OuterAttributeOrDoc* (IdentifierOrUnderscore $$:$$)? TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_v2wrytr3t04h`
A :t:`function pointer type` is an :t:`indirection type` that refers to a
:t:`function`.

:dp:`fls_5dd7icjcl3nt`
An :t:`unsafe function pointer type` is a function pointer type subject to
:t:`keyword` ``unsafe``.

:dp:`fls_B0SMXRqQMS1E`
A :t:`variadic part` indicates the presence of :t:`C`-like optional
parameters.

:dp:`fls_hbn1l42xmr3h`
A :t:`variadic part` shall be specified only when the :t:`ABI` of the
:t:`function pointer type` is either ``extern "C"`` or ``extern "cdecl"``.

:dp:`fls_g1iYVw7upBnH`
The :t:`return type` of a :t:`function pointer type` is determined as follows:

* :dp:`fls_8gpvNJfVlyaD`
  If the :t:`function pointer type` specifies a :s:`ReturnTypeWithoutBounds`, then the :t:`return type` is the specified :s:`ReturnTypeWithoutBounds`.

* :dp:`fls_KcI6yK0P8Onn`
  Otherwise the :t:`return type` is the :t:`unit type`.

.. rubric:: Undefined Behavior

:dp:`fls_52thmi9hnoks`
It is a :t:`validity invariant` for a :t:`value` of a :t:`function pointer type`
to be not :c:`null`.

.. rubric:: Examples

.. code-block:: rust

   unsafe extern "C" fn (value: i32, ...) -> f64

.. _fls_ppd1xwve3tr7:

Raw Pointer Types
~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   RawPointerTypeSpecification ::=
       $$*$$ ($$const$$ | $$mut$$) TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:dp:`fls_rpbhr0xukbx9`
A :t:`raw pointer type` is an :t:`indirection type` without validity guarantees.

:dp:`fls_bYWfGDAQcWfA`
A :t:`mutable raw pointer type` is a :t:`raw pointer type` subject to
:t:`keyword` ``mut``.

:dp:`fls_8uWfFAsZeRCs`
An :t:`immutable raw pointer type` is a :t:`raw pointer type` subject to
:t:`keyword` ``const``.

:dp:`fls_hrum767l6dte`
Comparing two :t:`[value]s` of :t:`[raw pointer type]s` compares the addresses
of the :t:`[value]s`.

:dp:`fls_k6ues2936pjq`
Comparing a :t:`value` of a :t:`raw pointer type` to a :t:`value` of a
:t:`dynamically sized type` compares the data being pointed to.

.. rubric:: Examples

.. code-block:: rust

   *const i128
   *mut bool

.. _fls_142vncdktbin:

Reference Types
~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ReferenceTypeSpecification ::=
       $$&$$ LifetimeIndication? $$mut$$? TypeSpecificationWithoutBounds

.. rubric:: Legality Rules

:dp:`fls_twhq24s8kchh`
A :t:`reference type` is an :t:`indirection type` with :t:`ownership`.

:dp:`fls_w4NbA7WhZfR2`
A :t:`shared reference type` is a :t:`reference type` not subject to
:t:`keyword` ``mut``.

:dp:`fls_ie0avzljmxfm`
A :t:`shared reference type` prevents the direct mutation of a referenced
:t:`value`.

:dp:`fls_15zdiqsm1q3p`
A :t:`shared reference type` implements the :std:`core::marker::Copy`
:t:`trait`. Copying a :t:`shared reference` performs a shallow copy.

:dp:`fls_csdjfwczlzfd`
Releasing a :t:`shared reference` has no effect on the :t:`value` it refers to.

:dp:`fls_GUZuiST7ucib`
A :t:`mutable reference type` is a :t:`reference type` subject to :t:`keyword`
``mut``.

:dp:`fls_vaas9kns4zo6`
A :t:`mutable reference type` allows the direct mutation of a referenced
:t:`value`.

:dp:`fls_n6ffcms5pr0r`
A :t:`mutable reference type` does not implement the :std:`copy::marker::Copy`
:t:`trait`.

.. rubric:: Undefined Behavior

:dp:`fls_ezh8aq6fmdvz`
It is :t:`validity invariant` for a :t:`value` of a :t:`reference type` to be
not :c:`null`.

.. rubric:: Examples

.. code-block:: rust

   &i16
   &'a mut f32

.. _fls_1ompd93w7c9f:

Trait Types
-----------

.. _fls_3xqobbu7wfsf:

Impl Trait Types
~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ImplTraitTypeSpecification ::=
       $$impl$$ TypeBoundList

   ImplTraitTypeSpecificationOneBound ::=
       $$impl$$ TraitBound

.. rubric:: Legality Rules

:dp:`fls_a6zlvyxpgsew`
An :t:`impl trait type` is a :t:`type` that implements a :t:`trait`, where the
:t:`type` is known at compile time.

:dp:`fls_ieyqx5vzas2m`
An :t:`impl trait type` shall appear only within a :t:`function parameter` or
the :t:`return type` of a :t:`function`.

:dp:`fls_3aKZB0ILIkZw`
An :t:`anonymous return type` is an :t:`impl trait type` ascribed to a
:t:`function` :t:`return type`.

:dp:`fls_Xo1ODwOyX7Vm`
An :t:`anonymous return type` behaves as if it contained all declared :t:`[type
parameter]s` of the :t:`return type`'s :t:`function` and its parent :t:`trait`
or :t:`implementation`.

:dp:`fls_kTGFLFymTWch`
An :t:`anonymous return type` derived from an :t:`async function` behaves as if
it contained all declared :t:`[type parameter]s` and :t:`[lifetime parameter]s`
of the :t:`return type`'s :t:`function` and its parent :t:`trait` or
:t:`implementation`.

:dp:`fls_ECjhEI7eCwAj`
An :t:`impl trait type` shall not contain :t:`[opt-out trait bound]s`.

.. rubric:: Examples

.. code-block:: rust

   fn anonymous_type_parameter
       (arg: impl Copy + Send + Sync) { ... }

   fn anonymous_return_type () -> impl MyTrait { ... }

.. _fls_qa98qdi42orq:

Trait Object Types
~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   TraitObjectTypeSpecification ::=
       $$dyn$$ TypeBoundList

   TraitObjectTypeSpecificationOneBound ::=
       $$dyn$$ TraitBound

.. rubric:: Legality Rules

:dp:`fls_sgrvona1nb6h`
A :t:`trait object type` is a :t:`type` that implements a :t:`trait`, where the
:t:`type` is not known at compile time.

:dp:`fls_eWac7zOda3lh`
The :t:`principal trait` of :t:`trait object type` is the first :t:`trait bound`.

:dp:`fls_9z8oleh0wdel`
The :t:`principal trait` shall denote an :t:`object safe` :t:`trait`.

:dp:`fls_hJII8XYAtZeY`
All non-:t:`principal trait` :t:`[trait bound]s` shall denote :t:`[auto trait]s`.

:dp:`fls_s0oy2c8t4yz9`
A :t:`trait object type` shall not contain :t:`[opt-out trait bound]s`.

:dp:`fls_CcoUug6b9ohU`
A :t:`trait object type` shall contain at most one :t:`lifetime bound`.

:dp:`fls_88b9bmhra55f`
A :t:`trait object type` is a :t:`dynamically sized type`. A
:t:`trait object type` permits late binding of :t:`[method]s`. A :t:`method`
invoked via a :t:`trait object type` involves dynamic dispatching.

.. rubric:: Examples

.. code-block:: rust

   dyn MyTrait
   dyn MyTrait + Send
   dyn MyTrait + 'static + Copy

.. _fls_3pbipk8ki18d:

Other Types
-----------

.. _fls_s45k21yn4qur:

Inferred Types
~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   InferredType ::=
       $$_$$

.. rubric:: Legality Rules

:dp:`fls_xdtgr5toulpb`
An :t:`inferred type` is a placeholder for a :t:`type` deduced by
:t:`type inference`.

:dp:`fls_3abhsuaa8nas`
An :t:`inferred type` shall not appear in the following positions:

* :dp:`fls_hBXlJhbhuoHY`
  Within the :s:`InitializationType` of a :s:`TypeAliasDeclaration`,

* :dp:`fls_Vxlr9ZcqiOvY`
  Within the :s:`ReturnType` of a :s:`FunctionDeclaration`,

* :dp:`fls_gE9VC8JXrl1N`
  Within the :s:`TypeAscription` of a :s:`ConstantDeclaration`, a
  :s:`ConstantParameter`, a :s:`FunctionParameterPattern`, a
  :s:`RecordStructField`, a :s:`StaticDeclaration`, or a :s:`TypedSelf`,

* :dp:`fls_ybyQjFamI1Q5`
  Within the :s:`TypeSpecification` of a :s:`FunctionParameter`, an
  :s:`ImplementingType`, a :s:`TupleStructField`, a :s:`TypeBoundPredicate`, or
  a :s:`TypeParameter`.

:dp:`fls_9d8wbugmar1m`
An :t:`inferred type` forces a tool to deduce a :t:`type`, if possible.

.. rubric:: Examples

.. code-block:: rust

   let values: Vec<_> = (0 .. 10).collect();

.. _fls_XJCXBAJHzP3D:

Type Parameters
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_dCIIVXGhXDlO`
A :t:`type parameter type` is a placeholder :t:`type` of a :t:`type parameter`
to be substituted by :t:`generic substitution`.

.. rubric:: Examples

.. code-block:: rust

   fn type_parameter<T>(parameter: T) {}

.. _fls_98lnexk53ru4:

Never Type
~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   NeverType ::=
       $$!$$

.. rubric:: Legality Rules

:dp:`fls_4u0v5uy95pyf`
The :t:`never type` is a :t:`type` that represents the result of a computation
that never completes.

:dp:`fls_xmtc10qzw0ui`
The :t:`never type` has no :t:`[value]s`.

.. rubric:: Undefined Behavior

:dp:`fls_22e8quna7ed5`
It is :t:`validity invariant` to not have a :t:`value` of the :t:`never type`.

.. rubric:: Examples

.. code-block:: rust

   let never_completes: ! = panic!();

.. _fls_olbj67eyxz2k:

Parenthesized Types
~~~~~~~~~~~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   ParenthesizedTypeSpecification ::=
       $$($$ TypeSpecification $$)$$

.. rubric:: Legality Rules

:dp:`fls_1dvo1epstrdv`
A :t:`parenthesized type` is a :t:`type` that disambiguates the interpretation
of :t:`[lexical element]s`.

.. rubric:: Examples

.. code-block:: rust

   &'a (dyn MyTrait + Send)

.. _fls_kgvleup5mdhq:

Type Aliases
------------

.. rubric:: Syntax

.. syntax::

   TypeAliasDeclaration ::=
       $$type$$ Name GenericParameterList? ($$:$$ TypeBoundList)? WhereClause?
         ($$=$$ InitializationType WhereClause?)? $$;$$

   InitializationType ::=
       TypeSpecification

.. rubric:: Legality Rules

:dp:`fls_bibigic4jjad`
A :t:`type alias` is an :t:`item` that defines a :t:`name` for a :t:`type`.

:dp:`fls_rosdkeck5ax2`
A :t:`type alias` shall not have a :s:`TypeBoundList` unless it is an
:t:`associated item`.

:dp:`fls_drxl7u3etfp9`
The last :t:`where clause` is rejected, but may still be consumed by
:t:`[macro]s`.

.. rubric:: Examples

.. code-block:: rust

   type Point = (f64, f64);

.. _fls_7pby13muw48o:

Representation
--------------

.. _fls_g1z6bpyjqxkz:

Type Layout
~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_kdbq02iguzgl`
All :t:`[value]s` have an :t:`alignment` and a :t:`size`.

:dp:`fls_26Xgem831Nqg`
A :dt:`dynamically sized type` is a :t:`type` that does not implement the :std:`core::marker::Sized` :t:`trait`.

:dp:`fls_ozYgHEHFTT5c`
A :dt:`fat pointer type` is an :t:`indirection type` whose contained :t:`type specification` is a :t:`dynamically sized type`.

:dp:`fls_muxfn9soi47l`
The :t:`alignment` of a :t:`value` specifies which addresses are valid for
storing the :t:`value`. :t:`Alignment` is measured in bytes, is at least one,
and always a power of two. A :t:`value` of :t:`alignment` ``N`` is stored at an
address that is a multiple of ``N``.

:dp:`fls_1pbwigq6f3ha`
The :t:`size` of a :t:`type` is the offset in bytes between successive elements
in :t:`array type` ``[T, N]`` where ``T`` is the :t:`type` of the :t:`value`,
including any padding for :t:`alignment`. :t:`Size` is a multiple of the
:t:`alignment`.

:dp:`fls_bk3nm2n47afu`
The :t:`size` of :t:`[scalar type]s` is as follows:

.. list-table::

   * - :dp:`fls_z3i758jshvhx`
     - **Type**
     - **Size**
   * - :dp:`fls_uixe1ruv52be`
     - :c:`bool`
     - 1
   * - :dp:`fls_7at60xlxm9u4`
     - :c:`u8`, :c:`i8`
     - 1
   * - :dp:`fls_395247pkxv48`
     - :c:`u16`, :c:`i16`
     - 2
   * - :dp:`fls_tbe9sc75timc`
     - :c:`u32`, :c:`i32`
     - 4
   * - :dp:`fls_7jaqx33re3hg`
     - :c:`u64`, :c:`i64`
     - 8
   * - :dp:`fls_asys0iz6m0md`
     - :c:`u128`, :c:`i128`
     - 16
   * - :dp:`fls_wfv5vcxl2lc7`
     - :c:`f32`
     - 4
   * - :dp:`fls_x8dfw50z9c`
     - :c:`f64`
     - 8
   * - :dp:`fls_nyxnnlwmt5gu`
     - :c:`char`
     - 4

:dp:`fls_lwmrljw9m0pb`
Types :c:`usize` and :c:`isize` have :t:`size` big enough to contain every
address on the target platform.

:dp:`fls_pzi6izljfv0f`
For :t:`type` :c:`str`, the :t:`layout` is that of :t:`slice type`
``[u8]``.

:dp:`fls_7cjbxleo998q`
For :t:`array type` ``[T; N]`` where ``T`` is the :t:`element type` and ``N``
is :t:`size operand`, the :t:`alignment` is that of ``T``, and the :t:`size` is
calculated as ``core::mem::size_of::<T>() * N``.

:dp:`fls_veotnstzigw2`
For a :t:`slice type`, the :t:`layout` is that of the :t:`array type` it slices.

:dp:`fls_nmoqk7jo1kzf`
For a :t:`tuple type`, the :t:`layout` is tool-defined. For a :t:`unit tuple`,
the :t:`size` is zero and the :t:`alignment` is one.

:dp:`fls_gd7wozpn2ecp`
For a :t:`closure type`, the :t:`layout` is tool-defined.

:dp:`fls_18ke90udyp67`
For a :t:`thin pointer`, the :t:`size` and :t:`alignment` are those of :t:`type`
:c:`usize`.

:dp:`fls_nrqG8i3fmpm4`
For a :t:`function pointer type`, the :t:`size` and :t:`alignment` are those of
a :t:`thin pointer`.

:dp:`fls_e5hivr6m5s3h`
For a :t:`fat pointer type`, the :t:`size` and :t:`alignment` are tool-defined, but
are at least those of a :t:`thin pointer`.
For a :t:`fat pointer type` whose contained :t:`type` is that of a :t:`slice` or :t:`trait object type` the :t:`size` is that of two times the size of :t:`type` :c:`usize` and the :t:`alignment` is that of :t:`type` :c:`usize`.

:dp:`fls_hlbsjggfxnt2`
For a :t:`trait object type`, the :t:`layout` is the same as the :t:`value`
being coerced into the :t:`trait object type` at runtime.

:dp:`fls_sdrb0k2r18my`
For a :t:`struct type`, the memory layout is undefined, unless the
:t:`struct type` is subject to :t:`attribute` :c:`repr`.

:dp:`fls_gt3tkbn4bsa6`
For a :t:`union type`, the memory layout is undefined, unless the
:t:`union type` is subject to :t:`attribute` :c:`repr`. All :t:`[union field]s`
share a common storage.

:dp:`fls_njvdevz0xqc0`
The :t:`size` of a :t:`recursive type` shall be finite.

.. _fls_ohhsmifo0urd:

Type Representation
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_mpqlyi3lgrfv`
:t:`Type representation` specifies the :t:`layout` of :t:`[field]s` of
:t:`[abstract data type]s`. :t:`Type representation` changes the bit padding
between :t:`[field]s` of :t:`[abstract data type]s` as well as their order, but
does not change the :t:`layout` of the :t:`[field]s` themselves.

:dp:`fls_9dhnanv21y9z`
:t:`Type representation` is classified into:

* :dp:`fls_3dwtkr7vzha0`
  :t:`C representation`,

* :dp:`fls_q465p1xuzxi`
  :t:`Default representation`,

* :dp:`fls_hrsdn21jmgx2`
  :t:`Primitive representation`,

* :dp:`fls_ergdb18tpx25`
  :t:`Transparent representation`.

:dp:`fls_8s1vddh8vdhy`
:t:`C representation` lays out a :t:`type` such that the :t:`type` is
interoperable with the :t:`C` language.

:dp:`fls_b005bktrkrxy`
:t:`Default representation` makes no guarantees about the :t:`layout`.

:dp:`fls_7plbkqlmed0r`
:t:`Primitive representation` is the :t:`type representation` of individual
:t:`[integer type]s`. :t:`Primitive representation` applies only to an
:t:`enum type` that is not a :t:`zero-variant enum type`. It is possible to
combine :t:`C representation` and :t:`primitive representation`.

:dp:`fls_ml4khttq3w5k`
:t:`Transparent representation` applies only to an :t:`enum type` with a
single :t:`enum variant` or a :t:`struct type` where the :t:`struct type` or
:t:`enum variant` has a single :t:`field` of non-zero :t:`size` and any number
of :t:`[field]s` of :t:`size` zero and :t:`alignment` one.

:dp:`fls_9q2iqzbup8oy`
:t:`[Type]s` subject to :t:`transparent representation` have the same
:t:`type representation` as the :t:`type` of their :t:`field` with non-zero
:t:`size`.

:dp:`fls_fsbf6ist38ix`
:t:`Type representation` may be specified using :t:`attribute` :c:`repr`. An
:t:`enum type`, a :t:`struct type`, or a :t:`union type` that is not subject to
:t:`attribute` :c:`repr` has :t:`default representation`.

:dp:`fls_qkkc8x2oghst`
:t:`Type representation` may be specified using :t:`attribute` :c:`[repr]` and
modified further using :t:`attribute` :c:`[repr]`'s :s:`Alignment`
:t:`[representation modifier]s`. A :t:`representation modifier` shall apply only
to a :t:`struct type` or a :t:`union type` subject to :t:`C representation` or
:t:`default representation`.

.. _fls_xc1hof4qbf6p:

Enum Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_p0c62ejo1u1t`
:t:`[Zero-variant enum type]s` shall not be subject to :t:`C representation`.

:dp:`fls_efp1kfgkpba8`
The :t:`size` and :t:`alignment` of an :t:`enum type` without :t:`[field]s`
subject to :t:`C representation`, :t:`default representation`, or
:t:`primitive representation` are those of its :t:`discriminant`.

:dp:`fls_s9c0a0lg6c0p`
The :t:`discriminant type` of an :t:`enum type` with :t:`C representation` is
the corresponding :t:`c signed int type` for the target platform's :t:`C`
:t:`ABI`.

:dp:`fls_slhvf3gmqz4h`
The :t:`discriminant type` of an :t:`enum type` with :t:`default representation`
is tool-defined.

:dp:`fls_u1zy06510m56`
The :t:`discriminant type` of an :t:`enum type` with
:t:`primitive representation` is the :t:`integer type` specified by the
:t:`primitive representation`.

:dp:`fls_ryvqkcx48u74`
It is a static error if the :t:`discriminant type` cannot hold all the
:t:`discriminant` :t:`[value]s` of an :t:`enum type`.

:dp:`fls_zhle0rb0vhpc`
An :t:`enum type` subject to :t:`transparent representation` shall have a single
:t:`enum variant` with

* :dp:`fls_45f57s1gmmh5`
  a single :t:`field` of non-zero :t:`size`, or

* :dp:`fls_hz012yus6b4g`
  any number of :t:`[field]s` of zero :t:`size` and :t:`alignment` one.

:dp:`fls_q5akku2idrwh`
An :t:`enum type` subject to :t:`C representation` or :t:`primitive
representation` has the same :t:`type representation` as a :t:`union type` with
:t:`C representation` that is laid out as follows:

* :dp:`fls_r6o1wv76yw6m`
  Each :t:`enum variant` corresponds to a :t:`struct` whose :t:`struct type` is
  subject to :t:`C representation` and laid out as follows:

  * :dp:`fls_3k1tcfxp0g63`
    The :t:`type` of the first :t:`field` of the :t:`struct type` is the
    :t:`discriminant type` of the :t:`enum type`.

  * :dp:`fls_ebs77rxvk9st`
    The remaining :t:`[field]s` of the :t:`struct type` are the :t:`[field]s` of
    the :t:`enum variant`, in the same declarative order.

:dp:`fls_k907i6w83s2`
An :t:`enum type` subject to :t:`transparent representation` has the same
:t:`type representation` as the single :t:`field` of non-zero :t:`size` of its
:t:`enum variant` if one is present, otherwise the :t:`enum type` has :t:`size`
zero and :t:`alignment` one.

.. _fls_rjxpof29a3nl:

Struct Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_jr9dykj6rydn`
The :t:`alignment` of a :t:`struct type` subject to :t:`C representation` is
the :t:`alignment` of the most-aligned :t:`field` in it.

:dp:`fls_6ck71twmnbg5`
The :t:`size` of a :t:`struct type` subject to :t:`C representation` is
determined as follows:

#. :dp:`fls_hydq3pvm00bn`
   Initialize a current offset to zero.

#. :dp:`fls_yzcdffahxcz`
   For each :t:`field` of the :t:`struct type` in declarative order:

   #. :dp:`fls_t2yqmphfd6he`
      Calculate the :t:`size` and :t:`alignment` of the :t:`field`.

   #. :dp:`fls_fa5nkvu07jlp`
      If the current offset is not a multiple of the :t:`[field]'s`
      :t:`alignment`, add byte padding to the current offset until it is a
      multiple of the :t:`alignment`. The offset of the :t:`field` is the
      current offset.

   #. :dp:`fls_x2pkmgbp63xx`
      Increase the current offset by the :t:`size` of the :t:`field`.

   #. :dp:`fls_y6dwc1ndm395`
      Proceed with the next :t:`field`.

#. :dp:`fls_2npku94ookdn`
   Round up the current offset to the nearest multiple of the :t:`[struct
   type]'s` :t:`alignment`.

#. :dp:`fls_h7nvs25rsi0y`
   The :t:`size` of the :t:`struct type` is the current offset.

:dp:`fls_iu93vpyihrpj`
A :t:`struct type` subject to :t:`transparent representation` shall have:

* :dp:`fls_7sjkej5otxo`
  A single :t:`field` of non-zero :t:`size`, or

* :dp:`fls_gwhceoy0m3or`
  Any number of :t:`[field]s` of :t:`size` zero and :t:`alignment` one.

:dp:`fls_hvkalvr4e2v0`
A :t:`struct type` subject to :t:`transparent representation` has the same
:t:`type representation` as the single :t:`field` of non-zero :t:`size` if one
is present, otherwise the :t:`struct type` has :t:`size` zero and :t:`alignment`
one.

.. _fls_cmq8ogs84ivh:

Union Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_opz1p1neldsg`
The :t:`size` of a :t:`union type` subject to :t:`C representation` is
the maximum of the :t:`[size]s` of all its :t:`[field]s`, rounded up to
:t:`alignment` of the :t:`union type`.

:dp:`fls_y5qtvbx5m90g`
The :t:`alignment` of a :t:`union type` subject to :t:`C representation` is the
maximum of the :t:`[alignment]s` of all of its :t:`[field]s`.

.. _fls_j02707n615z0:

Type Model
----------

.. _fls_3gapgqys3ceb:

Recursive Types
~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_z22std1crl49`
A :t:`recursive type` is a :t:`type` whose contained :t:`[type]s` refer back to
the containing :t:`type`, either directly or by referring to another :t:`type`
which refers back to the original :t:`recursive type`.

:dp:`fls_eddnwlr0rz59`
A :t:`type` that is not an :t:`abstract data type` shall not be recursive.

.. rubric:: Examples

.. code-block:: rust

   enum List<T> {
       Nil,
       Cons(T, Box<List<T>>)
   }

.. _fls_exe4zodlwfez:

Type Unification
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_ryvdhkgm7vzj`
:t:`Type unification` is the process by which :t:`type inference` propagates
known :t:`[type]s` across the :t:`type inference root` and assigns concrete
:t:`[type]s` to :t:`[type variable]s`, as well as a general mechanism to check
for compatibility between two :t:`[type]s` during :t:`method resolution`.

:dp:`fls_67VZrx6dw68H`
A :t:`type` is said to :t:`unify` with another :t:`type` when the domains,
ranges, and structures of both :t:`[type]s` are compatible according to the
rules detailed below.

:dp:`fls_aie0tr62vhw5`
Two types that :t:`unify` are said to be :t:`[unifiable type]s`.

:dp:`fls_3U7Ue6Xzuv9M`
:t:`Type unification` is a symmetric operation. If :t:`type` ``A`` unifies
with :t:`type` ``B``, then ``B`` also unifies with ``A`` and such
:t:`type unification` results in the same observable effects.

:dp:`fls_tIiw5WkTRNf7`
If one of the two :t:`[type]s` is a :t:`type variable`, :t:`type unification`
proceeds as follows:

#. :dp:`fls_EoEbldkKBQW0`
   If either :t:`type` is a :t:`global type variable`, the
   :t:`global type variable` is assigned the :t:`type` of the other unification
   operand.

#. :dp:`fls_16ZDp8PaKi5P`
   Otherwise, if either :t:`type` is a :t:`diverging type variable`, the
   :t:`diverging type variable` is assigned the :t:`type` of the other
   unification operand.

#. :dp:`fls_pGRLTACDvzv2`
   Otherwise, if one :t:`type` ``T`` is an :t:`integer type variable`, behavior
   depends on the other :t:`type` ``U``:

   #. :dp:`fls_fTy3FVt0fK9g`
      If ``U`` is an :t:`integer type` or an :t:`integer type variable`, the
      :t:`integer type variable` ``T`` is assigned :t:`type` ``U``.

   #. :dp:`fls_7IsrfUoPXSZU`
      Otherwise, :t:`type unification` fails.

#. :dp:`fls_Hb95CPyUpCmc`
   Otherwise, if one :t:`type` ``T`` is a :t:`floating-point type variable`,
   behavior depends on the other :t:`type` ``U``:

   #. :dp:`fls_jEZVWlfVPevb`
      If ``U`` is a :t:`floating-point type` or an
      :t:`floating-point type variable`, the :t:`floating-point type variable`
      ``T`` is assigned :t:`type` ``U``.

   #. :dp:`fls_nKcqFo7yIDBe`
      Otherwise, :t:`type unification` fails.

#. :dp:`fls_jkaiBnApJAtt`
   Otherwise, neither :t:`type` is a :t:`type variable`, and the rules below
   are in effect.

:dp:`fls_dhksyjrvx9a`
A :t:`scalar type` is unifiable only with itself.

:dp:`fls_hf0cfkrmt655`
The :t:`never type` is unifiable with any other :t:`type`.

:dp:`fls_k9dag68qpe93`
An :t:`array type` is unifiable only with another :t:`array type` when

* :dp:`fls_m6d9qj9q9u1i`
  The :t:`[element type]s` of both :t:`[array type]s` are unifiable, and

* :dp:`fls_gg3x25qvymmq`
  The sizes of both :t:`[array type]s` are the same.

:dp:`fls_ni296ev8x9v9`
A :t:`slice type` is unifiable only with another :t:`slice type` when the
:t:`[element type]s` of both :t:`[slice type]s` are unifiable.

:dp:`fls_i1m41c4wkfc0`
:t:`Type` :c:`str` is unifiable only with itself.

:dp:`fls_mpq64eal9jo3`
A :t:`tuple type` is unifiable only with another :t:`tuple type` when:

* :dp:`fls_kcr8npsmy0e5`
  The :t:`arity` of both :t:`[tuple type]s` is the same, and

* :dp:`fls_kq3lv1zbangz`
  The :t:`[type]s` of the corresponding :t:`[tuple field]s` are unifiable.

:dp:`fls_so2cgqmawlm7`
An :t:`abstract data type` is unifiable only with another
:t:`abstract data type` when:

* :dp:`fls_vsax8w6y794m`
  The two :t:`[abstract data type]s` are the same :t:`type`, and

* :dp:`fls_1j1wc3uxs7h6`
  The corresponding :t:`[generic substitution]s` are unifiable.

:dp:`fls_9dpea9ty0c2l`
A :t:`closure type` is unifiable only with another :t:`closure type` when:

* :dp:`fls_42oj1ekjihq1`
  The two :t:`[closure type]s` are the same closure, and

* :dp:`fls_gebpqqqvvklf`
  The corresponding :t:`[generic substitution]s` are unifiable.

:dp:`fls_i221hm7rssik`
A :t:`function item type` is unifiable only with another :t:`function item type`
when:

* :dp:`fls_74cug5zfv2wv`
  The two :t:`[function item type]s` are the same function, and

* :dp:`fls_keezxl8v4snf`
  The corresponding :t:`[generic substitution]s` are unifiable.

:dp:`fls_wz2etmkpvxed`
A :t:`function pointer type` is unifiable only with another
:t:`function pointer type` when:

* :dp:`fls_rmqcbb5ja4va`
  The :t:`[lifetime]s` are :t:`variance`-conformant, and

* :dp:`fls_uu8je75y5pss`
  The :t:`unsafety` is the same, and

* :dp:`fls_oksjiq3nmq2k`
  The :t:`ABI` is the same, and

* :dp:`fls_52ymp79ert2`
  The number of :t:`[function parameter]s` is the same, and

* :dp:`fls_g2u1dfhphrrg`
  The :t:`[type]s` of the corresponding :t:`[function parameter]s` are
  unifiable, and

* :dp:`fls_2xgq66qp3h95`
  The presence of a :t:`variadic part` is the same, and

* :dp:`fls_5dh8c5gg0hmk`
  The :t:`[return type]s` are unifiable.

:dp:`fls_ismr7wwvek4q`
A :t:`raw pointer type` is unifiable only with another :t:`raw pointer type`
when:

* :dp:`fls_x9933rjecrna`
  The :t:`mutability` is the same, and

* :dp:`fls_mizmcykgdisb`
  The :t:`[target type]s` are unifiable.

:dp:`fls_paoh0wttde2z`
A :t:`reference type` is unifiable only with another :t:`reference type` when:

* :dp:`fls_akko4dmp4nkw`
  The :t:`mutability` is the same, and

* :dp:`fls_8gldjjxbyyb4`
  The :t:`[target type]s` are unifiable.

:dp:`fls_8jad1ztcuxha`
An :t:`anonymous return type` is unifiable with another :t:`type` when:

* :dp:`fls_j3w9ap9zaqud`
  The :t:`[lifetime]s` are :t:`variance`-conformant, and

* :dp:`fls_yvllot5qnc4s`
  The other :t:`type` implements all :t:`[trait]s` specified by the
  :t:`anonymous return type`.

:dp:`fls_hza5n5eb18ta`
An :t:`impl trait type` is unifiable only with itself.

:dp:`fls_ww16urcjrj6i`
A :t:`trait object type` is unifiable only with another :t:`trait object type`
when:

* :dp:`fls_bnp6or49voxp`
  The :t:`[bound]s` are unifiable, and

* :dp:`fls_hdo4c849q3lo`
  The :t:`[lifetime]s` are unifiable.

:dp:`fls_w9dx5h7m31sj`
A :t:`type alias` is unifiable with another :t:`type` when the aliased :t:`type`
is unifiable with the other :t:`type`.

.. _fls_dw33yt5g6m0k:

Type Coercion
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_w5pjcj9qmgbv`
:t:`Type coercion` is an implicit operation that changes the :t:`type` of a
:t:`value`. Any implicit conversion allowed by :t:`type coercion` can be made
explicit using a :t:`type cast expression`.

:dp:`fls_5v0n2a32bk95`
A :t:`type coercion` takes place at a :t:`coercion site` or within a
:t:`coercion-propagating expression`.

:dp:`fls_j3kbaf43sgpj`
The following :t:`[construct]s` constitute a :dt:`coercion site`:

* :dp:`fls_wxrugvlazy6v`
  The :t:`[argument operand]s` of a :t:`call expression` or a
  :t:`method call expression`.

* :dp:`fls_bhzmble1itog`
  A :t:`constant` declaration.

* :dp:`fls_eu4bt3dw1b8c`
  A :t:`field` of an :t:`abstract data type`.

* :dp:`fls_apstt4elv2k7`
  A :t:`function` result.

* :dp:`fls_sp794uzfiofr`
  A :t:`let statement` with an explicit :t:`type specification`.

* :dp:`fls_xfqny6bwzsu9`
  A :t:`static` declaration.

:dp:`fls_u0e42y7nvn7e`
The following :t:`[expression]s` constitute a
:dt:`coercion-propagating expression`:

* :dp:`fls_p8hp5y506nam`
  Each :t:`operand` of an :t:`array expression`.

* :dp:`fls_fjc9xev8rcu6`
  The :t:`tail expression` of a :t:`block expression`.

* :dp:`fls_n1kh3z8d4q8y`
  The :t:`operand` of a :t:`parenthesized expression`.

* :dp:`fls_dgoypa3hcxc0`
  Each :t:`operand` of a :t:`tuple expression`.

:dp:`fls_h8dkehit8rza`
:t:`Type coercion` from a source :t:`type` to a target :t:`type` is allowed to
occur when:

* :dp:`fls_z00wtlna6grk`
  The source :t:`type` is a :t:`subtype` of the target :t:`type`.

* :dp:`fls_rfjdh79k0wou`
  The source :t:`type` ``T`` coerces to intermediate :t:`type` ``W``, and
  intermediate :t:`type` ``W`` coerces to target :t:`type` ``U``.

* :dp:`fls_e3lgrtqb7jwe`
  The source :t:`type` is ``&T`` and the target :t:`type` is ``*const T``.

* :dp:`fls_fwy2z11c1sji`
  The source :t:`type` is ``&T`` and the target :t:`type` is ``&U``, where ``T``
  implements the ``core::ops::Deref<Target = U>`` :t:`trait`.

* :dp:`fls_aujb44849tq1`
  The source :t:`type` is ``&mut T`` and the target :t:`type` is ``&T``.

* :dp:`fls_p3ym3ycrnd5m`
  The source :t:`type` is ``&mut T`` and the target :t:`type` is ``*mut T``.

* :dp:`fls_jmo42qgix5uw`
  The source :t:`type` is ``&mut T`` and the target :t:`type` is ``&U``, where
  ``T`` implements the ``core::ops::Deref<Target = U>`` :t:`trait`.

* :dp:`fls_tbt4236igdzb`
  The source :t:`type` is ``&mut T`` and the target :t:`type` is ``&mut U``,
  where ``T`` implements the ``core::ops::DerefMut<Target = U>`` :t:`trait`.

* :dp:`fls_7ri4jk2dydfn`
  The source :t:`type` is ``*mut T`` and the target :t:`type` is ``*const T``.

* :dp:`fls_6r3kn0nk5b8o`
  The source :t:`type` is ``type_constructor(T)`` and the target :t:`type` is
  ``type_constructor(U)``, where ``type_constructor`` is one of ``&W``,
  ``&mut W``, ``*const W``, or ``*mut W``, and ``U`` can be obtained from ``T``
  using :t:`unsized coercion`.

* :dp:`fls_ulcdetwp6x96`
  The source :t:`type` is a :t:`function item type`, the target :t:`type` is
  a :t:`function pointer type` and the source's :t:`function signature` is a :t:`subtype` of the target's :t:`function signature`.

* :dp:`fls_2uv1r0gni1fk`
  The source :t:`type` is a non-capturing :t:`closure type`, the target
  :t:`type` is a :t:`function pointer type` and the source's :t:`function signature` is a :t:`subtype` of the target's :t:`function signature`.

* :dp:`fls_sf0c3fbx8z57`
  The source :t:`type` is the :t:`never type` and the target :t:`type` is any
  :t:`type`.

* :dp:`fls_SYnFJBhi0IWj`
  The source :t:`type` is a :t:`trait object type` and the target :t:`type` is a
  :t:`trait object type` with the same :t:`[trait bound]s` and additional
  :t:`[auto trait]s`.

:dp:`fls_iiiu2q7pym4p`
An :t:`unsized coercion` is a :t:`type coercion` that converts a :t:`sized type`
into an :t:`unsized type`. :t:`Unsized coercion` from a source :t:`type` to a
target :t:`type` is allowed to occur when:

* :dp:`fls_jte6n2js32af`
  The source :t:`type` is :t:`array type` ``[T; N]`` and the target :t:`type` is
  :t:`slice type` ``[T]``.

* :dp:`fls_20pvqqayzqra`
  The source :t:`type` is ``T`` and the target :t:`type` is ``dyn U``, where
  ``T`` implements ``U + core::marker::Sized``, and ``U`` is :t:`object safe`.

* :dp:`fls_j8rcy0xvd155`
  The source type is

.. code-block:: rust

               S<..., T, ...> {
                   ...
                   last_field: X
               }

:dp:`fls_wuka4uyo3oj7`
where

* :dp:`fls_w15yo8yvuxq3`
  ``S`` is a :t:`struct type`,

* :dp:`fls_7aw3ifbvfgbd`
  ``T`` implements ``core::marker::Unsize<U>``,

* :dp:`fls_cnkth59djwgl`
  ``last_field`` is a :t:`struct field` of ``S``,

* :dp:`fls_4wbk7pqj010i`
  The :t:`type` of ``last_field`` involves ``T`` and if the :t:`type` of
  ``last_field`` is ``W<T>``, then ``W<T>`` implements
  ``core::marker::Unsize<W<U>>``,

* :dp:`fls_47u0039t0l8f`
  ``T`` is not part of any other :t:`struct field` of ``S``.

:dp:`fls_bmh6g3jju7eq`
and the target ``type`` is ``S<..., U, ...>``.

:dp:`fls_da4w32rsrwxc`
:dt:`Least upper bound coercion` is a :t:`multi-[type coercion]` that is used in
the following scenarios:

* :dp:`fls_zi5311z1w7re`
  To find the common :t:`type` of multiple :t:`if expression` branches.

* :dp:`fls_zst5pa29rpt`
  To find the common :t:`type` of multiple :t:`if let expression` branches.

* :dp:`fls_agw1aej616vf`
  To find the common :t:`type` for multiple :t:`match expression`
  :t:`[match arm]s`.

* :dp:`fls_tnbga5dl4gz8`
  To find the common :t:`type` of :t:`array expression` :t:`[operand]s`.

* :dp:`fls_yoreux8tn65x`
  To find the :t:`return type` of a :t:`closure expression` with multiple
  :t:`[return expression]s`.

* :dp:`fls_r11shke69uu6`
  To find the :t:`return type` of a :t:`function` with multiple
  :t:`[return expression]s`.

:dp:`fls_ky7ykpufb95t`
:t:`Least upper bound coercion` considers a set of source :t:`[type]s` ``T1``,
``T2``, ``...``, ``TN`` and target :t:`type` ``U``. The target :t:`type` is
obtained as follows:

#. :dp:`fls_8kvme0u4u8r6`
   Initialize target :t:`type` ``U`` to source :t:`type` ``T1``.

#. :dp:`fls_rl9yrdfwnu03`
   For each current source :t:`type` ``TC`` in the inclusive range ``T1`` to
   ``TN``

   #. :dp:`fls_iqtmhn8flws7`
      If ``TC`` can be coerced to ``U``, then continue with the next source
      :t:`type`.

   #. :dp:`fls_sr8d5har4s03`
      Otherwise, if ``U`` can be coerced to ``TC``, make ``TC`` the target
      :t:`type` ``U``.

   #. :dp:`fls_92pwnd1xbp5r`
      Otherwise, if ``TC`` and ``U`` are non-capturing :t:`[closure type]s`,
      :t:`[function item type]s`, :t:`[function pointer type]s`, or a
      combination of those :t:`[type]s`, and a :t:`function pointer type` exists
      that both ``TC`` and ``U`` can coerce to, make that
      :t:`function pointer type` be target :t:`type` ``U``.

   #. :dp:`fls_bWHQIL4DSN4S`
      Otherwise, no coercion is performed.

   #. :dp:`fls_ju4ypa5ysga0`
      Continue with the next source :t:`type`.


.. _fls_wsfw8xF3vniL:

Structural Equality
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_uVTpA7gbLCYX`
A :t:`type` is :t:`structurally equal` when its :t:`[value]s` can be compared
for equality by structure.

:dp:`fls_2DZAP6JJjJ9h`
The following :t:`[type]s` are :t:`structurally equal`:

* :dp:`fls_emcNJzl2tHSA`
  :c:`Bool`, :c:`char`, :t:`[function pointer type]s`, :t:`[integer type]s`,
  :c:`str`,  and :t:`[raw pointer type]s`.

* :dp:`fls_HpWSAfaTA1Dz`
  An :t:`abstract data type`, if it implements the
  :std:`core::cmp::Eq` and :std:`core::cmp::PartialEq` :t:`[trait]s` using
  :t:`[derive macro]s` :std:`core::cmp::Eq` and :std:`core::cmp::PartialEq`.

* :dp:`fls_5RcnETrW6f9m`
  :t:`[Array type]s` and :t:`[slice type]s`, if the :t:`[element type]` is
  :t:`structurally equal`.

* :dp:`fls_jMeWhn4sNTPF`
  :t:`[Reference type]s`, if their inner :t:`type` is :t:`structurally equal`.

* :dp:`fls_hBFlaUrrhqZ6`
  :t:`[Tuple type]s`, if the :t:`[type]s` of the :t:`[tuple field]s` are
  :t:`structurally equal`.

.. _fls_omaq7psg83n3:

Interior Mutability
~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_khy2e23i9o7z`
:t:`Interior mutability` is a property of :t:`[type]s` whose :t:`[value]s` can
be modified through :t:`[immutable reference]s`.

:dp:`fls_sWiU26n2xS3r`
A :t:`type` is subject to :t:`interior mutability` when it contains a
:std:`core::cell::UnsafeCell`.

.. _fls_lv7w7aalpwm5:

Type Inference
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_h8sedxew0d4u`
:t:`Type inference` is the process of automatically determining the :t:`type` of
:t:`[expression]s` and :t:`[pattern]s` within a :t:`type inference root`.

:dp:`fls_ybvrhh96fc7y`
A :t:`type inference root` is an :t:`expression` whose inner :t:`[expression]s`
and :t:`[pattern]s` are subject to :t:`type inference` independently of those
found in other :t:`[type inference root]s`.

:dp:`fls_EWBilpepaDcX`
The following :t:`[expression]s` are considered :t:`[type inference root]s`:

* :dp:`fls_NYSzcvf5nQpi`
  A :t:`constant argument`.

* :dp:`fls_htLp5J5ObgNh`
  The :t:`expression` of a :t:`constant initializer`.

* :dp:`fls_cPlCLGCcl7EK`
  The :t:`expression` of a :t:`static initializer`.

* :dp:`fls_KphY5qHev0Dc`
  The :t:`expression` of a :t:`discriminant initializer`.

* :dp:`fls_67Bf5kR5OtYW`
  The :t:`expression` of a :t:`constant parameter initializer`.

* :dp:`fls_sAS3vQpkjksr`
  The :t:`expression` of a :t:`constant argument`.

* :dp:`fls_Sowatt1V988J`
  A :t:`function body`.

* :dp:`fls_A1NVYkepoaMk`
  The :t:`size operand` of an :t:`array expression` or an :t:`array type`.

:dp:`fls_J77VeAlD8rsv`
A :t:`type inference root` imposes an :dt:`expected type` on its :t:`expression`
depending on the :t:`type inference root` as follows:

* :dp:`fls_P9uiamxA4HE3`
  The :t:`expected type` of a :t:`constant argument` is the :t:`type ascription`
  of the :t:`constant parameter`.

* :dp:`fls_5d4hw3gj4w4n`
  The :t:`expected type` of the :t:`expression` of a :t:`constant initializer`
  is the :t:`type` specified by the :t:`type ascription` of the related
  :t:`constant`.

* :dp:`fls_qlovdtcj1v1b`
  The :t:`expected type` of the :t:`expression` of a :t:`static initializer` is
  the :t:`type` specified by the :t:`type ascription` of the related
  :t:`static`.

* :dp:`fls_Z5gKFjZW5rRA`
  The :t:`expected type` of the :t:`expression` of a
  :t:`discriminant initializer` is determined as follows:

  * :dp:`fls_vYvumjTQH9Xg`
    If the :t:`enum type` that contains the :t:`discriminant` is subject to
    :t:`attribute` :c:`repr` that specifies a :t:`primitive representation`, the
    :t:`expected type` is the specified :t:`integer type`.

  * :dp:`fls_QaGKt99CmvF6`
    Otherwise, the :t:`expected type` is :c:`isize`.

* :dp:`fls_RJIeW597XRvS`
  The :t:`expected type` of a :t:`function body` is the :t:`return type` of the
  :t:`function`.

* :dp:`fls_veG2D64fIXvo`
  The :t:`expected type` of a :t:`size operand` of an :t:`array expression` or
  an :t:`array type` is :c:`usize`.

:dp:`fls_uvvn4usfsbhr`
A :t:`type variable` is a placeholder used during :t:`type inference` to stand
in for an undetermined :t:`type` of an :t:`expression` or a :t:`pattern`.

:dp:`fls_gDalJm1XS0mi`
A :t:`global type variable` is a :t:`type variable` that can refer to any
:t:`type`.

:dp:`fls_7ov36fpd9mwe`
An :t:`integer type variable` is a :t:`type variable` that can refer only to
:t:`[integer type]s`.

:dp:`fls_3hv3wxkhjjp1`
A :t:`floating-point type variable` is a :t:`type variable` that can refer only
to :t:`[floating-point type]s`.

:dp:`fls_bXQ63GYYDuMp`
A :t:`diverging type variable` is a :t:`type variable` that can refer to any
:t:`type` and originates from a :t:`diverging expression`.

:dp:`fls_JryXiKBIFvF3`
A :dt:`lifetime variable` is a placeholder used during :t:`type inference` to
stand in for an undetermined :t:`lifetime` of a :t:`type`.

:dp:`fls_rvj3XspFZ1u3`
The :t:`type inference` algorithm uses :t:`type unification` to propagate known
:t:`[type]s` of :t:`[expression]s` and :t:`[pattern]s` across the
:t:`type inference root` being inferred. In the rules detailed below, a static
error occurs when :t:`type unification` fails.

:dp:`fls_6GrNr2izovRN`
Performing :t:`type inference` may introduce a requirement that some :t:`type`
must implement a :t:`trait`, or that a :t:`type` or :t:`lifetime` must outlive
some other :t:`lifetime`. Such requirements are referred to as
:dt:`[obligation]s` and are detailed in the inference rules below.

:dp:`fls_9dSltJ6U98Fo`
If insufficient :t:`type` information is available at the time an
:t:`obligation` is introduced, it may be deferred to be resolved later.
Any time new :t:`type` information is derived during :t:`type inference`, the
tool attempts to resolve all outstanding :t:`[obligation]s` and propagate
any resulting :t:`type` information via :t:`type unification`.

:dp:`fls_v5dWGuBKvQSJ`
When an :t:`associated type` ``<Type as Trait>::Assoc`` is referenced within a
:t:`type inference root` (either explicitly within the source code, or via the
inferece rules below), an :t:`obligation` requiring that ``Type`` implements
``Trait`` is introduced.

:dp:`fls_SZgixDCAx6PQ`
:t:`Type inference` for a :t:`type inference root` proceeds as follows:

#. :dp:`fls_XYY1U9h9HlAa`
   Recursively process all :t:`[expression]s` and :t:`[statement]s` in the
   :t:`type inference root` in program order.

   #. :dp:`fls_1rnssw39aRWn`
      For each :t:`statement`, apply the :t:`statement` inference rules outlined below.

   #. :dp:`fls_aYJaZXcOVVyk`
      For each :t:`expression`, apply the :t:`expression` inference rules outlined below.

#. :dp:`fls_X8kLC7JwiF0A`
   If there are any remaining :t:`[integer type variable]s` that have not been
   unified with a concrete :t:`integer type`, perform integer type fallback by
   unifying them with :c:`i32`.

#. :dp:`fls_ZFQhOxO3jpby`
   If there are any remaining :t:`[floating-point type variable]s` that have not
   been unified with a concrete :t:`floating-point type`, perform floating-point
   type fallback by unifying them with :c:`f64`.

#. :dp:`fls_l1G52a0qqEes`
   If there are any remaining :t:`[diverging type variable]s` that have not been
   unified with a concrete :t:`type`, unify them with the :t:`unit type`.

#. :dp:`fls_Tx4Sx4Qy8y2d`
   If there are any remaining :t:`[global type variable]s` that have not been
   unified with a concrete :t:`type`, raise a static error.

#. :dp:`fls_Nbdth8N0PSRq`
   If there are any remaining :t:`[obligation]s` that do not hold or cannot be
   resolved with the available :t:`type` information, raise a static error.

:dp:`fls_hISRWZUuqE4Q`
The :t:`type inference` rules for :t:`[statement]s` are as follows:

* :dp:`fls_ygi1ACJ0RkfS`
  :t:`[Item statement]s` are not subject to :t:`type inference`.

* :dp:`fls_97Fxlv2KN6QF`
  :t:`[Expression statement]s` apply the :t:`expression` inference rules outlined below
  to the related :t:`expression`, with the :t:`expected type` set to the
  :t:`unit type` if the :t:`expression statement` lacks the character 0x3B
  (semicolon), unset otherwise.

* :dp:`fls_hzXqj6YT1mFr`
  :t:`[Let statement]s` are inferred as follows:

  #. :dp:`fls_Kv0dzoMODtdy`
     If the :t:`let statement` has a :t:`type ascription`, :t:`unify` that
     :t:`type` with the :t:`type` of the :t:`pattern`.

  #. :dp:`fls_5v6TR7oqOwFM`
     If the :t:`let statement` has a :t:`let initializer`, apply the
     :t:`expression` inference rules outlined below to the contained :t:`expression`,
     with the :t:`expected type` set to the type of the :t:`pattern`.

  #. :dp:`fls_Gwx0Kfx68DXL`
     If the :t:`let statement` has a :t:`let initializer` with a
     :t:`block expression`, apply the :t:`expression` inference rules outlined below to
     the contained :t:`block expression`, with the :t:`expected type` set to
     the :t:`never type`.

:dp:`fls_J6ydUCCJp1Sn`
:t:`Type inference` of :t:`[expression]s` may incorporate an :t:`expected type`,
derived from the context the :t:`expression` appears in. If the :t:`expression`
is a :t:`coercion site` or a :t:`coercion-propagating expression`, the :t:`type`
derived via :t:`type inference` may be coerced to the :t:`expected type`. If no
:t:`type coercion` to the :t:`expected type` is possible, or the :t:`expression`
is not a :t:`coercion site` or a :t:`coercion-propagating expression`, the
inferred :t:`expression` :t:`type` is unified with the :t:`expected type`.

:dp:`fls_FSQqHs8T4bUx`
The :t:`type inference` rules for :t:`[expression]s` are as follows:

* :dp:`fls_0HHC1iOk5dwz`
  An :t:`if expression` is inferred by inferring its :t:`subject expression`
  with an :t:`expected type` of :c:`bool`, then inferring its
  :t:`block expression` with the :t:`expected type` of the :t:`if expression`.
  Then, if the :t:`if expression` has an :t:`else expression`, apply the
  inference rules below to it.

* :dp:`fls_QZWTS0Giy3I3`
  An :t:`if let expression` is inferred by inferring its
  :t:`subject let expression` with the :t:`expected type` set to the :t:`type`
  of its :t:`pattern`, then inferring its :t:`block expression` with the
  :t:`expected type` of the :t:`if-let expression`. If the
  :t:`if let expression` has an :t:`else expression`, apply the inference rules
  below to it.

* :dp:`fls_KJsIu1lgVZxP`
  An :t:`else expression` that is part of an :t:`if expression` or
  :t:`if let expression` is inferred as follows:

  * :dp:`fls_KRQxdSav1KBA`
    If the :t:`else expression` has a :t:`block expression`, infer the
    :t:`block expression` with the :t:`expected type` of the :t:`if expression`
    or :t:`if let expression`.

  * :dp:`fls_Mcpwyvz47SoG`
    If the :t:`else expression` has an :t:`if expression`, infer that nested
    :t:`if expression` with the :t:`expected type` of the original
    :t:`if expression`, then :t:`unify` its :t:`type` with the :t:`type` of
    the original :t:`if expression` or :t:`if let expression`.

  * :dp:`fls_34AQ9g7xhdUj`
    Otherwise, the :t:`else expression` has an :t:`if let expression`. Infer
    that nested :t:`if let expression` with the :t:`expected type` of the
    original :t:`if expression`, then :t:`unify` its :t:`type` with the
    :t:`type` of the original :t:`if expression` or :t:`if let expression`.

* :dp:`fls_4ZT35povCL04`
  A :t:`match expression` is inferred as follows:

  #. :dp:`fls_62OcWZaVN9hh`
     :t:`Unify` the :t:`[type]s` of the :t:`[pattern]s` of every :t:`match arm`,
     then infer the :t:`subject expression` with the :t:`expected type` set to
     the :t:`type` of the :t:`[pattern]s`.

  #. :dp:`fls_st9onPgDrc8y`
     Infer the :t:`[operand]s` of all :t:`[match arm guard]s` with
     :t:`expected type` :c:`bool`.

  #. :dp:`fls_F999gqcBfff9`
     Infer the :t:`match arm body` of every :t:`match arm` with the
     :t:`expected type` of the :t:`match expression`.

* :dp:`fls_Esa4ST7lLp8T`
  A :t:`for loop expression` is inferred by unifying the :t:`type` of its
  :t:`pattern` with the :t:`type` ``<T as core::iter::IntoIterator>::Item``,
  where ``T`` is the :t:`type` of the :t:`subject expression`, and then
  inferring its :t:`loop body`.

* :dp:`fls_9GDElCkL1UbH`
  A :t:`while let loop expression` is inferred by unifying the :t:`type` of its
  :t:`subject let expression` with the :t:`type` of its :t:`pattern`, and then
  inferring its :t:`loop body`.

* :dp:`fls_0eATa6RtDNtA`
  An :t:`array expression` using an :t:`array element constructor` is inferred
  by attempting a :t:`least upper bound coercion` from each element :t:`type` to
  the :t:`expected type`. If no such :t:`type coercion` is possible, all element
  :t:`[type]s` are unified instead.

* :dp:`fls_q1JZZMxqWXCk`
  A :t:`negation expression` is inferred as follows:

  #. :dp:`fls_hH58ftCxBYzm`
     Determine the :t:`trait` corresponding to the operator according to the
     following table:

     .. list-table::

       * - :dp:`fls_aiSI99pbAYqT`
         - **Operator**
         - **Trait**
       * - :dp:`fls_zRdxowO4eDMN`
         - ``!``
         - :std:`core::ops::Not`
       * - :dp:`fls_IoceMi7HfqsK`
         - ``-``
         - :std:`core::ops::Neg`

  #. :dp:`fls_lDkPMB5UI58B`
     Infer the :t:`type` of the :t:`expression` to be the :t:`associated type`
     ``<T as Trait>::Output``, where ``T`` is the :t:`type` of the
     :t:`operand`, and ``Trait`` is the operator :t:`trait` determined from
     the table above.

* :dp:`fls_JKZHF3ZDHshw`
  A :t:`bit expression` or :t:`arithmetic expression` is inferred as follows:

  #. :dp:`fls_rT6zpG3cYhaF`
     Determine the :t:`trait` corresponding to the operator according to the
     following table:

     .. list-table::

       * - :dp:`fls_UFMyHzk6ucsT`
         - **Operator**
         - **Trait**
       * - :dp:`fls_hPZmcfQiNasT`
         - ``+``
         - :std:`core::ops::Add`
       * - :dp:`fls_rgC3Iea5p9Kr`
         - ``-``
         - :std:`core::ops::Sub`
       * - :dp:`fls_5jDBPymVKzDv`
         - ``*``
         - :std:`core::ops::Mul`
       * - :dp:`fls_f21GNntBOxaz`
         - ``/``
         - :std:`core::ops::Div`
       * - :dp:`fls_NpwLzJJH9cGw`
         - ``%``
         - :std:`core::ops::Mod`
       * - :dp:`fls_56J8BlLOuvr4`
         - ``&``
         - :std:`core::ops::BitAnd`
       * - :dp:`fls_jK2pIVxOmtJ8`
         - ``|``
         - :std:`core::ops::BitOr`
       * - :dp:`fls_fjV22WcosNnt`
         - ``^``
         - :std:`core::ops::BitXor`
       * - :dp:`fls_h3OVuCdsKPhV`
         - ``<<``
         - :std:`core::ops::Shl`
       * - :dp:`fls_be2djziKJw3I`
         - ``>>``
         - :std:`core::ops::Shr`

  #. :dp:`fls_nHt0LVSiwTB3`
     If the :t:`expression` is a :t:`shift left expression` or a
     :t:`shift right expression`, and the :t:`expected type` is an
     :t:`integer type`, :t:`unify` the :t:`type` of the :t:`left operand` with
     the :t:`expected type`.

  #. :dp:`fls_sLCBZ3vG1AWs`
     If the :t:`expression` is neither a :t:`shift left expression` nor a
     :t:`shift right expression`, and the :t:`expected type` is a
     :t:`numeric type`, :t:`unify` the :t:`[type]s` of both :t:`[operand]s` with
     the :t:`expected type`.

  #. :dp:`fls_mCISAdm7sjRs`
     Infer the :t:`type` of the :t:`expression` to be the :t:`associated type`
     ``<L as Trait<R>>::Output``, where ``L`` is the :t:`type` of the
     :t:`left operand`, ``Trait`` is the operator :t:`trait` determined from
     the table above, and ``R`` is the :t:`type` of the :t:`right operand`.

* :dp:`fls_Fv8fj9R8prUV`
  A :t:`compound assignment expression` is inferred as follows:

  #. :dp:`fls_QDWVv2nTufX7`
     Determine the :t:`trait` corresponding to the operator according to the
     following table:

     .. list-table::

       * - :dp:`fls_O2r51Xrmmj38`
         - **Operator**
         - **Trait**
       * - :dp:`fls_b96Zca6oFn82`
         - ``+=``
         - :std:`core::ops::AddAssign`
       * - :dp:`fls_07AIc06bGnZt`
         - ``-=``
         - :std:`core::ops::SubAssign`
       * - :dp:`fls_A36NBOl1FTCb`
         - ``*=``
         - :std:`core::ops::MulAssign`
       * - :dp:`fls_h3mmmIBR72kV`
         - ``/=``
         - :std:`core::ops::DivAssign`
       * - :dp:`fls_8edzBBIo7jF7`
         - ``%=``
         - :std:`core::ops::ModAssign`
       * - :dp:`fls_lUg26vFuSePP`
         - ``&=``
         - :std:`core::ops::BitAndAssign`
       * - :dp:`fls_21ay7EUUUmhx`
         - ``|=``
         - :std:`core::ops::BitOrAssign`
       * - :dp:`fls_8VgAhOgDOk0y`
         - ``^=``
         - :std:`core::ops::BitXorAssign`
       * - :dp:`fls_OVVY9CE0pGtJ`
         - ``<<=``
         - :std:`core::ops::ShlAssign`
       * - :dp:`fls_FojOvB6l3lAh`
         - ``>>=``
         - :std:`core::ops::ShrAssign`

  #. :dp:`fls_CVfHkJq1PixR`
     Introduce an :t:`obligation` ``L: $Trait<R>``, where ``L`` is the
     :t:`type` of the :t:`assigned operand`, ``Trait`` is the operator
     :t:`trait` determined from the table above, and ``R`` is the :t:`type` of
     the :t:`modifying operand`.

  #. :dp:`fls_0RZ7w0YqmzE3`
     The :t:`type` of the :t:`expression` is the :t:`unit type`.

* :dp:`fls_YppNCEPMYqWJ`
  A :t:`comparison expression` is inferred by introducing an :t:`obligation`
  ``L: PartialEq<R>``, where ``L`` is the :t:`type` of the :t:`left operand`,
  and ``R`` is the :t:`type` of the :t:`right operand`. The :t:`type` of the
  :t:`expression` is :c:`bool`.

* :dp:`fls_SZmiJjI43fQL`
  An :t:`assignment expression` is inferred by unifying the :t:`type` of its
  :t:`assignee operand` with the :t:`type` of its :t:`value operand`.

* :dp:`fls_ipWTrhF4xakC`
  A :t:`closure expression` is inferred by deducing its signature from the
  surrounding context, unifying the deduced :t:`closure parameter` :t:`[type]s`
  and return :t:`type` with the user-written :t:`closure parameter`
  :t:`[pattern]s` and :t:`[type ascription]s` and return :t:`type`, and then
  inferring the :t:`closure body` with the :t:`expected type` set to the
  closure's return :t:`type`. The closure signature is deduced as follows:

  * :dp:`fls_zgRFKbfdI6ro`
    If the :t:`expected type` is a :t:`function pointer type`, the closure
    signature is the signature of that :t:`function pointer type`.

  * :dp:`fls_z2UkZX5Qdmws`
    Otherwise, if there is a pending :t:`obligation` requiring that the
    :t:`expected type` implements :std:`core::ops::FnOnce` or a :t:`trait` that
    has :std:`core::ops::FnOnce` as one of its :t:`[supertrait]s`, derive the
    closure signature from the parameters and return :t:`type` of the
    :std:`core::ops::FnOnce` :t:`bound` or :t:`supertrait`.

  * :dp:`fls_U46IXItEKMCB`
    Otherwise, the closure signature remains undeduced. No outside type
    information is provided and the parameter :t:`[type]s` and return :t:`type`
    are subject to regular :t:`type inference`.

* :dp:`fls_TAJ3JJwIeDbQ`
  Other :t:`[expression]s` are inferred by applying the typing rules specified
  in the section for that :t:`expression`.

:dp:`fls_VrpaTruoBwtF`
If an :t:`expression` is a :t:`diverging expression`, its :t:`type` is a new
:t:`diverging type variable`.

.. _fls_85vx1qfa061i:

Traits
------

.. rubric:: Syntax

.. syntax::

   TraitDeclaration ::=
       $$unsafe$$? $$trait$$ Name GenericParameterList? ($$:$$ SupertraitList?)? WhereClause? TraitBody

   SupertraitList ::=
       TypeBoundList

   TraitBody ::=
       $${$$
         InnerAttributeOrDoc*
         AssociatedItem*
       $$}$$

.. rubric:: Legality Rules

:dp:`fls_tani6lesan9u`
A :t:`trait` is an :t:`item` that describes an interface a :t:`type` can
implement.

:dp:`fls_PiAR1B26SoZV`
A :t:`trait body` is a :t:`construct` that encapsulates the
:t:`[associated item]s`, :t:`[inner attribute]s`, and
:t:`[inner doc comment]s` of a :t:`trait`.

:dp:`fls_Y28596CVBzDG`
Within a :t:`trait`, the :t:`type` :c:`Self` acts as a placeholder for a
:t:`type` implementing the :t:`trait`, and behaves like a :t:`type parameter`.

:dp:`fls_AdbbUZZgMEsQ`
A :t:`local trait` is a :t:`trait` that is defined in the current :t:`crate`.

:dp:`fls_I9JaKZelMiby`
A :t:`subtrait` is a :t:`trait` with a :t:`supertrait`.

:dp:`fls_CYtxPjK3zq2T`
A :t:`supertrait` is a transitive :t:`trait` that a :t:`type` must additionally
implement.

:dp:`fls_ytn5cdonytyn`
A :t:`subtrait` shall not be its own :t:`supertrait`.

:dp:`fls_vucd1u38sq7i`
A :t:`trait` of the form

.. code-block:: rust

   	trait T: Bound {}

:dp:`fls_kyr81mi01me2`
is equivalent to a :t:`where clause` of the following form:

.. code-block:: rust

   	trait T where Self: Bound {}

:dp:`fls_YynbrIceKmsJ`
An :t:`auto trait` is a :t:`trait` that is implicitly and automatically
implemented by a :t:`type` when the types of its constituent :t:`[field]s`
implement the :t:`trait`.

:dp:`fls_Bd4HwdrRuXMm`
A :t:`type` that has no :t:`[field]s` implements all :t:`[auto trait]s`.

:dp:`fls_UzfG5ic8PUIH`
If determining whether a :t:`type` ``T`` implements an :t:`auto trait`
would recursively depend on whether ``T`` implements said :t:`auto trait`, this
requirement is ignored and assumed to hold.

:dp:`fls_02D6ku4Sd6yL`
The following :t:`[trait]s` are :t:`[auto trait]s`:

* :dp:`fls_RLFIzQeAPhG6`
  :std:`core::marker::Send`

* :dp:`fls_avSxO0LEka2x`
  :std:`core::marker::Sync`

* :dp:`fls_ft8axGGOe3aL`
  :std:`core::marker::Unpin`

* :dp:`fls_M4EoT1NMyxJS`
  :std:`core::panic::UnwindSafe`

* :dp:`fls_Tir2kpKNP1KC`
  :std:`core::panic::RefUnwindSafe`

:dp:`fls_WxHiKr8BGGvz`
No other :t:`[trait]s` are :t:`[auto trait]s`.

.. rubric:: Examples

.. code-block:: rust

   trait Sequence<T> {
       fn length(&self) -> u32;
       fn element_at(&self, position: u32) -> T;
   }

:dp:`fls_mjg7yrq66hh0`
Shape is a supertrait of Circle.

.. code-block:: rust

   trait Shape {
       fn area(&self) -> f64;
   }

:dp:`fls_ydowwijzirmm`
Circle is a subtrait of Shape.

.. code-block:: rust

   trait Circle: Shape {
       fn radius(&self) -> f64;
   }

.. _fls_4ikc07mfrez5:

Object Safety
~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_lrdki56hpc3k`
A :t:`trait` is :t:`object safe` when:

* :dp:`fls_5wlltclogfkw`
  Its :t:`[supertrait]s` are :t:`object safe`, and

* :dp:`fls_droy0w5gtqaw`
  :std:`core::marker::Sized` is not a :t:`supertrait`, and

* :dp:`fls_46gd1q80c6bn`
  It lacks :t:`[associated constant]s`, and

* :dp:`fls_kwo4cknx0yat`
  Its :t:`[associated function]s` are :t:`object safe`, and

* :dp:`fls_vmLLL82EQasI`
  Its :t:`[associated type alias]es` specify a :std:`core::marker::Sized`
  :t:`[trait bound]` for :c:`Self` in a :t:`type bound predicate`.

:dp:`fls_uixekv82g2e5`
An :t:`associated function` is :t:`object safe` when it is either an
:t:`object safe` dispatchable :t:`function` or an :t:`object safe`
non-dispatchable :t:`function`.

:dp:`fls_72tvfoemwpyy`
A dispatchable :t:`function` is :t:`object safe` when:

* :dp:`fls_j7nb34o87l1z`
  It lacks :t:`[generic parameter]s`, and

* :dp:`fls_k1vc9vd8at92`
  Is a :t:`method` that does not use :c:`Self` in its :t:`function signature`
  except in the :t:`type` of its :t:`self parameter` or as the :t:`type` of a
  :t:`type bound predicate`, and

* :dp:`fls_kqylg31sm5wv`
  It lacks :t:`[type bound predicate]s` with :c:`Self` as the predicate's
  :t:`type` and :t:`[trait]s` as the predicate's :t:`[trait bound]s` other than
  :std:`core::marker::Send`, :std:`core::marker::Sync` and
  :std:`core::marker::Unpin`

:dp:`fls_aer3gaur7avp`
A :t:`function` is :t:`object safe` when it specifies a
:std:`core::marker::Sized` :t:`[trait bound]` for :c:`Self` in a
:t:`type bound predicate`.

.. _fls_jeoas4n6su4:

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
       $$($$ TraitBound $$)$$

   TraitBound ::=
       ($$?$$ | ForGenericParameterList)? TypePath

   ForGenericParameterList ::=
       $$for$$ GenericParameterList

.. rubric:: Legality Rules

:dp:`fls_5g508z6c7q5f`
A :t:`bound` imposes a constraint on a :t:`generic parameter` by limiting the
set of possible :t:`[generic substitution]s`.

:dp:`fls_BqLPVaSyyXRG`
A :t:`bound` does not impose a constraint on a :t:`generic parameter` of a
:t:`type alias` unless it is an :t:`associated item`.

:dp:`fls_grby8tmmd8sb`
A :t:`lifetime bound` is a :t:`bound` that imposes a constraint on the
:t:`[lifetime]s` of :t:`[generic parameter]s`.

:dp:`fls_knut10hoz6wc`
A :t:`trait bound` is a :t:`bound` that imposes a constraint on the
:t:`[trait]s` of :t:`[generic parameter]s`.

:dp:`fls_sf6zg0ez9hbb`
A :s:`ForGenericParameterList` shall not specify :s:`[ConstantParameter]s` or
:s:`[TypeParameter]s`.

:dp:`fls_vujl3fblz6x2`
A :t:`higher-ranked trait bound` is a :t:`bound` that specifies an infinite
list of :t:`[bound]s` for all possible :t:`[lifetime]s` specified by the
:s:`ForGenericParameterList`.

:dp:`fls_AzuZmR9DXSQh`
An :t:`opt-out trait bound` is a :t:`trait bound` with :s:`Punctuation` ``?``
that nullifies an implicitly added :t:`trait bound`.

:dp:`fls_1Sm2Yq1Ow76f`
An :t:`outlives bound` is a :t:`trait bound` which requires that a
:t:`lifetime parameter` or :t:`type` outlives a :t:`lifetime parameter`.

:dp:`fls_tx4uspewnk7w`
:t:`Outlives bound` ``'a: 'b`` indicates that ``'a`` outlives ``'b``.

:dp:`fls_5kj8bmvb8xfc`
:t:`Outlives bound` ``T: 'a`` indicates that all :t:`[lifetime parameter]s` of
``T`` outlive ``'a``.

:dp:`fls_J9DEsd06Ttu9`
An :t:`implied bound` is a :t:`bound` that is not expressed in syntax, but is
is the byproduct of relations between :t:`[lifetime parameter]s` and
:t:`[function parameter]s`, between :t:`[lifetime parameter]s` and a
:t:`return type`, and between :t:`[lifetime parameter]s` and :t:`[field]s`.

:dp:`fls_IfHRxSasGAih`
A :t:`reference` of the form ``&'a T``, where ``'a`` is a
:t:`lifetime parameter` and ``T`` is a :t:`type`, yields :t:`implied bound`
``T: 'a``.

:dp:`fls_K8nPGP5xbLb7`
If an :t:`outlives bound` applies to the :t:`type` of a :t:`field`, then this
bound also applies to the related :t:`abstract data type` as an
:t:`implied bound`.

:dp:`fls_Uw6y5ZlaK6RY`
If an :t:`outlives bound` applies to the :t:`type` of a :t:`function parameter`
or to a :t:`return type`, then this bound also applies to the related
:t:`function` as an :t:`implied bound`.

.. rubric:: Examples

.. code-block:: rust

   fn draw<T: Shape>(shape: T) { ... }

.. _fls_yqcygq3y6m5j:

Lifetimes
~~~~~~~~~

.. rubric:: Syntax

.. syntax::

   Lifetime ::=
       $$'$$ NonKeywordIdentifier

   AttributedLifetime ::=
       OuterAttributeOrDoc* Lifetime

   AttributedLifetimeList ::=
       AttributedLifetime ($$,$$ AttributedLifetime)* $$,$$?

.. rubric:: Legality Rules

:dp:`fls_nne91at3143t`
A :t:`lifetime` specifies the expected longevity of a :t:`value`.

:dp:`fls_vbclxg9dq4yo`
A :t:`lifetime bound` shall apply to :t:`[type]s` and other :t:`[lifetime]s`.

.. rubric:: Examples

.. code-block:: rust

   &'a i32
   &'static Shape

:dp:`fls_gcszhqg6hnva`
See :p:`4.12. <fls_85vx1qfa061i>` for the declaration of Shape.

.. _fls_ikfvbeewame7:

Subtyping and Variance
~~~~~~~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_atq2cltx487m`
:t:`Subtyping` is a property of :t:`[type]s`, allowing one :t:`type` to be used
where another :t:`type` is expected.

:dp:`fls_df87d44kgwcv`
:t:`Variance` is a property of :t:`[lifetime parameter]s` and
:t:`[type parameter]s` that describes the circumstances under which a
:t:`generic type` is a :t:`subtype` of an instantiation of itself with
different :t:`[generic argument]s`.

:dp:`fls_7ex941yysuhq`
A :t:`type` is its own :t:`subtype`.

:dp:`fls_7qud6i05ze2`
``F<T>`` is said to be

* :dp:`fls_wpm0p0gtctvi`
  :dt:`Covariant` over ``T``, when ``T`` being a :t:`subtype` of ``U`` implies
  that ``F<T>`` is a :t:`subtype` of ``F<U>``, or

* :dp:`fls_3rfs58i2kg6l`
  :dt:`Contravariant` over ``T``, when ``T`` being a :t:`subtype` of ``U``
  implies that ``F<U>`` is a :t:`subtype` of ``F<T>``, or

* :dp:`fls_kbo3e3bosr0m`
  :dt:`Invariant` over ``T``.

:dp:`fls_n36p6w2a75sm`
:t:`Variance` is determined as follows:

.. list-table::

   * - :dp:`fls_xw7eo3us0ow4`
     - **Type**
     - **Variance in 'a**
     - **Variance in T**
   * - :dp:`fls_qc6jma5g9vpn`
     - ``&'a T``
     - :t:`covariant`
     - :t:`covariant`
   * - :dp:`fls_hpiiwxzg16rj`
     - ``&'a mut T``
     - :t:`covariant`
     - :t:`invariant`
   * - :dp:`fls_aspdlqluwh9w`
     - ``*const T``
     -
     - :t:`covariant`
   * - :dp:`fls_8ohuze7hqtc1`
     - ``*mut T``
     -
     - :t:`invariant`
   * - :dp:`fls_7pkqgxabojkn`
     - ``[T]``
     -
     - :t:`covariant`
   * - :dp:`fls_ln9pqd4xu5e`
     - ``[T; N]``
     -
     - :t:`covariant`
   * - :dp:`fls_z4jo3ojkcu9v`
     - ``fn() -> T``
     -
     - :t:`covariant`
   * - :dp:`fls_67w6yslr3e25`
     - ``fn(T) -> ()``
     -
     - :t:`contravariant`
   * - :dp:`fls_ojal3qytkqql`
     - ``fn(T) -> T``
     -
     - :t:`invariant`
   * - :dp:`fls_owp42z12l4lc`
     - ``core::call::UnsafeCell<T>``
     -
     - :t:`invariant`
   * - :dp:`fls_i1vuix3gj9ej`
     - ``core::marker::PhantomData<T>``
     -
     - :t:`covariant`
   * - :dp:`fls_mlf39pl0b931`
     - ``dyn Trait<T> + 'a``
     - :t:`covariant`
     - :t:`invariant`

:dp:`fls_yknymnlsasyw`
A :t:`trait` is :t:`invariant` in all inputs, including the :c:`Self` parameter.

:dp:`fls_xkzo7nj40rbn`
:t:`[Lifetime parameter]s` and :t:`[type parameter]s` are subject to
:t:`variance`.

:dp:`fls_abn5ycx11zpm`
The :t:`variance` of a :t:`generic parameter` of an :t:`abstract data type` or
a :t:`tuple type` is determined as follows:

#. :dp:`fls_hvfyog9ygn6q`
   For each :t:`generic parameter` ``G``:

   #. :dp:`fls_mduolmcawb30`
      Initialize :t:`variance` ``V`` of the :t:`generic parameter` to ``any``.

   #. :dp:`fls_y81gmqweqc9w`
      For each :t:`field` of the :t:`abstract data type` or the
      :t:`tuple type`:

      #. :dp:`fls_etgfvgvymn8o`
         If :t:`field` :t:`type` ``T`` uses ``G``, then

         #. :dp:`fls_4kjxxrsk1igf`
            If ``V`` is ``any``, set ``V`` to the :t:`variance` of ``T`` over
            ``G``.

         #. :dp:`fls_y4zmb3vrym7p`
            Otherwise if ``V`` and the :t:`variance` of ``T`` over ``G`` differ,
            set ``V`` to :t:`invariant`.

   #. :dp:`fls_9ae3idezsths`
      It is a static error if :t:`variance` ``V`` is ``any``.

:dp:`fls_WD6uyTCziRac`
:t:`[Expression]s` and :t:`[statement]s` may impose :t:`subtyping` requirements
on their :t:`[subexpression]s`. Such requirements are applied after
:t:`type inference`, on the :t:`[inferred type]s` of the respective
:t:`[expression]s` and :t:`[pattern]s`.

:dp:`fls_xURR0owesaIE`
It is a static error if any :t:`subtyping` requirements are not met.

:dp:`fls_CvZNYIfnOUcc`
The :t:`subtyping` requirements for :t:`[statement]s` are as follows:

* :dp:`fls_xrLYQX7W9OnR`
  :t:`[Item statement]s` impose no additional :t:`subtyping` requirements.

* :dp:`fls_DWPN8DRysgMa`
  :t:`[Let statement]s` require that the :t:`type` of the :t:`expression` of the
  :t:`let initializer` (if any) is a :t:`subtype` of the :t:`type` of the
  :t:`let statement`'s :t:`pattern`.

* :dp:`fls_tOn5oKtp300J`
  :t:`[Expression statement]s` impose the :t:`subtyping` requirements for the
  contained :t:`expression`, as outlined below.

:dp:`fls_HqXTVi7gr9wR`
The :t:`subtyping` requirements for :t:`[expression]s` are as follows:

* :dp:`fls_lxfPvK7NDNlj`
  The requirements for any :t:`arithmetic expression`, :t:`bit expression`,
  :t:`comparison expression`, :t:`compound assignment expression`,
  :t:`index expression`, or :t:`negation expression` are the same requirements
  as for an explicit invocation of the corresponding operator :t:`trait`
  :t:`method`.

* :dp:`fls_awJeqcAHn5O3`
  An :t:`assignment expression` requires that the :t:`type` of its
  :t:`value operand` is a :t:`subtype` of the :t:`type` of its
  :t:`assignee operand`.

* :dp:`fls_AD1IXzsnlwyg`
  A :t:`type cast expression` requires that the :t:`type` of its :t:`operand` is
  a :t:`subtype` of its :t:`target type`.

* :dp:`fls_p2II8E6cc9zt`
  A :t:`call expression` or :t:`method call expression` requires that the
  :t:`[type]s` of its :t:`[argument operand]s` are :t:`[subtype]s` of the
  :t:`[type]s` of the corresponding parameter.

* :dp:`fls_pN5C4FHWEWdK`
  A :t:`return expression` requires that the :t:`type` of its :t:`operand` is
  a :t:`subtype` of the :t:`return type` of the containing :t:`function` or
  :t:`closure expression`.

* :dp:`fls_RdqV0hI8h8BI`
  A :t:`break expression` requires that its :t:`break type` is a :t:`subtype` of
  the :t:`type` of the :t:`block expression` or :t:`loop expression` that the
  :t:`break expression` breaks out of.

* :dp:`fls_Uv5CcMHPX79J`
  Other :t:`[expression]s` do not impose any additional :t:`subtyping`
  requirements.

:dp:`fls_LyvV4pOG7E4l`
Any :t:`type coercion` resulting in a :t:`method` invocation imposes the same
:t:`subtyping` requirements as an explicit invocation of that :t:`method` would.

.. _fls_l9ebxrlxyawd:

Lifetime Elision
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_9wtuclhm7yz5`
:t:`Lifetime elision` is a set of rules that automatically insert
:t:`[lifetime parameter]s` and/or :t:`[lifetime argument]s` when they are
elided in the source code.

:dp:`fls_JmP6O9zj8fkV`
A :t:`lifetime` may be elided either implicitly or explicitly.

:dp:`fls_5ZAQ9p7jQuc2`
A :t:`lifetime` is elided explicitly if it is the ``'_`` :t:`lifetime`.

:dp:`fls_YmUQ8ZiQuycp`
A :t:`lifetime` is elided implicitly if it is absent.

:dp:`fls_sIMN6Sd8xUZz`
:t:`Lifetime elision` rules are introduced by certain :t:`[construct]s` and may
be nested.

:dp:`fls_dIyisjNIx9dC`
An elided :t:`lifetime` is subject to the set of :t:`lifetime elision` rules
introduced by the innermost :t:`construct` containing the elided :t:`lifetime`.

:dp:`fls_cD0ZYi23VqWg`
It is a static error to elide a :t:`lifetime` in a position where no
:t:`lifetime elision` rules are active.

:dp:`fls_sA4Lqc5o6cX3`
:t:`[Lifetime]s` cannot be implicitly elided within :t:`[impl trait type]s`.
If no :t:`lifetime bound` is present, the :t:`impl trait type` is not considered
to be bound by any :t:`lifetime`.

.. _fls_HEtHxXBcg7JA:

Function Lifetime Elision
^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_lAdIRCFFlydD`
:t:`Function lifetime elision` is a form of :t:`lifetime elision` that applies
to :t:`[function]s`, :t:`[function pointer type parameter]s`, and :t:`[path]s`
that resolve to one of the :std:`core::ops::Fn`, :std:`core::ops::FnMut`, and
:std:`core::ops::FnOnce` :t:`[trait]s`.

:dp:`fls_dpudys82dhdc`
An :dt:`input lifetime` is one of the following :t:`[lifetime]s`:

* :dp:`fls_pjil71kk0r25`
  Any :t:`lifetime` related to a :t:`function parameter`.

* :dp:`fls_1jnn9bsb71k7`
  Any :t:`lifetime` related to a :t:`function pointer type parameter`.

* :dp:`fls_2p29p1fvi182`
  Any :t:`lifetime` related to the :t:`[function parameter]s` of the
  :std:`core::ops::Fn`, :std:`core::ops::FnMut`, and :std:`core::ops::FnOnce`
  :t:`[trait]s`.

:dp:`fls_d4u3y82hdadc`
A :dt:`self input lifetime` is an :t:`input lifetime` of a :t:`self parameter`
that is a :t:`lifetime` of a :t:`reference type` whose referent is :c:`Self`.

* :dp:`fls_ks8wlufmhz6d`
  Any :t:`lifetime` related to an :t:`implementing type` and an
  :t:`implemented trait` of an :t:`implementation`.

:dp:`fls_hsg9kfyvh35m`
An :dt:`output lifetime` is one of the following :t:`[lifetime]s`:

* :dp:`fls_ofqy10q4a9jk`
  Any :t:`lifetime` related to the :t:`return type` of a :t:`function`.

* :dp:`fls_yofbo96tjppf`
  Any :t:`lifetime` related to the :t:`return type` of a
  :t:`function pointer type`.

* :dp:`fls_vf7cxiir91ps`
  Any :t:`lifetime` related to the :t:`[return type]s` of the
  :std:`core::ops::Fn`, :std:`core::ops::FnMut`, and :std:`core::ops::FnOnce`
  :t:`[trait]s`.

:dp:`fls_g56br27hq2zj`
:t:`Lifetime elision` proceeds as follows:

#. :dp:`fls_1j204m1wy333`
   Each :t:`elided` :t:`input lifetime` is a distinct :t:`lifetime parameter` in
   its related :t:`construct`.

#. :dp:`fls_6km3cbchuxr2`
   If a :t:`construct` has exactly one :t:`input lifetime`, then that
   :t:`lifetime` is assigned to all :t:`elided` :t:`[output lifetime]s`.

#. :dp:`fls_crb6m6b3cdwh`
   If a :t:`function` has a :t:`self parameter` with exactly 1
   :t:`self input lifetime`, then the :t:`lifetime` of the
   :t:`self input lifetime` is assigned to all :t:`elided`
   :t:`[output lifetime]s`.

#. :dp:`fls_ac9tdlfwp5et`
   Otherwise this is a static error.

.. rubric:: Examples

:dp:`fls_qtjc7334wzhj`
Given :t:`function` ``f`` of the form

.. code-block:: rust

   fn f <'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

:dp:`fls_vcmmkp9uruhr`
its :t:`lifetime` :t:`elided` form is

.. code-block:: rust

   fn f <T: ToCStr>(&mut self, args: &[T]) -> &mut Command;

.. _fls_u5lQkU2rS6uV:

Static Lifetime Elision
^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_l4RDXaFwnQZ6`
:t:`Static lifetime elision` is a form of :t:`lifetime elision` that applies to
the :t:`type ascription` of :t:`[constant]s` and :t:`[static]s`.

:dp:`fls_8irr97rZWfSC`
An :t:`elided` :t:`lifetime` of a :t:`reference type` or :t:`path` in the
:t:`type specification` of a :t:`constant` or :t:`static` is inferred to be the
``'static'`` lifetime.

:dp:`fls_37udexenqv3p`
The :t:`lifetime` of an :t:`associated implementation constant` shall not be
:t:`elided`.

:dp:`fls_xi86he5vvill`
The :t:`lifetime` of an :t:`associated trait constant` shall not be :t:`elided`.

.. rubric:: Examples

:dp:`fls_2GKCEI9MwMn9`
Given :t:`static` ``S`` of the form

.. code-block:: rust

   static S: &[&usize] = &[];

:dp:`fls_f3yZ31dRuTPG`
its :t:`lifetime` :t:`elided` form is

.. code-block:: rust

   static S: &'static [&'static usize] = &[];

.. _fls_XTBOhK2Yk4lA:

Trait Object Lifetime Elision
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_fuBYWRrgxlbQ`
:t:`Trait object lifetime elision` is a form of :t:`lifetime elision` that
applies to :t:`[trait object type]s`.

:dp:`fls_URl9CeIVsiWs`
An :t:`elided` :t:`lifetime` of a :t:`trait object type` is inferred as follows:

* :dp:`fls_SHhw6lYHeYyQ`
  If the :t:`trait object type` is used as the :t:`type specification` of a
  :t:`reference type`, then the :t:`lifetime` of the :t:`reference type` is the
  :t:`elided` :t:`lifetime`,

* :dp:`fls_lC2rwdPLRwaf`
  If the :t:`trait object type` is used as a :t:`generic argument` and

  * :dp:`fls_e36Hh4oJvfhv`
    if the corresponding :t:`generic parameter` has exactly one
    :t:`lifetime bound`, then the :t:`lifetime` of that :t:`bound` is the
    :t:`elided` :t:`lifetime`,

  * :dp:`fls_ptejalcnIQtm`
    Otherwise it is a static error to infer the :t:`lifetime` :t:`bound`.

* :dp:`fls_rGbdKtTijby4`
  If the :t:`trait` of the :t:`trait object type` has exactly one
  :t:`lifetime bound` specified, then the :t:`lifetime` of that :t:`bound` is
  the inferred :t:`lifetime`,

* :dp:`fls_JhmQpUoExiNZ`
  If the :t:`trait` of the :t:`trait object type` has no :t:`[lifetime bound]s`
  specified, then the :t:`elided` :t:`lifetime` is the ``'static``
  :t:`lifetime` unless it is :t:`elided` in :t:`[expression]s` where it is
  instead inferred,

* :dp:`fls_cglZigwAnASl`
  Otherwise it is a static error to infer the :t:`lifetime bound`.

.. rubric:: Examples

:dp:`fls_MipY2emZFF6d`
Given :t:`type alias` ``T`` of the form

.. code-block:: rust

   type T<'a> = &'a dyn Trait;

:dp:`fls_YPesUZqYHVUX`
its :t:`lifetime` :t:`elided` form is

.. code-block:: rust

   type T<'a> = &'a (dyn Trait + 'a);

.. _fls_ZQPv1ybdDsE1:

Impl Header Lifetime Elision
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_FUdsmzN0T8XP`
:t:`Impl header lifetime elision` is a form of :t:`lifetime elision` that
applies to the :t:`implementing type` and :t:`implemented trait` (if any) of an
:t:`implementation`.

:dp:`fls_3p5BdLn3JbKz`
The :t:`impl header lifetime elision` rules are as follows:

* :dp:`fls_PfS5AlkN6ANl`
  Every explicitly elided :t:`lifetime` is replaced with a new
  :t:`lifetime parameter` defined on the :t:`implementation`.

* :dp:`fls_QV6zE5Mdor5E`
  An implicitly elided :t:`lifetime` of a :t:`reference` is also replaced with a
  new :t:`lifetime parameter` defined on the :t:`implementation`.

* :dp:`fls_P2V1DqrdCjCi`
  An implicitly elided :t:`lifetime` in any other :t:`type` is a static error.

.. rubric:: Examples

:dp:`fls_MBa35hfS8J9l`
Given an :t:`implementation` of the form

.. code-block:: rust

    impl Trait<&u8, Strukt<'_>> for &i32 {}

:dp:`fls_w0vwdmO8qV9j`
its :t:`lifetime` :t:`elided` form is

.. code-block:: rust

    impl<'a, 'b, 'c> Trait<&'a u8, Strukt<'b>> for &'c i32 {}

:dp:`fls_vImY3BrpNvNY`
where ``'a``, ``'b``, and ``'c`` are anonymous :t:`[lifetime parameter]s` that
cannot be named by user-written code.
