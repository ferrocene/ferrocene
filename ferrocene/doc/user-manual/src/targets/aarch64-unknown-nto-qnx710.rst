.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-unknown-nto-qnx710:

:target:`aarch64-unknown-nto-qnx710`
====================================

The ``aarch64-unknown-nto-qnx710`` Ferrocene target provides support for QNX on
ARMv8-A processors operating in Aarch64 mode.

.. note::
    
    QNX SDP only supports :ref:`x86_64-unknown-linux-gnu` and :ref:`x86_64-pc-windows-msvc` as host platforms.

    Currently, Ferrocene only qualifies cross compilation from :ref:`x86_64-unknown-linux-gnu`
    to :target:`aarch64-unknown-nto-qnx710`. :ref:`x86_64-pc-windows-msvc`
    support is experimental.
    
    QNX does not support :ref:`aarch64-apple-darwin` as a host platform. QNX is
    deprecating support for :target:`x86_64-apple-darwin` as a host platform.

Prerequisites
-------------

This target requires `QNX Software Development Platform 7.1.0 (QNX SDP 7.1.0)
<https://blackberry.qnx.com/en/products/foundation-software/qnx-software-development-platform/sdp-7-1>`_
to be installed.

Typically this is done through `QNX Software Center
<https://www.qnx.com/download/group.html?programid=29178>`_.

Ferrocene is qualified using a specific version QNX SDP version. In safety
critical contexts you must ensure 7.1.0.00472T202006132107S (also known as
7.1 BuildID 472) is used.

.. code-block::

    QNX_VERSION="7.1.0.00472T202006132107S"

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -installBaseline com.qnx.qnx710/$QNX_VERSION \
        -destination qnx/qnx710-472 \
        -cleanInstall

In an existing QNX 7.1.0 install you can check for the presence of the
``.packages/metadata/com.qnx.qnx710/7.1.0.00472T202006132107S/`` directory.

Ferrocene documents how our internal QNX toolchains are installed and
configured in :doc:`internal procedures (QNX) <internal-procedures:partners/qnx>`.
Your organizational and licensing needs may differ.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-aarch64-unknown-nto-qnx710``

Required shell environment
------------------------------

To use the target, the following procedures must be undertaken in the shell
running the build.

You must ensure ``$HOME`` is set to a valid path, then source ``qnxsdp-env.sh``
from your QNX SDP 7.1.0 installation:

.. code-block::

    source $QNX_SDP_710_INSTALL/qnxsdp-env.sh


On :ref:`x86_64-pc-windows-msvc`, there exists a ``qnxsdp-env.bat`` if
required. Ferrocene is internally tested using ``bash.exe`` provided by
`Git for Windows <https://www.git-scm.com/download/win>`_.


Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-unknown-nto-qnx710``
