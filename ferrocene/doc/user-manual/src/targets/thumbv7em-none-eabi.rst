.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _thumbv7em-none-eabi:

:target:`thumbv7em-none-eabi`
==============================

The ``thumbv7em-none-eabi`` Ferrocene target provides support for
bare-metal ARMv7E-M processors using the 
`T32 ISA <https://developer.arm.com/Architectures/T32%20Instruction%20Set%20Architecture>`_
with soft floating point.

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
