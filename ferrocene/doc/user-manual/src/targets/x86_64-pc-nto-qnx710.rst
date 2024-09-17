.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-pc-nto-qnx710:

:target:`x86_64-pc-nto-qnx710`
==============================

The ``x86_64-pc-nto-qnx710`` Ferrocene target provides support for QNX on
x86-64 processors.

.. note::
    
    QNX SDP only supports :ref:`x86_64-unknown-linux-gnu` and :ref:`x86_64-pc-windows-msvc` as host platforms.

    Currently, Ferrocene only tests cross compiling from :ref:`x86_64-unknown-linux-gnu`
    to :target:`x86_64-pc-nto-qnx710`. Compiling from :ref:`x86_64-pc-windows-msvc`
    will be tested in the future.
    
    QNX does not support :ref:`aarch64-apple-darwin` as a host platform. QNX is
    deprecating support for :target:`x86_64-apple-darwin` as a host platform.

Prerequisites
-------------

This target requires `QNX Software Development Platform 7.1.0 (QNX SDP 7.1.0)
<https://blackberry.qnx.com/en/products/foundation-software/qnx-software-development-platform/sdp-7-1>`_
to be installed.

Typically this is done through `QNX Software Center
<https://www.qnx.com/download/group.html?programid=29178>`_.

Ferrocene documents how our internal QNX toolchains are installed and
configured in :doc:`internal procedures (QNX) <internal-procedures:partners/qnx>`.
Your organizational and licensing needs may differ.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-x86_64-pc-nto-qnx710``

Required shell environment
------------------------------

To use the target, the following procedures must be undertaken in the shell
running the build.

You must source ``qnxsdp-env.sh`` from your QNX SDP 7.1.0 installation:

.. code-block::

    source $QNX_SDP_710_INSTALL/qnxsdp-env.sh

    CC_x86_64-pc-nto-qnx710=qcc
    CFLAGS_x86_64-pc-nto-qnx710=-Vgcc_ntox86_64_cxx
    CXX_x86_64-pc-nto-qnx710=qcc
    AR_x86_64_pc_nto_qnx710=ntox86_64-ar

On :ref:`x86_64-pc-windows-msvc`, there exists a ``qnxsdp-env.bat`` if
required. Ferrocene is internally tested using ``bash.exe`` provided by
`Git for Windows <https://www.git-scm.com/download/win>`_.


Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=x86_64-pc-nto-qnx710``
