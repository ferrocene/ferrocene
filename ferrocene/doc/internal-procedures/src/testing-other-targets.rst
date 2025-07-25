.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Testing Other Targets 
=====================

It's often possible to test targets other than your host tuple.

Windows and macOS hosts can test Linux hosts using `Lima <https://lima-vm.io/>`_ (macOS/Linux) or
`WSL2 <https://learn.microsoft.com/en-us/windows/wsl/install>`_ (Windows). While Lima supports other
instruction set architectures, WSL2 users must rely on QEMU.

Additionally, Ferrocene supports testing a number of targets which are not supported by upstream.
When testing locally, special tools or configuration may be required.

In general, any "bare-metal" target listed in :doc:`user-manual:targets/index` requires special
setup inside a Linux based environment, native or one supporting nested virtualization (such as
Lima or WSL2.)

Host Setup
----------

Unless otherwise noted, all bare-metal targets are tested via QEMU on a Linux host.
On macOS, a tool like Lima or Docker must be used. On Windows, WSL2 must be used.

:target-with-tuple:`x86_64-unknown-linux-gnu`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You need to have all the normal prerequisites from :doc:`internal-procedures:setup-local-env`
installed, as well as a few extras:

.. code-block:: bash

   sudo apt install qemu-user-static binfmt-support


:target-with-tuple:`aarch64-apple-darwin`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Install Lima, if you don't have it:

.. code-block:: bash

    brew install lima

You can create a guest with the following:

.. code-block:: yaml

    cat <<- EOF | limactl create --name=ferrocene
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
    EOF

Start the guest:

.. code-block:: bash
    
    limactl start ferrocene


Shell into the guest:

.. code-block:: bash
    
    limactl shell ferrocene

You can also point `Visual Studio Code's SSH extension <https://code.visualstudio.com/docs/remote/ssh>`_ at it
using `these steps <https://github.com/lima-vm/lima/discussions/1890#discussioncomment-7221563>`_.

Finally, ensure the guest is configured according to :doc:`internal-procedures:setup-local-env` as well as the :target-with-tuple:`x86_64-unknown-linux-gnu` on this page.

.. Warning::
    
    It is recommended to not share ``build/`` directories between multiple hosts, both for performance and correctness. To avoid this,
    you should ``cd ~`` in the guest and clone a new copy of the Ferrocene repository into the dedicated guest storage.

    Please ensure you always work from the guest-local repository.

:target-with-tuple:`x86_64-pc-windows-msvc`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Setup WSL2, if you don't have it:

.. code-block:: bash

    wsl --install --distribution Ubuntu-24.04

Ensure ```nestedVirtualization`` is set in the guest ``/etc/wsl.conf``, here is an example
configuration:

.. code-block::

    [user]
    default=ana

    [boot]
    systemd=true

    [wsl2]
    nestedVirtualization=true
    
If you changed your configuration, make sure to restart the environment with ``wsl --shutdown``.

Shell into the guest:

.. code-block:: bash
    
    wsl
    
You can also point `Visual Studio Code WSL extension <https://code.visualstudio.com/docs/remote/wsl-tutorial>`_ at it.

Finally, ensure the guest is configured according to :doc:`internal-procedures:setup-local-env` as well as the :target-with-tuple:`x86_64-unknown-linux-gnu` on this page.

.. Warning::
    
    It is recommended to not share ``build/`` directories between multiple hosts, both for performance and correctness. To avoid this,
    you should ``cd ~`` in the guest and clone a new copy of the Ferrocene repository into the dedicated guest storage.

    Please ensure you always work from the guest-local repository.

Target Procedures
-----------------

Currently bare metal targets have a similar procedure for testing.

.. note::

   Currently, these targets use our *secret sauce*.
   This will eventually be an open source component, but for now, it's our little bit of arcane magic.

:target-with-tuple:`aarch64-unknown-none`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. Warning::
    
    In a :target:`aarch64-unknown-linux-gnu` environment -- such as a guest on
    :target:`aarch64-apple-darwin` -- you **must** skip to the final step, running the tests using::
    
        export QEMU_CPU=cortex-a53
        ./x test --stage 1 --target aarch64-unknown-ferrocenecoretest library/core

    Incorrectly configuring your :target:`aarch64-unknown-linux-gnu` environment using the other steps 
    will damage to the environment and result in "Too many levels of symbolic links" errors.

Install the necessary packages:

.. code-block:: bash

    sudo apt install g++-aarch64-linux-gnu gcc-aarch64-linux-gnu binutils-aarch64-linux-gnu libc6-dev-arm64-cross qemu-system-aarch64

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

In order to avoid build errors such as "``--fix-cortex-a53-843419`` is only supported on AArch64
targets," ensure the following is in your ``config.toml``:

.. code-block:: bash

    [target.aarch64-unknown-ferrocenecoretest]
    cc = "aarch64-linux-gnu-gcc"
    profiler = false

After, you can run the tests:

.. code-block:: bash

    export QEMU_CPU=cortex-a53
    ./x test --stage 1 --target aarch64-unknown-ferrocenecoretest library/core

:target-with-tuple:`thumbv7em-none-eabihf` & :target-with-tuple:`thumbv7em-none-eabi`
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

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

Now set the target:

.. code-block:: bash

    export TARGET="thumbv7em-ferrocenecoretest-eabihf"
    # or 
    export TARGET="thumbv7em-ferrocenecoretest-eabi"

In order to test this target, the build process will acquire a copy of our *secret sauce* from AWS. Ensure you're authenticated, following the section in
:doc:`internal-procedures:setup-local-env` if your environment is not yet set up.

Ensure the following is in your ``config.toml``:

.. code-block:: toml

    [target.thumbv7em-ferrocenecoretest-eabi]
    cc = 'arm-none-eabi-gcc'
    profiler = false

    [target.thumbv7em-ferrocenecoretest-eabihf]
    cc = 'arm-none-eabi-gcc'
    profiler = false


You can now run the tests:

.. code-block:: bash

    export QEMU_CPU=cortex-m4
    ./x test --stage 1 --target $TARGET library/core
