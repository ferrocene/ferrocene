.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Constraints
===========

.. constraint:: Review diff
   :id: CONSTR_RUSTFMT_REVIEW_DIFF
   :mitigates: ERR_RUSTFMT_CHANGED_SEMANTICS

   After executing ``rustfmt``, the user shall review the modifications
   ``rustfmt`` performed and ensure that there are no semantic changes.

.. constraint:: Test coverage
   :id: CONSTR_RUSTFMT_CHECK_COVERAGE
   :mitigates: ERR_RUSTFMT_CHANGED_SEMANTICS

   Before executing ``rustfmt``, the user shall ensure that the code is covered by
   sufficient tests that ensure the correctness of the code.

   After executing ``rustfmt``, the user shall ensure that all tests pass.
