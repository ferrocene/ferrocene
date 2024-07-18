.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Tool Analysis
=============

Potential Errors
----------------

.. list-table::
   :align: left
   :header-rows: 1

   * - Error identifier
     - Use case
     - Description
     - Risk
     - Mitigation
     - Detectable
   * - .. id:: RUSTFMT_ERR_01
     - :id:`RUSTFMT_UC1_FMT`
     - rustfmt emits code with a different semantic meaning than the input
     - Change in program semantics
     - | :id:`RUSTFMT_AVD_CHECK_DIFF_001`, 
       | :id:`RUSTFMT_AVD_TESTS_002`
     - YES

.. end of table


.. _rustfmt_avd:

Detection Measures and Usage Restriction
----------------------------------------

.. list-table::
   :align: left
   :header-rows: 1

   * - Measure identifier
     - Description
   * - .. id:: RUSTFMT_AVD_CHECK_DIFF_001
     - After running rustfmt, the user must review the changes in the source
       code to ensure no semantics changed.
   * - .. id:: RUSTFMT_AVD_TESTS_002
     - After running rustfmt, sufficient tests need to pass to ensure the
       correctness of the program.

.. end of table

ISO 26262 Tool Classification
-----------------------------

During this analysis, we highlighted some of the potential errors concerning
rustfmt that impacts the safety-related software code. Hence, the tool
impact is **TI2**.

Moreover, this analysis shows us that the likelihood of detecting these
potential errors is very high. Therefore, the tool error detection class is
**TD1**.

Using clause 11.4.5.4 in part 8 of the [|iso_ref|] standard, we can conclude that in
the worst case the Tool Classification Level is **TCL1**. Therefore we need no
qualification methods.

.. _rustfmt_iec_tool_classification:

IEC 61508 Tool Classification
-----------------------------

rustfmt is a industrial [|iec_ref|] class T3 tool, because it generates outputs
which can directly contribute to the executable code of the safety-related
system.

Nevertheless, we need no qualification methods, because the detection measures
and usage restrictions listed above give us high confidence to detect all
potential errors. 
