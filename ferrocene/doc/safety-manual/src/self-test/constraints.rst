.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Constraints
===========

.. constraint:: Do not modify toolchain
   :id: CONSTR_SELF_TEST_DO_NOT_MODIFY
   :mitigates: ERR_SELF_TEST_FALSE_NEGATIVES, ERR_SELF_TEST_FALSE_POSITIVES

   The user must not modify the toolchain while the tool is executing.

.. constraint:: Installed targets
   :id: CONSTR_SELF_TEST_INSTALLED_TARGETS
   :mitigates: ERR_SELF_TEST_MISSING_TARGET_CHECKS

   Ensure that ``ferrocene-self-test`` checks all installed targets,
   by looking if they appear on console output.

.. constraint:: Follow tool suggestions
   :id: CONSTR_SELF_TEST_ERROR_SUGGESTIONS

   Follow tool suggestions on how to handle toolchain errors it finds.

.. constraint:: Successful execution
   :id: CONSTR_SELF_TEST_EXECUTION_SUCCESS

   After executing the tool,
   the user must ensure that it exits with success.
