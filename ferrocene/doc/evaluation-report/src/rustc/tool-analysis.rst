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
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTC_ERR_INSTALL_01
     - :id:`RUSTC_UC0_INST`
     - Ferrocene was not correctly installed
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_INSTALL_001`
     - NO

.. end of table


Rust Driver
^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTC_ERR_DRIVER_02
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An used environment variable is set to an incorrect value
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_CLEAN_ENV_002`
     - YES
   * - .. id:: RUSTC_ERR_DRIVER_03
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An invalid option is passed
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - YES
   * - .. id:: RUSTC_ERR_DRIVER_04
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Error diagnostics are not correctly emited
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003` AND :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_05
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The output is generated with missing part
     - Wrong code
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_06
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The behavior is incorrect because of concurrent modification
     - Undefined behavior
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_07
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - A warning is generated instead of an error
     - Undefined behavior
     - :id:`RUSTC_AVD_WARNING_AS_ERROR_005`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_08
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The compilation has a wrong behavior
     - Wrong code
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_09
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An incomplete input is accepted leading to an undefined behavior
     - Undefined behavior
     - :id:`RUSTC_AVD_TEST_007`
     - YES
   * - .. id:: RUSTC_ERR_DRIVER_10
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Some object files are silently not generated
     - Use an artifact from a previous build
     - :id:`RUSTC_AVD_CLEAN_004`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_29
     - :id:`RUSTC_UC6_PROC_MACRO`
     - An shared object file of a incorrect or outdated ``proc-macro`` crate is passed to rustc
     - Wrong code
     - :id:`RUSTC_AVD_CLEAN_004`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_31
     - :id:`RUSTC_UC6_PROC_MACRO`
     - A proc-macro crate uses a dynamic library which is different than the one it was validated with, such as an updated library provided by a system package
     - Wrong code
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_DRIVER_32
     - :id:`RUSTC_UC6_PROC_MACRO`
     - A proc-macro contains Undefined Behavior, which corrupts the memory space of the compiler
     - Wrong code
     - :id:`RUSTC_AVD_MITIGATE_KPS_009`, :id:`RUSTC_AVD_ENSURE_MEMORY_SAFETY_010`
     - NO

.. end of table


Rust Front-End
^^^^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTC_ERR_RUST_FE_11
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Input has invalid contents
     - Invalid code generated
     - :id:`RUSTC_AVD_TEST_007`
     - YES
   * - .. id:: RUSTC_ERR_RUST_FE_12
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Error diagnostics is invalid
     - Invalid code generated
     - :id:`RUSTC_AVD_WARNING_AS_ERROR_005`
     - NO
   * - .. id:: RUSTC_ERR_RUST_FE_13
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Invalid output generated from valid input
     - Invalid code generated
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_RUST_FE_14
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
     - NO
   * - .. id:: RUSTC_ERR_RUST_FE_15
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Invalid input is accepted
     - Undefined behavior
     - :id:`RUSTC_AVD_TEST_007`
     - YES
   * - .. id:: RUSTC_ERR_RUST_FE_16
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - YES
   * - .. id:: RUSTC_ERR_RUST_FE_30
     - :id:`RUSTC_UC6_PROC_MACRO`
     - A proc macro implementation function is invoked incorrectly by rustc.
     - Undefined behavior
     - :id:`RUSTC_AVD_TEST_007`, :id:`RUSTC_AVD_PROC_MACRO_NO_LINKER_SCRIPTS_011`
     - YES

.. end of table


LLVM
^^^^

.. list-table::
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTC_ERR_LLVM_17
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Input parameter has invalid value
     - Most likely LLVM will crash. Invalid code could also be generated
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_LLVM_18
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An object file is invalid
     - Invalid code generated
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - NO
   * - .. id:: RUSTC_ERR_LLVM_19
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An object file or static library is not correctly translated to machine code
     - Undefined behavior
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_LLVM_20
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The behavior is incorrect because of concurrent modifications
     - Invalid code generated
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
     - NO
   * - .. id:: RUSTC_ERR_LLVM_21
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An object or static library exposes additional symbols
     - Internal functionality might become callable from the outside
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_LLVM_22
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Output does not contain expected variables or functions
     - Invalid code generated
     - :id:`RUSTC_AVD_CHECK_CLEAN_ENV_002` AND :id:`RUSTC_AVD_CLEAN_004` AND :id:`RUSTC_AVD_TEST_007`
     - NO

.. end of table


Linking
^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTC_ERR_LINK_23
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Invalid input is accepted
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - NO
   * - .. id:: RUSTC_ERR_LINK_24
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Invalid executable or library produced
     - Undefined behavior
     - :id:`RUSTC_AVD_TEST_007`
     - NO
   * - .. id:: RUSTC_ERR_LINK_25
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - The behavior is incorrect because of concurrent modifications
     - Undefined behavior
     - :id:`RUSTC_AVD_PARALLEL_BUILD_006`
     - NO
   * - .. id:: RUSTC_ERR_LINK_26
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Incorrect number of inputs are accepted
     - Undefined behavior
     - :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`
     - YES
   * - .. id:: RUSTC_ERR_LINK_27
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - An input is missing
     - Invalid code generated but won't run
     - :id:`RUSTC_AVD_CHECK_INSTALL_001`
     - YES
   * - .. id:: RUSTC_ERR_LINK_28
     - :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`, :id:`RUSTC_UC6_PROC_MACRO`
     - Error diagnostics not emmited
     - Invalid or missing code not detected by user may be linked against subsequent stage
     - :id:`RUSTC_AVD_TEST_007`
     - NO

.. end of table


Core library
^^^^^^^^^^^^

.. list-table::
   :align: left
   :header-rows: 1
   :widths: 15, 15, 25, 20, 25, 5

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: CORE_ERR_01
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`
     - Source code contains calls to uncertified functions
     - Uncertified code generated
     - :id:`CORE_AVD_SUBSET_001`
     - YES
   * - .. id:: CORE_ERR_02
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`
     - Source code contains macros
     - Generated code is incorrect
     - :id:`CORE_AVD_MACROS_002`
     - YES
   * - .. id:: CORE_ERR_03
     - :id:`RUSTC_UC1_RLIB`, :id:`RUSTC_UC2_STATICLIB`, :id:`RUSTC_UC3_EXEC`, :id:`RUSTC_UC4_EXEC_RLIB`, :id:`RUSTC_UC5_EXEC_CLIB`
     - Source code contains architecture specific functions
     - Functions are used incorrectly
     - :id:`CORE_AVD_ARCH_003`
     - YES

.. end of table

Detection Measures and Usage Restriction
----------------------------------------

.. list-table::
   :align: left
   :header-rows: 1

   * - Measure identifier
     - Description
   * - .. id:: RUSTC_AVD_CHECK_INSTALL_001
     - The toolchain Installation shall be checked in order to ensure the validity of the build results.
   * - .. id:: RUSTC_AVD_CHECK_CLEAN_ENV_002
     - User must verify that environment variables used by the toolchain are correctly set.
   * - .. id:: RUSTC_AVD_CHECK_BUILD_SCRIPT_003
     - User must verify that the list of build actions is correct.
   * - .. id:: RUSTC_AVD_CLEAN_004
     - Before building, the user must ensure that the build environment is clean of former compilation artifacts.
   * - .. id:: RUSTC_AVD_WARNING_AS_ERROR_005
     - All Warnings should be considered errors, the build should NOT display any warning.
   * - .. id:: RUSTC_AVD_PARALLEL_BUILD_006
     - Concurrent file updates during the build operations are prohibited.
   * - .. id:: RUSTC_AVD_TEST_007
     - Testing must be performed on the final application or libraries, or on any parts built, using an environment as close as possible to the final build.
   * - .. id:: RUSTC_AVD_PROC_MACRO_NO_ATTRIBUTE_008
     - The user shall ensure that no attributes are used on a macro implementation function. The only exceptions are ``proc_macro``, ``proc_macro_derive`` or ``proc_macro_attribute``.
   * - .. id:: RUSTC_AVD_MITIGATE_KPS_009
     - The user shall implement mitigation strategies for known problems documented in the :ref:`known-problems:Known Problems` manual.
   * - .. id:: RUSTC_AVD_ENSURE_MEMORY_SAFETY_010
     - The user shall identify and evaluate the risks related to all instances of unsafe code as defined in :doc:`specification:unsafety`, and follow the guidelines outlined in :doc:`Handling Unsafety <safety-manual:rustc/unsafety>`.
   * - .. id:: RUSTC_AVD_PROC_MACRO_NO_LINKER_SCRIPTS_011
     - The user shall ensure that no linker scripts are used when compiling a proc-macro crate.
   * - .. id:: CORE_AVD_SUBSET_001
     - User must verify that only the certified subset of the core library is used.
   * - .. id:: CORE_AVD_MACROS_002
     - User must verify that code generated by macros is correct.
   * - .. id:: CORE_AVD_ARCH_003
     - User must verify that architecture specific functions are used correctly.

.. end of table

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

Using clause 11.4.5.4 in part 8 of the [|iso_26262_ref|] standard, we can conclude that in
the worst case the Tool Classification Level is **TCL3** and therefore we choose
the following qualification methods:

* 1b. Evaluation of the tool development process in accordance with 11.4.8
* 1c. Validation of the software tool in accordance with 11.4.9

According to clause 11.4.2 in part 8 of the [|iso_26262_ref|] standard, this choice
depends on the user's software development life-cycle and their validation strategy.
The user has the responsibility to determine whether this level, or a better one, is
applicable.


IEC 61508 Tool Classification
-----------------------------

Ferrocene provides a development environment capable of compiling and linking
programs for the target architecture to conform with industrial [|iec_61508_ref|]
class T3.

IEC 62304 Tool Classification
-----------------------------

[|iec_62304_ref|] does not provide an own scheme to classify and qualify tools used in its context, but recommends the application
of techniques and tools as defined in [|iec_61508_ref|]. Therefore, with the qualification of Ferrocene adhering to an IEC 61508 Tool Classification,
Ferrocene can be used for development, release and maintenance of medical device software up to Class C.
