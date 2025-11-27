.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Working with the CI
===================

Ferrocene's Continuous Integration is primarily driven by a combination of
YAML, Docker, Python and Bash scripts.


Imitating the CI
----------------

Sometimes it is useful to imitate the CI to reproduce failures.

The Linux jobs on CI typically run in containers and can be almost exactly
reproduced. Mac and Windows jobs on CI cannot run in containers and are more
difficult to reproduce.

If you are presented the option, it's a good idea to reproduce Linux CI
failures first.


Installing Docker (with Buildkit)
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

You do not need to download or use `Docker Desktop
<https://www.docker.com/products/docker-desktop/>`_ with Ferrocene. If you
choose to use Docker Desktop, install it as directed on the Website and skip
the rest of this section.

On Linux, install Docker and ``docker-buildx`` from your package manager. On
Ubuntu 20.04 and up:

.. code-block:: bash

   apt install docker.io docker-buildx

On some distributions, you may need to start the service with
``systemctl enable --now docker``. If you just installed ``docker-buildx``,
restart the Docker service.

On MacOS, you can use the Docker CLI connected to a Lima host:

.. code-block:: bash

   brew install lima docker docker-buildx
   limactl start template://docker

After setting up the instance, Lima will inform you of how to configure Docker to
use it instead with Docker.

On Windows, it's generally recommended to do CI imitation within WSL2. Enable
WSL2 from an administrator Powershell if you haven't done so already, then setup
``ubuntu-24.04``.

.. code-block:: Powershell

   wsl --install
   wsl --install -d Ubuntu-24.04

From there, follow the Linux instructions above.

Reproducing CI precisely with ``act``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

The following shell script shows how to invoke `act <https://nektosact.com/introduction.html>`_
to run CI precisely as Github Actions would.
You may have to change the "event" type; see `the act manual <https://nektosact.com/usage/index.html#events>`_.

.. code-block:: sh

    branch=${1:-$(git branch --show-current)}
    workflow=${2:-"automation-pull-subtrees"}
    event=${3:-"--detect-event"}

    gh auth token | read GITHUB_TOKEN
    export GITHUB_TOKEN
    exec act --workflows .github/workflows/${workflow}.yml $event \
        --matrix branch:$branch \
        --actor ferrocene/ferrocene --env GITHUB_REF=refs/heads/${branch} \
        --env GITHUB_REPOSITORY=ferrocene/ferrocene --secret GITHUB_TOKEN \
        --platform ubuntu-24.04=catthehacker/ubuntu:act-24.04

If you're running on macOS Silicon, you may need to explicitly set ``--container-architecture=linux/amd64``.
See `the docker registry <https://github.com/catthehacker/docker_images/pkgs/container/ubuntu>`_ for a list of supported architectures.

If you want to build a local docker image and use that for running the workflow, you can do so with the following commands:

.. code-block:: sh

   docker build --file ferrocene/ci/docker-images/ubuntu-20/Dockerfile . --tag ferrocene-images/ci:ubuntu-20-main
   act --pull=false --platform ubuntu-24.04=localhost/ferrocene-images/ci:ubuntu-20-main # ...

The remaining sections of this document are about reproducing the *configuration* of CI using a local build.

Using Python scripts and tools
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Ferrocene uses ``uv`` to manage Python and its dependencies. ``uv`` can be
considered as a mix of ``rustup`` and ``cargo`` for Python, and all of our
Python usage depends on it.

`Installation instructions are available in the uv documentation
<https://docs.astral.sh/uv/getting-started/installation/>`_. You don't need to
install a recent Python separately, as ``uv`` manages its own installations of
Python.

Thanks to ``uv``, you can just invoke any script to execute it, without worrying
about dependencies::

   ferrocene/ci/scripts/detect-conflict-markers.py

For that to work, all of our Python scripts meant to be executed by CI must have
the following shebang:

.. code-block:: python

   #!/usr/bin/env -S uv run

This will instruct the OS to invoke the script using ``uv``, which will make
sure the correct Python version is used. It's also possible to use ``uv``'s
`script support <https://docs.astral.sh/uv/guides/scripts/>`_ to install
(isolated) dependencies whenever the script is executed, by adding the following
comment at the top of the Python file:

.. code-block:: python

    # /// script
    # requires-python = ">= 3.12"
    # dependencies = ["requests ~= 2.32"]
    # ///

When multiple scripts need to share code, please `setup a library project
<https://docs.astral.sh/uv/concepts/projects/init/#libraries>`_ (with its own
dependencies in the library's ``pyproject.toml``) and depend on it from the script:

.. code-block:: python

    # /// script
    # requires-python = ">= 3.12"
    # dependencies = ["local-package"]
    #
    # [tool.uv.sources]
    # local-package = { path = "relative/path/to/the/project", editable = true }
    # ///

.. note::

   When depending on local libraries, the name of the library must appear *both*
   in the ``dependencies`` list and in the ``[tool.uv.sources]`` table. Make
   sure to enable editable support in local libraries.

To run standalone tools written in Python (like REUSE), you can use ``uvx`` to
download and execute them::

   uvx reuse lint


Using the CI ``config.toml``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

To create the ``config.toml`` used by the CI, set ``FERROCENE_HOST`` to your host tuple,
then run the ``configure.sh``:

.. code-block:: bash

   CI=true FERROCENE_HOST=aarch64-unknown-linux-gnu ./ferrocene/ci/configure.sh


Using the CI Docker images
^^^^^^^^^^^^^^^^^^^^^^^^^^

Docker images used in CI can be found in ``ferrocene/ci/docker-images``. Each
image documents how to build it at the top of the ``Dockerfile``.

To build the most common image, ``ubuntu-20``:

.. code-block:: bash

   cd $FERROCENE_REPO
   docker build --tag ubuntu-20 --file ferrocene/ci/docker-images/ubuntu-20/Dockerfile .


Reproducing CI jobs
^^^^^^^^^^^^^^^^^^^

Most CI jobs are formatted similar to this:

.. code-block:: YAML

  x86_64-linux-dist-targets:
    executor: docker-ubuntu-20
    resource_class: large # 4-core
    environment:
      FERROCENE_HOST: x86_64-unknown-linux-gnu
      FERROCENE_TARGETS: << pipeline.parameters.targets--x86_64-unknown-linux-gnu--std-only >>
      SCRIPT: |
        ./x.py --stage 2 dist rust-std
    steps:
      - ferrocene-job-dist:
          restore-from-job: x86_64-linux-build

Jobs can only be reproduced on the host tuple specified in ``FERROCENE_HOST``.
Ensure your host is correct.

If ``FERROCENE_TARGETS`` is different than your host tuple,
update your ``config.toml``'s ``[build]`` section's ``host`` and ``target`` to
reflect that.

For Mac or Windows jobs, you can coarsely reproduce the CI by running the
lines in ``SCRIPT``, line by line if desired.

For Linux jobs, enter the Docker container specified by the ``executor`` line:

.. code-block:: bash

   docker run --rm --tty --interactive --workdir /ferrocene \
      --mount "type=bind,src=$(pwd),dst=/ferrocene" \
      ubuntu-20 bash

Inside the container, run ``./x clean`` then run the lines of the ``SCRIPT``
of the job:

.. code-block:: bash

   ./x clean
   ./x --stage 2 dist rust-std

Preserving build artifacts
~~~~~~~~~~~~~~~~~~~~~~~~~~

If you wish to preserve your ``build/`` artifacts, it may make sense to
re-clone the Ferrocene repository inside the container or mount a dedicated
directory for ``/ferrocene/build``:

.. code-block:: bash

   docker run --rm --tty --interactive --workdir /ferrocene \
      --mount "type=bind,src=$(pwd),dst=/ferrocene" \
      --mount "type=bind,src=$(pwd)/container-build,dst=/ferrocene/build" \
      ubuntu-20 bash

Note that you will need to create the `container-build` directory in the host
system beforehand.

Sharing miscellaneous directories with the container
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

If you need to share any extra directories from the host with the container you
can use a similar strategy as the one used to mount the ferrocene repository.

For example, if you want to share the host `~/.aws` directory configuration so
you can authenticate in the container, run:

.. code-block:: bash

   docker run --rm --tty --interactive --workdir /ferrocene \
      --mount "type=bind,src=$(pwd),dst=/ferrocene" \
      --mount "type=bind,src=$HOME/.aws,dst=/home/ci/.aws" \
      ubuntu-20 bash

Remember that any directories mounted with the `bind` type must exist in the
host system beforehand.

Making changes to the CI
------------------------

Effort should be made to avoid tying too tightly to CircleCI. Ferrocene is
likely to change CI providers in the future.

Non-trivial ``run:`` tasks in the CI should be made scripts in
``ferrocene/ci/scripts/``.


Tooling Pragmatism
^^^^^^^^^^^^^^^^^^

The tooling chosen for our CI is not necessarily borne out of preference, but
practicality. If you are adding new components to the CI, choose practical
and simple over fancy and interesting.

Ferrocene's CI should, as a rule, be unremarkable, boring, and reliable.

In general, Python scripts are preferred over Bash scripts, as Python is more
portable.

New scripts should be written in Python unless they are trivial and only run
on Linux/macOS jobs.

Before attempting to fix a bug in a Bash script, evaluate if it would make sense
to convert it to Python. During evaluation, check for these signals that a bash
script should be rewritten in Python:

* The script runs in Windows jobs at all
   * ``bash.exe`` on Windows is problematic and has been the source of a number of
     bugs.
* The script calls ``shasum`` or other hashing related functionality
   * There are tangible differences between Linux, macOS, and Windows' Bash
     implementations.
* The script deals with ``.tar`` files
   * Windows' ``tar`` implementations (both BSDtar and GNUtar) are problematic
     with symlinks and slow, while Python's implementation has proven more
     reliable.
