.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Link with ISO 26262 Requirements
================================

[|iso_ref|] requirements describe three steps for Software Tool Qualification:

#. Planning of usage (Part 8 - Section 11.4.4)
#. Evaluation of a software tool by analysis (Part 8 - Section 11.4.5)
#. Qualification of a software tool (Part 8 - Sections
   11.4.6, 11.4.7, 11.4.8, 11.4.9)

Since this is the first Ferrocene qualification, consequently, Part 8 -
Section 11.4.7 is not considered as applicable. Ferrocene confidence will
be established starting from this qualified version.

In addition, Ferrocene follows an open source development model.
Nevertheless, Ferrocene is the qualified version of the open source Rust
compiler, and will therefore follow a develoment process in accordance to
11.4.8.

In order to respect these three steps, we'll manage five documents:

* :doc:`evaluation-plan:index`
* :doc:`evaluation-report:index`
* :doc:`qualification-plan:index`
* :doc:`qualification-report:index`
* :doc:`safety-manual:index`

The :doc:`index` (this document) and :doc:`evaluation-report:index`, describe
all phases for the Tool Confidence Level classification (Step 2: Evaluation of a
software tool by analysis):

* Presentation of how we evaluate the confidence level.
* Analysis of potential errors with the assessment of the tool impact and the
  tool error detection probability.
* Definition of the qualification method to ensure that there are no potential
  errors without mitigation or detection means.
* Test suites description.

The :doc:`qualification-plan:index` describes how the tool is qualified
(Step 3: qualification of a software tool):

* Tool description and architecture.
* Who is responsible for this qualification.
* Development process.
* Validation process.

The :doc:`qualification-report:index` describes the result of the tool
validation (Step 3: qualification of a software tool):

* Tool identification (version/name).
* Environments.
* Configuration data.
* Qualification results (Test Reports, etc.).
* Usage Constraints or errors identified in the course of this qualification,
  with mitigations.

The :doc:`safety-manual:index` describes the usage, constraints, and limitations
of the tool according to this qualification (Step 1: Planning of usage + some
result from Step 2):

* List of the potential errors with mitigations or workarounds.
* Description of the tool usage and other information (configuration data,
  environment, etc.).
