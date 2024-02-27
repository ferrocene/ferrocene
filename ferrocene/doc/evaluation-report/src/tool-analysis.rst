.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Tool Analysis
=============

Potential Errors
----------------

Installation
^^^^^^^^^^^^

.. list-table:: ERR_INSTALL
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
   * - .. id:: ERR_INSTALL_01
     - :id:`UC0_INST`
     - Ferrocene was not correctly installed
     - Undefined behavior
     - :id:`AVD_CHECK_INSTALL_001`

.. end of table


Rust Driver
^^^^^^^^^^^

.. list-table:: ERR_DRIVER
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
   * - .. id:: ERR_DRIVER_02
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An used environment variable is set to an incorrect value
     - Undefined behavior
     - :id:`AVD_CHECK_CLEAN_ENV_002`
   * - .. id:: ERR_DRIVER_03
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An invalid option is passed
     - Undefined behavior
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: ERR_DRIVER_04
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Error diagnostics are not correctly emited
     - Undefined behavior
     - :id:`AVD_CHECK_BUILD_SCRIPT_003` AND :id:`AVD_TEST_007`
   * - .. id:: ERR_DRIVER_05
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The output is generated with missing part
     - Wrong code
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: ERR_DRIVER_06
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The behavior is incorrect because of concurrent modification
     - Undefined behavior
     - :id:`AVD_PARALLEL_BUILD_006`
   * - .. id:: ERR_DRIVER_07
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - A warning is generated instead of an error
     - Undefined behavior
     - :id:`AVD_WARNING_AS_ERROR_005`
   * - .. id:: ERR_DRIVER_08
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The compilation has a wrong behavior
     - Wrong code
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_DRIVER_09
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An incomplete input is accepted leading to an undefined behavior
     - Undefined behavior
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_DRIVER_10
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Some object files are not generated silently
     - Use an artifact from a previous build
     - :id:`AVD_CLEAN_004`

.. end of table


Rust Front-End
^^^^^^^^^^^^^^^^

.. list-table:: ERR_COMPIL
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
   * - .. id:: ERR_RUST_FE_11
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Input has invalid contents
     - Invalid code generated
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_RUST_FE_12
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Error diagnostics is invalid
     - Invalid code generated
     - :id:`AVD_WARNING_AS_ERROR_005`
   * - .. id:: ERR_RUST_FE_13
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Invalid output generated from valid input
     - Invalid code generated
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_RUST_FE_14
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - :id:`AVD_PARALLEL_BUILD_006`
   * - .. id:: ERR_RUST_FE_15
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Invalid input is accepted
     - Undefined behavior
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_RUST_FE_16
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`

.. end of table


LLVM
^^^^

.. list-table:: ERR_LLVM
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
   * - .. id:: ERR_LLVM_17
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Input parameter has invalid value
     - Most likely LLVM will crash. Invalid code could also be generated
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_LLVM_18
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An object file is invalid
     - Invalid code generated
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: ERR_LLVM_19
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An object file or static library is not correctly translated to machine code
     - Undefined behavior
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_LLVM_20
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - :id:`AVD_PARALLEL_BUILD_006`
   * - .. id:: ERR_LLVM_21
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An object or static library exposes additional symbols
     - Internal functionality might become callable from the outside
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_LLVM_22
     - :id:`UC1_RLIB`, :id:`UC2_STATICLIB`, :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Output does not contain expected variables or functions
     - Invalid code generated
     - :id:`AVD_CHECK_CLEAN_ENV_002` AND :id:`AVD_CLEAN_004` AND :id:`AVD_TEST_007`

.. end of table


Linking
^^^^^^^

.. list-table:: ERR_LINK
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
   * - .. id:: ERR_LINK_23
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Invalid input is accepted
     - Undefined behavior
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: ERR_LINK_24
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Invalid executable or library produced
     - Undefined behavior
     - :id:`AVD_TEST_007`
   * - .. id:: ERR_LINK_25
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - The behavior is incorrect because of concurrent modifications
     - Undefined behavior
     - :id:`AVD_PARALLEL_BUILD_006`
   * - .. id:: ERR_LINK_26
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - :id:`AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: ERR_LINK_27
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - An input is missing
     - Invalid code generated but won't run
     - :id:`AVD_CHECK_INSTALL_001`
   * - .. id:: ERR_LINK_28
     - :id:`UC3_EXEC`, :id:`UC4_EXEC_RLIB`, :id:`UC5_EXEC_CLIB`
     - Error diagnostics not emmited
     - Invalid or missing code not detected by user may be linked against subsequent stage
     - :id:`AVD_TEST_007`

.. end of table


Detection Measures and Usage Restriction
----------------------------------------

.. list-table:: Detection measures and usage restriction
   :align: left
   :header-rows: 1

   * - Measure identifier
     - Description
   * - .. id:: AVD_CHECK_INSTALL_001
     -  The toolchain Installation shall be checked in order to ensure the validity of the build results.
   * - .. id:: AVD_CHECK_CLEAN_ENV_002
     -  User must verify that environment variables used by the toolchain are correctly set.
   * - .. id:: AVD_CHECK_BUILD_SCRIPT_003
     -  User must verify that the list of build actions is correct.
   * - .. id:: AVD_CLEAN_004
     -  Before building, the user must ensure that the build environment is clean of former compilation artifacts.
   * - .. id:: AVD_WARNING_AS_ERROR_005
     -  All Warnings should be considered errors, the build should NOT display any warning.
   * - .. id:: AVD_PARALLEL_BUILD_006
     -  Concurrent file updates during the build operations are prohibited.
   * - .. id:: AVD_TEST_007
     -  Testing must be performed on the final application or libraries, or on any parts built, using an environment as close as possible to the final build.


Potential Errors by Classes Traceability Matrix
-----------------------------------------------

Potential errors are the result of the HazOp analysis, it should be documented
in the HazOp Report documents.

Tool Evaluation Results
-----------------------

During this analysis, we highlighted some of the potential errors concerning
Ferrocene that impacts the safety-related software code. Hence, the tool
impact is **TI2**.

Moreover, this analysis shows us that the likelihood of detecting these
potential errors is very low. Therefore, the tool error detection class is
**TD3**.

Using clause 11.4.5.4 in part 8 of the [|iso_ref|] standard, we can conclude that in
the worst case the Tool Classification Level is **TCL3** and therefore we choose
the following qualification methods:

* 1b. Evaluation of the tool development process in accordance with 11.4.8
* 1c. Validation of the software tool in accordance with 11.4.9

According to clause 11.4.2 in part 8 of the [|iso_ref|] standard, this choice
depends on the user's software development life-cycle and their validation strategy.
The user has the responsibility to determine whether this level, or a better one, is
applicable.


IEC 61508 Tool Classification
-----------------------------

Ferrocene provides a development environment capable of compiling,
and linking programs for the target architecture to conform with automotive
[|iso_ref|] TCL 3/ASIL D level and industrial [|iec_ref|] T3 TQL level.
