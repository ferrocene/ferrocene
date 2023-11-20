.. SPDX-License-Identifier: MIT OR Apache-2.0
   SPDX-FileCopyrightText: The Ferrocene Developers

.. _aarch64-unknown-none:

ARMv8-A bare metal
==================

The ``aarch64-unknown-none`` Ferrocene target provides support for
bare-metal ARMv8-A processors operating in Aarch64 mode.

Prerequisites
-------------

This target uses the LLVM ``rust-lld`` linker and is qualified for use with the
version of ``rust-lld`` bundled with Ferrocene.

Required compiler flags
-----------------------

To use the target, the following additional flags must be provided to
``rustc``:

* ``--target=aarch64-unknown-none``

Mitigations in the linker for Arm Cortex-A53 Errata 843419 are automatically
enabled.
