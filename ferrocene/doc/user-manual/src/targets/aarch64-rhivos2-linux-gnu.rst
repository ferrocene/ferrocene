.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-rhivos2-linux-gnu:

:target:`aarch64-rhivos2-linux-gnu`
===================================
.. note::

   This is a variant of the generic :target:`aarch64-unknown-linux-gnu` target that specifically targets
   RHIVOS2 automotive linux. As per the RHIVOS2 guidelines, qualified use requires compilation on the matching
   host platform RedHat Enterprise Linux 10 using using the :ref:`aarch64-unknown-linux-gnu` host compiler.

The ``aarch64-rhivos2-linux-gnu`` Ferrocene target provides support for RedHat In Vehicle Operating System 2
(RHIVOS2) on aarch64 using glibc 2.31 or higher.

Prerequisites
-------------

While this target is technically a full linux and capable of hosting a compiler,
the target is only qualified if cross-compiled from RHEL 10 on aarch64 in the
version specified by the RHIVOS2 documentation. This requirement stems from the
RHIVOS assumptions of use. The host compiler is :target:`aarch64-unknown-linux-gnu`.

This target uses the LLVM ``ld.lld`` linker. To locate the system C libraries
required to create a functional Linux binary, this target drives the ``ld.lld``
linker using your system's C compiler as a linker driver.

You must have a C compiler which:

- Supports ``-fuse-ld=ld.lld`` option to select ``ld.lld`` as the linker

- Supports ``-B`` option to modify the tool search path so it can find Ferrocene's
  copy of ``ld.lld``

- Does not load plugins into the linker (e.g. the GCC LTO plugin)

- Supplies to ``ld.lld`` only those linker arguments specified in the
  :doc:`Safety Manual <safety-manual:rustc/options>`

Please refer to the RedHat Enterprise Linux 10 documentation for installation instructions.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-aarch64-rhivos2-linux-gnu``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

- ``--target=aarch64-rhivos2-linux-gnu``

- ``-C linker=<your-c-compiler>``

  - e.g. ``-C linker=/usr/bin/gcc``

If your C compiler loads the GCC LTO plugins by default, you will also need to
switch off GCC LTO with:

- ``-Clink-arg=-fno-lto``

.. _aarch64-ferrocene-linux-gnu:
