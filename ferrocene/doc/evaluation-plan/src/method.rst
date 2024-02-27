.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Evaluation Method
=================

The [|iso_ref|] standard requires that the software Tool Confidence Level (TCL)
shall be determined at the very beginning of the qualification process. For this
evaluation, the Tool Impact (TI) and then the Tool error Detection level (TD)
of each error will be determined. This is done by evaluating the use cases of
the tool based on the usage in the activities of safety-critical product
development that this tool replaces or simplifies.

Through this analysis, the following potential error are identified:

* Potential errors with the possibility to detect and to mitigate them.
* Potential errors without the possibility to detect or to mitigate them.
* Potential errors that do not appear in use cases.
* Potential errors without an impact for safety.

Errors, with the possibility of detection and mitigation, are added in
the :doc:`safety-manual:index` with the following form:

.. list-table::

   * - Error identifier 
     - An identifier for the error with format ERR_NAME_XXX where NAME is a short name describing the faulty feature, and XXX is a number.
   * - Error description 
     - The description of the error.
   * - Condition of appearance 
     - Condition to reproduce the error.
   * - Effects 
     - The effects of the error for the safety-critical products.
   * - Workaround/Mitigation 
     - The description of the mitigation and/or the workaround.

.. end of table

For potential errors with no mitigation or test mitigation only, a test will be
added in the :doc:`qualification-plan:index` to verify these potential errors
cannot happen.

Next sections will describe each step needed for the tool evaluation.

Identify Use Cases
------------------

For each tool, the use cases needed for the Tool Qualification Process need to
be identified. For each use case,  the input artifacts (e.g a C file, a Rust
file, etc.), the output artifacts (object file, executable, etc.), and usage
constraints must also be listed. The following table represents all the
information that is required to describe a use case.

.. list-table::

   * - Use case identifier 
     - UC_XXXX where XXXX is an identifier for the use case.
   * - Actor(s) 
     - The actor who controls this use case.
   * - Input(s) 
     - List of the input artifacts.
   * - Output(s) 
     - List of the output artifacts.
   * - Constraint(s) 
     - List of the tool constraints for qualification (Environment, usage limitation, etc.), If applicable.
   * - Description 
     - The description of the use cases.

.. end of table

Identify Potential Errors From Use Cases
----------------------------------------

In order to determine the potential errors of Ferrocene, the HazOp
methodology, applied to software, will be used. This method describes how to
organize a brainstorming session in order to detect all potential errors. It is
based on the use of specific words called "Guide Words" that guide
brainstorming sessions around deviations in toolchain usages. These guide words
should be used systematically with each parameter and applied to each toolchain
node and the associated deviation should be evaluated by answering the following
questions:

* Does the hazard exist?
* Can the hazard happen?
* Is there a way to detect it?
* Is there a way to mitigate it?
* What is the impact of this hazard?

All relevant potential errors should be named with a unique identifier, and
should be linked with the associated detection measure and with the associated
mitigation if any. The following table describes a potential error entry in the
HazOp analysis:

.. list-table::

   * - Error identifier 
     - ERR_TOOLID_XXX where TOOLID is the name of the tool causing the error, and XXX a number.
   * - Use case 
     - The use case where the error is determined.
   * - Description 
     - The potential error descriptions.
   * - Risk 
     - Possibles effects of the potential error.
   * - Mitigation 
     - The detection or mitigation method identifier, if available.
   * - Detectable
     - YES or NO, indicates if the error could be detected.

.. end of table

The associated detection measure and/or mitigation should be identified with an
unique name, and with a detailed description of the measure. The following
table describes a countermeasure entry:

.. list-table::

   * - Measure identifier 
     - CHK_NAME_XXX | AVD_NAME_XXX where NAME is a short name describing the feature and XXX is a number.
   * - Description 
     - The description of the check or the avoidance mechanism.

.. end of table

HazOp Organization
^^^^^^^^^^^^^^^^^^

For this software tool qualification, we decided to optimize the assessment
process by considering the following organization:

#. A meeting is done to choose the guide words, node, and parameters.
#. The first analysis is done by each HazOp team member in a shared document.
#. Several other meetings allow us to discuss the analysis of each member.
#. Report is directly updated by members.
#. An expert reads, the second time, the sessions' report and adds remarks
   based on their domain knowledge, e.g. "impossible deviation", or adding some
   missing deviations.
#. Finally, an analysis of these results is performed in order to merge similar
   potential errors in an hazard class. This class groups potential errors with
   the same safety concerns, the same node, the same risks and the same
   mitigation.

HazOp Parameters
^^^^^^^^^^^^^^^^
For the HazOp process, guide words should be applied to parameters in a
systematic manner, to determine the associated deviation. For our HazOp process,
we choose the following parameters:

* **input:** An input parameter such as command line arguments, input files, or
  environment settings.
* **output:** An output parameter such as an output file or the console output.
* **action:** A behavior of a software process.

See :doc:`evaluation-report:index` for specific guide words and specific nodes.

Determine Tool Impact (TI)
--------------------------

As described in ISO-26262 Part 8 - Section 11.4.5.2(a), if a malfunction can
introduce or fail to detect errors in a safety-critical product, we have to
select the tool impact **TI2**, if no such possibility exists we can consider
the Tool Impact as **TI1** means no impact.

Determine Tool Error Detection levels (TD)
------------------------------------------

Secondly, if the tool impact is **TI2**, we have to analyze all use cases to
detect potential errors. During this analysis, for each potential error, we
evaluate the detection or prevention likelihood by considering a Tool error
Detection probability (TD), defined as follows:

* **TD1**: if there is a **high** probability that a malfunction and its
  corresponding erroneous output will be prevented or detected.
* **TD2**: if there is a **medium** probability that a malfunction and its
  corresponding erroneous output will be prevented or detected.
* **TD3**: in all other cases.

If there exist several detection or prevention measures for one potential error,
we can use the highest probability measure for the TD calculation. If there are
several errors for a tool or a use case, we choose the worst error detection
probability for the tool or the use case.

Compute the Tool Confidence Level
---------------------------------

The [|iso_ref|] standard (Part 8 - Section 11.4.5.5) determines the TCL by
applying the TI and TD values with the following table:

.. list-table::
   :header-rows: 1
   
   * - 
     - TD1
     - TD2
     - TD3
   * - TI1 
     - TCL1 
     - TCL1 
     - TCL1
   * - TI2 
     - TCL1 
     - TCL2 
     - TCL3

.. end of table

Compute the IEC 61508 Tool Classification
-----------------------------------------

Ferrocene is an off-line tool for this certification. Since no runtime is
qualified, the IEC 61508 standard defines three classes for such tools:

* **T1:** the tool generates no outputs which can directly or indirectly
  contribute to the executable code (including data) of the safety-related
  system;
* **T2:** the tool supports the test or verification of the design or executable
  code, where errors in the tool can fail to reveal defects but cannot directly
  create errors in the executable software;
* **T3:** the tool generates outputs which can directly or indirectly contribute
  to the executable code of the safety-related system.

Select the Appropriate Qualification Method
-------------------------------------------

When the Tool Confidence Level is known, we should choose the recommended
qualification method according to the TCL value, depending if it's Tool
Confidence Level is TCL2 or TCL3.

Remark: we do not need a qualification for tools with a Tool Confidence Level
TCL1.

The following tables from the [|iso_ref|] standard indicate the appropriate
qualification method:

TCL2
^^^^

.. list-table::
   :header-rows: 1

   * - 
     - Method
     - ASIL A
     - ASIL B
     - ASIL C
     - ASIL D
   * - 1a
     - Increased confidence from use in accordance with 11.4.7
     - ++ 
     - ++ 
     - ++ 
     - \+
   * - 1b
     - Evaluation of the tool development process in accordance with 11.4.8
     - ++ 
     - ++ 
     - ++ 
     - \+
   * - 1c
     - Validation of the software tool in accordance with 11.4.9
     - \+ 
     - \+ 
     - \+ 
     - ++
   * - 1d
     - Development in accordance with a safety standard
     - \+ 
     - \+ 
     - \+ 
     - \+

.. end of table


TCL3
^^^^

.. list-table::
   :header-rows: 1

   * - 
     - Method
     - ASIL A
     - ASIL B
     - ASIL C
     - ASIL D
   * - 1a
     - Increased confidence from use in accordance with 11.4.7
     - ++ 
     - ++ 
     - \+ 
     - \+
   * - 1b
     - Evaluation of the tool development process in accordance with 11.4.8
     - ++ 
     - ++ 
     - \+ 
     - \+
   * - 1c
     - Validation of the software tool in accordance with 11.4.9
     - \+ 
     - \+ 
     - ++ 
     - ++
   * - 1d
     - Development in accordance with a safety standard
     - \+ 
     - \+ 
     - ++ 
     - ++

.. end of table

Legend
^^^^^^

.. list-table::

   * - ++
     - Highly recommended
   * - \+
     - Recommended

.. end of table
