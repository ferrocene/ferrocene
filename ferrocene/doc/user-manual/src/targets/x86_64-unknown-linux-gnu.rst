.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-unknown-linux-gnu:

:target:`x86_64-unknown-linux-gnu`
==================================

The ``x86_64-unknown-linux-gnu`` Ferrocene target provides support for Linux on
x86_64 using glibc 2.27 or higher.

Prerequisites
-------------

This target uses the LLVM ``ld.lld`` linker but to locate the system C libraries
required to link a functional Linux binary, this target drives the ``ld.lld``
linker using your system's C compiler as a linker driver.

You must have a C compiler which:

- Supports ``-fuse-ld=ld.lld`` option to select ``ld.lld`` as the linker

- Supports ``-B`` option to modify the tool search path so it can find Ferrocene's
  copy of ``ld.lld``

- Does not load plugins into the linker (e.g. the GCC LTO plugin)

- Supplies to ``ld.lld`` only those linker arguments specified in the
  :doc:`Safety Manual <safety-manual:options>`

On Ubuntu 18.04 LTS you can install a suitable C compiler with:

.. code-block::

   $ sudo apt install gcc

Archives to install
-------------------

The following archives are needed when :doc:`installing <../install>` this
target as a host platform:

* ``rustc-x86_64-unknown-linux-gnu``
* ``rust-std-x86_64-unknown-linux-gnu``
* ``ferrocene-self-test-x86_64-unknown-linux-gnu``

The following archives are needed when :doc:`installing <../install>` this
target as a cross-compilation target:

* ``rust-std-x86_64-unknown-linux-gnu``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

- ``--target=x86_64-unknown-linux-gnu``

- ``-C linker=<your-c-compiler>``

  - e.g. ``-C linker=/usr/bin/gcc``

If your C compiler loads the GCC LTO plugins by default, you will also need to
switch off GCC LTO with:

- ``-Clink-arg=-fno-lto``
