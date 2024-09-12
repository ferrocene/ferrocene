.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

``ferrocene-self-test`` tool
============================

.. req:: Purpose
   :id: REQ_LSGKACM

   The tool is used to ensure that the Ferrocene toolchain is installed correctly.

.. req:: Environment
   :id: REQ_THXJWWD

   The tool should be executed on any of the host platforms listed in
   :doc:`user-manual:targets/index`.

.. req:: Environment
   :id: REQ_3RG87TT

   The tool should exit with a failure when PATH environment variable does not exist.

.. req:: Environment
   :id: REQ_67EQEVF

   The tool should only execute with success if it was distributed with the same Ferrocene toolchain
   that it is checking.

.. req:: Execution
   :id: REQ_P2OXII1

   The tool should be executed on the command line.

.. req:: Command line interface
   :id: REQ_9A6740M

   The tool accepts no command line arguments,
   and quits with a fail exit status when they are present.

.. req:: Output
   :id: REQ_42QU5MK

   The tool displays the checks it performs,
   and quits with a fail exit status when any of the checks fails.

Checks for executables
----------------------

.. req::
   :id: REQ_0OAUY3U

   These executables should be checked:

   - ``rustc``
   - ``rustdoc``
   - ``cargo`` (if installed)

.. req::
   :id: REQ_N1VBW46
   :implements: REQ_0OAUY3U

   The executable should exist in the ``bin`` directory, relative to the root of the installation.

.. req::
   :id: REQ_R2UQ8D3
   :implements: REQ_0OAUY3U

   The executable should be a regular file.

.. req::
   :id: REQ_NUP1G0D
   :implements: REQ_0OAUY3U

   The executable should have read and execute permissions set for all users.

.. req::
   :id: REQ_GVLWOTQ
   :implements: REQ_0OAUY3U

   Check the behavior of ``--verbose --version`` command-line options.

.. req::
   :id: REQ_6OAFM70
   :implements: REQ_GVLWOTQ

   The executable should exit successfully.

.. req::
   :id: REQ_ABPRHHQ
   :implements: REQ_GVLWOTQ

   The output of the executable should be UTF-8 text.

.. req::
   :id: REQ_SL5USTK
   :implements: REQ_GVLWOTQ

   The output of the executable should have at least 3 key-value pairs,
   with these 3 keys: ``host``, ``commit-hash``, and ``release``:

   - ``host`` is the target triple of the platform that the executable is built for
   - ``commit-hash`` is an identifier for the Git commit from which the Ferrocene release was built
   - ``release`` is the name of the upstream release from which the Ferrocene release was based

Checks for targets
------------------

.. note::

   In the subsections below, we have these substitutions:

   - ``$target`` refers to any of the targets listed
   - ``$hash`` is a 16 character hexadecimal string

All targets
^^^^^^^^^^^

.. note::

   These checks are for all locally-installed targets listed in :doc:`user-manual:targets/index`.

.. req::
   :id: REQ_0640QY8

   Inside of ``lib/rustlib/$target/lib`` directory,
   relative to the root of the installation,
   these regular files should be checked:

   - ``libcore-$hash.rlib``
   - ``liballoc-$hash.rlib``

.. req::
   :id: REQ_XWYY918
   :implements: REQ_0640QY8

   The files should exist.

.. req::
   :id: REQ_TI55HCF
   :implements: REQ_0640QY8

   The files should not have duplicates, which can happen if the ``$hash`` is different.

Targets with ``std``
^^^^^^^^^^^^^^^^^^^^

.. note::

   These checks are for all locally-installed targets
   which are also marked as having Full standard library support in
   :doc:`user-manual:targets/index`.

.. req::
   :id: REQ_RUCUMJJ

   Inside of ``lib/rustlib/$target/lib`` directory,
   relative to the root of the installation,
   these regular files should be checked:

   - ``libstd-$hash.rlib``
   - ``libtest-$hash.rlib``
   - ``libproc_macro-$hash.rlib``

.. req::
   :id: REQ_GAPK9QF
   :implements: REQ_RUCUMJJ

   The files should exist.

.. req::
   :id: REQ_IJN9ZPU
   :implements: REQ_RUCUMJJ

   The files should not have duplicates, which can happen if the ``$hash`` is different.

Checks for linkers
------------------

.. note::

   These checks are for the host platform, which is where ``ferrocene-self-test`` is executed.

   ``$target`` refers to this host platform.

.. req::
   :id: REQ_QQDV24N

   Inside of ``lib/rustlib/$target/bin`` directory,
   relative to the root of the installation,
   should exist the regular file named ``rust-lld``.

.. req::
   :id: REQ_J42HAPX

   Inside of ``lib/rustlib/$target/bin/gcc-ld`` directory,
   relative to the root of the installation,
   should exist the regular file named ``ld.lld``,
   which is the linker wrapper.

Platforms that need a C compiler
^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

.. note::

   The following checks apply to these Host platforms:

   - :target:`x86_64-unknown-linux-gnu`
   - :target:`aarch64-unknown-linux-gnu`

.. req::
   :id: REQ_GR1AK1Q

   Search for a system C compiler in the ``PATH`` environment variable.

.. req::
   :id: REQ_FCE5QJ5

   Use the system C compiler to compile a sample program,
   and use the linker wrapper for the linking stage.

.. req::
   :id: REQ_5Q3NRL3

   Check that the system C compiler passes ``-Wl,$arg`` arguments to the linker,
   where ``$arg`` is command line arguments that the system linker accepts.

.. req::
   :id: REQ_1MN4JOQ

   Ensure that the linker command line arguments that can be accepted are of
   :ref:`the form documented in the Safety Manual <linker-options>`.

Checks for compilation
----------------------

All targets
^^^^^^^^^^^

.. note::

   These checks are for all locally-installed targets listed in :doc:`user-manual:targets/index`.

.. req::
   :id: REQ_99TXVWC

   Check if we can compile the following Rust crate types:

   - ``lib``
   - ``staticlib``
   - ``bin``

.. req::
   :id: REQ_SV3CV3N

   Check that *only* the following artefacts are produced by ``rustc`` for each crate type compilation,
   where ``$basename`` is the file name without the extension:

   - ``$basename.rlib`` for ``lib`` crate type
   - ``$basename.a`` for ``staticlib`` crate type
   - ``$basename`` for ``bin`` crate type

Host targets
^^^^^^^^^^^^

.. note::

   These checks are for the host platform, which is where ``ferrocene-self-test`` is executed.

.. req::
   :id: REQ_8TNOYG8

   Check if a sample program that ``rustc`` produced can be executed.

.. req::
   :id: REQ_B07M5S2

   Check if the output of the program is as expected.
