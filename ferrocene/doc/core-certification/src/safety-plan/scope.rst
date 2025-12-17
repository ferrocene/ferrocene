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
   * - |iec_61508_ref|
     - |iec_61508_core_sil|

The core library is evaluated as an "assessment of non-compliant development” (according to Route 3S of |iec_61508_ref| section 7.4.2.12). This assessment targets a full compliance statement to the standards above, as far as it is applicable for a Software Safety Element out of Context.

Certified version
-----------------

The certified version of the core library is |rust_version|.

.. _certified-core-targets:

Certified targets
-----------------

The core library is certified only on "certified targets".
Certified targets are like qualified targets, but furthermore ship with a
pre-compiled certified core standard library. Each certified target has a
qualified "base" target. Refer to :ref:`qualified-targets` for more information
about qualified targets.

.. warning::

    Qualified targets are not certified. Subset targets are not certified. Code
    which wishes to use the certified core library must use a certified target,
    not a qualified target.

Like qualified targets, only stable releases of certified targets are certified.
Other releases, such as beta, should be considered Quality Managed. Such releases can be certified upon request.

The following targets are certified for use with the pre-compiled certified core standard library:

.. list-table::
   :header-rows: 1

   * - Target
     - Qualified (Base) target tuple
     - Certified target tuple

   * - :ref:`aarch64-unknown-none`
     - ``aarch64-unknown-none``
     - ``aarch64-ferrocene-none``

   * - :ref:`aarch64-unknown-nto-qnx710`
     - ``aarch64-unknown-nto-qnx710``
     - ``aarch64-ferrocene-none``

   * - :ref:`thumbv7em-none-eabi`
     - ``thumbv7em-none-eabi``
     - ``thumbv7em-ferrocene-none-eabi``

   * - :ref:`thumbv7em-none-eabihf`
     - ``thumbv7em-none-eabihf``
     - ``thumbv7em-ferrocene-none-eabihf``

   * - :ref:`x86_64-unknown-linux-gnu`
     - ``x86_64-unknown-linux-gnu``
     - ``x86_64-ferrocene-none``


.. note::

    The ``x86_64-pc-nto-qnx710`` target has no certified equivalent.
    This target remains qualified for use with the Ferrocene compiler,
    but the core library for this target is not certified.

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
