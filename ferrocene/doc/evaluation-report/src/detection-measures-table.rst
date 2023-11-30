.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

.. csv-table:: Detection measures and usage restriction
   :header: **Measure identifier**, **Description**
   :align: left
   :delim: !
   :class: longtable

   .. id:: AVD_CHECK_INSTALL_001! The toolchain Installation shall be checked in order to ensure the validity of the build results.
   .. id:: AVD_CHECK_CLEAN_ENV_002! User must verify that environment variables used by the toolchain are correctly set.
   .. id:: AVD_CHECK_BUILD_SCRIPT_003! User must verify that the list of build actions is correct.
   .. id:: AVD_CLEAN_004! Before building, the user must ensure that the build environment is clean of former compilation artifacts.
   .. id:: AVD_WARNING_AS_ERROR_005! All Warnings should be considered errors, the build should NOT display any warning.
   .. id:: AVD_PARALLEL_BUILD_006! Concurrent file updates during the build operations are prohibited.
   .. id:: AVD_TEST_007! Testing must be performed on the final application or libraries, or on any parts built, using an environment as close as possible to the final build.

.. end of table
