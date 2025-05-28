.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbv7em-none-eabihf:

:target:`thumbv7em-none-eabihf`
===============================

.. note::
   
   Currently the baseline target is qualified, without any specific
   ``-Ctarget-cpu`` or ``-Ctarget-feature`` flags mentioned in `the rustc book
   <https://doc.rust-lang.org/1.86/rustc/platform-support/thumbv7em-none-eabi.html#target-cpu-and-target-feature-options>`_.

   Please contact support if your use case demands fully optimized builds for
   a specific CPU configuration, or double-precision FPU support.


The ``thumbv7em-none-eabihf`` Ferrocene target provides support for
bare-metal ARMv7E-M processors with the 
`T32 ISA <https://developer.arm.com/Architectures/T32%20Instruction%20Set%20Architecture>`_,
using the *hard-float ABI* with a single precision FPU. This includes the Arm
Cortex-M4F and the single-precision variant of the Arm Cortex-M7F. The
double-precision variant of the Arm Cortex-M7F is also supported but only
single-precision operations will be executed on the FPU.

On this target, functions accepting ``f32`` or ``f64`` will have those
arguments passed via FPU registers. For more information on the
differences between the *hard-float* and *soft-float* ABIs, see the
`rustc book <https://doc.rust-lang.org/1.86/rustc/platform-support/arm-none-eabi.html#instruction-sets>`_.

For the *soft-float* ABI, :ref:`thumbv7em-none-eabi` is also available.


Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-thumbv7em-none-eabihf``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=thumbv7em-none-eabihf``
