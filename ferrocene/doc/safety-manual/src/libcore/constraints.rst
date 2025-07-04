.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

Only use certified subset
---------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_008

If building certified software, the user shall only use the certified subset of libcore.

Compile with panic abort
---------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_009

If building certified software, the user shall always provide the ``-C panic=abort`` option to ferrocene.

Only use stable functions
---------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_010

If building certified software, the user shall not use unstable functions in the certified subset of libcore.

Use the same version of libcore and rustc
-----------------------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_011

If building certified software, the user shall verify that the version of libcore and the version of rustc used to compile code match. This is ensured by following the [Installation Procedures](https://public-docs.ferrocene.dev/main/safety-manual/procedures.html).

Using operands
--------------

.. id::  RUSTC_AVD_LIBCORE_OPERANTS_012

If building certified software, the user shall manually verify that all operands such as addition, multiplication and bit-shifting use defined
behaviour as shown on the last line of the following code example, and not as in the line before the last::

.. code-block:: rust
  :linenos:

   // this is in libcore:

   pub trait Add<Rhs = Self> {
       type Output;
       const fn add(self, rhs: Rhs) -> Self::Output;
   }

   impl Add<i32> for i32 { /* ... */ } // certified
   impl Add<i64> for i64 { /* ... */ } // uncertified

   // this is in user code:

   let a = 5_i32 + 10_i32; // legal
   let b = 20_i64 + 40_i64; // illegal
