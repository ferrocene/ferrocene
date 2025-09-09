.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-apple-darwin:

:target:`aarch64-apple-darwin`
================================

.. note::
   
   Quality managed targets are targets which are suitable for use in production 
   outside of safety critical contexts. Support is available. The Ferrocene test
   suite successfully executed on the target. Known problems are tracked and
   available.

The ``aarch64-apple-darwin`` Ferrocene target provides support for macOS (Darwin) on
Apple Silicon.

Prerequisites
-------------

This target uses the LLVM ``ld.lld`` linker. In order to locate the system C
libraries required to link a functional macOS binary, this target drives the
``ld.lld`` linker using your system's C compiler as a linker driver.

Install one of:

* `Xcode command line tools`, or
* `Xcode <https://developer.apple.com/xcode/resources/>`_

.. note::

   If using `Xcode`, you will require an `Apple Developer Program <https://developer.apple.com/programs/>`_ membership.

`Xcode command line tools` can be installed via ``xcode-select``:

.. code-block::

    xcode-select --install


Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a host platform:

* ``rustc-aarch64-apple-darwin``
* ``rust-std-aarch64-apple-darwin``
* ``ferrocene-self-test-aarch64-apple-darwin``

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-aarch64-apple-darwin``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

- ``--target aarch64-apple-darwin``
