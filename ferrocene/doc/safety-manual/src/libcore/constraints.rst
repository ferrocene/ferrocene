.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

Only use certified subset
-------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_008

If building certified software, the user shall only use the certified subset of libcore.

Compile with panic abort
------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_009

If building certified software, the user shall always provide the ``-C panic=abort`` option to ferrocene.

Only use stable functions
-------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_010

If building certified software, the user shall not use unstable functions in the certified subset of libcore.

Use the same version of libcore and rustc
-----------------------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_011

If building certified software, the user shall verify that the version of libcore and the version of rustc used to compile code match. This is ensured by following the [Installation Procedures](https://public-docs.ferrocene.dev/main/safety-manual/procedures.html).

Using operands
--------------

.. id::  RUSTC_AVD_LIBCORE_OPERANTS_012

If building certified software, the user shall manually verify that the implementations for operators they use in their source code are certified. They have to check all occurrences of operands listed in the ``core::ops`` module. For each occurrence they have to check that the implementation for the combination of types is certified.

For example there are many implementation for addition (``trait Add``).

One of those implementations could be certified, e.g. ``impl Add<i32> for i32``. This would mean that it is legal in certified contexts to add two ``i32`` s to each other.

Another implementation could be uncertified, e.g. ``impl Add<i64> for i64``. This would mean that it is not legal in certified contexts to add two ``i64`` s to each other.

Therefore the source code has to be reviewed for all syntax that denotes an operator and it has to be verified that the implementation of that operator for that combination of types is certified.

.. code-block:: rust
  :linenos:

  // this is in libcore:

  trait Add<Rhs = Self> {
      type Output;
      const fn add(self, rhs: Rhs) -> Self::Output;
  }

  impl Add<i32> for i32 { /* ... */ } // certified
  impl Add<i64> for i64 { /* ... */ } // uncertified

  // this is in user code:

  let a = 5_i32 + 10_i32; // legal
  let b = 20_i64 + 40_i64; // illegal
