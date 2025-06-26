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

If building certified software, the user shall always provide the :cli:option:`-C panic=abort` option to ferrocene.

Avoiding unstable functions
---------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_010

If building certified software, the user shall use the  unstable functions in certified subset of libcore **only** in tests not in production code.

Use the same version of libcore and rustc
-----------------------------------------

.. id::  RUSTC_AVD_LIBCORE_SUBSET_011

If building certified software, the user must verify that the version of libcore and the version of rustc used to compile code match.
