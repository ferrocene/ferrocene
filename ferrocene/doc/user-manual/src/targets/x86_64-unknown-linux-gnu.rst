.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

x86_64 Linux (glibc)
====================

The ``x86_64-unknown-linux-gnu`` Ferrocene target provides support for
Linux on x86_64 using glibc.

Prerequisites
-------------

This target requires the ``x86_64-linux-gnu-gcc`` compiler, and is qualified
for use with version 7.5.x, as shipped by Ubuntu 18.04 LTS.

On Ubuntu 18.04 LTS you can install it with:

.. code-block::

   $ sudo apt install gcc

Once installed, you can ensure the version is correct by running:

.. code-block::

   $ x86_64-linux-gnu-gcc --version

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=x86_64-unknown-linux-gnu``
* ``-Clinker=x86_64-linux-gnu-gcc``
* ``-Clinker-flavor=gcc``
