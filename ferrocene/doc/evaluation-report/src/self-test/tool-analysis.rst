.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Tool Analysis
=============

Potential Errors
----------------

.. hazop-error::
   :id: ERR_SELF_TEST_MISSING_TARGET_CHECKS
   :caused_by: USE_SELF_TEST_CHECK

   | **Risk:** Invalid output
   | **Detectable:** YES

   Tool does not perform checks for some installed targets.

.. hazop-error::
   :id: ERR_SELF_TEST_FALSE_NEGATIVES
   :caused_by: USE_SELF_TEST_CHECK

   | **Risk:** Invalid output
   | **Detectable:** NO

   Tool fails to detect a faulty toolchain installation.

.. hazop-error::
   :id: ERR_SELF_TEST_FALSE_POSITIVES
   :caused_by: USE_SELF_TEST_CHECK

   | **Risk:** A broken Ferrocene Installation (:id:`RUSTC_ERR_INSTALL_01`)
   | **Detectable:** YES

   Tool falsely indicates a faulty toolchain installation.

.. hazop-error::
   :id: ERR_SELF_TEST_MODIFIES_TOOLCHAIN
   :caused_by: USE_SELF_TEST_CHECK

   | **Risk:** Invalid output
   | **Detectable:** NO

   Tool creates, changes, or deletes toolchain files/directories.

.. _self-test_avd:

Detection Measures and Usage Restriction
----------------------------------------

Detection measures and usage restrictions are documented in the :doc:`safety
manual <safety-manual:self-test/constraints>`.

.. _self_test_iso_tool_classification:

ISO 26262 Tool Classification
-----------------------------

During this analysis, we highlighted some of the potential errors concerning
``ferrocene-self-test`` that impacts the safety-related software code.
Hence, the tool impact is **TI2**.

Moreover, this analysis shows us that the likelihood of detecting some of the
potential errors is very low.
Therefore, the tool error detection class is **TD3**.

Using clause 11.4.5.4 in part 8 of the [|iso_ref|] standard, we can conclude
that in the worst case the Tool Classification Level is **TCL3** up to ASIL D.
Therefore, we choose the following qualification methods:

- **1b.** Evaluation of the tool development process in accordance with 11.4.8
- **1c.** Validation of the software tool in accordance with 11.4.9

According to clause 11.4.2 in part 8 of the [|iso_ref|] standard,
this choice depends on the user's software development life-cycle and their validation strategy.
The user has the responsibility to determine whether this level, or a better one, is applicable.

.. _self_test_iec_tool_classification:

IEC 61508 Tool Classification
-----------------------------

``ferrocene-self-test`` verifies the installed toolchain,
and is therefore an industrial [|iec_ref|] class T2 tool.

IEC 62304 Tool Classification
-----------------------------

.. include:: ../partials/62304-tool-classification.rst
