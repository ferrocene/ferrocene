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

Installing Python
^^^^^^^^^^^^^^^^^

Ferrocene's CI pins Python 3.12.x currently. In general, any modern Python
should be sufficient for local CI imitation.

On Linux, install Python with your distribution's package manager, for example:

.. code-block:: bash

   apt install python

On macOS, Python is probably already installed (if you have a working XCode toolchain or
XCode Developer Tools installed). To install XCode Developer Tools:

.. code-block:: bash
   
   xcode-select --install


.. warning::
   
   Avoid installing Python or ``pip`` via Homebrew, because it can lead to long term
   system dysfunction. If you have already done so, consider removing them.

On Windows, get Python from the
`website <https://www.python.org/downloads/windows/>`_, or use `winget
<https://learn.microsoft.com/en-us/windows/package-manager/winget/>`_:

.. code-block:: powershell

   winget install Python.Python.3.12


Using the Python virtual environment
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

Ferrocene uses ``uv`` to manage Python dependencies. There are a number of
ways to install `listed in the repository <https://github.com/astral-sh/uv>`_,
but we recommend the ``curl`` (Linux, MacOS) or ``powershell`` (Windows)
methods:

.. code-block:: bash

   curl -LsSf https://astral.sh/uv/install.sh | sh

.. code-block:: powershell

   powershell -c "irm https://astral.sh/uv/install.ps1 | iex"

Follow the onscreen instructions post-install to ensure ``uv`` is present on your
``$PATH``.

Set up the virtual environment:

.. code-block:: bash

   uv venv
   source .venv/bin/activate
   uv pip sync requirements.txt

You need to source the virtual enviroment each time you wish to use it:

.. code-block:: bash

   source .venv/bin/activate


Using the CI ``config.toml``
^^^^^^^^^^^^^^^^^^^^^^^^^^^^

To create the ``config.toml`` used by the CI, set ``FERROCENE_HOST`` to your host triple,
then run the ``configure.sh``:

.. code-block:: bash

   CI=true FERROCENE_HOST=<your_host_triple> ./ferrocene/ci/configure.sh 


Using the CI Docker images
^^^^^^^^^^^^^^^^^^^^^^^^^^

Docker images used in CI can be found in ``ferrocene/ci/docker-images``. Each
image documents how to build it at the top of the ``Dockerfile``.

To build the most common image, ``ubuntu-20``:

.. code-block:: bash

   cd $FERROCENE_REPO
   docker build -t ubuntu-20 -f ferrocene/ci/docker-images/ubuntu-20/Dockerfile .


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

Jobs can only be reproduced on the host triple specified in ``FERROCENE_HOST``.
Ensure your host is correct.

If ``FERROCENE_TARGETS`` is different than your host triple,
update your ``config.toml``'s ``[build]`` section's ``host`` and ``target`` to
reflect that.

For Mac or Windows jobs, you can coarsely reproduce the CI by running the
lines in ``SCRIPT``, line by line if desired.

For Linux jobs, before running the container, make sure you are logged 
in to ``aws`` with ``aws sso login --profile ferrocene-ci``.
Then enter the Docker container specified by the ``executor`` line. Mount the 
ferrocene directory as well as your identifier for ``aws``. 

.. code-block:: bash

   docker run --rm --tty --interactive --workdir /ferrocene \
      --mount "type=bind,src=$(pwd),dst=/ferrocene" \
      --mount "type=bind,src=$(HOME/.aws/),dst=/home/ci/.aws" \
      ubuntu-20 bash

Inside the container, run:

.. code-block:: bash

   aws sts get-caller-identity --profile ferrocene-ci
   
.. note::

   If you wish to preserve your ``build/`` artifacts, it may make sense to
   re-clone the Ferrocene repository inside the container.

Inside the container, run ``./x clean`` then run the lines of the ``SCRIPT``
of the job:

.. code-block:: bash

   ./x clean
   ./x --stage 2 dist rust-std


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
