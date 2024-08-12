.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. default-domain:: qualification

Tool Analysis
=============

Potential Errors
----------------

.. hazop-error::
   :id: ERR_RUSTFMT_CHANGED_SEMANTICS
   :caused_by: USE_RUSTFMT_FORMAT

   | **Risk:** Change in program semantics
   | **Detectable:** YES

   rustfmt emits code with a different semantic meaning than the input.

.. _rustfmt_avd:

Detection Measures and Usage Restriction
----------------------------------------

Detection measures and usage restrictions are documented in the :doc:`safety
manual <safety-manual:rustfmt/constraints>`.

.. _rustfmt_iso_tool_classification:

ISO 26262 Tool Classification
-----------------------------

During this analysis, we highlighted some of the potential errors concerning
rustfmt that impacts the safety-related software code. Hence, the tool
impact is **TI2**.

Moreover, this analysis shows us that the likelihood of detecting these
potential errors is very high. Therefore, the tool error detection class is
**TD1**.

Using clause 11.4.5.4 in part 8 of the [|iso_ref|] standard, we can conclude
that in the worst case the Tool Classification Level is **TCL1** up to ASIL D.
Therefore we need no qualification methods.

.. _rustfmt_iec_tool_classification:

IEC 61508 Tool Classification
-----------------------------

rustfmt is a industrial [|iec_ref|] class T3 tool, because it generates outputs
which can directly contribute to the executable code of the safety-related
system.

Nevertheless, we need no qualification methods, because the detection measures
and usage restrictions listed above give us high confidence to detect all
potential errors. 

IEC 62304 Tool Classification
-----------------------------

[|iec_med_ref|] does not provide an own scheme to classify and qualify tools used in its context, but recommends the application
of techniques and tools as defined in [|iec_ref|]. Therefore, with the qualification of Ferrocene adhering to an IEC 61508 Tool Classification,
Ferrocene can be used for development, release and maintenance of medical device software up to Class C.
