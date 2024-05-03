.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

Tool Options
============

Ferrocene Options
-----------------

Ferrocene is qualified exclusively for the following command line options:

- Users shall pass command line option ``--edition 2021`` to each invocation of
  the Ferrocene compiler.

- Users shall pass command line option ``-C opt-level=2`` to each invocation of
  the Ferrocene compiler.

- Users shall pass all target-specific command line options, as listed in the
  page of the target in the :doc:`Compilation Targets
  <user-manual:targets/index>` section of the User Manual.

.. |Tool| replace:: Ferrocene
.. |Link| replace:: :doc:`user-manual:rustc/cli`
.. include:: /partials/options.rst
   :start-line: 6

.. _linker-options:

Linker Options
--------------

Ferrocene is qualified using the ``rust-lld`` linker supplied with
Ferrocene. The linker can either be driven directly by Ferrocene, or via a
system C compiler acting as a *linker driver*, depending on whether the target
requires access to system-specific C libraries to correctly link an executable
or shared library.

The :doc:`Compilation Targets <user-manual:targets/index>` section of the User
Manual specifies for each target whether the linker is used directly, or via a
system C compiler as a linker driver.

Where a C compiler is acting as a *linker driver*, the C compiler is given a
path to a binary called ``ld.lld`` to use as its linker. The ``ld.lld`` binary
is a small wrapper around ``rust-lld`` which puts ``rust-lld`` into a *GNU ld
compatible mode* and otherwise passes on any arguments given straight through to
``rust-lld``.

Any C compiler acting as a linker driver, must:

- Pass to the underlying linker any arguments which do not start with a `-`
  character and which end with a recognised file extension for Object Code
  (typically ``.o``) or a Static Library containing Object Code (typically
  ``.a``).

- Accept arguments of the form ``-Wl,<linker-arg>``, passing ``<linker-arg>`` to
  the underlying linker as an argument.

- Accept the ``-fuse-ld=lld`` argument, which must cause the C compiler to use
  ``lld`` as the linker instead of its default linker.

- Accept the ``-B <path>`` argument, which must cause the C compiler search
  the path ``<path>`` for the ``lld`` linker binary.

Regardless of whether a C compiler is used as a *linker driver* or not, only the
following arguments may be presented to the ``rust-lld`` linker for the linking
of Ferrocene executables and shared libraries:

.. list-table::
   :align: left
   :header-rows: 1

   * - Flag
     - Description

   * - ``<path>``
     - Input objects for linking

   * - ``-o <path>``
     - The *Output object name* option

   * - ``-L <path>``
     - The *Library Path* option

   * - ``-X`` or ``--discard-all``
     - The *Discard All* option

   * - ``-z <keyword>``
     - The *Keyword* option

   * - ``-l <somelibrary>`` or ``--library-path <somelibrary>``
     - The *Link* option

   * - ``-m <mode>``
     - The *Emulation* option

   * - ``-EL``
     - The *Little Endian* option

   * - ``--pie`` or ``--pic-executable``
     - The *PIC Executable* option

   * - ``--no-pie``
     - The *Non-PIC Executable* option

   * - ``-I <path>`` or ``--dynamic-linker <path>``
     - The *Dynamic Linker* option

   * - ``--sysroot``
     - The *SysRoot* option

   * - ``--build-id``
     - The *Build ID* option

   * - ``--eh-frame-header``
     - The *Exception Frame Header* option

   * - ``--hash-style <style>``
     - The *Hash Style* option

   * - ``--as-needed`` and ``--no-as-needed``
     - The *As Needed* options

   * - ``--push-state`` and ``--pop-state``
     - The *Push and Pop* options

   * - ``--fix-cortex-a53-843419``
     - The *Arm Cortex-A53 Errata fix* option

Alternative forms of the above options are acceptable:

- Using a single dash (``-``) instead of two dashes (``--``)

- Using the ``--option=value`` form instead of ``--option value`` form
