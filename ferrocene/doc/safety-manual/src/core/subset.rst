.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Certified subset
================

The `Certified core library API docs <../../certification/api-docs/core/index.html>`_ is the authoritative document stating which items are included in the certified subset of the core library.

Compliance with subset
----------------------

To prove compliance with the subset, follow these steps:

1. Enable the lint at the root of each relevant crate: ``#![warn(ferrocene::unvalidated)]``
2. Compile the code with `-D warnings` enabled.
3. If it succeeds, your code is compliant with the subset.
4. If it fails, do one of the following:

   - If the unvalidated function is in your own crate, add ``#[ferrocene::prevalidated]``
     above it.

   - If the unvalidated function is in ``core``, contact Ferrous Systems to discuss the option of adding your desired function to the certified subset.

   - Remove the call to the non-compliant function and rewrite the code to maintain the semantics.

   - Keep using the function, but prove safety yourself. See :ref:`core/subset:Use uncertified core functions`. Once you have proven the safety of the call, add ``#[allow(ferrocene::unvalidated)]`` directly above the call.

See :ref:`scope:Qualification scope` for a list of targets which have a pre-certified core
library subset.

Use uncertified core functions
------------------------------

For functions in the certified subset of the core library, Ferrous Systems proves the safety to use it in all contexts that are in scope of the certification.

For functions outside of the certified subset, this safety is not proven by Ferrous Systems.

Nevertheless, using those functions is still possible for customers if they prove the safety themselves.
This could be more feasible because only the exact usage of the function has to be proven safe, and not all possible uses.
