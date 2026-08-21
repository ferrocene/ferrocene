.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Cross-compiling
===============

Building
--------

This requires less setup than a full test environment (below), but doesn't allow running
binaries for the target.

Toolchain Setup
^^^^^^^^^^^^^^^

You need a compiler for the target.
For a ``x86_64-unknown-linux-gnu`` target and a MacOS host, you can install a cross toolchain:

.. code-block:: bash

   brew install messense/macos-cross-toolchains/x86_64-unknown-linux-gnu

If you have ``x86_64-unknown-linux-gnu-binutils`` already installed, you may need to
also run ``brew link --overwrite x86_64-unknown-linux-gnu``.

Running a build
^^^^^^^^^^^^^^^

.. code-block:: bash

  ./x.py build --target x86_64-unknown-linux-gnu

Testing Other Targets
---------------------

It's often possible to test targets other than your host tuple.

For MacOS hosts, we use Apple's `container <https://github.com/apple/container>`_ tool, while for
Windows hosts we use `WSL2 <https://learn.microsoft.com/en-us/windows/wsl/install>`_. With these
tools we can set up a Linux environment using the same architecture as the host machine, which we
can then use to run tests for other targets using QEMU.

Additionally, Ferrocene supports testing a number of targets which are not supported by upstream.
When testing locally, special tools or configuration may be required.

In general, any "bare-metal" target listed in :doc:`user-manual:targets/index` requires special
setup inside a Linux based environment, native or one supporting nested virtualization (such as
Lima or WSL2.)

Host Setup
^^^^^^^^^^

Unless otherwise noted, all bare-metal targets are tested via QEMU on a Linux host. On macOS, a
Linux VM must be set up using a tool like Apple's ``container``. On Windows, WSL2 must be used.

:target-with-tuple:`aarch64-apple-darwin`
"""""""""""""""""""""""""""""""""""""""""

First, install ``container``, following the instructions on
`that project's Github page <https://github.com/apple/container>`_. This tool has a similar
command-line interface to Docker, but with some small differences.

.. Note::

    ``container`` layers a Docker-like environment on top of a Linux VM, rather than
    running on a VM directly. This means that you cannot use ``binfmt-misc`` like you can on
    bare-metal Linux or on WSL. Instead, you need to build a patched version of QEMU to handle
    proper inheritance of emulation across processes, which is included in the following steps.

Next, you will need to acquire a base image. We have a private repository of images at
``harbor.infra.ferrous-systems.net`` which are used for CI and so have most of what we need
pre-installed; here we use the ``ubuntu-24-main`` image from ``ferrocene-images/ci``

Log in to the registry:

.. code-block:: bash

    container registry login --username <email> --password-stdin harbor.infra.ferrous-systems.net
    # Paste the "CLI secret" for your harbor account. This can be found on the User Profile
    # page on harbor. Then press Enter then Ctrl-D (both are necessary)
    # Logging in may take a couple of minutes, but will print "Login succeeded" when it's done

Next, configure and create a container:

.. code-block:: bash

    container create -t --cpus 10 --memory 16g --name ferrocene-dev \
        harbor.infra.ferrous-systems.net/ferrocene-images/ci:ubuntu-24-main

Adjust the CPU and memory allocations as appropriate to your device, remembering to leave
some memory free for the host. The above numbers are selected for an M5 Macbook Air,
which has 24GB of total memory, split as 16GB for the VM + 8GB for the host.

If you use 1Password, and want to forward your SSH agent into the container,
run the following instead:

.. code-block:: bash

    container create -t --cpus 10 --memory 16g --name ferrocene-dev \
        -v $SSH_AUTH_SOCK:/tmp/ssh-auth.sock \
        -e SSH_AUTH_SOCK="/tmp/ssh-auth.sock" \
        harbor.infra.ferrous-systems.net/ferrocene-images/ci:ubuntu-24-main

Note: The ``-t`` argument is extremely important! Without it, the container will immediately exit
when you run ``container start ferrocene-dev`` later on

Now start the container and install some packages we will need:

.. code-block:: bash

    container start ferrocene-dev
    container exec -it --uid 0 ferrocene-dev /bin/bash
    # This will open a root shell in the container. In that shell, run:
    unminimize # This is a script from Ubuntu which expands the minimal environment used for CI
               # into a more user-friendly environment for development work
    apt install sudo less nano libglib2.0-dev flex bison
    # Then close the root shell

If not using the Ferrocene CI image, you will also need to install the packages listed for
:target-with-tuple:`x86_64-unknown-linux-gnu` on the :doc:`internal-procedures:setup-local-env`
page.

Push information we'll need for logging into things:

.. code-block:: bash

    container cp ~/.aws/config ferrocene-dev:/home/ci/.aws/config
    # If you did not forward the SSH agent above, upload your keys directly:
    container cp ~/.ssh/id_ed25519 ferrocene-dev:/home/ci/.ssh/id_ed25519
    container cp ~/.ssh/id_ed25519.pub ferrocene-dev:/home/ci/.ssh/id_ed25519.pub

Now open a normal user shell in the container to finish setup:

.. code-block:: bash

    container exec -it ferrocene-dev /bin/bash
    # In the shell this opens:

    # First fix the permissions of the files uploaded above
    sudo chown -R ci:ci ~/.aws
    # If you used 1Password:
    sudo chown ci:ci /tmp/ssh-auth.sock
    sudo chmod 0600 /tmp/ssh-auth.sock
    # If you uploaded your SSH keys:
    sudo chown -R ci:ci ~/.ssh
    chmod 0600 ~/.ssh/id_ed25519

    # Build our patched version of qemu
    cd ~
    git clone git@github.com:ferrocene/qemu.git -b ferrocene/release/11.0.1-patched --depth 1
    mkdir qemu-build
    cd qemu-build
    ../qemu/configure --disable-system --prefix=/opt/qemu-ferrocene
    ninja
    sudo ninja install

    # Login to AWS
    aws sso login --profile ferrocene-ci --use-device-code

    # Check out ferrocene
    cd ~
    git clone git@github.com:ferrocene/ferrocene.git
    cd ferrocene
    ferrocene/ci/scripts/setup-uv.sh
    git submodule update --init --recursive

    # Set up bootstrap.toml
    nano bootstrap.toml

    # and check that everything is set up properly
    ./x test bootstrap

Finally, to run cross-platform tests, the following steps need to be done *per target* inside
the container:

.. Note::

    These instructions must be used *instead of* the ones in the "Target Procedures" section below

Install the appropriate cross-toolchain. E.g. for x86-64 Linux, run:

.. code-block:: bash

    sudo apt install gcc-x86-64-linux-gnu g++-x86-64-linux-gnu

Then, to run the tests against the desired target, run:

.. code-block:: bash

    ./x build remote-test-server --target x86_64-unknown-linux-gnu
    QEMU_LD_PREFIX=/usr/x86_64-linux-gnu /opt/qemu-ferrocene/bin/qemu-x86_64 \
        build/aarch64-unknown-linux-gnu/stage2-tools-bin/remote-test-server -v \
        --bind 127.0.0.1:12345
    # This should print "starting test server" and then wait...

    # So open a new shell and run:
    container exec -it ferrocene-dev /bin/bash
    cd ferrocene
    TEST_DEVICE_ADDR=127.0.0.1:12345 ./x test library/core library/alloc \
        --target x86_64-unknown-linux-gnu

.. Note::

    The above commands are for the :target-with-tuple:`x86_64-unknown-linux-gnu` target.
    For other targets, you will need to change ``QEMU_LD_PREFIX``, the qemu executable,
    and the ``--target`` argument to the commands. The ``/build/aarch64-unknown-linux-gnu``
    part does *not* change, as this uses the tuple for the VM we are building from
    (which will always be aarch64 Linux in this section)

For targets which do not have an OS (generally with a ``-none`` somewhere in the target tuple)
the procedure is a little different:

.. code-block:: bash

    ./x build remote-test-server --target thumbv7em-ferrocene.facade-eabihf # Replace 'none' with 'ferrocene.facade'
    opt/qemu-ferrocene/bin/qemu-x86_64 \
        build/aarch64-unknown-linux-gnu/stage2-tools-bin/remote-test-server -v \
        --bind 127.0.0.1:12345
    # This should print "starting test server" and then wait...

    # So open a new shell and run:
    container exec -it ferrocene-dev /bin/bash
    cd ferrocene
    TEST_DEVICE_ADDR=127.0.0.1:12345 \
        RUSTDOCFLAGS="--cfg=ferrocene_facade_secretsauce -Z unstable-options --test-args '--exclude-should-panic'" \
        ./x test library/core library/alloc \
        --target thumbv7em-ferrocene.facade-eabihf

.. Note::

    When running a large number of tests, the remote test server seems to return occasional spurious
    errors. These can be distinguished from real test failures by the error message
    ``client.read_exact(&mut header) failed with Connection reset by peer (os error 104)``

    Some test failures can cause the remote test server to crash, which causes all future tests
    to be rejected. This can be detected by seeing tests fail with the error message
    ``TcpStream::connect(device_address) failed with Connection refused (os error 111)``

.. Note::

    The path to the remote-test-server binary does not depend on the target platform,
    so you will need to rerun ``./x build remote-test-server --target <target>`` each
    time you want to switch target

.. Warning::

    It is recommended to not share ``build/`` directories between multiple hosts, both for performance and correctness. To avoid this,
    you should ``cd ~`` in the guest and clone a new copy of the Ferrocene repository into the dedicated guest storage.

    Please ensure you always work from the guest-local repository.

:target-with-tuple:`x86_64-pc-windows-msvc`
"""""""""""""""""""""""""""""""""""""""""""

Setup WSL2, if you don't have it:

.. code-block:: bash

    wsl --install --distribution Ubuntu-24.04

Ensure ``nestedVirtualization`` is set in the guest ``/etc/wsl.conf``, here is an example
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

:target-with-tuple:`x86_64-unknown-linux-gnu`
"""""""""""""""""""""""""""""""""""""""""""""

You need to have all the normal prerequisites from :doc:`internal-procedures:setup-local-env`
installed, as well as a few extras:

.. code-block:: bash

   sudo apt install qemu-user-static binfmt-support

.. Note::

    These packages must also be installed in the VMs used on Windows.

Target Procedures
^^^^^^^^^^^^^^^^^

Currently bare metal targets have a similar procedure for testing.

.. note::

   Currently, these targets use our *secret sauce*.
   This will eventually be an open source component, but for now, it's our little bit of arcane magic.

:target-with-tuple:`aarch64-unknown-none`
"""""""""""""""""""""""""""""""""""""""""

.. Warning::

    In a :target:`aarch64-unknown-linux-gnu` environment -- such as a guest on
    :target:`aarch64-apple-darwin` -- you **must** skip to the final step, running the tests using::

        export QEMU_CPU=cortex-a53
        ./x test --stage 1 --target aarch64-unknown-ferrocene.facade library/core

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

    [target."aarch64-unknown-ferrocene.facade"]
    cc = "aarch64-linux-gnu-gcc"
    profiler = false

After, you can run the tests:

.. code-block:: bash

    export QEMU_CPU=cortex-a53
    ./x test --stage 1 --target aarch64-unknown-ferrocene.facade library/core

:target-with-tuple:`thumbv7em-none-eabihf` & :target-with-tuple:`thumbv7em-none-eabi`
"""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""""

Install the necessary packages:

.. code-block:: bash

    sudo apt install gcc-arm-none-eabi

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

    export TARGET="thumbv7em-ferrocene.facade-eabihf"
    # or
    export TARGET="thumbv7em-ferrocene.facade-eabi"

In order to test this target, the build process will acquire a copy of our *secret sauce* from AWS. Ensure you're authenticated, following the section in
:doc:`internal-procedures:setup-local-env` if your environment is not yet set up.

Ensure the following is in your ``config.toml``:

.. code-block:: toml

    [target."thumbv7em-ferrocene.facade-eabi"]
    cc = 'arm-none-eabi-gcc'
    profiler = false

    [target."thumbv7em-ferrocene.facade-eabihf"]
    cc = 'arm-none-eabi-gcc'
    profiler = false


You can now run the tests:

.. code-block:: bash

    export QEMU_CPU=cortex-m4
    ./x test --stage 1 --target $TARGET library/core
