.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Certification scope
===================

The core library shall be suitable to be used in safety applications according to following safety standards up to the specified safety level:

.. list-table::
   :align: left
   :header-rows: 1

   * - Safety Standard
     - Safety Level
     - Evaluation method
   * - |iec_61508_ref|
     - |iec_61508_core_sil|
     - Assessment of non-compliant development (Route 3S of |iec_61508_ref| section 7.4.2.12)
   * - |iso_26262_ref|
     - |iso_26262_core_asil|
     - SEooC SW Development; Tailored following the standard (-2/-6/-8) tailoring

This assessment targets a full compliance statement to the standards above, as far as it is applicable for a Software Safety Element out of Context.

Certified version
-----------------

The certified version of the core library is |rust_version|.

.. _certified-core-targets:

Certified subset
----------------

The certification does not cover the entirety of the core library, but instead a subset. This is to reduce the effort of the certification.

The subset included in the safety certification is defined and documented in the :doc:`Safety Manual <safety-manual:core/subset>`.

Systematic capabilities
~~~~~~~~~~~~~~~~~~~~~~~

All public functions of the certified subset are considered "software safety functions" and are going to be certified for all safety standards up to the safety level specified. That means our customers can use all of those functions for use cases up to the highest safety level specified. Since we consider all of them safety relevant we do not consider independence.

The systematic capability of these functions is based on:

- The requirements and the documented completeness of these requirements and their implementation in the code and tests
- The absence of any undocumented and untested code in the safety certification scope
- The required test coverage
- The adherence of the code within the safety scope to the Coding Guidelines
