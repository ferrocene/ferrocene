.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Link with IEC 62304 Requirements
================================

The [|iec_med_ref|] requirements describe the consideration of software tools with respect to:

#. Planning of tool usage in the software development plan (Section 5.1.4)
#. Supporting items to be controlled, including tools, items or settings used to develop the medical device software (Section 5.1.10)
#. Documentation of the procedures used to release the software, including the retrieval of tools used (Section B.5.8)
#. Recommendations of techniques, tools and methods, following the principles of [|iec_ref|], encouraging the usage of [|iec_ref|] as source of good software methods (Section C.7)

As there is no specific method defined in IEC 62304 to qualify tools, but the usage of good software practices of [|iec_ref|] is encouraged, the 
application of tool classification and qualification according to [|iec_ref|] is used to qualify Ferrocene.

In addition, Ferrocene follows an open source development model.
Nevertheless, Ferrocene is the qualified version of the open source Rust
toolchain, and will therefore follow a develoment process in accordance to
[|iso_ref|] Clause 11.4.8, as well as classification accordance to [|iec_ref|].

In order to respect these three steps, we'll manage five documents:

* :doc:`evaluation-plan:index`
* :doc:`evaluation-report:index`
* :doc:`qualification-plan:index`
* :doc:`qualification-report:index`
* :doc:`safety-manual:index`

The :doc:`safety-manual:index` describes the usage, constraints, and limitations
of the tool according to this qualification:

* Description of the tool usage and other information (configuration data,
  environment, settings etc.).
* List of the potential errors with mitigations or workarounds to enable risk management.

Following the recommendation of following best principles of [|iec_ref|], the :doc:`link-with-iec-requirements` 
describes how the tool qualification activities of [|iec_ref|] are applied.
