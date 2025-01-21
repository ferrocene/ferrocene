.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Other Targets 
=====================

It's often possible to test targets other than your host tuple. Windows and macOS hosts can target
Linux using `Lima <https://lima-vm.io/>`_ (Mac/Linux) or
`WSL2 <https://learn.microsoft.com/en-us/windows/wsl/install>`_ (Windows). Lima can be used to test
other architectures such as :ref:`x86_64-unknown-linux-gnu`, :target:`aarch64-unknown-linux-gnu`, or
:target:`riscv64gc-unknown-linux-gnu`, while WSL2 can do similar with `QEMU <https://www.qemu.org/>`_.

Additionally, Ferrocene supports testing a number of targets which are not supported by upstream.
When testing locally, special tools or configuration may be required.

In general, any "bare-metal" target listed in :doc:`user-manual:targets/index` requires special
setup inside a Linux based environment.

Host Setup
----------

Unless otherwise noted, all bare-metal targets are tested in QEMU on a Linux host.
On macOS, a tool like Lima or Docker must be used. On Windows, WSL2 must be used.

Please refer to the relevant per-host setup instructions below.

:ref:`x86_64-unknown-linux-gnu`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You need to have all the normal prerequisites for testing Rust installed, as well as a few extras:

.. code-block:: bash

   sudo apt install awscli ninja-build bzip2 cmake gcc g++ qemu-user-static binfmt-support


:ref:`aarch64-apple-darwin`
^^^^^^^^^^^^^^^^^^^^^^^^^^^

You need to use Lima. It's recommended to use the 'default' template and modify it as shown below.

.. note::

   The guest needs to have at more than the default of 4GiB of memory to not trigger the OOM
   killer during build. 8GiB is recommended.

Here is an sample config for your ``~/.lima/default/lima.yaml``:

.. code-block:: yaml

    minimumLimaVersion: "1.0.0"
    images:
    - location: "https://cloud-images.ubuntu.com/releases/24.10/release-20241212/ubuntu-24.10-server-cloudimg-arm64.img"
      arch: "aarch64"
      digest: "sha256:fb39312ffd2b47b97eaef6ff197912eaa3e0a215eb3eecfbf2a24acd96ee1125"
    - location: "https://cloud-images.ubuntu.com/releases/24.10/release/ubuntu-24.10-server-cloudimg-arm64.img"
      arch: "aarch64"
    mounts:
    - location: "~"
    - location: "/tmp/lima"
      writable: true
    ssh:
      forwardAgent: true
    cpus: 8
    memory: "8GiB"
    mountTypesUnsupported: ["9p"]

If you changed your configuration, make sure to restart the environment with ``limactl stop default`` then ``limactl start default``.

Enter the shell with ``limactl shell default``.

:ref:`x86_64-pc-windows-msvc`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You need to use WSL2. It's recommended to use the latest stable Ubuntu.

Here is a sample configuration for your ``/etc/wsl.conf`` (in the guest):

.. code-block::

    [user]
    default=ana

    [boot]
    systemd=true

    [wsl2]
    nestedVirtualization=true
    
If you changed your configuration, make sure to restart the environment with ``wsl --shutdown``.

Enter the shell with ``wsl``, or `point Visual Studio Code <https://code.visualstudio.com/docs/remote/wsl-tutorial>`_ at it.

Target Procedures
-----------------

In general, bare-metal targets follow a similar strategy: Use `binfmt_misc` to run the
test target binaries.

:ref:`aarch64-unknown-none`
^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. Note::
    
    In a ``aarch64-unknown-linux-gnu`` environment you may simply skip to the final step,
    no QEMU is needed.

Install the necessary packages:

.. code-block:: bash

    sudo apt install gcc-aarch64-linux-gnu qemu-system-aarch64

If you don't already have a ``/usr/share/binfmts/qemu-aarch64`` file, create one:

.. code-block:: bash

    package qemu-aarch64
    interpreter /usr/bin/qemu-aarch64-static
    magic \x7fELF\x02\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\xb7\x00
    mask \xff\xff\xff\xff\xff\xff\xff\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff
    credentials no
    preserve no
    fix_binary no
    
Then make sure it's imported:

.. code-block:: bash
    
   sudo update-binfmts --import qemu-aarch64

You can now run the tests:

.. code-block:: bash

    ./x test --stage 1 --target aarch64-unknown-ferrocenecoretest library/core

:target:`thumbv7em-none-eabihf` & :target:`thumbv7em-none-eabi`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Install the necessary packages:

.. code-block:: bash

    sudo apt install gcc-arm-none-eabi qemu-system-arm

If you don't already have a ``/usr/share/binfmts/qemu-arm`` file, create one:

.. code-block:: bash

    package qemu-arm
    interpreter /usr/bin/qemu-arm-static
    magic \x7fELF\x01\x01\x01\x00\x00\x00\x00\x00\x00\x00\x00\x00\x02\x00\x28\x00
    mask \xff\xff\xff\xff\xff\xff\xff\x00\xff\xff\xff\xff\xff\xff\xff\xff\xfe\xff\xff\xff
    credentials no
    preserve no
    fix_binary no
    
Then make sure it's imported:

.. code-block:: bash
    
   sudo update-binfmts --import qemu-arm

Currently, this target uses our *secret sauce*. Eventually this will be an open source component of Ferrocene, but for now, it's our little bit of arcane magic.

First, set the target:

.. code-block:: bash

    export TARGET="thumbv7em-ferrocenecoretest-eabihf"
    # or 
    export TARGET="thumbv7em-ferrocenecoretest-eabi"

Next, it's possible to build the *secret sauce*, or to download it. Generally, it's easier to download it, but if necessary you can find the repository in the `Ferrocene <https://github.com/ferrocene>`_ organization.

Refer to the ``.circleci/workflows.yml`` file on the ``setup-secret-sauce`` command to see which date/hash of the sauce to download.

.. code-block:: bash

    export SAUCE_DATE=20250121
    export SAUCE_HASH=1671dac
    
    mkdir -p /tmp/ferrocene/$TARGET
    aws s3 cp s3://ferrocene-ci-mirrors/coretest-secret-sauce/$SAUCE_DATE/$SAUCE_HASH/$TARGET.tar.gz /tmp/ferrocene/
    tar xf /tmp/ferrocene/$TARGET.tar.gz --directory=/tmp/ferrocene/$TARGET

You can now run the tests:

.. code-block:: bash

    export QEMU_CPU=cortex-m4
    ./x test --stage 1 --target $TARGET library/core
