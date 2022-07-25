.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: Critical Section GmbH

================================
Ferrocene Language Specification
================================

.. raw:: html

   <p align="center"><a href="https://spec.ferrocene.dev">Read the
   specification &raquo;</a></p>

The Ferrocene Language Specification (FLS) is a document describing the Rust
language. It was created by a joint effort between `Ferrous Systems`_ and
`AdaCore`_ as one of the prerequisites for qualifying `Ferrocene`_, a Rust
toolchain qualified for safety-critical environments. The FLS is compiled of
existing Rust documentation, but presented with a rigorous structure in order
to meet the requirements of qualification.

The FLS is not intended to be used as the normative specification of the Rust
language nor is it's meant to replace the decision-making processes of the Rust
project. Any difference between the FLS and the behavior of the rustc compiler
is considered an error on our part and the FLS will be updated accordingly.

The Ferrocene Language Specification text is licensed under either the ``MIT``
or ``Apache-2.0`` licenses, at your option. Individual files might have
different licensing. Licensing metadata is present in each file, and the full
licenses text is present in the ``LICENSES/`` directory.

.. _Ferrous Systems: https://ferrous-systems.com
.. _AdaCore: https://adacore.com
.. _Ferrocene: https://ferrocene.dev

Building the specification
==========================

FLS uses `Sphinx`_ to build a rendered version of the specification. To
simplify building the rendered version, we created a script called ``make.py``
that takes care of installing the expected Sphinx release and invoking it with
the right flags.

You can build the rendered version by running::

   ./make.py

By default, Sphinx uses incremental rebuilds to generate the content that
changed since the last invocation. If you notice a problem with incremental
rebuilds, you can pass the ``-c`` flag to clear the existing artifacts before
building::

   ./make.py -c

The rendered version will be available in ``build/html/``.

You can also start a local server on port 8000 with automatic rebuild and
reload whenever you change a file by passing the ``-s`` flag::

   ./make.py -s

Checking links consistency
==========================

It's possible to run Rust's linkchecker tool on the rendered documentation, to
see if there are broken links. To do so, pass the ``--check-links`` flag::

   ./make.py --check-links

This will clone the source code of the tool, build it, and execute it on the
rendered documentation.

.. _Sphinx: https://www.sphinx-doc.org

Updating build dependencies
===========================

The FLS uses ``pip-tools`` to manage the Python dependencies used for builds,
as it allows pinning hashes for the dependencies. While it doesn't add any
additional burden when installing dependencies (the format it outputs is
understood by `pip`), you have to install it when regenerating the
``requirements.txt`` file.

To install `pip-tools`, we recommend first installing `pipx`_, and then
running::

   pipx install pip-tools

Once that's done, you can change the list of desired dependencies in the
``requirements.in`` file, and run this command to regenerate the
``requirements.txt`` file::

   pip-compile --generate-hashes

.. _pipx: https://pypa.github.io/pipx/
