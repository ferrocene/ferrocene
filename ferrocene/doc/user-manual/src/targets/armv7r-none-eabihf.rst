.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _armv7r-none-eabihf:

:target:`armv7r-none-eabihf`
=================================================

The ``armv7r-none-eabihf`` Ferrocene target provides support for
bare-metal ARMv7-R processors with the ARM ISA, using the *hard-float ABI* with
a double-precision capable FPU. This includes the Arm Cortex-R4, Cortex-R5,
Cortex-R7 and Cortex-R8 with FPU architecture VFPv3-D16. 

On this target, functions accepting ``f32`` or ``f64`` will have those
arguments passed via FPU registers. For more information on the
differences between the *hard-float* and *soft-float* ABIs, see the
`rustc book <../../rustc/platform-support/arm-none-eabi.html#instruction-sets>`_.


Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-armv7r-none-eabihf``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=armv7r-none-eabihf``

Testing Facade (Experimental)
-----------------------------

Bare metal targets cannot use test harnesses that require ``libc``. Instead, a testing facade that emulates the bare metal target can
be used instead, but also provides access to ``std``
functionality.

The following additional archive is needed when :doc:`installing </rustc/install>`:

* ``rust-std-armv7r-ferrocene.facade-eabihf``

This target is the same as the one it proxies, except it includes a Linux ``libc``,
which means it can use ``std`` for testing and enriched interactive development on a
:target:`aarch64-unknown-linux-gnu` or :ref:`x86_64-unknown-linux-gnu` host.

For more information, consult :doc:`Testing Facades </rustc/testing-facades>`.
