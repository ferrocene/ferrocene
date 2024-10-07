.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-pc-windows-msvc:

:target:`x86_64-pc-windows-msvc`
================================

.. warning::
   
   Experimental targets cannot be used in safety-critical contexts, and there is
   no guarantee that the Ferrocene test suite is successfully executed on the
   target. They are provided as a preview, with limited support available. They
   should not be used in production.

The ``x86_64-pc-windows-msvc`` Ferrocene target provides support Microsoft Windows on x86_64 using MSVC.

Prerequisites
-------------

This target uses Microsoft's ``Link.exe`` linker. It can be obtained from
`the Visual Studio installation page <https://visualstudio.microsoft.com/downloads/>`_.

.. note::

   Your organization may require a Visual Studio license from Microsoft. See the
   `Licensing Terms <https://visualstudio.microsoft.com/license-terms/>`_.

Install one of:

* The Visual Studio edition best suited for you, or
* ``Build Tools for Visual Studio 2022`` under ``Tools for Visual Studio``

``Build Tools for Visual Studio 2022`` can be installed via ``winget``:

.. code-block::

    winget install --id=Microsoft.VisualStudio.2022.BuildTools  -e

If prompted for which components to enable, enable ``Desktop
development with C++``, or select all of the following individual components:

* The most recent "C++ x64/x86 build tools"
* The most recent "Windows 11 SDK"

The specific versions of these components do not particularly matter to Ferrocene,
you may use whatever version your dependencies require.

Archives to install
-------------------


The following archives are needed when :doc:`installing </rustc/install>` this
target as a host platform:

* ``rustc-x86_64-pc-windows-msvc``
* ``rust-std-x86_64-pc-windows-msvc``
* ``ferrocene-self-test-x86_64-pc-windows-msvc``

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-x86_64-pc-windows-msvc``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

- ``--target=x86_64-pc-windows-msvc``
