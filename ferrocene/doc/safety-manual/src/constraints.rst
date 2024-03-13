.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

.. list-table:: User Requirements
   :align: left
   :header-rows: 1

   * - Requirement ID
     - Associated Requirement ID
     - Requirement Definition
   * - .. id:: CSTR_0010_INSTALL
     - :id:`AVD_CHECK_INSTALL_001`
     - Before using the Ferrocene toolchain, the user shall ensure that the
       appropriate tool package has been installed and installation verified
       according to the verification procedure described in
       :doc:`Installation Procedures <safety-manual:procedures>`.
   * - .. id:: CSTR_0020_CLEAN_ENV
     - :id:`AVD_CHECK_CLEAN_ENV_002`
     - Before using the Ferrocene toolchain, the user shall ensure that
       toolchain is available on the user ``PATH`` variable, and that none of
       the disallowed environment variables described in
       :ref:`Degraded Environment <degraded-environment:Degraded environment>`
       are set.
   * - .. id:: CSTR_0030_BUILD_MONITORING
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
     - The user shall inspect the build logs to verify that all actions have
       been completed, executed in the correct order, and the correctness of the
       build, in particular that the proper versions of the tools have been used
       with the appropriate options.
   * - .. id:: CSTR_0040_CLEAN
     - :id:`AVD_CLEAN_004`
     - Before building the final version of the safety-related software, the
       user shall ensure that the build environment is clean of former
       compilation artifacts as described in
       :ref:`Cleaning the build space <usage:Cleaning the build space>`.
   * - .. id:: CSTR_0050_WARNING_ERROR
     - :id:`AVD_WARNING_AS_ERROR_005`
     - The user shall ensure that for the final build, the option to treat all
       warnings as errors is activated as described in
       :ref:`Warnings and errors <usage:Warnings and errors>`.
   * - .. id:: CSTR_0060_PARALLEL
     - :id:`AVD_PARALLEL_BUILD_006`
     - The user shall not perform source modification when the build of the
       safety-related software is in progress.
   * - .. id:: CSTR_0070_TEST
     - :id:`AVD_TEST_007`
     - The integrated testing objectives of |iso_ref|, sections 11.4.9, must be
       performed on the final application or libraries or on parts of them built
       using build protocols as close as possible to those used for the final
       build.
   * - .. id:: CSTR_0080_KP
     - N/A
     - The user shall implement mitigation strategies for known problems
       documented in the :ref:`known-problems:Known Problems` manual.
   * - .. id:: CSTR_0090_NEW_KP
     - N/A
     - The user shall regularly inspect the new known problems reported via the
       mechanism documented in
       :ref:`Change Tracking <change-tracking:Change Tracking>`. In case of
       a new safety impact, the user shall ensure the appropriate mitigation
       strategies are put in place.
   * - .. id:: CSTR_0100_UNSAFETY
     - N/A
     - The user shall identify and evaluate the risks related to all instances
       of unsafe code as defined in :doc:`specification:unsafety`, and follow
       the guidelines outlined in
       :ref:`Handling Unsafety <unsafety:Handling Unsafety>`.
   * - .. id:: CSTR_0110_INTERRUPTS
     - N/A
     - The user shall structure their code such that hardware interrupts will
       not result in data race conditions.
