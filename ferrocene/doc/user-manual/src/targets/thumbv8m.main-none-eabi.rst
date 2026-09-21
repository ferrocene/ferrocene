.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbv8m.main-none-eabi:

:target:`thumbv8m.main-none-eabi`
===============================================

The ``thumbv8m.main-none-eabi`` Ferrocene target provides support for
bare-metal ARMv8-M.mainnline processors with the
`T32 ISA <https://developer.arm.com/Architectures/T32%20Instruction%20Set%20Architecture>`_,
using the *soft-float ABI*. This includes the Arm Cortex-M33, Arm Cortex-M35P, Cortex-M55 and
Cortex-M85.

On this target, functions accepting ``f32`` or ``f64`` will have those
arguments packed into integer registers. For more information on the
differences between the *hard-float* and *soft-float* ABIs, see the
`rustc book <../../rustc/platform-support/arm-none-eabi.html#instruction-sets>`_.

For the *hard-float* ABI, :ref:`thumbv8m.main-none-eabihf` is also available.

By default, this target will *not* emit DSP or MVE instructions and it does not
support using those instructions in inline assembly.

Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-thumbv8m.main-none-eabi``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=thumbv8m.main-none-eabi``

Testing Facade (Experimental)
-----------------------------

Bare metal targets cannot use test harnesses that require ``libc``. Instead, a testing facade that emulates the bare metal target can
be used instead, but also provides access to ``std``
functionality.

The following additional archive is needed when :doc:`installing </rustc/install>`:

* ``rust-std-thumbv8m.main-ferrocene.facade-eabi``

This target is the same as the one it proxies, except it includes a Linux ``libc``,
which means it can use ``std`` for testing and enriched interactive development on a
:target:`aarch64-unknown-linux-gnu` or :ref:`x86_64-unknown-linux-gnu` host.

For more information, consult :doc:`Testing Facades </rustc/testing-facades>`.
