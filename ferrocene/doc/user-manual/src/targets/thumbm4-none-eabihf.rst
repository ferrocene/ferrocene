.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbm4-none-eabihf:

:target:`thumbm4-none-eabihf`
=============================

.. note::

    This target is similar to the baseline :ref:`thumbv7em-none-eabihf`
    target, except the target CPU is set to ``cortex-m4`` as if
    ``-C target-cpu=cortex-m4`` was passed.


The ``thumbm4-none-eabihf`` Ferrocene target provides support for
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

For the *soft-float* ABI with the target CPU set to ``generic``, :ref:`thumbv7em-none-eabi`
is also available.


Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-thumbm4-none-eabihf``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=thumbm4-none-eabihf``

Testing Facade (Experimental)
-----------------------------

Bare metal targets cannot use test harnesses that require ``libc``. Instead, a testing facade that emulates the bare metal target can
be used instead, but also provides access to ``std``
functionality.

The following additional archive is needed when :doc:`installing </rustc/install>`:

* ``rust-std-thumbm4-ferrocene.facade-eabihf``

This target is the same as the one it proxies, except it includes a Linux ``libc``,
which means it can use ``std`` for testing and enriched interactive development on a
:target:`aarch64-unknown-linux-gnu` or :ref:`x86_64-unknown-linux-gnu` host.

For more information, consult :doc:`Testing Facades </rustc/testing-facades>`.

.. _thumbm4-ferrocene-none-eabihf:

Certified equivalent
--------------------

This :ref:`qualified <qualified-targets>` target's certified equivalent is
``thumbm4-ferrocene-none-eabihf``. To use the certified core library, the
following additional flags must be provided to ``rustc``:

* ``--target=thumbm4-ferrocene-none-eabihf``

Refer to :ref:`certified-core-targets` for more information about certified targets.
