.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

Review diff
-----------

.. id:: RUSTFMT_CSTR_0010_CHECK_DIFF

Associated requirement ID: :id:`RUSTFMT_AVD_CHECK_DIFF_001`

After executing ``rustfmt``, the user shall review the modifications
``rustfmt`` performed and ensure that there are no semantic changes.

Test coverage
-------------

.. id:: RUSTFMT_CSTR_0020_TESTS

Associated requirement ID: :id:`RUSTFMT_AVD_TESTS_002`

Before executing ``rustfmt``, the user shall ensure that the code is covered by
sufficient tests that ensure the correctness of the code.

After executing ``rustfmt``, the user shall ensure that all tests pass.
