.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbv8m.main-none-eabihf:

:target:`thumbv8m.main-none-eabihf`
=================================================

The ``thumbv8m.main-none-eabihf`` Ferrocene target provides support for
bare-metal ARMv8-M Mainline processors with the
`T32 ISA <https://developer.arm.com/Architectures/T32%20Instruction%20Set%20Architecture>`_,
using the *hard-float ABI* with a single precision FPU. This includes the Arm
Cortex-M33, the Arm Cortex-M35P, the Arm Cortex-M55 and the Arm Cortex-M85. In the
case of the Arm Cortex-M55 and Arm Cortex-M85, only single-precision operations will
be executed on the FPU despite the fact that the FPU supports double-precision operations.

On this target, functions accepting ``f32`` or ``f64`` will have those
arguments passed via FPU registers. For more information on the
differences between the *hard-float* and *soft-float* ABIs, see the
`rustc book <../../rustc/platform-support/arm-none-eabi.html#instruction-sets>`_.

For the *soft-float* ABI, :ref:`thumbv8m.main-none-eabi` is also available.

By default, this target will *not* emit DSP or MVE instructions and it does not
support using those instructions in inline assembly.

Please contact support if your use case requires DSP and/or MVE instructions;
double-precision FPU support; and/or fully optimized builds for a
specific CPU configuration.

Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-thumbv8m.main-none-eabihf``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=thumbv8m.main-none-eabihf``

Testing Facade (Experimental)
-----------------------------

Bare metal targets cannot use test harnesses that require ``libc``. Instead, a testing facade that emulates the bare metal target can
be used instead, but also provides access to ``std``
functionality.

The following additional archive is needed when :doc:`installing </rustc/install>`:

* ``rust-std-thumbv8m.main-ferrocene.facade-eabihf``

This target is the same as the one it proxies, except it includes a Linux ``libc``,
which means it can use ``std`` for testing and enriched interactive development on a
:target:`aarch64-unknown-linux-gnu` or :ref:`x86_64-unknown-linux-gnu` host.

For more information, consult :doc:`Testing Facades </rustc/testing-facades>`.
