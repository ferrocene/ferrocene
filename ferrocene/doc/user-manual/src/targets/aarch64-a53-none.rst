.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-a53-none:

:target:`aarch64-a53-none`
==============================

.. note::

    This target is similar to the baseline :ref:`aarch64-a53-none`
    target, except the target CPU is set to ``cortex-a53`` as if
    ``-C target-cpu=cortex-a53`` was passed.

The ``aarch64-a53-none`` Ferrocene target provides support for
bare-metal ARMv8-A Cortex-a53 processors operating in Aarch64 mode.

Prerequisites
-------------

This target has no pre-requisites.

Archives to install
-------------------

The following archives are needed when :doc:`installing </rustc/install>` this
target as a cross-compilation target:

* ``rust-std-aarch64-a53-none``

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-a53-none``

Testing Facade (Experimental)
-----------------------------

Bare metal targets cannot use test harnesses that require ``libc``. Instead, a testing facade that emulates the bare metal target can
be used instead, but also provides access to ``std``
functionality.

The following additional archive is needed when :doc:`installing </rustc/install>`:

* ``rust-std-aarch64-a53-ferrocene.facade``

This target is the same as the one it proxies, except it includes a Linux ``libc``,
which means it can use ``std`` for testing and enriched interactive development on a
:target:`aarch64-unknown-linux-gnu` or :ref:`x86_64-unknown-linux-gnu` host.

For more information, consult :doc:`Testing Facades </rustc/testing-facades>`.

.. _aarch64-ferrocene.a53-none:

Certified equivalent
--------------------

This :ref:`qualified target<qualified-targets>`'s certified equivalent is
``aarch64-ferrocene.a53-none``. To use the certified core library, the
following additional flags must be provided to ``rustc``:

* ``--target=aarch64-ferrocene.a53-none``

Refer to :ref:`certified-core-targets` for more information about certified targets.
