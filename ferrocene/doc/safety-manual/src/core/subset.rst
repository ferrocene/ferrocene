.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Certified subset
================

The `Certified core library API docs <../../certification/api-docs/core/index.html>`_ is the autoritative document stating which items are included in the certified subset of the core library.

Compliance with subset
----------------------

To prove compliance with the subset, follow following steps:

1. Ensure that your code successfully compiles with your compilation target.
2. Ensure that it also compiles with the certified equivalent listed in the table below.
3. If it succeeds, your code is compliant with the subset.
4. If it fails, do one of the following:

    1. Remove the non-compliant function(s) and rewrite the code to maintain the semantics.
    2. Contact Ferrous Systems to discuss the option of adding your desired function(s) the certified subset.
    3. Keep using the function(s), but prove safety yourself. See :ref:`core/subset:Use uncertified core functions`.

.. list-table::
   :align: left
   :header-rows: 1

   * - Compilation target
     - Certified equivalent
   * - | ``aarch64-unknown-none``
       | ``aarch64-unknown-nto-qnx710``
     - ``aarch64-unknown-ferrocene.certified``
   * - ``thumbv7em-none-eabi``
     - ``thumbv7em-ferrocene.certified-eabi``
   * - ``thumbv7em-none-eabihf``
     - ``thumbv7em-ferrocene.certified-eabihf``
   * - | ``x86_64-unknown-linux-gnu``
       | ``x86_64-pc-nto-qnx710``
     - ``x86_64-unknown-ferrocene.certified``

Use uncertified core functions
------------------------------

For functions in the certified subset of the core library, Ferrous Systems proves the safety to use it in all contexts that are in scope of the certification.

For functions outside of the certified subset this safety is not proven by Ferrous Systems.

Nevertheless using those functions is still possible for customers if they prove the safety themselves. Often this is even more feasible because only the exact usage of the function has to be proven safe and not all possible uses.
