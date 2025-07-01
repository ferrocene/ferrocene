.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

Building certified software
---------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_008

If building certified software, the user shall only use the certified subset of libcore.

Compiling
---------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_009

If building certified software, the user shall always provide the ``-C panic=abort`` option to ferrocene.

Avoiding unstable functions
---------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_010

If building certified software, the user shall use the unstable functions in the certified subset of libcore **only** in tests not in production code.

Use the same version of libcore and rustc
-----------------------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_011

If building certified software, the user shall verify that the version of libcore and the version of rustc used to compile code match.

Using operands
--------------

.. id::  RUSTC_AVD_LIBCORE_OPERANTS_012

If building certified software, the user shall manually verify that all operands such as addition, multiplication and bit-shifting use defined
behaviour as shown on the last line of the following code example, and not as in the line before the last::

.. code-block:: rust
  :linenos:

   // this is in libcore:
   trait Add<Other> {
      fn add(self, other: Other) { /* ... */ }
   }

   impl Add<i32> for i32 { /* ... */ } // certified
   impl Add<i16> for i32 { /* ... */ } // uncertified

   // this is in user code:

   let a: i32 = 5;
   let b: i32 = 10;
   let c: i16 = 20;

   let d = a + b; // legal
   let e = a + c; // illegal
