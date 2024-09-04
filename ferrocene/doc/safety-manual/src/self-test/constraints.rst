.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Constraints
===========

.. constraint:: Do not modify toolchain
   :id: CONSTR_SEL_TEST_DO_NOT_MODIFY
   :mitigates: ERR_SELF_TEST_FALSE_NEGATIVES, ERR_SELF_TEST_FALSE_POSITIVES

   While executing the tool,
   the user must ensure that the toolchain is not modified.

.. constraint:: Installed targets
   :id: CONSTR_SEL_TEST_INSTALLED_TARGETS
   :mitigates: ERR_SELF_TEST_TARGET_CHECKS

   Ensure all installed targets are checked by looking at console output.

.. constraint:: Verify displayed errors
   :id: CONSTR_SEL_TEST_ERRORS
   :mitigates: ERR_SELF_TEST_FALSE_POSITIVES

   Follow tool suggestions on how to mitigate the error, then re-run.
   If the errors persist, verify manually where feasible.

.. constraint:: Successful execution
   :id: CONSTR_SEL_TEST_EXECUTION_SUCCESS

   After executing the tool,
   the user must ensure that it exits with success.
