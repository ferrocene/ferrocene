.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

ARMv8-A bare metal
==================

The ``aarch64-unknown-none`` Ferrocene target provides support for
bare-metal ARMv8-A.

Prerequisites
-------------

This target requires the ``aarch64-linux-gnu-gcc`` compiler, and is qualified
for use with version 7.5.x, as shipped by Ubuntu 18.04 LTS.

On Ubuntu 18.04 LTS you can install it with:

.. code-block::

   $ sudo apt install gcc-aarch64-linux-gnu

Once installed, you can check the version is correct by running:

.. code-block::

   $ aarch64-linux-gnu-gcc --version

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-unknown-none``
* ``-Clinker=aarch64-linux-gnu-gcc``
* ``-Clinker-flavor=gcc``
* ``-Clink-arg=-ffreestanding``
* ``-Clink-arg=-nostdlib``
