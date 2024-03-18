.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Installing Ferrocene
====================

This chapter describes how to install and validate the Ferrocene toolchain.

Before proceeding, you should identify the :doc:`targets <targets/index>` you
want to install. You must pick the target of the host platform you're going to
install Ferrocene on, and optionally one or more cross-compilation targets
(depending on the hardware you'll deploy the executable or library on).

Prerequisites
-------------

The prerequisites of Ferrocene depend on the targets you are going to install.
For each of the targets you identified above, follow the "prerequisites"
section of the target documentation.

Installation
------------

.. note::

   We are developing a tool similar to ``rustup``, able to manage your
   Ferrocene installations and install Ferrocene without relying on a web
   browser. It will be available soon.

Manual installation
~~~~~~~~~~~~~~~~~~~

Ferrocene installation archives are available for manual download at
`releases.ferrocene.dev <https://releases.ferrocene.dev>`_. The website
requires authentication through the customer portal, and doesn't allow
programmatic access.

On the channel list, select the channel you want to view the latest release
published in it (and its files). For each of the targets you identified above,
download all the archives listed in the "installation archives" section of the
target documentation.

You must extract all downloaded archives in the **same** directory to finish
the Ferrocene installation. Ferrocene binaries will then be available in the
``bin`` subdirectory inside of the extraction location.

For example, to install Ferrocene in ``/opt/ferrocene`` you can run:

.. code-block::

   sudo tar -C /opt/ferrocene -xf path/to/first-archive.tar.xz
   sudo tar -C /opt/ferrocene -xf path/to/second-archive.tar.xz
   # ...repeat for all the downloaded archives

To make Ferrocene available system-wide, consult the manual of your operating
system on how to add the ``bin`` sub-directory into the system ``$PATH``.

.. note::

   Ferrocene can be installed anywhere on the system, including inside of a
   user's home directory. It is portable, can be moved to different
   storage locations after its installation, and you can have multiple
   installations of Ferrocene in different directories.

.. caution::

   Ferrocene does **not** support in-place upgrades: you cannot install
   Ferrocene inside a directory that contains another Ferrocene installation.
   To upgrade Ferrocene, you must remove all the contents of the existing
   directory before extracting the new Ferrocene version.

   Because of this, we do not recommend installing Ferrocene in shared
   directories like ``/usr/local``, as removing old Ferrocene versions would
   prove tricky.

Validation
----------

.. caution::

   The procedure described below relies on a tool that is not qualified, and
   thus can't be used in a safety critical environment: in that case, you must
   follow the instructions in :qualification:id:`CSTR_0010_INSTALL`.

The Ferrocene toolchain contains a validation binary called
``ferrocene-self-test``, useful to verify the installation of the Ferrocene
toolchain. The tool will analyze the installation and the surrounding
environment, and note if known installation problems are detected. To execute
the tool, run:

.. code-block::

   PATH_TO_INSTALLATION_DIRECTORY/bin/ferrocene-self-test

All the performed checks will be displayed. If any check fails, an error will
be emitted along with an ID. You can look up the identifier in :doc:`the error
codes list <self-test/error-codes>` to learn more about the failure and ways to
fix it.
