.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-pc-windows-msvc:

:target:`x86_64-pc-windows-msvc`
================================

The ``x86_64-pc-windows-msvc`` Ferrocene target provides support for Host as
well as Cross-compilation for Microsoft Windows on x86_64 using the native
ABI (MSVC).

Prerequisites
-------------

You need to install Microsoft Build Tools either via
`installing Visual Studio <https://visualstudio.microsoft.com/downloads/>`_
or installing the Build Tools directly via the same installer.

Alternatively, you can `install Build Tools via winget
<https://winstall.app/apps/Microsoft.VisualStudio.2022.BuildTools>`_.

.. code-block::

    winget install --id=Microsoft.VisualStudio.2022.BuildTools  -e

You can also check the `instructions for installing the Windows
MSVC tools for Rust
<https://rust-lang.github.io/rustup/installation/windows-msvc.html>`_.

.. note::
   For further support with our Windows installer please check the
   `documentation for Ferrocene toolchain manager - CriticalUp
   <https://criticalup.ferrocene.dev/install.html#windows>`_.