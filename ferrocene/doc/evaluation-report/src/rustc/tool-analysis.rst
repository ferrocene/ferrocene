.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Tool Analysis
=============

Potential Errors
----------------

Installation
^^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Description
     - Risk
     - Detectable
     - Mitigation
   * - .. id:: RUSTC_ERR_INSTALL_01
     - Ferrocene was not correctly installed
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_CHECK_INSTALL_001`

.. end of table

Use cases
*********

* :id:`RUSTC_UC0_INST`


Rust Driver
^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Description
     - Risk
     - Detectable
     - Mitigation
   * - .. id:: RUSTC_ERR_DRIVER_02
     - An used environment variable is set to an incorrect value
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_CHECK_CLEAN_ENV_002`
   * - .. id:: RUSTC_ERR_DRIVER_03
     - An invalid option is passed
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: RUSTC_ERR_DRIVER_04
     - Error diagnostics are not correctly emited
     - Undefined behavior
     - NO
     - | :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`,
       | :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_DRIVER_05
     - The output is generated with missing part
     - Wrong code
     - NO
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: RUSTC_ERR_DRIVER_06
     - The behavior is incorrect because of concurrent modification
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
   * - .. id:: RUSTC_ERR_DRIVER_07
     - A warning is generated instead of an error
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_WARNING_AS_ERROR_005`
   * - .. id:: RUSTC_ERR_DRIVER_08
     - The compilation has a wrong behavior
     - Wrong code
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_DRIVER_09
     - An incomplete input is accepted leading to an undefined behavior
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_DRIVER_10
     - Some object files are silently not generated
     - Use an artifact from a previous build
     - NO
     - :id:`RUSTC_AVD_CLEAN_004`

.. end of table

Use cases
*********

* :id:`RUSTC_UC1_RLIB`,
* :id:`RUSTC_UC2_STATICLIB`,
* :id:`RUSTC_UC3_EXEC`,
* :id:`RUSTC_UC4_EXEC_RLIB`,
* :id:`RUSTC_UC5_EXEC_CLIB`


Rust Front-End
^^^^^^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Description
     - Risk
     - Detectable
     - Mitigation
   * - .. id:: RUSTC_ERR_RUST_FE_11
     - Input has invalid contents
     - Invalid code generated
     - YES
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_RUST_FE_12
     - Error diagnostics is invalid
     - Invalid code generated
     - NO
     - :id:`RUSTC_AVD_WARNING_AS_ERROR_005`
   * - .. id:: RUSTC_ERR_RUST_FE_13
     - Invalid output generated from valid input
     - Invalid code generated
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_RUST_FE_14
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - NO
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
   * - .. id:: RUSTC_ERR_RUST_FE_15
     - Invalid input is accepted
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_RUST_FE_16
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`

.. end of table

Use cases
*********

* :id:`RUSTC_UC1_RLIB`,
* :id:`RUSTC_UC2_STATICLIB`,
* :id:`RUSTC_UC3_EXEC`,
* :id:`RUSTC_UC4_EXEC_RLIB`,
* :id:`RUSTC_UC5_EXEC_CLIB`


LLVM
^^^^

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Description
     - Risk
     - Detectable
     - Mitigation
   * - .. id:: RUSTC_ERR_LLVM_17
     - Input parameter has invalid value
     - Most likely LLVM will crash. Invalid code could also be generated
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_LLVM_18
     - An object file is invalid
     - Invalid code generated
     - NO
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: RUSTC_ERR_LLVM_19
     - An object file or static library is not correctly translated to machine code
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_LLVM_20
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - NO
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
   * - .. id:: RUSTC_ERR_LLVM_21
     - An object or static library exposes additional symbols
     - Internal functionality might become callable from the outside
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_LLVM_22
     - Output does not contain expected variables or functions
     - Invalid code generated
     - NO
     - | :id:`RUSTC_AVD_CHECK_CLEAN_ENV_002`,
       | :id:`RUSTC_AVD_CLEAN_004`,
       | :id:`RUSTC_AVD_TEST_007`

.. end of table

Use cases
*********

* :id:`RUSTC_UC1_RLIB`,
* :id:`RUSTC_UC2_STATICLIB`,
* :id:`RUSTC_UC3_EXEC`,
* :id:`RUSTC_UC4_EXEC_RLIB`,
* :id:`RUSTC_UC5_EXEC_CLIB`


Linking
^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Description
     - Risk
     - Detectable
     - Mitigation
   * - .. id:: RUSTC_ERR_LINK_23
     - Invalid input is accepted
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: RUSTC_ERR_LINK_24
     - Invalid executable or library produced
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_TEST_007`
   * - .. id:: RUSTC_ERR_LINK_25
     - The behavior is incorrect because of concurrent modifications
     - Undefined behavior
     - NO
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
   * - .. id:: RUSTC_ERR_LINK_26
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - YES
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
   * - .. id:: RUSTC_ERR_LINK_27
     - An input is missing
     - Invalid code generated but won't run
     - YES
     - :id:`RUSTC_AVD_CHECK_INSTALL_001`
   * - .. id:: RUSTC_ERR_LINK_28
     - Error diagnostics not emmited
     - Invalid or missing code not detected by user may be linked against subsequent stage
     - NO
     - :id:`RUSTC_AVD_TEST_007`

.. end of table

Use cases
*********

* :id:`RUSTC_UC3_EXEC`,
* :id:`RUSTC_UC4_EXEC_RLIB`,
* :id:`RUSTC_UC5_EXEC_CLIB`


Detection Measures and Usage Restriction
----------------------------------------

.. list-table::
   :align: left
   :header-rows: 1

   * - Measure identifier
     - Description
   * - .. id:: RUSTC_AVD_CHECK_INSTALL_001
     -  The toolchain Installation shall be checked in order to ensure the validity of the build results.
   * - .. id:: RUSTC_AVD_CHECK_CLEAN_ENV_002
     -  User must verify that environment variables used by the toolchain are correctly set.
   * - .. id:: RUSTC_AVD_CHECK_BUILD_SCRIPT_003
     -  User must verify that the list of build actions is correct.
   * - .. id:: RUSTC_AVD_CLEAN_004
     -  Before building, the user must ensure that the build environment is clean of former compilation artifacts.
   * - .. id:: RUSTC_AVD_WARNING_AS_ERROR_005
     -  All Warnings should be considered errors, the build should NOT display any warning.
   * - .. id:: RUSTC_AVD_PARALLEL_BUILD_006
     -  Concurrent file updates during the build operations are prohibited.
   * - .. id:: RUSTC_AVD_TEST_007
     -  Testing must be performed on the final application or libraries, or on any parts built, using an environment as close as possible to the final build.


Potential Errors by Classes Traceability Matrix
-----------------------------------------------

Potential errors are the result of the HazOp analysis, it should be documented
in the HazOp Report documents.

ISO 26262 Tool Classification
-----------------------------

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

Ferrocene provides a development environment capable of compiling and linking
programs for the target architecture to conform with industrial [|iec_ref|]
class T3.
