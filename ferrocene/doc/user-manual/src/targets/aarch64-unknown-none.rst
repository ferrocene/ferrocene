.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-unknown-none:

:target:`aarch64-unknown-none`
==============================

The ``aarch64-unknown-none`` Ferrocene target provides support for
bare-metal ARMv8-A processors operating in Aarch64 mode.

Prerequisites
-------------

This target has no pre-requisites.

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-unknown-none``
