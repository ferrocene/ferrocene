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

- Users shall pass command line option
  ``-C llvm-args=-protect-from-escaped-allocas=true`` to each invocation of the
  Ferrocene compiler.

- Users shall pass all target-specific command line options, as listed in the
  page of the target in the :doc:`Compilation Targets
  <user-manual:targets/index>` section of the User Manual.

These CLI options must be utilized for each invocation of the Ferrocene
toolset to remain within the qualified scope.

For convenience, :doc:`user-manual:cli` provides further details on CLI options
available to the end user.

CLI options that impact code generation and can create unsafe conditions are
tagged with a caution note. A CLI option or one of its arguments may be outside
the scope of the Ferrocene |iso_ref| qualification, in which case the CLI
option or argument must not be used in a safety critical context. For
convenience to the end user, these limitations are provided directly in the CLI
caution note when applicable.

Other CLI options which are not marked with a caution note, for example those
used for debugging and testing, can be used since their behavior is clearly
defined and tested by the Ferrocene testsuite.

If the end user requires CLI options other than those identified in this
chapter, they must verify that the feature exists and is compatible with their
target, that their safety context is compatible with the use of the CLI option or
feature, and develop appropriate verification activities and tests to ensure
the correct functionality of the generated code.

Linker Options
--------------

Ferrocene is qualified using the ``rust-lld`` linker supplied with
Ferrocene. The linker can either be driven directly by Ferrocene, or via a
system C compiler acting as a *linker driver*, depending on whether the target
requires access to system-specific C libraries to correctly link an executable
or shared library.

The :doc:`Compilation Targets <user-manual:targets/index>` section of the User
Manual specifies for each target whether the linker is used directly, or via a
system C compiler as a *linker-driver*.

Where a C compiler is acting as a *linker driver*, the C compiler is given a
path to a binary called ``ld.lld`` to use as its linker. The ``ld.lld`` binary
is a small wrapper around ``rust-lld`` which puts ``rust-lld`` into a *GNU ld
compatible mode* and otherwise passes on any arguments given straight through to
``rust-lld``.

Any C compiler acting as a *linker driver*, must:

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

Regardless of whether a C compiler is used as a *linker-driver* or not, only the
following arguments may be presented to the ``rust-lld`` linker for the linking
of Ferrocene executables and shared libraries:

- Input objects for linking

- The *Output object name* option, ``-o <path>``

- The *Library Path* option, ``-L <path>``

- The *Discard All* option ``-X`` or ``--discard-all``

- The *Keyword* option, ``-z <keyword>``

- The *Link* option, ``-l <somelibrary>`` or ``--library-path <somelibrary>``

- The *Emulation* option, ``-m <mode>``

- The *Little Endian* option, ``-EL``

- The *PIC Executable* option, ``--pie`` or ``--pic-executable``

- The *Non-PIC Executable* option, ``--no-pie``

- The *Dynamic Linker* option, ``-I <path>`` or ``--dynamic-linker <path>``

- The *SysRoot* option, ``--sysroot``

- The *Build ID* option, ``--build-id``

- The *Exception Frame Header* option, ``--eh-frame-header``

- The *Hash Style* option, ``--hash-style <style>``

- The *As Needed* options, ``--as-needed`` and ``--no-as-needed``

- The *Push and Pop* options, ``--push-state`` and ``--pop-state``

- The *Arm Cortex-A53 Errata fix* option, ``--fix-cortex-a53-843419``

Alternative forms of the above options are acceptable:

- Using a single dash (``-``) instead of two dashes (``--``)

- Using the ``--option=value`` form instead of ``--option value`` form
