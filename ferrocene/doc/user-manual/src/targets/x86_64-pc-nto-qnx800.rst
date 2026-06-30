.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _x86_64-pc-nto-qnx800:

:target:`x86_64-pc-nto-qnx800`
==============================

The ``x86_64-pc-nto-qnx800`` Ferrocene target provides support for QNX on
x86-64 processors.

.. note::

    QNX SDP 8.0.0 only supports :ref:`x86_64-unknown-linux-gnu` and
    :ref:`x86_64-pc-windows-msvc` as host platforms.

    Currently, Ferrocene only qualifies cross compilation to this target from
    :ref:`x86_64-unknown-linux-gnu`. Cross-compilation from
    :ref:`x86_64-pc-windows-msvc` is experimental and cross-compilation from
    :ref:`aarch64-apple-darwin` is unsupported.

Prerequisites
-------------

This target requires `QNX Software Development Platform 8.0.0 (QNX SDP 8.0.0)
<https://blackberry.qnx.com/en/products/foundation-software/qnx-software-development-platform>`_
to be installed.

Typically this is done through `QNX Software Center
<https://www.qnx.com/download/group.html?programid=29178>`_.

Ferrocene is qualified using a specific version QNX SDP version. In safety
critical contexts you must ensure 8.0.0.00141T202311271501L (also known as
8.0 BuildID 141) is used.

.. code-block::

    QNX_VERSION="8.0.0.00141T202311271501L"

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -installBaseline com.qnx.qnx800/$QNX_VERSION \
        -destination qnx/qnx800-141 \
        -cleanInstall

In an existing QNX 8.0.0 install you can check for the presence of the
``.packages/metadata/com.qnx.qnx800/8.0.0.00141T202311271501L/`` directory.

Ferrocene documents how our internal QNX toolchains are installed and
configured in :doc:`internal procedures (QNX) <internal-procedures:partners/qnx>`.
Your organizational and licensing needs may differ.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-x86_64-pc-nto-qnx800``

Required shell environment
------------------------------

To use the target, the following procedures must be undertaken in the shell
running the build.

You must ensure ``$HOME`` is set to a valid path, then source ``qnxsdp-env.sh``
from your QNX SDP 8.0.0 installation:

.. code-block::

    source $QNX_SDP_800_INSTALL/qnxsdp-env.sh


On :ref:`x86_64-pc-windows-msvc`, there exists a ``qnxsdp-env.bat`` if
required. Ferrocene is internally tested using ``bash.exe`` provided by
`Git for Windows <https://www.git-scm.com/download/win>`_.


Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=x86_64-pc-nto-qnx800``
