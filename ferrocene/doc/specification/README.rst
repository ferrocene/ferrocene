.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

================================
Ferrocene Language Specification
================================

.. raw:: html

   <p align="center"><a href="https://spec.ferrocene.dev">Read the
   specification &raquo;</a></p>

The Ferrocene Language Specification (FLS) is a document describing the Rust
language. It was created as one of the prerequisites for qualifying
`Ferrocene`_, a Rust toolchain qualified for safety-critical environments. The
FLS is compiled of existing Rust documentation, but presented with a rigorous
structure in order to meet the requirements of qualification.

The FLS is not intended to be used as the normative specification of the Rust
language, nor is it meant to replace the decision-making processes of the Rust
project. Any difference between the FLS and the behavior of the Rust compiler
is considered an error on our part and the FLS will be updated accordingly.

The Ferrocene Language Specification text is licensed under either the ``MIT``
or ``Apache-2.0`` licenses, at your option. Individual files might have
different licensing. Licensing metadata is present in each file, and the full
licenses text is present in the ``LICENSES/`` directory.

.. _Ferrocene: https://ferrocene.dev

Building the specification
==========================

FLS uses `Sphinx`_ to build a rendered version of the specification, and `uv`_ to install and manage
Python dependencies (including Sphinx itself). To simplify building the rendered version, we created
a script called ``make.py`` that takes care of invoking Sphinx with the right flags.

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
.. _uv: https://docs.astral.sh/uv/

Updating build dependencies
===========================

The FLS uses ``uv`` to manage the Python dependencies used for builds. If you change the list of
dependencies in ``pyproject.toml`` they will automatically be installed the next time you run
``make.py``. If you want to update the packages in the lockfile, run::

   uv lock --upgrade
