.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Installing Ferrocene
====================

This chapter describes how to install and validate the Ferrocene toolchain.

Prerequisites
-------------

Ferrocene has platform-specific prerequisites, that vary based on the
compilation targets you want to use. Follow the "Prerequisites" section of the
target page for each of the targets you plan to use.

Installation
------------

Ferrocene is shipped as multiple tarballs, and all of them need to be
installed. Refer to the list provided to you after the sale for the set of
tarballs you need to download and install.

Once each tarball is downloaded, extract it inside the directory you want the
Ferrocene toolchain to be installed (in the rest of this page denoted as
``path_to_install_dir``):

.. code-block::

   $ tar -C path_to_install_dir -xf tarball_name.tar.xz

Finally, if ``path_to_install_dir`` is not in your system ``$PATH``, consult the
manual of your operative system on how to add ``path_to_install_dir`` to your
``$PATH``.

Validation
----------

The Ferrocene contains a checker called ``ferrocene-self-test`` for verifying
the installation of the Ferrocene toolchain in a non-certification context.
The tool itself is not qualified. Consequently, in certification context,
manual check of the :doc:`Safety Manual Constraints
<safety-manual:constraints>` is necessary.

To execute the checker, run:

.. code-block::

   $ path_to_install_dir/bin/ferrocene-self-test

The Ferrocene self-test tool emits all the checks it performs to ``stderr``.

In case the Ferrocene toolchain was not properly installed, the Ferrocene
self-test tool should report an error, followed by an error code.

For detailed explanation of error codes, along with common causes and
suggestions, please consult the
:doc:`Ferrocene self-test error codes <user-manual:self-test/error-codes>`.
