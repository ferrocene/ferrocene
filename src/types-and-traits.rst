.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

.. default-domain:: spec

.. _fls_vgb6ev541b2r:

Types and Traits
================

:dp:`fls_4rhjpdu4zfqj`
A :t:`type` defines a set of :t:`[value]s` and a set of operations that act on
those :t:`[value]s`.

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

.. _fls_963gsjp2jas2:

Type Classification
-------------------

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
Operation ``a <= b`` shall be equivalent to ``a == b | a < b``.

.. rubric:: Undefined Behavior

:dp:`fls_2sd39mj05mb9`
It is undefined behavior for a :t:`value` of :t:`type` :c:`bool` to have a bit
pattern other than ``0x00`` and ``0x01``.

.. _fls_wrvjizrqf3po:

Char Type
~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_vnwbs0exbwcn`
:c:`Char` is a :t:`type` whose :t:`[value]s` are represented as a 32-bit
unsigned word in the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive ranges
of :t:`Unicode`.

.. rubric:: Undefined Behavior

:dp:`fls_juysxea25owj`
It is undefined behavior for a :t:`value` of :t:`type` :c:`char` to be outside
the 0x000 to 0xD7FF or the 0xE000 to 0x10FFFF inclusive ranges of :t:`Unicode`.

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
:t:`Type` :c:`usize` has the same number of bits as the platform's :t:`pointer
type`, and at least 16-bits wide.

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
:t:`Type` :c:`isize` has the same number of bits as the platform's :t:`pointer
type`, and at least 16-bits wide.

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
See :p:`4.3.1. <fls_uj0kpjwyld60>` for the declaration of ``array``.

.. code-block:: rust

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
It is undefined behavior for a :t:`value` of :t:`type` :c:`str` to denote an
invalid UTF-8 sequence of characters.

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
If the :t:`type` of a :t:`tuple field` is a :t:`dynamically-sized type`,
then the :t:`tuple field` shall be the last :t:`tuple field` in the
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
An :t:`enum type` is an :t:`abstract data type` that contains :t:`[enum
variant]s`.

:dp:`fls_il9a1olqmu38`
A :t:`zero-variant enum type` has no :t:`[value]s`.

:dp:`fls_wQTFwl88VujQ`
An :dt:`enum variant` is a :t:`construct` that declares one of the
possible variations of an :t:`enum`.

:dp:`fls_g5qle7xzaoif`
The :t:`name` of an :t:`enum variant` shall denote a unique :t:`name` within the
related :s:`EnumDeclaration`.

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
   Else, if the :t:`enum variant` is the first :t:`enum variant` in the
   :s:`EnumVariantList`, then the :t:`value` is zero.

#. :dp:`fls_8ajw5trd23wi`
   Otherwise the :t:`value` is one greater than the :t:`value` of the
   :t:`discriminant` of the previous :t:`enum variant`.

:dp:`fls_w9xj26ej869w`
It is a static error if two :t:`[enum variant]s` have the same
:t:`[discriminant]s` with the same :t:`value`.

:dp:`fls_wqbuof7kxsrg`
It is a static error if the :t:`value` of a :t:`discriminant` exceeds the
maximum :t:`value` of the :t:`type` of the :t:`expression` of a :t:`discriminant
initializer`.

.. rubric:: Undefined Behavior

:dp:`fls_f046du2fkgr6`
It is undefined behavior for a :t:`value` of an :t:`enum type` to have a
:t:`discriminant` other than a :t:`discriminant` specified by the :t:`enum
type`.

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
The :t:`name` of a :t:`record struct field` shall denote a unique :t:`name`
within the :s:`RecordStructDeclaration`.

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
A :t:`union type` is an :t:`abstract data type` similar to a :t:`C`-like union.

:dp:`fls_1caus8ybmfli`
The :t:`name` of a :t:`union field` shall denote a unique :t:`name` within the
related :s:`RecordStructDeclaration`.

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
A :t:`closure type` that does not move or mutate its :t:`[capture target]s`
implements the :std:`core::ops::Fn` :t:`trait`.

:dp:`fls_3jeootwe6ucu`
A :t:`closure type` that does not encapsulate :t:`[capture target]s` is
coercible to a :t:`function pointer type`.

:dp:`fls_63jqtyw0rz8c`
A :t:`closure type` implicitly implements the :std:`core::marker::Copy`
:t:`trait` if

* :dp:`fls_az5hkn72e3fz`
  It does not encapsulate :t:`[capture target]s` by :t:`by unique immutable
  reference` or :t:`by mutable reference`, or

* :dp:`fls_vvc8c910dmeh`
  The :t:`[type]s` of all :t:`[capture target]s` implement the
  :std:`core::marker::Copy` :t:`trait`.

:dp:`fls_3c4g9njja5s5`
A :t:`closure type` implicitly implements the :std:`core::clone::Clone`
:t:`trait` if

* :dp:`fls_yr55fbspw7s9`
  It does not encapsulate :t:`[capture target]s` :t:`by unique immutable
  reference` or :t:`by mutable reference`, or

* :dp:`fls_pt65037r6hjr`
  The :t:`[type]s` of all :t:`[capture target]s` implement the
  :std:`core::clone::Clone` :t:`trait`.

:dp:`fls_2nuhy0ujgq18`
A :t:`closure type` implicitly implements the :std:`core::marker::Send`
:t:`trait` if:

* :dp:`fls_vamgwed199ct`
  The :t:`[type]s` of all :t:`[capture target]s` that employ :t:`by immutable
  reference`, :t:`by mutable reference`, or :t:`by move` :t:`capture mode`
  implement the :std:`core::marker::Sync` :t:`trait`, and

* :dp:`fls_f96a5r1v7te7`
  The :t:`[type]s` of all :t:`[capture target]s` that employ :t:`by unique
  immutable reference`, :t:`by mutable reference`, :t:`by copy`, or :t:`by move`
  :t:`capture mode` implement the :std:`core::marker::Send` :t:`trait`.

:dp:`fls_5jh07heok8sy`
A :t:`closure type` implicitly implements the :std:`core::marker::Sync`
:t:`trait` if the :t:`[type]s` of all :t:`[capture target]s` implement the
:std:`core::marker::Sync` :t:`trait`.

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
A :t:`function item type` implements the :std:`core::ops::Fn` :t:`trait`, the
:std:`core::ops::FnMut` :t:`trait`, the :std:`core::ops::FnOnce` :t:`trait`, the
:std:`core::marker::Copy` :t:`trait`, the :std:`core::clone::Clone` :t:`trait`,
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

.. rubric:: Undefined Behavior

:dp:`fls_52thmi9hnoks`
It is undefined behavior to have a :t:`value` of a :t:`function pointer type`
that is :c:`null`.

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

:dp:`fls_ie0avzljmxfm`
A :t:`shared reference type` prevents the direct mutation of a referenced
:t:`value`.

:dp:`fls_15zdiqsm1q3p`
A :t:`shared reference type` implements the :std:`core::marker::Copy`
:t:`trait`. Copying a :t:`shared reference` performs a shallow copy.

:dp:`fls_csdjfwczlzfd`
Releasing a :t:`shared reference` has no effect on the :t:`value` it refers to.

:dp:`fls_vaas9kns4zo6`
A :t:`mutable reference type` allows the direct mutation of a referenced
:t:`value`.

:dp:`fls_n6ffcms5pr0r`
A :t:`mutable reference type` does not implement the :std:`copy::marker::Copy`
:t:`trait`.

.. rubric:: Undefined Behavior

:dp:`fls_ezh8aq6fmdvz`
It is undefined behavior if a :t:`value` of a :t:`reference type` is :c:`null`.

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
       $$dyn$$? TypeBoundList

   TraitObjectTypeSpecificationOneBound ::=
       $$dyn$$? TraitBound

.. rubric:: Legality Rules

:dp:`fls_sgrvona1nb6h`
A :t:`trait object type` is a :t:`type` that implements a :t:`trait`, where the
:t:`type` is not known at compile time.

:dp:`fls_9z8oleh0wdel`
The first :t:`trait bound` of a :t:`trait object type` shall denote an
:t:`object safe` :t:`trait`. Any subsequent :t:`[trait bound]s` shall denote
:t:`[auto trait]s`.

:dp:`fls_s0oy2c8t4yz9`
A :t:`trait object type` shall not contain :t:`[opt-out trait bound]s`.

:dp:`fls_88b9bmhra55f`
A :t:`trait object type` is a :t:`dynamically sized type`. A :t:`trait object
type` permits late binding of :t:`[method]s`. A :t:`method` invoked via a
:t:`trait object type` involves dynamic dispatching.

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
An :t:`inferred type` is a placeholder for a :t:`type` deduced by :t:`type
inference`.

:dp:`fls_3abhsuaa8nas`
An :t:`inferred type` shall not appear within an :t:`item signature`.

:dp:`fls_9d8wbugmar1m`
An :t:`inferred type` forces a tool to deduce a :t:`type`, if possible.

.. rubric:: Examples

.. code-block:: rust

   let values: Vec<_> = (0 .. 10).collect();

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
It is undefined behavior to have a :t:`value` of the :t:`never type`.

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
of :t:`[lexical element]s`

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
         $$=$$ InitializationType WhereClause? $$;$$

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

:dp:`fls_muxfn9soi47l`
The :t:`alignment` of a :t:`value` specifies which addresses are valid for
storing the :t:`value`. :t:`Alignment` is measured in bytes, is at least one,
and always a power of two. A :t:`value` of :t:`alignment` ``N`` is stored at an
address that is a multiple of ``N``.

:dp:`fls_1pbwigq6f3ha`
The :t:`size` of a :t:`value` is the offset in bytes between successive elements
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

:dp:`fls_e5hivr6m5s3h`
For a :t:`fat pointer`, the :t:`size` and :t:`alignment` are tool-defined, but
are at least those of a :t:`thin pointer`.

:dp:`fls_hlbsjggfxnt2`
For a :t:`trait object type`, the :t:`layout` is the same as the :t:`value`
being coerced into the :t:`trait object type` at runtime.

:dp:`fls_sdrb0k2r18my`
For a :t:`struct type`, the memory layout is undefined, unless the :t:`struct
type` is subject to :t:`attribute` :c:`repr`.

:dp:`fls_gt3tkbn4bsa6`
For a :t:`union type`, the memory layout is undefined, unless the :t:`union
type` is subject to :t:`attribute` :c:`repr`. All :t:`[union field]s` share a
common storage.

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
:t:`[integer type]s`. :t:`Primitive representation` applies only to an :t:`enum
type` that is not a :t:`zero-variant enum type`. It is possible to combine :t:`C
representation` and :t:`primitive representation`.

:dp:`fls_ml4khttq3w5k`
:t:`Transparent representation` applies only to an :t:`enum type` with a
single :t:`enum variant` or a :t:`struct type` where the :t:`struct type` has a
single :t:`field` of non-zero :t:`size` and any number of :t:`[field]s` of
:t:`size` zero and :t:`alignment` one.

:dp:`fls_9q2iqzbup8oy`
:t:`[Type]s` subject to :t:`transparent representation` have the same :t:`type
representation` as a :t:`struct type` with a single :t:`field` of a non-zero
:t:`size`.

:dp:`fls_fsbf6ist38ix`
:t:`Type representation` may be specified using :t:`attribute` :c:`repr`. An
:t:`enum type`, a :t:`struct type`, or a :t:`union type` that is not subject to
:t:`attribute` :c:`repr` has :t:`default representation`.

:dp:`fls_qkkc8x2oghst`
:t:`Type representation` may be modified using :t:`attribute` :c:`[repr]'s`
:c:`align` and :c:`packed` :t:`[representation modifier]s`. A :t:`representation
modifier` shall apply only to a :t:`struct type` or a :t:`union type` subject to
:t:`C representation` or :t:`default representation`.

.. _fls_xc1hof4qbf6p:

Enum Type Representation
^^^^^^^^^^^^^^^^^^^^^^^^

.. rubric:: Legality Rules

:dp:`fls_p0c62ejo1u1t`
:t:`[Zero-variant enum type]s` shall not be subject to :t:`C representation`.

:dp:`fls_efp1kfgkpba8`
The :t:`size` and :t:`alignment` of an :t:`enum type` without :t:`[field]s`
subject to :t:`C representation`, :t:`default representation`, or :t:`primitive
representation` are those of its :t:`discriminant`.

:dp:`fls_s9c0a0lg6c0p`
The :t:`discriminant type` of an :t:`enum type` with :t:`C representation` is
the :t:`type` of a :t:`C` ``enum`` for the target platform's :t:`C` :t:`ABI`.

:dp:`fls_slhvf3gmqz4h`
The :t:`discriminant type` of an :t:`enum type` with :t:`default representation`
is tool-defined.

:dp:`fls_u1zy06510m56`
The :t:`discriminant type` of an :t:`enum type` with :t:`primitive
representation` is the :t:`primitive type` specified by the :t:`primitive
representation`.

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
The :t:`alignment` of a :t:`struct type` subject to :t:`C representation` is the
:t:`alignment` of the most-aligned :t:`field` in it.

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
A :t:`recursive type` is a category of :t:`type` that may define other
:t:`[type]s` within its :t:`type specification`.

:dp:`fls_eddnwlr0rz59`
A :t:`recursive type` shall include at least one :t:`abstract data type` in
its recursion.

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
:t:`Type unification` is a measure of compatibility between two :t:`[type]s`. A
:t:`type` is said to :t:`unify` with another :t:`type` when the domains, ranges
and structures of both :t:`[type]s` are compatible.

:dp:`fls_aie0tr62vhw5`
Two types that :t:`unify` are said to be :t:`[unifiable type]s`.

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
Type :c:`str` is unifiable only with itself.

:dp:`fls_mpq64eal9jo3`
A :t:`tuple type` is unifiable only with another :t:`tuple type` when:

* :dp:`fls_kcr8npsmy0e5`
  The :t:`arity` of both :t:`[tuple type]s` is the same, and

* :dp:`fls_kq3lv1zbangz`
  The :t:`[type]s` of the corresponding :t:`[tuple field]s` are unifiable.

:dp:`fls_so2cgqmawlm7`
An :t:`abstract data type` is unifiable only with another :t:`abstract data
type` when:

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
A :t:`function pointer type` is unifiable only with another :t:`function pointer
type` when:

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
  :t:`anonymous return type`, and

:dp:`fls_hza5n5eb18ta`
An :t:`impl trait type` is unifiabe only with itself.

:dp:`fls_ww16urcjrj6i`
A :t:`trait object type` is unifiable only with another :t:`trait object type`
when:

* :dp:`fls_bnp6or49voxp`
  The :t:`[bound]s` are unifiable, and

* :dp:`fls_hdo4c849q3lo`
  The :t:`[lifetime]s` are unifiable.

:dp:`fls_zh5hhq2x9h4q`
A :t:`global type variable` is unifiable with any other type.

:dp:`fls_3xpp05fm0zpb`
A :t:`floating-point type variable` is unifiable only with a :t:`floating-point
type`.

:dp:`fls_qtuq6q3ylic`
An :t:`integer type variable` is unifiable only with an :t:`integer type`.

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

* :dp:`fls_sp794uzfiofr`
  A :t:`let statement` with an explicit :t:`type specification`.

* :dp:`fls_bhzmble1itog`
  A :t:`constant` declaration.

* :dp:`fls_xfqny6bwzsu9`
  A :t:`static` declaration.

* :dp:`fls_wxrugvlazy6v`
  The :t:`argument [operand]s` of a :t:`call expression` or a :t:`method call
  expression`.

* :dp:`fls_eu4bt3dw1b8c`
  The :t:`instantiation` of a :t:`field` of an :t:`abstract data type`.

* :dp:`fls_apstt4elv2k7`
  A :t:`function` result.

:dp:`fls_u0e42y7nvn7e`
The following :t:`[expression]s` constitute a :dt:`coercion-propagating
expression`:

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
  ``type_constructor(U)``, where ``type_constructor`` is one of ``&W``, ``&mut
  W``, ``*const W``, or ``*mut W``, and ``U`` can be obtained from ``T`` using
  :t:`unsized coercion`.

* :dp:`fls_ulcdetwp6x96`
  The source :t:`type` is a :t:`function item type` and the target :t:`type` is
  a :t:`function pointer type`.

* :dp:`fls_2uv1r0gni1fk`
  The source :t:`type` is a non-capturing :t:`closure type` and the target
  :t:`type` is a :t:`function pointer type`.

* :dp:`fls_sf0c3fbx8z57`
  The source :t:`type` is the :t:`never type` and the target :t:`type` is any
  :t:`type`.

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
  The :t:`type` of ``last_field`` involves ``T`` and if the
  :t:`type` of ``last_field`` is ``W<T>``, then ``W<T>`` implements
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
  To find the common :t:`type` for multiple :t:`match expression` :t:`[match
  arm]s`.

* :dp:`fls_tnbga5dl4gz8`
  To find the common :t:`type` of :t:`array expression` :t:`[operand]s`.

* :dp:`fls_yoreux8tn65x`
  To find the :t:`return type` of a :t:`closure expression` with multiple
  :t:`[return expression]s`.

* :dp:`fls_r11shke69uu6`
  To find the :t:`return type` of a :t:`function` with multiple :t:`[return
  expression]s`.

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
      Otherwise if ``U`` can be coerced to ``TC``, make ``TC`` the target
      :t:`type` ``U``.

   #. :dp:`fls_92pwnd1xbp5r`
      Otherwise compute the mutual supertype of ``TC`` and ``U``, make the
      mutual supertype be target :t:`type` ``U``. It is a static error if the
      mutual supertype of ``TC`` and ``U`` cannot be computed.

   #. :dp:`fls_ju4ypa5ysga0`
      Continue with the next source :t:`type`.

.. _fls_lv7w7aalpwm5:

Type Inference
~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_h8sedxew0d4u`
:t:`Constant` declarations, :t:`[let statement]s`, and :t:`static`
declarations impose an :dt:`expected type` on their respective initialization
:t:`[expression]s`. :t:`Type inference` is the process of deducing the expected
:t:`type` of an arbitrary :t:`value`.

:dp:`fls_uvvn4usfsbhr`
A :t:`type variable` is a placeholder for a :t:`type`. A :t:`global type
variable` is a :t:`type variable` that can refer to any :t:`type`.

:dp:`fls_5d4hw3gj4w4n`
The :t:`expected type` of the :t:`constant initializer` of a :t:`constant` is
the :t:`type` specified by its :t:`type ascription`.

:dp:`fls_v6z48i1b7vxv`
The :t:`expected type` of the initialization :t:`expression` of a :t:`let
statement` is determined as follows:

#. :dp:`fls_qob4wjgza3i8`
   If the :t:`let statement` appears with a :t:`type ascription`, then the
   :t:`expected type` is the :t:`type` specified by its :t:`type ascription`.

#. :dp:`fls_7vdr0mh7kmpz`
   Otherwise the :t:`expected type` is a :t:`global type variable`.

:dp:`fls_qlovdtcj1v1b`
The :t:`expected type` of the :t:`static initializer` of a :t:`static` is the
:t:`type` specified by its :t:`type ascription`.

:dp:`fls_biyyicl3c3kn`
:t:`[Arithmetic expression]s`, :t:`[await expression]s`,
:t:`[bit expression]s`, :t:`[block expression]s`, :t:`[borrow expression]s`,
:t:`[dereference expression]s`, :t:`[call expression]s`,
:t:`[else expression]s`, :t:`[error propagation expression]s`,
:t:`[if expression]s`, :t:`[if let expression]s`, :t:`[loop expression]s`,
:t:`[match expression]s`, :t:`[negation expression]s`, and
:t:`[parenthesized expression]s` are :dt:`[type imposing expression]s`.

:dp:`fls_o94mhge1j3iw`
A :t:`type imposing expression` imposes its :t:`expected type` onto a nested
:t:`construct`, as follows:

* :dp:`fls_3ihttknfccxr`
  An :t:`addition expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::Add::Output`.

* :dp:`fls_rta6ehkzp3hg`
  A :t:`division expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::Div::Output`.

* :dp:`fls_f2whukg3x1yo`
  A :t:`multiplication expression` imposes its :t:`expected type` onto
  :t:`associated type` :std:`core::ops::Mul::Output`.

* :dp:`fls_w9fp1usbb15`
  A :t:`remainder expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::Rem::Output`.

* :dp:`fls_5s2eh0qjq6vk`
  A :t:`subtraction expression` imposes its :t:`expected type` onto
  :t:`associated type` :std:`core::ops::Sub::Output`.

* :dp:`fls_rpxxg2u4hzhc`
  An :t:`await expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::future::Future::Output`.

* :dp:`fls_vj1071lxoyyv`
  A :t:`bit and expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::BitAnd::Output`.

* :dp:`fls_y6owsf8jnx35`
  A :t:`bit xor expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::BitXor::Output`.

* :dp:`fls_i9dhdmiqde99`
  A :t:`bit or expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::BitOr::Output`.

* :dp:`fls_bystnhv1olg5`
  A :t:`shift left expression` imposes its :t:`expected type` onto
  :t:`associated type` :std:`core::ops::Shl::Output`.

* :dp:`fls_trvksnbx7opg`
  A :t:`shift right expression` imposes its :t:`expected type` onto
  :t:`associated type` :std:`core::ops::Shr::Output`.

* :dp:`fls_8ct11ekq3p5q`
  A :t:`block expression` imposes its :t:`expected type` onto its :t:`tail
  expression`. If the :t:`block expression` is associated with a :t:`loop
  expression`, then the :t:`block expression` imposes its :t:`expected type`
  onto each :t:`break expression` within its :t:`statement` list. If the
  :t:`block expression` is associated with a :t:`function`, then the :t:`block
  expression` imposes its :t:`expected type` onto each :t:`return expression`
  within its :t:`statement` list.

* :dp:`fls_eee1t7hynswa`
  A :t:`borrow expression` imposes its :t:`expected type` onto its :t:`operand`.

* :dp:`fls_ax86vtmz4hrb`
  A :t:`dereference expression` imposes its :t:`expected type` onto its
  :t:`operand`.

* :dp:`fls_kviulvlfvww2`
  A :t:`call expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::FnOnce::Output`.

* :dp:`fls_4hsgi1voem9y`
  An :t:`error propagation expression` imposes its :t:`expected type` onto its
  operand.

* :dp:`fls_8zpltmxy41rd`
  An :t:`if expression` imposes its :t:`expected type` onto its :t:`block
  expression` and else expression.

* :dp:`fls_qdmyerpgnwha`
  An :t:`if let expression` imposes its :t:`expected type` onto its :t:`block
  expression` and :t:`else expression`.

* :dp:`fls_gmojdinhct0b`
  A :t:`lazy boolean expression` imposes its :t:`expected type` onto its
  :t:`[operand]s`.

* :dp:`fls_d8f7xb8r3aud`
  A :t:`loop expression` imposes its :t:`expected type` onto its :t:`block
  expression`.

* :dp:`fls_ds3nkfar77in`
  A :t:`match expression` imposes its :t:`expected type` onto the
  :t:`expression-with-block` or :t:`expression-without-block` of every
  :t:`intermediate match arm` and the :t:`expression` of its :t:`final match
  arm`.

* :dp:`fls_xhax58ebkqik`
  A :t:`negation expression` imposes its :t:`expected type` onto :t:`associated
  type` :std:`core::ops::Neg::Output`.

* :dp:`fls_m896wu8zax5k`
  A :t:`parenthesized expression` imposes its :t:`expected type` onto its
  :t:`operand`.

* :dp:`fls_8ft8d4x1q08p`
  A :t:`return expression` imposes its :t:`expected type` onto its :t:`operand`.

:dp:`fls_aaumn7viouu7`
:t:`[Array expression]s`, :t:`[index expression]s`, :t:`[assignment
expression]s`, :t:`[closure expression]s`, :t:`[comparison expression]s`,
:t:`[compound assignment expression]s`, :t:`[field access expression]s`,
:t:`[lazy boolean expression]s`, :t:`[method call expression]s`, :t:`[range
expression]s`, :t:`[struct expression]s`, :t:`[tuple expression]s`, and
:t:`[type cast expression]s` are :dt:`[type resolving expression]s`.

:dp:`fls_r7dyhfmdentz`
A :t:`type resolving expression` provides a :dt:`resolving type`, which is the
:t:`type` of the :t:`expression` itself.

:dp:`fls_3hv3wxkhjjp1`
A :t:`floating-point type variable` is a :t:`type variable` that can refer only
to :t:`[floating-point type]s`.

:dp:`fls_8zkvwpkgob6d`
The :t:`resolving type` of a :t:`float literal` is determined as follows:

#. :dp:`fls_1dvk2vvdw0oj`
   If the :t:`float literal` has a :t:`float suffix`, then the :t:`resolving
   type` is the :t:`type` specified by its :t:`float suffix`.

#. :dp:`fls_gp9gcxiapfxv`
   Otherwise the :t:`resolving type` is a :t:`floating-point type variable`.

:dp:`fls_7ov36fpd9mwe`
An :t:`integer type variable` is a :t:`type variable` that can refer only to
:t:`[integer type]s`.

:dp:`fls_v9lyy98dgm98`
The :t:`resolving type` of an :t:`integer literal` is determined as follows:

#. :dp:`fls_i3v9yqp7j4n`
   If the :t:`integer literal` has an :t:`integer suffix`, then the
   :t:`resolving type` is the :t:`type` specified by its :t:`integer suffix`.

#. :dp:`fls_z03x5pk7q9dd`
   Otherwise the :t:`resolving type` is an :t:`integer type variable`.

:dp:`fls_ybvrhh96fc7y`
:t:`[Constant argument]s`, :t:`constant` declarations, :t:`[function]s`, and
:t:`static` declarations are referred to as :dt:`[type inference root]s`.

:dp:`fls_j28usox2uzep`
:t:`Type inference` for a single :t:`type inference root` proceeds as follows:

#. :dp:`fls_7pwr5jeis2n8`
   Determine unique :t:`expected type` ``ET`` for the :t:`type inference root`.

#. :dp:`fls_wqyw2u3tjzmv`
   Resolve the initialization :t:`expression` of the :t:`type inference root`
   against ``ET`` as follows:

   #. :dp:`fls_a0d3x44wboz4`
      If the :t:`expression` is a :t:`type imposing expression`, then

      #. :dp:`fls_62yj5vkp0iox`
         Make ``ET`` the :t:`type` of the :t:`expression`.

      #. :dp:`fls_h0e7634x6go9`
         Impose ``ET`` on any nested :t:`construct` depending on the nature of
         the :t:`expression`, recursively.

   #. :dp:`fls_7zzz1ao7k42e`
      If the :t:`expression` is a :t:`type resolving expression`, then

      #. :dp:`fls_9swsddkfjw1r`
         Determine :t:`resolving type` ``RT`` the :t:`expression`.

      #. :dp:`fls_59p9pd4jo8wt`
         Resolve ``ET`` against ``RT``.

#. :dp:`fls_ynsjdua73fcl`
   If there are :t:`[expression]s` whose :t:`type` ``T`` is a :t:`floating-point
   type variable`, replace ``T`` with :t:`type` :c:`f64`.

#. :dp:`fls_oz057wsgk05e`
   If there are :t:`[expression]s` whose :t:`type` ``T`` is an :t:`integer type
   variable`, replace ``T`` with :t:`type` :c:`i32`.

#. :dp:`fls_2eu3zcuznfrk`
   If there are :t:`[expression]s` whose :t:`type` is a :t:`global type
   variable`, then this is a static error.

:dp:`fls_iqf4muk5nrot`
Resolving :t:`expected type` ``ET`` against :t:`resolving type` ``RT`` for an
:t:`expression` proceeds as follows:

#. :dp:`fls_qdpf7tahw1go`
   If both ``ET`` and ``RT`` denote a :t:`concrete type`, then ``ET`` and ``RT``
   shall be :t:`unifiable`.

#. :dp:`fls_yqsl1gg27b5o`
   If ``ET`` denotes a :t:`global type variable` and ``RT`` denotes a
   :t:`concrete type`, then ``ET`` is replaced with ``RT``, effectively changing
   the :t:`type` of all :t:`[expression]s` that previously held ``ET``.

#. :dp:`fls_c4i80gd8cdub`
   If ``ET`` denotes a :t:`floating-point type variable` and ``RT`` denotes a
   :t:`floating point type`, then ``ET`` is replaced with ``RT``, effectively
   changing the :t:`type` of all :t:`[expression]s` that previously held ``ET``.

#. :dp:`fls_acd7b3m1qm3a`
   If ``ET`` denotes an :t:`integer type variable` and ``RT`` denotes an
   :t:`integer type`, then ``ET`` is replaced with ``RT``, effectively changing
   the :t:`type` of all :t:`[expression]s` that previously held ``ET``.

#. :dp:`fls_riivz4mlwr4y`
   Otherwise this is a static error.

.. _fls_85vx1qfa061i:

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
       TypeBoundList

.. rubric:: Legality Rules

:dp:`fls_tani6lesan9u`
A :t:`trait` is an :t:`item` that describes an interface a :t:`type` can
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
A trait is :t:`object safe` when:

* :dp:`fls_5wlltclogfkw`
  Its :t:`[supertrait]s` are :t:`object safe`, and

* :dp:`fls_droy0w5gtqaw`
  :std:`core::marker::Sized` is not a :t:`supertrait`, and

* :dp:`fls_46gd1q80c6bn`
  It lacks :t:`[associated constant]s`, and

* :dp:`fls_kwo4cknx0yat`
  Its :t:`[associated function]s` are :t:`object safe`.

:dp:`fls_uixekv82g2e5`
An :t:`associated function` is :t:`object safe` when it is either an :t:`object
safe` dispatchable :t:`function` or an :t:`object safe` non-dispatchable
:t:`function`.

:dp:`fls_72tvfoemwpyy`
A dispatchable :t:`function` is :t:`object safe` when:

* :dp:`fls_j7nb34o87l1z`
  It lacks :t:`[generic parameter]s`, and

* :dp:`fls_k1vc9vd8at92`
  Is a :t:`method` that does not use :c:`Self` except as the :t:`type` of its
  :t:`receiver`, and

* :dp:`fls_32nk904hwjao`
  Is a :t:`method` whose :t:`receiver` is either ``&Self``, ``&mut Self``, or
  ``core::pin::Pin<T>`` where T is one of the previous :t:`[receiver]s`, and

* :dp:`fls_kqylg31sm5wv`
  It lacks a :t:`where clause` that specifies the :std:`core::marker::Sized`
  :t:`trait`.

:dp:`fls_aer3gaur7avp`
A non-dispatchable :t:`function` is :t:`object safe` when it specifies a
:std:`core::marker::Sized` :t:`[trait bound]` for :c:`Self`.

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
       $$($$ $$?$$? ForGenericParameterList? TypePath $$)$$

   TraitBound ::=
       $$?$$? ForGenericParameterList? TypePath

   ForGenericParameterList ::=
       $$for$$ GenericParameterList

.. rubric:: Legality Rules

:dp:`fls_5g508z6c7q5f`
A :t:`bound` imposes a constraint on :t:`[generic parameter]s` by limiting the
set of possible :t:`[generic substitution]s`.

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

:dp:`fls_tx4uspewnk7w`
:t:`Bound` ``'a: 'b`` is read as ``'a`` outlives ``'b``, or in other words,
``'a`` lasts as long as ``'b``.

:dp:`fls_5kj8bmvb8xfc`
:t:`Bound` ``T: 'a`` indicates that all :t:`[lifetime parameter]s` of ``T``
outlive ``'a``.

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
:t:`Variance` is a property of :t:`[lifetime parameter]s` and :t:`[type
parameter]s` that describes the circumstances under which a :t:`generic type`
is a :t:`subtype` of an instantiation of itself with different :t:`[generic
argument]s`.

:dp:`fls_7ex941yysuhq`
A :t:`type` is its own :t:`subtype`.

:dp:`fls_7qud6i05ze2`
``F<T>`` is said to be

* :dp:`fls_wpm0p0gtctvi`
  :dt:`Covariant` over ``T`` when ``T`` being a :t:`subtype` of ``U`` implies
  that ``F<T>`` is a :t:`subtype` of ``F<U>``, or

* :dp:`fls_3rfs58i2kg6l`
  :dt:`Contravariant` over ``T`` when ``T`` being a :t:`subtype` of ``U``
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
an :t:`tuple type` is determined as follows:

#. :dp:`fls_hvfyog9ygn6q`
   For each :t:`generic parameter` ``G``

   #. :dp:`fls_mduolmcawb30`
      Initialize :t:`variance` ``V`` of the :t:`generic parameter` to ``any``.

   #. :dp:`fls_y81gmqweqc9w`
      For each :t:`field` of the :t:`abstract data type` or the :t:`tuple type`

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

.. _fls_l9ebxrlxyawd:

Lifetime Elision
~~~~~~~~~~~~~~~~

.. rubric:: Legality Rules

:dp:`fls_9wtuclhm7yz5`
:t:`Lifetime elision` is a set of relaxations on the use of :t:`[lifetime]s`.

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

* :dp:`fls_ks8wlufmhz6d`
  Any :t:`lifetime` related to an :t:`implementing type` and an :t:`implemented
  trait` of an :t:`implementation`.

:dp:`fls_hsg9kfyvh35m`
An :dt:`output lifetime` is one of the following :t:`[lifetime]s`:

* :dp:`fls_ofqy10q4a9jk`
  Any :t:`lifetime` related to the :t:`return type` of a :t:`function`.

* :dp:`fls_yofbo96tjppf`
  Any :t:`lifetime` related to the :t:`return type` of a :t:`function pointer
  type`.

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
   If a :t:`function` has a :t:`receiver` of the form ``&self``, ``&mut self``,
   or ``self: T`` where ``T`` is a :t:`type` with a :t:`lifetime`, then the
   :t:`lifetime` of the :t:`receiver` is assigned to all :t:`elided` :t:`[output
   lifetime]s`.

#. :dp:`fls_ac9tdlfwp5et`
   Otherwise this is a static error.

:dp:`fls_37udexenqv3p`
The :t:`lifetime` of an :t:`associated implementation constant` shall not be
:t:`elided`.

:dp:`fls_xi86he5vvill`
The :t:`lifetime` of an :t:`associated trait constant` shall not be :t:`elided`.

.. rubric:: Examples

:dp:`fls_qtjc7334wzhj`
Given function ``f`` of the form

.. code-block:: rust

   fn f <'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command;

:dp:`fls_vcmmkp9uruhr`
its lifetime elided form is

.. code-block:: rust

   fn f <T: ToCStr>(&mut self, args: &[T]) -> &mut Command;

