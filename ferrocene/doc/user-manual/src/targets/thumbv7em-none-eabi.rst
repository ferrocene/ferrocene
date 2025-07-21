.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbv7em-none-eabi:

:target:`thumbv7em-none-eabi`
===============================================

.. note::
   
   Currently the baseline target is qualified, without any specific
   ``-Ctarget-cpu`` or ``-Ctarget-feature`` flags mentioned in `the rustc book 
   <https://doc.rust-lang.org/1.86/rustc/platform-support/thumbv7em-none-eabi.html#target-cpu-and-target-feature-options>`_.

   Please contact support if your use case demands fully optimized builds for
   a specific CPU configuration.


The ``thumbv7em-none-eabi`` Ferrocene target provides support for
bare-metal ARMv7E-M processors with the 
`T32 ISA <https://developer.arm.com/Architectures/T32%20Instruction%20Set%20Architecture>`_,
using the *soft-float ABI*. This includes the Arm Cortex-M4 and Arm Cortex-M7.

On this target, functions accepting ``f32`` or ``f64`` will have those
arguments packed into integer registers. For more information on the
differences between the *hard-float* and *soft-float* ABIs, see the
`rustc book <https://doc.rust-lang.org/1.86/rustc/platform-support/arm-none-eabi.html#instruction-sets>`_.

For the *hard-float* ABI, :ref:`thumbv7em-none-eabihf` is also available.


Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-thumbv7em-none-eabi``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=thumbv7em-none-eabi``
