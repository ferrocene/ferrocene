.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

QNX
===

One of the partner projects of Ferrocene is `QNX
<https://blackberry.qnx.com/>`_, an embedded OS designed for automotive use.
This page contains the information relevant to Ferrous Systems employees
working on Ferrocene.

Setting up a development toolchain
----------------------------------

.. note::
    These instructions are are intended to be run on a x86_64 Linux host only.
    `WSL2 <https://learn.microsoft.com/en-us/windows/wsl/install>`_ or `Lima
    <https://github.com/lima-vm/lima>`_ work sufficiently if needed.

    Afterward, you may move copy your ``qnx`` folder to the host Windows or
    Mac installation.

    QNX Software Center is also available for Windows, but setup may have
    different steps which we have not tested. QNX Software Center is not
    available for Mac.



Download the `QNX Software Center (for Linux Hosts)
<https://www.qnx.com/download/group.html?programid=29178>`_ and place it in
``qnx/qnx-software-center.run``.

.. code-block::

    chmod +x qnx/qnx-software-center.run
    qnx/qnx-software-center.run --tar xvf -C qnx


Then, install QNX 7.1.0:

.. code-block::
    
    LICENSE_KEY="FILL_ME_IN"
    QNX_USER="FILL_ME_IN"
    QNX_PASSWORD="FILL_ME_IN"

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -activateLicenseKey $LICENSE_KEY
    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -mirrorBaseline qnx710
    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -installBaseline com.qnx.qnx710 \
        -destination qnx/qnx710 \
        -cleanInstall

To optionally add the Mac host toolchain:

.. code-block::

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -installPackage com.qnx.qnx710.host.macosx.x86_64 \
        -destination qnx/qnx710

To optionally add the Windows host toolchain:

.. code-block::

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -installPackage com.qnx.qnx710.host.win.x86_64 \
        -destination qnx/qnx710

The Linux host toolchain should already be installed, but if you are using a
different host platform to install, optionally add the Linx host toolchain: 

.. code-block::

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -installPackage com.qnx.qnx710.host.linux.x86_64 \
        -destination qnx/qnx710


To activate your QNX toolchain:

.. code-block::

    source qnx/qnx710/qnxsdp-env.sh


If required, you can then archive the installation and your licenses, then move it to the host:

    tar -cv -I 'zstd -T0' -f qnx/qnx710.tar.xz -C qnx/ qnx710



Creating a CI/CD deployment
---------------------------

.. note::
    These instructions are are intended to be run on a x86_64 Linux host only.
    `WSL2 <https://learn.microsoft.com/en-us/windows/wsl/install>`_ or `Lima
    <https://github.com/lima-vm/lima>`_ work sufficiently if needed.

QNX artifacts are built using a 'deployment' of QNX which contains a license
provided by QNX for our CI/CD.

.. warning::
    This license may not be used for individual development. Ferrocene also
    has several individual licenses available for internally for developers, to
    obtain one, ask your manager.

To create the deployment, first, ensure you have a `myQNX
<https://www.qnx.com/account/index.html>`_ account and that a
**QNX Software Development Platform Subscription - Build Server License**
is deployed to it.

Download the `QNX Software Center (for Linux Hosts)
<https://www.qnx.com/download/group.html?programid=29178>`_ and place it in
``qnx/qnx-software-center.run``.

Install the QNX Software Center:

.. code-block::

    chmod +x qnx/qnx-software-center.run
    qnx/qnx-software-center.run --tar xvf -C qnx


Create a deployment containing Linux, Mac, and Windows toolchains:

.. code-block::
    
    LICENSE_KEY="FILL_ME_IN"
    QNX_USER="FILL_ME_IN"
    QNX_PASSWORD="FILL_ME_IN"

    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -activateLicenseKey $LICENSE_KEY
    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -mirrorBaseline qnx710
    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -installBaseline com.qnx.qnx710 \
        -installPackage com.qnx.qnx710.host.macosx.x86_64 \
        -installPackage com.qnx.qnx710.host.win.x86_64 \
        -installPackage com.qnx.qnx710.host.linux.x86_64 \
        -destination qnx/qnx710 \
        -cleanInstall
    qnx/qnxsoftwarecenter/qnxsoftwarecenter_clt \
        -myqnx.user $QNX_USER -myqnx.password $QNX_PASSWORD \
        -deploySdpInstallation qnx/qnx710 \
        -deployLicense $LICENSE_KEY \
        -installationDeployAs qnx/qnx710-deployment

Finally, create an archive of the deployment and upload it to the S3 URL which the CI attempts to pull from:

.. code-block::

    tar -cv -I 'zstd -T0' -f qnx/qnx710-deployment.tar.xz -C qnx/qnx710-deployment/ qnx710
    aws s3 cp qnx/qnx710-deployment.tar.xz s3://ferrocene-ci-mirrors/manual/qnx/qnx710-deployment.tar.xz