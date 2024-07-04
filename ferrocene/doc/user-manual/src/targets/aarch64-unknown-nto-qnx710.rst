.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-unknown-nto-qnx710:

:target:`aarch64-unknown-nto-qnx710`
==============================

The ``aarch64-unknown-nto-qnx710`` Ferrocene target provides support for QNX on
ARMv8-A processors operating in Aarch64 mode.

.. note::
    
    QNX only supports :ref:`x86_64-unknown-linux-gnu` and
    :ref:`x86_64-pc-windows-msvc` as host platforms.
    
    QNX does not support :ref:`aarch64-apple-darwin` as a host platform. QNX is
    deprecating support for :target:`x86_64-apple-darwin` as a host platform.

Prerequisites
-------------

This target uses `QNX Software Development Platform 7.1.0 (QNX SDP 7.1.0)
<https://blackberry.qnx.com/en/products/foundation-software/qnx-software-development-platform/sdp-7-1>`_.



Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-aarch64-unknown-nto-qnx710``

Required shell environment
------------------------------

To use the target, the following procedures must be undertaken in the shell
running the build.

You must source ``qnxsdp-env.sh`` from your QNX SDP 7.1.0 installation:

.. code-block::

    source $QNX_SDP_710_INSTALL/qnxsdp-env.sh

    CC_aarch64-unknown-nto-qnx710=qcc
    CFLAGS_aarch64-unknown-nto-qnx710=-Vgcc_ntoaarch64le_cxx
    CXX_aarch64-unknown-nto-qnx710=qcc
    AR_aarch64_unknown_nto_qnx710=ntoaarch64-ar

On :ref:`x86_64-pc-windows-msvc`, there exists a ``qnxsdp-env.bat`` if
required. Ferrocene is internally tested using ``bash.exe`` provided by
`Git for Windows <https://www.git-scm.com/download/win>`_.


Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-unknown-nto-qnx710``
