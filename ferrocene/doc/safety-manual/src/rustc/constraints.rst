.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Constraints
===========

Installation
------------

.. id:: RUSTC_CSTR_0010_INSTALL

Associated requirement ID: :id:`RUSTC_AVD_CHECK_INSTALL_001`

Before using the Ferrocene toolchain, the user shall ensure that the
appropriate tool package has been installed, and installation verified
according to the verification procedure described in
:doc:`Installation Procedures </procedures>`.

Cleaning environment variables
------------------------------

.. id:: RUSTC_CSTR_0020_CLEAN_ENV

Associated requirement ID: :id:`RUSTC_AVD_CHECK_CLEAN_ENV_002`

Before using the Ferrocene toolchain, the user shall ensure that the
toolchain is available on the user ``PATH`` variable, and that none of
the disallowed environment variables described in
:ref:`Degraded Environment <rustc/degraded-environment:Degraded environment>`
are set.

Build monitoring
----------------

.. id:: RUSTC_CSTR_0030_BUILD_MONITORING

Associated requirement ID: :id:`RUSTC_AVD_CHECK_BUILD_SCRIPT_003`

The user shall inspect the build logs to verify that all actions have
been completed, executed in the correct order, and the correctness of the
build, and in particular that the proper versions of the tools have been used
with the appropriate options.

Cleaning the build environment
------------------------------

.. id:: RUSTC_CSTR_0040_CLEAN

Associated requirement ID: :id:`RUSTC_AVD_CLEAN_004`

Before building the final version of the safety-related software, the
user shall ensure that the build environment is clean of former
compilation artifacts, as described in
:ref:`Cleaning the build space <rustc/usage:Cleaning the build space>`.

Treating warnings as errors
---------------------------

.. id:: RUSTC_CSTR_0050_WARNING_ERROR

Associated requirement ID: :id:`RUSTC_AVD_WARNING_AS_ERROR_005`

The user shall ensure that, for the final build, the option to treat all
warnings as errors is activated as described in
:ref:`rustc/usage:Warnings and errors`.

Source modification while building
----------------------------------

.. id:: RUSTC_CSTR_0060_PARALLEL

Associated requirement ID: :id:`RUSTC_AVD_PARALLEL_BUILD_006`

The user shall not perform source modification when the build of the
safety-related software is in progress.

Testing
-------

.. id:: RUSTC_CSTR_0070_TEST

Associated requirement ID: :id:`RUSTC_AVD_TEST_007`

The integrated testing objectives of |iso_26262_ref| (Part 8 - Clause 11.4.9) must be
performed on the final application or libraries, or on any parts built,
using build protocols as close as possible to those used for the final
build.

Problem mitigation
------------------

.. id:: RUSTC_CSTR_0080_KP

Associated requirement ID: :id:`RUSTC_AVD_MITIGATE_KPS_009`

The user shall implement mitigation strategies for known problems
documented in the :ref:`known-problems:Known Problems` manual.

Tracking new problems
---------------------

.. id:: RUSTC_CSTR_0090_NEW_KP

The user shall regularly inspect the new known problems reported via the
mechanism documented in
:ref:`Change Tracking <qualification-plan:change-tracking:Change Tracking>`. In case of
a new safety impact, the user shall ensure the appropriate mitigation
strategies are put in place.

Ensuring memory safety
----------------------

.. id:: RUSTC_CSTR_0100_UNSAFETY

Associated requirement ID: :id:`RUSTC_AVD_ENSURE_MEMORY_SAFETY_010`

The user shall identify and evaluate the risks related to all instances
of unsafe code as defined in :doc:`specification:unsafety`, and follow
the guidelines outlined in
:ref:`Handling Unsafety <rustc/unsafety:Handling Unsafety>`.

Avoiding data races due to hardware interrupts
----------------------------------------------

.. id:: RUSTC_CSTR_0110_INTERRUPTS

The user shall structure their code such that hardware interrupts will
not result in data race conditions.

Verifying linker scripts
------------------------

.. id:: RUSTC_CSTR_0120_LINKER_SCRIPTS

The user shall ensure that the linker scripts they use are interpreted correctly by ``rust-lld``.

No attributes on a proc macro implementation function
-----------------------------------------------------

.. id:: RUSTC_CSTR_0130_PROC_MACRO_NO_ATTRIBUTE

Associated requirement ID: :id:`RUSTC_AVD_PROC_MACRO_NO_ATTRIBUTE_008`

The user shall ensure that no attributes are used on a proc macro implementation function. The only exceptions are ``proc_macro``, ``proc_macro_derive`` or ``proc_macro_attribute``.

.. note::

   This is a precautionary measure and will be lifted or widened in the future.

No linker scripts with proc-macro crates
----------------------------------------

.. id:: RUSTC_CSTR_0150_PROC_MACRO_NO_LINKER_SCRIPT

Associated requirement ID: :id:`RUSTC_AVD_PROC_MACRO_NO_LINKER_SCRIPTS_011`

The user shall ensure that no linker scripts are used when compiling a proc-macro crate.

.. note::

   This is a precautionary measure and will be lifted or widened in the future.
